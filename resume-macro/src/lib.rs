use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_quote, DeriveInput};
use darling::{
    ast::{Data, Fields, NestedMeta},
    FromDeriveInput, FromField, FromMeta, FromVariant,
};

// Keep the original CleanupDocs macro
#[proc_macro_derive(CleanupDocs)]
pub fn cleanup_docs_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    impl_cleanup_docs_macro(&ast)
}

fn impl_cleanup_docs_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = &ast.generics.split_for_impl();
    let gen = quote! {
        #[doc(hidden)]
        impl #impl_generics !Send for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !Sync for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !Unpin for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !core::panic::RefUnwindSafe for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !core::panic::UnwindSafe for #name #ty_generics #where_clause {}
        #[doc(hidden)]
        impl #impl_generics !std::marker::Freeze for #name #ty_generics #where_clause {}
    };
    gen.into()
}

/// Parsed `#[tech(path)]` / `#[tech(path, "description")]`.
#[derive(Debug, Clone)]
struct Technology {
    path: syn::Path,
    description: Option<String>,
}

impl darling::FromMeta for Technology {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        if items.is_empty() {
            return Err(darling::Error::custom("technology requires at least a path"));
        }

        // First item is the path (e.g., languages::Rust or just Tokio)
        let path = match &items[0] {
            NestedMeta::Meta(syn::Meta::Path(p)) => p.clone(),
            _ => return Err(darling::Error::custom("technology must be a path")),
        };

        // Second item (if present) is the description string
        let description = if items.len() > 1 {
            match &items[1] {
                NestedMeta::Lit(syn::Lit::Str(s)) => Some(s.value()),
                _ => return Err(darling::Error::custom("technology description must be a string literal")),
            }
        } else {
            None
        };

        Ok(Technology { path, description })
    }
}

// Helper functions

/// The item's doc comment as Markdown. Like rustdoc, strips the indentation
/// common to all lines rather than trimming each one, so nested lists survive.
fn extract_docs(attrs: &[syn::Attribute]) -> String {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.value {
                        if let syn::Lit::Str(lit) = &expr_lit.lit {
                            return Some(lit.value());
                        }
                    }
                }
            }
            None
        })
        // `split`, not `lines`: a blank `///` is an empty string and must stay a
        // (paragraph-breaking) line.
        .flat_map(|doc| doc.split('\n').map(str::to_string).collect::<Vec<_>>())
        .collect();
    let indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| line.get(indent..).unwrap_or("").trim_end())
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

// ============================================================================
// `#[derive(Project)]`
//
// Type-based code generation: every construct the derive emits is its own
// small struct implementing `ToTokens`, and `derive_project` is the control
// code that parses the attributes, builds those parts, and composes them.
// Repeatable `#[role(...)]` attributes yield one role impl each, so a single
// struct can carry several roles (e.g. Developer then TeamLead).
//
//     #[derive(Project)]
//     #[project(name = "Rambler News")]  // optional display name; defaults to the ident
//     #[role(Developer, span(2017, 2018), langs(Python), doc = "…")]
//     #[role(TeamLead,  span(2018, 2022), langs(Rust, Python), doc = "…")]
//     #[tech(frameworks::Actix, "Actor & web framework.")]
//     pub struct News;
//
// The company a project belongs to is the `WorkHistory` variant listing it
// (see `#[derive(Work)]`), not an attribute here.
// ============================================================================

/// Parsed optional `#[project(name = "Display name")]`.
#[derive(FromDeriveInput)]
#[darling(attributes(project), supports(struct_any))]
struct ProjectOpts {
    ident: syn::Ident,
    #[darling(default)]
    name: Option<String>,
}

/// Parsed `#[role(Trait, span(from, to), langs(A, B), param(Name), doc = "...")]`.
struct RoleSpec {
    role: syn::Ident,
    from: Option<u32>,
    to: Option<u32>,
    langs: Vec<syn::Ident>,
    /// Optional override for the impl's type-parameter name; inferred from the
    /// role trait when absent (`TeamLead` → `Team`, else `Language`/`Languages`).
    param: Option<syn::Ident>,
    doc: String,
}

fn nested_int(meta: &NestedMeta) -> Option<u32> {
    match meta {
        NestedMeta::Lit(syn::Lit::Int(i)) => i.base10_parse().ok(),
        _ => None,
    }
}

impl FromMeta for RoleSpec {
    /// Parses `role(Developer, span(2017, 2018), langs(Python, Rust), doc = "...")`.
    ///
    /// The leading bare path is the role trait; the rest are recognised by key,
    /// mirroring [`Technology::from_list`].
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let role = match items.first() {
            Some(NestedMeta::Meta(syn::Meta::Path(p))) => p.get_ident().cloned().ok_or_else(|| {
                darling::Error::custom("role trait must be a simple identifier").with_span(p)
            })?,
            _ => {
                return Err(darling::Error::custom(
                    "role(...) must start with a role trait, e.g. role(Developer, ...)",
                ))
            }
        };

        let mut from = None;
        let mut to = None;
        let mut langs = Vec::new();
        let mut param = None;
        let mut doc = String::new();
        for item in &items[1..] {
            match item {
                // `span(2017, 2018)` — start and (optional) end year.
                NestedMeta::Meta(syn::Meta::List(l)) if l.path.is_ident("span") => {
                    let years: Vec<u32> = NestedMeta::parse_meta_list(l.tokens.clone())?
                        .iter()
                        .filter_map(nested_int)
                        .collect();
                    from = years.first().copied();
                    to = years.get(1).copied();
                }
                // `langs(Python, Rust)` — the trait bound on the role's type parameter.
                NestedMeta::Meta(syn::Meta::List(l)) if l.path.is_ident("langs") => {
                    for lang in &NestedMeta::parse_meta_list(l.tokens.clone())? {
                        if let NestedMeta::Meta(syn::Meta::Path(p)) = lang {
                            if let Some(id) = p.get_ident() {
                                langs.push(id.clone());
                            }
                        }
                    }
                }
                // `param(Stack)` — explicit name for the impl's type parameter.
                NestedMeta::Meta(syn::Meta::List(l)) if l.path.is_ident("param") => {
                    for item in &NestedMeta::parse_meta_list(l.tokens.clone())? {
                        if let NestedMeta::Meta(syn::Meta::Path(p)) = item {
                            param = p.get_ident().cloned();
                        }
                    }
                    if param.is_none() {
                        return Err(darling::Error::custom(
                            "param(...) takes a single identifier, e.g. param(Stack)",
                        )
                        .with_span(l));
                    }
                }
                // `doc = "..."` — the role impl's docstring.
                NestedMeta::Meta(syn::Meta::NameValue(nv)) if nv.path.is_ident("doc") => {
                    match &nv.value {
                        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => {
                            doc = s.value()
                        }
                        _ => {
                            return Err(
                                darling::Error::custom("doc must be a string literal").with_span(nv)
                            )
                        }
                    }
                }
                _ => {
                    return Err(darling::Error::custom(
                        "unexpected item in role(...); expected span(...), langs(...), param(...), or doc = \"...\"",
                    ))
                }
            }
        }

        Ok(RoleSpec { role, from, to, langs, param, doc })
    }
}

impl RoleSpec {
    /// The span as the views print it: `2017-2018`, `2024-present`, `2017`,
    /// or empty without a span.
    fn period(&self) -> String {
        match (self.from, self.to) {
            (Some(from), Some(to)) if from == to => from.to_string(),
            (Some(from), Some(to)) => format!("{from}-{to}"),
            (Some(from), None) => format!("{from}-present"),
            _ => String::new(),
        }
    }
}

/// `#[doc = "..."] impl crate::technologies::<path> for <ident> {}`
struct TechImpl<'a> {
    ident: &'a syn::Ident,
    path: &'a syn::Path,
    doc: String,
}

impl ToTokens for TechImpl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, path, doc } = self;
        // No description -> no doc attribute, matching the hand-written
        // doc-less impls (e.g. `impl Qt for Satago {}`).
        let doc_attr = if doc.is_empty() {
            quote! {}
        } else {
            quote! { #[doc = #doc] }
        };
        tokens.extend(quote! {
            #doc_attr
            impl crate::technologies::#path for #ident {}
        });
    }
}

/// One role impl, e.g.
/// `impl<'period, 'f2017, 't2018, Language> Developer<Language> for &'period News
///  where Language: Python, 'period: 'f2017 + 't2018 {}`
struct RoleImpl<'a> {
    ident: &'a syn::Ident,
    role: &'a syn::Ident,
    param: syn::Ident,
    langs: Vec<syn::Path>,
    lifetimes: Vec<syn::Lifetime>,
    doc: String,
}

impl ToTokens for RoleImpl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, role, param, langs, lifetimes, doc } = self;

        // With span lifetimes the role is implemented for `&'period Struct` so
        // the employment period can bound the reference; without a span the
        // hand-written style is a plain `impl<Language> Role<Language> for Struct`.
        let (generics, self_ty, allow_attr) = if lifetimes.is_empty() {
            (quote! { <#param> }, quote! { #ident }, quote! {})
        } else {
            (
                quote! { <'period, #(#lifetimes,)* #param> },
                quote! { &'period #ident },
                quote! { #[allow(clippy::extra_unused_lifetimes)] },
            )
        };

        let mut predicates: Vec<proc_macro2::TokenStream> = Vec::new();
        if !langs.is_empty() {
            let bounds = langs.iter();
            predicates.push(quote! { #param: #(#bounds)+* });
        }
        for lifetime in lifetimes {
            predicates.push(quote! { 'period: #lifetime });
        }
        let where_clause = if predicates.is_empty() {
            quote! {}
        } else {
            quote! { where #(#predicates),* }
        };

        let doc_attr = if doc.is_empty() {
            quote! {}
        } else {
            quote! { #[doc = #doc] }
        };

        tokens.extend(quote! {
            #doc_attr
            #allow_attr
            impl #generics crate::roles::#role<#param> for #self_ty
            #where_clause
            {}
        });
    }
}

/// The six negative auto-trait impls that keep rustdoc clean.
struct AutoTraitCleanup<'a> {
    ident: &'a syn::Ident,
}

impl ToTokens for AutoTraitCleanup<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident;
        tokens.extend(quote! {
            #[doc(hidden)] impl !Send for #ident {}
            #[doc(hidden)] impl !Sync for #ident {}
            #[doc(hidden)] impl !Unpin for #ident {}
            #[doc(hidden)] impl !core::panic::RefUnwindSafe for #ident {}
            #[doc(hidden)] impl !core::panic::UnwindSafe for #ident {}
            #[doc(hidden)] impl !std::marker::Freeze for #ident {}
        });
    }
}

/// The hidden `WorkProject` impl feeding the template/binary views; collected
/// in declaration order by `#[derive(Work)]`.
struct ProjectConst<'a> {
    ident: &'a syn::Ident,
    name: String,
    docs: String,
    /// Some role has an open-ended span (`span(2024)`).
    ongoing: bool,
    /// `(period, doc)` of each `#[role(...)]` that has a doc.
    roles: Vec<(String, String)>,
    techs: Vec<&'a syn::Path>,
}

impl ToTokens for ProjectConst<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { ident, name, docs, ongoing, roles, techs } = self;
        let key = ident.to_string();
        let roles = roles.iter().map(|(period, doc)| {
            quote! { crate::resume_data::ProjectRole { period: #period, doc: #doc } }
        });
        // Display names come from the technology traits themselves (see
        // `technologies!`), so `Postgresql` renders as "PostgreSQL" everywhere.
        let techs = techs.iter().map(|path| {
            quote! { <dyn crate::technologies::#path as crate::resume_data::Technology>::NAME }
        });
        tokens.extend(quote! {
            #[doc(hidden)]
            impl crate::resume_data::WorkProject for #ident {
                const PROJECT: crate::resume_data::Project = crate::resume_data::Project {
                    key: #key,
                    name: #name,
                    description: #docs,
                    ongoing: #ongoing,
                    roles: &[#(#roles),*],
                    technologies: &[#(#techs),*],
                };
            }
        });
    }
}

/// Derive macro for projects within work sections.
///
/// Builds the rustdoc trait impls (technologies + one impl per `#[role(...)]`)
/// and the hidden `WorkProject` data impl from attributes. See the module
/// comment above for the attribute syntax.
#[proc_macro_derive(Project, attributes(project, role, tech))]
pub fn derive_project(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ProjectOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    // Each repeated `#[role(...)]` / `#[tech(...)]` attribute is parsed by darling
    // (`RoleSpec` / the reused `Technology`); we only select them by name here.
    let roles: Vec<RoleSpec> = match input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("role"))
        .map(|a| RoleSpec::from_meta(&a.meta))
        .collect::<darling::Result<_>>()
    {
        Ok(roles) => roles,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let techs: Vec<Technology> = match input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("tech"))
        .map(|a| Technology::from_meta(&a.meta))
        .collect::<darling::Result<_>>()
    {
        Ok(techs) => techs,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = &opts.ident;
    let name = opts.name.clone().unwrap_or_else(|| ident.to_string());
    let docs = extract_docs(&input.attrs);

    // Build the ToTokens parts, then compose them. A `#[tech(...)]` without a
    // description yields a doc-less impl — no catalog fallback, so generic
    // filler text can't silently replace the hand-written context.
    let tech_impls: Vec<TechImpl> = techs
        .iter()
        .map(|tech| {
            let doc = tech.description.clone().unwrap_or_default();
            TechImpl { ident, path: &tech.path, doc }
        })
        .collect();

    let role_impls: Vec<RoleImpl> = roles
        .iter()
        .map(|spec| {
            // `TeamLead` reads as leading a `Team`; everything else codes in a
            // `Language` — or `Languages` when the role spans more than one, matching
            // the hand-written impls (e.g. `Developer<Languages>` for Paywall).
            let param = spec.param.clone().unwrap_or_else(|| {
                if spec.role == "TeamLead" {
                    format_ident!("Team")
                } else if spec.langs.len() > 1 {
                    format_ident!("Languages")
                } else {
                    format_ident!("Language")
                }
            });
            let langs: Vec<syn::Path> = spec
                .langs
                .iter()
                .map(|lang| parse_quote!(crate::technologies::languages::#lang))
                .collect();
            let mut lifetimes = Vec::new();
            match (spec.from, spec.to) {
                // `span(2017, 2017)` — within a single year: `'in2017`.
                (Some(from), Some(to)) if from == to => {
                    lifetimes.push(syn::Lifetime::new(
                        &format!("'in{}", from),
                        proc_macro2::Span::call_site(),
                    ));
                }
                // `span(2017, 2018)` -> `'f2017 + 't2018`; `span(2024)` -> `'f2024` (ongoing).
                (from, to) => {
                    if let Some(from) = from {
                        lifetimes.push(syn::Lifetime::new(
                            &format!("'f{}", from),
                            proc_macro2::Span::call_site(),
                        ));
                    }
                    if let Some(to) = to {
                        lifetimes.push(syn::Lifetime::new(
                            &format!("'t{}", to),
                            proc_macro2::Span::call_site(),
                        ));
                    }
                }
            }
            RoleImpl {
                ident,
                role: &spec.role,
                param,
                langs,
                lifetimes,
                doc: spec.doc.clone(),
            }
        })
        .collect();

    let project = ProjectConst {
        ident,
        name,
        docs,
        ongoing: roles.iter().any(|spec| spec.from.is_some() && spec.to.is_none()),
        roles: roles
            .iter()
            .filter(|spec| !spec.doc.is_empty())
            .map(|spec| (spec.period(), spec.doc.clone()))
            .collect(),
        techs: techs.iter().map(|tech| &tech.path).collect(),
    };
    let cleanup = AutoTraitCleanup { ident };

    TokenStream::from(quote! {
        #project
        #(#tech_impls)*
        #(#role_impls)*
        #cleanup
    })
}

/// Parsed `#[company(name = "...", url = "...", role = "...", period = "...", summary = "...")]`
/// on a work-history variant; the variant's tuple fields are its project types.
#[derive(FromVariant)]
#[darling(attributes(company))]
struct CompanyOpts {
    fields: Fields<CompanyProject>,
    name: String,
    url: String,
    role: String,
    period: String,
    summary: String,
}

#[derive(FromField)]
struct CompanyProject {
    ty: syn::Type,
}

#[derive(FromDeriveInput)]
#[darling(supports(enum_any))]
struct WorkOpts {
    ident: syn::Ident,
    data: Data<CompanyOpts, ()>,
}

impl ToTokens for CompanyOpts {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { fields, name, url, role, period, summary } = self;
        let projects = fields.iter().map(|project| &project.ty);
        tokens.extend(quote! {
            crate::resume_data::WorkSection {
                company: #name,
                url: #url,
                role: #role,
                period: #period,
                description: #summary,
                projects: &[#(<#projects as crate::resume_data::WorkProject>::PROJECT),*],
            }
        });
    }
}

/// Derive for the work-history enum ("typed root"): one variant per company,
/// its tuple fields the `#[derive(Project)]` types, in display order.
///
/// Emits a hidden `WorkRoot` impl whose `SECTIONS` follow declaration order, so
/// the views list companies and projects exactly as rustdoc does, and a listed
/// type without `#[derive(Project)]` fails to compile.
///
/// ```ignore
/// #[derive(Work)]
/// pub enum WorkHistory {
///     /// [Rambler&Co:](self::Rambler) *2015 - 2022*
///     #[company(name = "Rambler&Co", url = "https://rambler.ru", role = "Team Lead",
///               period = "2015-2022", summary = "One of the biggest media holdings in Russia.")]
///     Rambler(News, MediaUp),
/// }
/// ```
#[proc_macro_derive(Work, attributes(company))]
pub fn derive_work(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match WorkOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let ident = &opts.ident;
    let sections = opts.data.take_enum().unwrap_or_default();
    TokenStream::from(quote! {
        #[doc(hidden)]
        impl crate::resume_data::WorkRoot for #ident {
            const SECTIONS: &'static [crate::resume_data::WorkSection] = &[#(#sections),*];
        }
    })
}

/// Parsed `#[oss(url = "...", role = "Author", language = "Rust", rank = 1)]`.
#[derive(FromDeriveInput)]
#[darling(attributes(oss), supports(struct_any))]
struct OssOpts {
    ident: syn::Ident,
    url: String,
    role: String,
    /// Display order within the role group (lower first); unranked sort last.
    #[darling(default = "default_oss_rank")]
    rank: u32,
    #[darling(default)]
    language: String,
}

fn default_oss_rank() -> u32 {
    1000
}

/// Split a leading Markdown link such as `[name](url) - ` off an open-source doc
/// comment: the link text is the project's display name, and the rendered
/// heading already links it, so the rest is the description.
fn split_leading_link(doc: &str) -> (Option<String>, String) {
    let trimmed = doc.trim_start();
    let Some(text) = trimmed.strip_prefix('[') else {
        return (None, doc.to_string());
    };
    let Some(text_end) = text.find("](") else {
        return (None, doc.to_string());
    };
    let after_text = &text[text_end + 2..];
    let Some(url_end) = after_text.find(')') else {
        return (None, doc.to_string());
    };
    let rest = after_text[url_end + 1..]
        .trim_start()
        .trim_start_matches(|c: char| c == '-' || c == '—' || c == ':')
        .trim_start()
        .to_string();
    (Some(text[..text_end].to_string()), rest)
}

/// Derive for an open-source project. Submits an [`crate::resume_data::OpenSourceProject`]
/// to inventory. The display name is the text of the doc comment's leading link
/// (falling back to the struct ident); the description is the rest of the doc.
#[proc_macro_derive(Oss, attributes(oss))]
pub fn derive_oss(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match OssOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let (name, description) = split_leading_link(&extract_docs(&input.attrs));
    let name = name.unwrap_or_else(|| opts.ident.to_string());
    let url = &opts.url;
    let role = &opts.role;
    let rank = opts.rank;
    let language = &opts.language;
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::OpenSourceProject {
                name: #name,
                url: #url,
                role: #role,
                rank: #rank,
                language: #language,
                description: #description,
            }
        }
    })
}

/// Parsed `#[conference(name = "...", year = "...", role = "...", url = "...")]`.
#[derive(FromDeriveInput)]
#[darling(attributes(conference), supports(struct_any))]
struct ConferenceOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    name: String,
    year: String,
    role: String,
    #[darling(default)]
    url: String,
}

/// Derive for a conference / speaking engagement. Submits a [`crate::resume_data::Conference`].
#[proc_macro_derive(Conference, attributes(conference))]
pub fn derive_conference(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match ConferenceOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let docs = extract_docs(&input.attrs);
    let name = &opts.name;
    let year = &opts.year;
    let role = &opts.role;
    let url = &opts.url;
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::Conference {
                name: #name,
                year: #year,
                role: #role,
                description: #docs,
                url: #url,
            }
        }
    })
}

/// Parsed `#[education(institution = "...", degree = "...", field = "...", period = "...")]`.
#[derive(FromDeriveInput)]
#[darling(attributes(education), supports(struct_any))]
struct EducationOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    institution: String,
    degree: String,
    field: String,
    period: String,
}

/// Derive for an education entry. Submits an [`crate::resume_data::Education`].
#[proc_macro_derive(Education, attributes(education))]
pub fn derive_education(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match EducationOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let docs = extract_docs(&input.attrs);
    let institution = &opts.institution;
    let degree = &opts.degree;
    let field = &opts.field;
    let period = &opts.period;
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::Education {
                institution: #institution,
                degree: #degree,
                field: #field,
                period: #period,
                description: #docs,
            }
        }
    })
}


/// Parsed `#[publication(title = "...", url = "...", year = "...")]`.
#[derive(FromDeriveInput)]
#[darling(attributes(publication), supports(struct_any))]
struct PublicationOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    title: String,
    url: String,
    year: String,
}

/// Derive for a publication / article. Submits a [`crate::resume_data::Publication`];
/// the description is the struct's doc comment.
#[proc_macro_derive(Publication, attributes(publication))]
pub fn derive_publication(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match PublicationOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let docs = extract_docs(&input.attrs);
    let title = &opts.title;
    let url = &opts.url;
    let year = &opts.year;
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::Publication {
                title: #title,
                url: #url,
                year: #year,
                description: #docs,
            }
        }
    })
}

/// Parsed `#[profile(name = "...", tagline = "...", summary = "...", email = "...", ...)]`.
#[derive(FromDeriveInput)]
#[darling(attributes(profile), supports(struct_any))]
struct ProfileOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    name: String,
    tagline: String,
    summary: String,
    email: String,
    /// As displayed, e.g. `+995 598 32 58 81`.
    phone: String,
    telegram: String,
    github: String,
    linkedin: String,
}

/// Derive for the profile / contact block. Submits a single
/// [`crate::resume_data::Profile`]; repeatable `#[skill("...")]` attributes
/// become the (ordered) skills list.
#[proc_macro_derive(Profile, attributes(profile, skill))]
pub fn derive_profile(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match ProfileOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let mut skills = Vec::new();
    for attr in &input.attrs {
        if attr.path().is_ident("skill") {
            match attr.parse_args::<syn::LitStr>() {
                Ok(skill) => skills.push(skill.value()),
                Err(e) => return TokenStream::from(e.to_compile_error()),
            }
        }
    }

    let name = &opts.name;
    let tagline = &opts.tagline;
    let summary = &opts.summary;
    let email = &opts.email;
    let phone = &opts.phone;
    let telegram = &opts.telegram;
    let github = &opts.github;
    let linkedin = &opts.linkedin;
    let skills = skills.iter();
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::Profile {
                name: #name,
                tagline: #tagline,
                summary: #summary,
                email: #email,
                phone: #phone,
                telegram: #telegram,
                github: #github,
                linkedin: #linkedin,
                skills: &[#(#skills),*],
            }
        }
    })
}

/// Parsed `#[interest(kind = "tech|hobby", name = "...", desc = "...")]`.
#[derive(FromMeta)]
struct InterestSpec {
    kind: String,
    name: String,
    #[darling(default)]
    desc: String,
}

/// Derive for personal interests / hobbies. Each repeatable `#[interest(...)]`
/// attribute submits one [`crate::resume_data::Interest`].
#[proc_macro_derive(Interest, attributes(interest))]
pub fn derive_interest(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let specs: Vec<InterestSpec> = match input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("interest"))
        .map(|a| InterestSpec::from_meta(&a.meta))
        .collect::<darling::Result<_>>()
    {
        Ok(specs) => specs,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let submits = specs.iter().map(|spec| {
        let kind = &spec.kind;
        let name = &spec.name;
        let desc = &spec.desc;
        quote! {
            inventory::submit! {
                crate::resume_data::Interest {
                    kind: #kind,
                    name: #name,
                    description: #desc,
                }
            }
        }
    });
    TokenStream::from(quote! { #(#submits)* })
}
