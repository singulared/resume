//! Resume generator CLI.
//!
//! Collects all resume data via `inventory` and renders it to Markdown, HTML,
//! and PDF. Subcommands:
//!   * `generate` — render the resume (Markdown / HTML / PDF / all)
//!   * `list`     — print the data collected from inventory
//!   * `check`    — verify every section has data (non-zero exit if any is empty)
//!   * `stars`    — fetch GitHub star counts for the open-source repos and cache
//!     them; `generate --stars` reads the cache to show the counts

use anyhow::{bail, Context, Result};
use askama::Template;
use clap::{Args, Parser, Subcommand, ValueEnum};
use resume::WorkHistory;
use resume::resume_data::*;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use std::{env, fs, thread};

/// Askama template filters. `markdownify` renders Markdown doc-comment fields to
/// HTML (pair with `|safe`) and `md` prepares them for the Markdown view; both
/// drop rustdoc intra-doc links. `stars` shows the cached count loaded via
/// [`filters::set_stars`].
mod filters {
    use super::{format_count, repo_path};
    use askama::Result;
    use pulldown_cmark::{html, BrokenLink, CowStr, Event, Options, Parser, Tag, TagEnd};
    use std::collections::HashMap;
    use std::ops::Range;
    use std::sync::OnceLock;

    /// Star counts (keyed by `owner/repo`), loaded from the cache before rendering.
    static STARS: OnceLock<HashMap<String, u64>> = OnceLock::new();

    pub fn set_stars(stars: HashMap<String, u64>) {
        let _ = STARS.set(stars);
    }

    fn options() -> Options {
        Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES
    }

    /// Markdown doc-comment text to HTML.
    pub fn markdownify(s: &str) -> Result<String> {
        let source = strip_intra_doc_links(s);
        let mut out = String::new();
        html::push_html(&mut out, Parser::new_ext(&source, options()));
        Ok(out)
    }

    /// Markdown doc-comment text for the Markdown view.
    pub fn md(s: &str) -> Result<String> {
        Ok(strip_intra_doc_links(s))
    }

    /// The first paragraph: the short form, like rustdoc's item summaries.
    pub fn first_paragraph(s: &str) -> Result<String> {
        Ok(s.split("\n\n").next().unwrap_or_default().to_string())
    }

    /// `2024-present` -> `2024 — Present`, `2021-2023` -> `2021 — 2023`; other
    /// text (`2021, 2022`) as is.
    pub fn period(s: &str) -> Result<String> {
        Ok(match s.split_once('-') {
            Some((from, to)) if from.len() == 4 && from.bytes().all(|b| b.is_ascii_digit()) => {
                format!("{from} — {}", if to == "present" { "Present" } else { to })
            }
            _ => s.to_string(),
        })
    }

    /// `https://www.linkedin.com/in/singulared/` -> `linkedin.com/in/singulared`.
    pub fn host(url: &str) -> Result<String> {
        let url = url.split_once("://").map_or(url, |(_, rest)| rest);
        Ok(url.trim_start_matches("www.").trim_end_matches('/').to_string())
    }

    /// `@singulared` -> `t.me/singulared`.
    pub fn telegram(handle: &str) -> Result<String> {
        Ok(format!("t.me/{}", handle.trim_start_matches('@')))
    }

    /// Compact cached star count (`1.8k`) for a `github.com/owner/repo` URL.
    /// Empty when the URL isn't a GitHub repo, has no cached count, or the repo
    /// has no stars, so the template can skip it.
    pub fn stars(url: &str) -> Result<String> {
        let count = repo_path(url).and_then(|repo| STARS.get()?.get(&repo).copied());
        Ok(match count {
            Some(count) if count > 0 => format_count(count),
            _ => String::new(),
        })
    }

    /// Rustdoc intra-doc links (`[MediaUp]`, `[News](crate::work::Rambler::News)`)
    /// point nowhere outside rustdoc: keep the link text, drop the link. All
    /// other source text is copied verbatim.
    fn strip_intra_doc_links(source: &str) -> String {
        let parser =
            Parser::new_with_broken_link_callback(source, options(), Some(reference_as_target));
        let mut out = String::with_capacity(source.len());
        let mut copied = 0;
        // Inside an intra-doc link: the link's source span, and its text's span.
        let mut link: Option<(Range<usize>, Option<Range<usize>>)> = None;
        for (event, range) in parser.into_offset_iter() {
            match event {
                Event::Start(Tag::Link { dest_url, .. }) if is_intra_doc(&dest_url) => {
                    link = Some((range, None));
                }
                Event::End(TagEnd::Link) => {
                    if let Some((span, text)) = link.take() {
                        out.push_str(&source[copied..span.start]);
                        out.push_str(text.map_or("", |text| &source[text]));
                        copied = span.end;
                    }
                }
                _ => {
                    if let Some((_, text)) = &mut link {
                        *text = Some(match text.take() {
                            Some(text) => text.start.min(range.start)..text.end.max(range.end),
                            None => range,
                        });
                    }
                }
            }
        }
        out.push_str(&source[copied..]);
        out
    }

    /// Turns an undefined reference like `[MediaUp]` into a link to `MediaUp`, so
    /// it is stripped like an inline intra-doc link.
    fn reference_as_target(link: BrokenLink<'_>) -> Option<(CowStr<'_>, CowStr<'_>)> {
        Some((link.reference, CowStr::Borrowed("")))
    }

    /// A link target only rustdoc resolves: a bare item name or a `::` path.
    fn is_intra_doc(dest: &str) -> bool {
        !dest.starts_with(['#', '/', '.']) && (!dest.contains(':') || dest.contains("::"))
    }
}

/// `https://github.com/owner/repo[/...]` -> `Some("owner/repo")`, else `None`.
fn repo_path(url: &str) -> Option<String> {
    let path = url
        .trim_end_matches('/')
        .strip_prefix("https://github.com/")
        .or_else(|| url.trim_end_matches('/').strip_prefix("http://github.com/"))?;
    let mut segments = path.split('/');
    match (segments.next(), segments.next()) {
        (Some(owner), Some(repo)) if !owner.is_empty() && !repo.is_empty() => {
            Some(format!("{owner}/{repo}"))
        }
        _ => None,
    }
}

/// Compact star count, e.g. 1800 -> "1.8k", 712 -> "712".
fn format_count(n: u64) -> String {
    if n >= 1000 {
        let thousands = n as f64 / 1000.0;
        if (thousands.fract() * 10.0).round() == 0.0 {
            format!("{thousands:.0}k")
        } else {
            format!("{thousands:.1}k")
        }
    } else {
        n.to_string()
    }
}

// The stylesheet (both schemes and the three styles, as CSS custom properties)
// and the on-page "Aa" menu script, embedded at compile time.
const THEME_CSS: &str = include_str!("../../templates/themes/resume.css");
const APPEARANCE_JS: &str = include_str!("../../templates/themes/appearance.js");

/// IBM Plex, Latin subsets only, as `(family, weight, woff2)`. Embedded as data
/// URIs so the HTML stays a single file and the PDF renders the same offline.
/// © 2017 IBM Corp., SIL Open Font License 1.1: see `templates/fonts/OFL.txt`.
const FONTS: [(&str, &str, &[u8]); 6] = [
    ("IBM Plex Sans", "100 700", include_bytes!("../../templates/fonts/ibm-plex-sans-latin.woff2")),
    ("IBM Plex Mono", "400", include_bytes!("../../templates/fonts/ibm-plex-mono-400-latin.woff2")),
    ("IBM Plex Mono", "500", include_bytes!("../../templates/fonts/ibm-plex-mono-500-latin.woff2")),
    ("IBM Plex Mono", "600", include_bytes!("../../templates/fonts/ibm-plex-mono-600-latin.woff2")),
    ("IBM Plex Serif", "500", include_bytes!("../../templates/fonts/ibm-plex-serif-500-latin.woff2")),
    ("IBM Plex Serif", "600", include_bytes!("../../templates/fonts/ibm-plex-serif-600-latin.woff2")),
];

/// Google Fonts' "latin" subset range, matching the embedded files.
const LATIN_RANGE: &str = "U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, \
    U+02DC, U+0304, U+0308, U+0329, U+2000-206F, U+20AC, U+2122, U+2191, U+2193, U+2212, \
    U+2215, U+FEFF, U+FFFD";

/// `@font-face` rules for [`FONTS`].
fn font_faces() -> String {
    let mut css = String::from("/* IBM Plex © 2017 IBM Corp., SIL Open Font License 1.1 */\n");
    for (family, weight, woff2) in FONTS {
        css.push_str(&format!(
            "@font-face {{ font-family: '{family}'; font-style: normal; font-weight: {weight}; \
             font-display: swap; src: url(data:font/woff2;base64,{}) format('woff2'); \
             unicode-range: {LATIN_RANGE}; }}\n",
            base64(woff2)
        ));
    }
    css
}

/// Standard base64 with padding.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, &byte)| n | (u32::from(byte) << (16 - 8 * i)));
        for i in 0..4 {
            out.push(if i <= chunk.len() {
                char::from(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize])
            } else {
                '='
            });
        }
    }
    out
}

/// Color scheme for the HTML. The PDF always prints on light paper.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum Theme {
    /// Follow `prefers-color-scheme` until the visitor picks a theme in the
    /// on-page menu. This is the default.
    #[default]
    Auto,
    /// Default to the light scheme.
    Light,
    /// Default to the dark scheme.
    Dark,
}

impl Theme {
    /// `data-theme-default` value for `<html>`, or "" to follow the system.
    fn data_attr(self) -> &'static str {
        match self {
            Theme::Auto => "",
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }
}

/// Default style of the HTML/PDF; visitors can switch it in the on-page menu.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum Style {
    /// Serif headings, quiet.
    Minimal,
    /// Numbered sections and mono labels. This is the default.
    #[default]
    Editorial,
    /// Monospace with accent bars.
    Punk,
}

impl Style {
    /// `data-aesthetic` value for `<html>`.
    fn data_attr(self) -> &'static str {
        match self {
            Style::Minimal => "minimal",
            Style::Editorial => "editorial",
            Style::Punk => "punk",
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Markdown,
    Html,
    Pdf,
    All,
}

#[derive(Parser)]
#[command(name = "resume", version)]
#[command(about = "Generate Maxim Belousov's resume from Rust code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render the resume in one or more formats.
    Generate(GenerateArgs),
    /// Print the resume data collected from inventory.
    List,
    /// Verify every section has data (exits non-zero if any is empty).
    Check,
    /// Fetch GitHub star counts for the open-source repos and cache them.
    Stars(StarsArgs),
}

#[derive(Args)]
struct GenerateArgs {
    /// Output format to generate.
    #[arg(short, long, value_enum, default_value = "all")]
    format: OutputFormat,

    /// Output directory.
    #[arg(short, long, default_value = "target")]
    output: PathBuf,

    /// Default color scheme of the HTML.
    #[arg(short, long, value_enum, default_value = "auto")]
    theme: Theme,

    /// Default style of the HTML and the PDF.
    #[arg(short, long, value_enum, default_value = "editorial")]
    style: Style,

    /// Star-count cache from `resume stars`. Star counts are shown only when
    /// this is passed (e.g. `--stars stars.json`).
    #[arg(long)]
    stars: Option<PathBuf>,

    /// Verbose output.
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Args)]
struct StarsArgs {
    /// Where to write the star-count cache (JSON `{ "owner/repo": count }`).
    #[arg(short, long, default_value = "stars.json")]
    output: PathBuf,

    /// Delay between GitHub API requests, in milliseconds. GitHub's secondary
    /// rate limit triggers on rapid bursts even with a token; ≥1000 is advised.
    #[arg(short, long, default_value_t = 1000)]
    delay: u64,
}

/// All resume data, collected from inventory and ready to render.
struct ResumeData {
    personal: PersonalInfo,
    work: Vec<WorkSection>,
    opensource: Vec<OpenSourceProject>,
    conferences: Vec<Conference>,
    education: Vec<Education>,
    publications: Vec<Publication>,
    interests: Vec<Interest>,
}

impl ResumeData {
    fn project_count(&self) -> usize {
        self.work.iter().map(|w| w.projects.len()).sum()
    }
}

#[derive(Template)]
#[template(path = "resume.md", escape = "none")]
struct MarkdownResume {
    personal: PersonalInfo,
    work: Vec<WorkSection>,
    opensource: Vec<OpenSourceProject>,
    conferences: Vec<Conference>,
    education: Vec<Education>,
    publications: Vec<Publication>,
    interests: Vec<Interest>,
}

#[derive(Template)]
#[template(path = "resume.html")]
struct HtmlResume {
    personal: PersonalInfo,
    work: Vec<WorkSection>,
    opensource: Vec<OpenSourceProject>,
    conferences: Vec<Conference>,
    education: Vec<Education>,
    publications: Vec<Publication>,
    interests: Vec<Interest>,
    css: String,
    script: &'static str,
    /// Default `data-theme` on `<html>` ("" = follow the system).
    initial_theme: &'static str,
    /// Default `data-aesthetic` on `<html>`.
    style: &'static str,
}

fn collect_resume_data(verbose: bool) -> Result<ResumeData> {
    if verbose {
        println!("🔍 Collecting resume data from inventory...");
    }

    // Companies and their projects in `WorkHistory` declaration order.
    let work = WorkHistory::SECTIONS.to_vec();

    let mut opensource: Vec<OpenSourceProject> =
        inventory::iter::<OpenSourceProject>().cloned().collect();
    // Authors first, then contributors; within a group by manual rank, then name.
    opensource.sort_by(|a, b| match (a.role, b.role) {
        ("Author", "Contributor") => Ordering::Less,
        ("Contributor", "Author") => Ordering::Greater,
        _ => a.rank.cmp(&b.rank).then_with(|| a.name.cmp(b.name)),
    });

    let mut conferences: Vec<Conference> = inventory::iter::<Conference>().cloned().collect();
    conferences.sort_by(|a, b| b.year.cmp(a.year));

    let mut education: Vec<Education> = inventory::iter::<Education>().cloned().collect();
    education.sort_by(|a, b| b.period.cmp(a.period));

    let profile = inventory::iter::<Profile>()
        .next()
        .context("no #[derive(Profile)] in scope — personal info is missing")?;
    let personal = PersonalInfo {
        name: profile.name,
        tagline: profile.tagline,
        summary: profile.summary,
        email: profile.email,
        phone: profile.phone,
        telegram: profile.telegram,
        github: profile.github,
        linkedin: profile.linkedin,
        skills: profile.skills.to_vec(),
    };

    let mut publications: Vec<Publication> = inventory::iter::<Publication>().cloned().collect();
    publications.sort_by(|a, b| b.year.cmp(a.year));

    let mut interests: Vec<Interest> = inventory::iter::<Interest>().cloned().collect();
    // Technical interests before hobbies (stable within each group).
    interests.sort_by_key(|interest| u8::from(interest.kind != "tech"));

    let data = ResumeData {
        personal,
        work,
        opensource,
        conferences,
        education,
        publications,
        interests,
    };

    if verbose {
        println!("📊 Found:");
        println!("   - {} work sections", data.work.len());
        println!("   - {} projects", data.project_count());
        println!("   - {} opensource projects", data.opensource.len());
        println!("   - {} conferences", data.conferences.len());
        println!("   - {} education entries", data.education.len());
        println!("   - {} publications", data.publications.len());
        println!("   - {} interests", data.interests.len());
    }

    Ok(data)
}

fn generate_markdown(output_dir: &Path, data: &ResumeData, verbose: bool) -> Result<PathBuf> {
    if verbose {
        println!("\n📝 Generating Markdown...");
    }
    let template = MarkdownResume {
        personal: data.personal.clone(),
        work: data.work.clone(),
        opensource: data.opensource.clone(),
        conferences: data.conferences.clone(),
        education: data.education.clone(),
        publications: data.publications.clone(),
        interests: data.interests.clone(),
    };
    let rendered = template.render().context("Failed to render Markdown template")?;
    let output_path = output_dir.join("resume.md");
    fs::write(&output_path, rendered).context("Failed to write Markdown file")?;
    if verbose {
        println!("   ✓ {}", output_path.display());
    }
    Ok(output_path)
}

fn generate_html(
    output_dir: &Path,
    data: &ResumeData,
    theme: Theme,
    style: Style,
    verbose: bool,
) -> Result<PathBuf> {
    if verbose {
        println!("\n🌐 Generating HTML ({style:?} style, {theme:?} theme)...");
    }
    let template = HtmlResume {
        personal: data.personal.clone(),
        work: data.work.clone(),
        opensource: data.opensource.clone(),
        conferences: data.conferences.clone(),
        education: data.education.clone(),
        publications: data.publications.clone(),
        interests: data.interests.clone(),
        css: font_faces() + THEME_CSS,
        script: APPEARANCE_JS,
        initial_theme: theme.data_attr(),
        style: style.data_attr(),
    };
    let rendered = template.render().context("Failed to render HTML template")?;
    let output_path = output_dir.join("resume.html");
    fs::write(&output_path, rendered).context("Failed to write HTML file")?;
    if verbose {
        println!("   ✓ {}", output_path.display());
    }
    Ok(output_path)
}

fn generate_pdf(html_path: &Path, output_dir: &Path, verbose: bool) -> Result<PathBuf> {
    if verbose {
        println!("\n📄 Generating PDF...");
    }
    let pdf_path = output_dir.join("resume.pdf");
    let output = Command::new("chromium")
        .args([
            "--headless",
            "--disable-gpu",
            &format!("--print-to-pdf={}", pdf_path.display()),
            "--no-pdf-header-footer",
            html_path.to_str().context("Invalid HTML path")?,
        ])
        .output()
        .context("Failed to execute chromium. Make sure chromium is installed.")?;
    if !output.status.success() {
        bail!("Chromium failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    if verbose {
        println!("   ✓ {}", pdf_path.display());
    }
    Ok(pdf_path)
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    fs::create_dir_all(&args.output).context("Failed to create output directory")?;

    // Star badges are opt-in: rendered only when a cache is explicitly passed.
    if let Some(path) = &args.stars {
        let data = fs::read_to_string(path).with_context(|| {
            format!(
                "Failed to read star cache {} (run `resume stars` to create it)",
                path.display()
            )
        })?;
        let counts: HashMap<String, u64> =
            serde_json::from_str(&data).context("Failed to parse star cache")?;
        filters::set_stars(counts);
    }

    let data = collect_resume_data(args.verbose)?;

    let mut generated = Vec::new();
    match args.format {
        OutputFormat::Markdown => {
            generated.push(generate_markdown(&args.output, &data, args.verbose)?);
        }
        OutputFormat::Html => {
            generated.push(generate_html(&args.output, &data, args.theme, args.style, args.verbose)?);
        }
        OutputFormat::Pdf => {
            let html = generate_html(&args.output, &data, args.theme, args.style, args.verbose)?;
            generated.push(generate_pdf(&html, &args.output, args.verbose)?);
        }
        OutputFormat::All => {
            generated.push(generate_markdown(&args.output, &data, args.verbose)?);
            let html = generate_html(&args.output, &data, args.theme, args.style, args.verbose)?;
            generated.push(html.clone());
            generated.push(generate_pdf(&html, &args.output, args.verbose)?);
        }
    }

    if args.verbose {
        println!("\n✨ Resume generation complete!");
    } else {
        println!("✨ Generated {} file(s):", generated.len());
        for file in &generated {
            println!("   - {}", file.display());
        }
    }
    Ok(())
}

fn run_list(data: &ResumeData) {
    println!("Resume data collected from inventory:\n");
    println!(
        "  Work        {} companies, {} projects",
        data.work.len(),
        data.project_count()
    );
    for section in &data.work {
        let projects: Vec<&str> = section.projects.iter().map(|p| p.name).collect();
        let projects = if projects.is_empty() {
            "—".to_string()
        } else {
            projects.join(", ")
        };
        println!("    · {} ({}) — {}", section.company, section.period, projects);
    }

    let authors = data.opensource.iter().filter(|o| o.role == "Author").count();
    let contributors = data.opensource.len() - authors;
    println!(
        "\n  Open Source {} ({} author, {} contributor)",
        data.opensource.len(),
        authors,
        contributors
    );
    for project in &data.opensource {
        println!("    · {} ({}, {})", project.name, project.role, project.language);
    }

    println!("\n  Conferences {}", data.conferences.len());
    for conference in &data.conferences {
        println!("    · {} ({}) — {}", conference.name, conference.year, conference.role);
    }

    println!("\n  Education   {}", data.education.len());
    for entry in &data.education {
        println!("    · {} — {} ({})", entry.institution, entry.degree, entry.period);
    }

    println!("\n  Publications {}", data.publications.len());
    for publication in &data.publications {
        println!("    · {} ({})", publication.title, publication.year);
    }

    println!("\n  Interests   {}", data.interests.len());
    for interest in &data.interests {
        println!("    · {} [{}]", interest.name, interest.kind);
    }
}

fn run_check(data: &ResumeData) -> Result<()> {
    let sections = [
        ("work sections", data.work.len()),
        ("projects", data.project_count()),
        ("opensource projects", data.opensource.len()),
        ("conferences", data.conferences.len()),
        ("education entries", data.education.len()),
        ("publications", data.publications.len()),
        ("interests", data.interests.len()),
    ];

    let mut empty = Vec::new();
    for (name, count) in sections {
        if count == 0 {
            println!("✗ {name}: empty");
            empty.push(name);
        } else {
            println!("✓ {name}: {count}");
        }
    }

    // README.md is the rustdoc front page; its intro must be the profile summary.
    let readme_in_sync = squash_whitespace(README).contains(&squash_whitespace(data.personal.summary));
    if readme_in_sync {
        println!("✓ README intro matches #[profile(summary)]");
    } else {
        println!("✗ README intro differs from #[profile(summary)]");
    }

    if !empty.is_empty() {
        bail!("empty section(s): {}", empty.join(", "));
    }
    if !readme_in_sync {
        bail!("README.md does not contain the profile summary");
    }
    println!("\nAll sections populated.");
    Ok(())
}

/// The README, for checking it against the profile summary.
const README: &str = include_str!("../../README.md");

/// Collapse whitespace runs to single spaces, so line wrapping doesn't matter.
fn squash_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Fetch the star count for `owner/repo` from the GitHub API via `curl`.
fn fetch_stars(repo: &str, token: Option<&str>) -> Result<u64> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-sS",
        "-H",
        "Accept: application/vnd.github+json",
        "-H",
        "User-Agent: resume-generator",
    ]);
    if let Some(token) = token {
        cmd.args(["-H", &format!("Authorization: Bearer {token}")]);
    }
    cmd.arg(format!("https://api.github.com/repos/{repo}"));

    let output = cmd.output().context("failed to run curl (is it installed?)")?;
    if !output.status.success() {
        bail!("curl exited with {}", output.status);
    }

    // Success responses carry `stargazers_count`; errors (rate limit, not found)
    // carry a `message` instead — surface it so failures are self-explanatory.
    #[derive(Deserialize)]
    struct GhResponse {
        stargazers_count: Option<u64>,
        message: Option<String>,
    }
    let response: GhResponse =
        serde_json::from_slice(&output.stdout).context("could not parse the GitHub response")?;
    match (response.stargazers_count, response.message) {
        (Some(stars), _) => Ok(stars),
        (None, Some(message)) => bail!("{message}"),
        (None, None) => bail!("no star count in the response"),
    }
}

fn run_stars(args: StarsArgs) -> Result<()> {
    let token = env::var("GITHUB_TOKEN").ok();
    if token.is_none() {
        eprintln!("note: GITHUB_TOKEN not set — using the unauthenticated API (60 requests/hour).\n");
    }

    let delay = Duration::from_millis(args.delay);
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut failed = 0usize;
    let mut first = true;
    for project in inventory::iter::<OpenSourceProject>() {
        let Some(repo) = repo_path(project.url) else {
            continue;
        };
        // Throttle between requests to stay under GitHub's secondary rate limit.
        if !first {
            thread::sleep(delay);
        }
        first = false;
        match fetch_stars(&repo, token.as_deref()) {
            Ok(stars) => {
                println!("  ⭐ {stars:>6}  {repo}");
                counts.insert(repo, stars);
            }
            Err(error) => {
                eprintln!("  ✗         {repo}: {error:#}");
                failed += 1;
            }
        }
    }

    let json = serde_json::to_string_pretty(&counts).context("Failed to serialize star cache")?;
    fs::write(&args.output, json)
        .with_context(|| format!("Failed to write {}", args.output.display()))?;
    println!(
        "\nWrote {} star counts to {}{}.",
        counts.len(),
        args.output.display(),
        if failed > 0 {
            format!(" ({failed} failed)")
        } else {
            String::new()
        }
    );
    Ok(())
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Commands::Generate(args) => run_generate(args),
        Commands::List => {
            run_list(&collect_resume_data(false)?);
            Ok(())
        }
        Commands::Check => run_check(&collect_resume_data(false)?),
        Commands::Stars(args) => run_stars(args),
    }
}
