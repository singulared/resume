//! Resume generator CLI.
//!
//! Collects all resume data via `inventory` and renders it to Markdown, HTML,
//! and PDF. Subcommands:
//!   * `generate` — render the resume (Markdown / HTML / PDF / all)
//!   * `list`     — print the data collected from inventory
//!   * `check`    — verify every section has data (non-zero exit if any is empty)
//!   * `stars`    — fetch GitHub star counts for the open-source repos and cache
//!                  them; `generate` reads the cache to render static star badges

use anyhow::{Context, Result};
use askama::Template;
use clap::{Args, Parser, Subcommand, ValueEnum};
use resume::WorkHistory;
use resume::resume_data::*;
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Askama template filters. `markdownify` renders Markdown doc-comment fields to
/// HTML (pair with `|safe`); `stars_badge` renders a static shields.io star badge
/// from the cached count loaded via [`filters::set_stars`].
mod filters {
    use pulldown_cmark::{html, Options, Parser};
    use std::collections::HashMap;
    use std::sync::OnceLock;

    /// Star counts (keyed by `owner/repo`), loaded from the cache before rendering.
    static STARS: OnceLock<HashMap<String, u64>> = OnceLock::new();

    pub fn set_stars(stars: HashMap<String, u64>) {
        let _ = STARS.set(stars);
    }

    pub fn markdownify(s: &str) -> askama::Result<String> {
        let parser = Parser::new_ext(s, Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES);
        let mut out = String::new();
        html::push_html(&mut out, parser);
        Ok(out)
    }

    /// A *static* shields.io star badge (`⭐ <count>`) for a `github.com/owner/repo`
    /// URL, using the cached count. Static badges never call GitHub, so they can't
    /// hit the live-badge rate limits. Returns "" when the URL isn't a GitHub repo
    /// or has no cached count, so the template can skip the badge.
    pub fn stars_badge(url: &str) -> askama::Result<String> {
        let Some(repo) = super::repo_path(url) else {
            return Ok(String::new());
        };
        let Some(count) = STARS.get().and_then(|stars| stars.get(&repo)).copied() else {
            return Ok(String::new());
        };
        Ok(format!(
            "https://img.shields.io/badge/%E2%AD%90-{}-1e66f5",
            super::format_count(count)
        ))
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

// One stylesheet with both schemes (CSS custom properties), embedded at compile
// time. The scheme is chosen by the reader's `prefers-color-scheme`, the on-page
// toggle, or a forced `data-theme` (the --theme flag).
const THEME_CSS: &str = include_str!("../../templates/themes/resume.css");

/// Color scheme for the output.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum Theme {
    /// Auto-detect from `prefers-color-scheme` (HTML), with an on-page toggle to
    /// override. PDF renders light. This is the default.
    #[default]
    Auto,
    /// Force the light scheme.
    Light,
    /// Force the dark scheme.
    Dark,
}

impl Theme {
    /// `data-theme` value to force on `<html>`, or "" to leave auto-detection on.
    fn data_attr(self) -> &'static str {
        match self {
            Theme::Auto => "",
            Theme::Light => "light",
            Theme::Dark => "dark",
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

    /// Color scheme for the HTML/PDF output.
    #[arg(short, long, value_enum, default_value = "auto")]
    theme: Theme,

    /// Star-count cache from `resume stars`. Star badges are rendered only when
    /// this is passed (e.g. `--stars stars.json`); omitted = no badges.
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
    /// Forced `data-theme` on `<html>` ("" = auto-detect).
    initial_theme: &'static str,
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
        ("Author", "Contributor") => std::cmp::Ordering::Less,
        ("Contributor", "Author") => std::cmp::Ordering::Greater,
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
    std::fs::write(&output_path, rendered).context("Failed to write Markdown file")?;
    if verbose {
        println!("   ✓ {}", output_path.display());
    }
    Ok(output_path)
}

fn generate_html(output_dir: &Path, data: &ResumeData, theme: Theme, verbose: bool) -> Result<PathBuf> {
    if verbose {
        println!("\n🌐 Generating HTML ({theme:?} theme)...");
    }
    let template = HtmlResume {
        personal: data.personal.clone(),
        work: data.work.clone(),
        opensource: data.opensource.clone(),
        conferences: data.conferences.clone(),
        education: data.education.clone(),
        publications: data.publications.clone(),
        interests: data.interests.clone(),
        css: THEME_CSS.to_string(),
        initial_theme: theme.data_attr(),
    };
    let rendered = template.render().context("Failed to render HTML template")?;
    let output_path = output_dir.join("resume.html");
    std::fs::write(&output_path, rendered).context("Failed to write HTML file")?;
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
        anyhow::bail!("Chromium failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    if verbose {
        println!("   ✓ {}", pdf_path.display());
    }
    Ok(pdf_path)
}

fn run_generate(args: GenerateArgs) -> Result<()> {
    std::fs::create_dir_all(&args.output).context("Failed to create output directory")?;

    // Star badges are opt-in: rendered only when a cache is explicitly passed.
    if let Some(path) = &args.stars {
        let data = std::fs::read_to_string(path).with_context(|| {
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
            generated.push(generate_html(&args.output, &data, args.theme, args.verbose)?);
        }
        OutputFormat::Pdf => {
            let html = generate_html(&args.output, &data, args.theme, args.verbose)?;
            generated.push(generate_pdf(&html, &args.output, args.verbose)?);
        }
        OutputFormat::All => {
            generated.push(generate_markdown(&args.output, &data, args.verbose)?);
            let html = generate_html(&args.output, &data, args.theme, args.verbose)?;
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
        anyhow::bail!("empty section(s): {}", empty.join(", "));
    }
    if !readme_in_sync {
        anyhow::bail!("README.md does not contain the profile summary");
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
        anyhow::bail!("curl exited with {}", output.status);
    }

    // Success responses carry `stargazers_count`; errors (rate limit, not found)
    // carry a `message` instead — surface it so failures are self-explanatory.
    #[derive(serde::Deserialize)]
    struct GhResponse {
        stargazers_count: Option<u64>,
        message: Option<String>,
    }
    let response: GhResponse =
        serde_json::from_slice(&output.stdout).context("could not parse the GitHub response")?;
    match (response.stargazers_count, response.message) {
        (Some(stars), _) => Ok(stars),
        (None, Some(message)) => anyhow::bail!("{message}"),
        (None, None) => anyhow::bail!("no star count in the response"),
    }
}

fn run_stars(args: StarsArgs) -> Result<()> {
    let token = std::env::var("GITHUB_TOKEN").ok();
    if token.is_none() {
        eprintln!("note: GITHUB_TOKEN not set — using the unauthenticated API (60 requests/hour).\n");
    }

    let delay = std::time::Duration::from_millis(args.delay);
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut failed = 0usize;
    let mut first = true;
    for project in inventory::iter::<OpenSourceProject>() {
        let Some(repo) = repo_path(project.url) else {
            continue;
        };
        // Throttle between requests to stay under GitHub's secondary rate limit.
        if !first {
            std::thread::sleep(delay);
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
    std::fs::write(&args.output, json)
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
