use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_quote, DeriveInput};
use syn::parse::Parser;
use darling::{ast::NestedMeta, FromDeriveInput, FromMeta};

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

// Darling-based attributes for work sections

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(resume), supports(struct_any))]
struct ResumeWorkOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    #[allow(dead_code)]
    data: darling::ast::Data<(), ()>,

    company: String,
    url: String,
    role: String,
    period: String,
}

#[derive(Debug, Clone)]
struct Technology {
    path: syn::Path,
    description: Option<String>,
}

#[derive(Debug, Clone)]
struct PeriodLifetime(syn::Lifetime);

impl darling::FromMeta for PeriodLifetime {
    fn from_string(value: &str) -> darling::Result<Self> {
        // Ensure it starts with a quote
        if !value.starts_with('\'') {
            return Err(darling::Error::custom(format!(
                "Lifetime must start with ', got: {}",
                value
            )));
        }
        let lifetime = syn::Lifetime::new(value, proc_macro2::Span::call_site());
        Ok(PeriodLifetime(lifetime))
    }
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

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(resume), supports(struct_any))]
struct ResumeProjectOpts {
    ident: syn::Ident,

    parent: String,
    #[darling(default, multiple, rename = "technology")]
    technologies: Vec<Technology>,
    /// Optional: role trait to implement (e.g., "Developer")
    #[darling(default)]
    role: Option<syn::Ident>,
    /// Optional: period lifetime bounds (e.g., 'f2024 for 'period: 'f2024)
    #[darling(default, multiple, rename = "period")]
    periods: Vec<PeriodLifetime>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(resume), supports(struct_any))]
struct ResumeOpenSourceOpts {
    ident: syn::Ident,

    role: String,
    url: String,
    #[darling(default)]
    language: String,
}

/// Derive macro for work sections
///
/// Usage:
/// ```ignore
/// #[derive(ResumeWork)]
/// #[resume(
///     company = "Company Name",
///     url = "https://example.com",
///     role = "Software Engineer",
///     period = "2024-present"
/// )]
/// struct CompanyName;
/// ```
#[proc_macro_derive(ResumeWork, attributes(resume))]
pub fn derive_resume_work(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ResumeWorkOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let docs = extract_docs(&input.attrs);
    let company = &opts.company;
    let url = &opts.url;
    let role = &opts.role;
    let period = &opts.period;

    let expanded = quote! {
        // Submit to inventory for template generation
        inventory::submit! {
            crate::resume_data::WorkSection {
                key: #company,
                company: #company,
                url: #url,
                role: #role,
                period: #period,
                description: #docs,
                projects: vec![],
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for projects within work sections
///
/// Automatically generates trait implementations for all technologies and role.
///
/// Usage:
/// ```ignore
/// #[derive(ResumeProject)]
/// #[resume(
///     parent = "CompanyName",
///     role = Developer,
///     period = "'f2024",  // Optional: adds 'period: 'f2024 bound to role impl
///     technology(frameworks::Tokio),  // Framework - uses default description
///     technology(frameworks::Axum, "Custom description for Axum"),  // Custom description
///     technology(languages::Rust, "Main programming language"),
///     technology(databases::Postgresql),
///     technology(messages::RabbitMQ)
/// )]
/// #[achievements(
///     "Implemented feature X",
///     "Optimized performance by 50%"
/// )]
/// struct ProjectName;
/// ```
#[proc_macro_derive(ResumeProject, attributes(resume, achievements))]
pub fn derive_resume_project(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ResumeProjectOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let ident = &opts.ident;
    let name = opts.ident.to_string();
    let docs = extract_docs(&input.attrs);
    let parent = &opts.parent;
    let technologies = &opts.technologies;

    // Collect all tech strings for inventory
    let tech_strings: Vec<String> = technologies.iter()
        .map(|t| {
            // Get the last segment of the path as the name
            t.path.segments.last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default()
        })
        .collect();

    let achievements = extract_achievements(&input.attrs);
    let role = &opts.role;
    let periods = &opts.periods;

    // Generate trait implementations
    let trait_impls = {
        let tech_traits: Vec<_> = technologies.iter().map(|tech| {
            let tech_path = &tech.path;
            let tech_name = tech.path.segments.last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default();
            let tech_doc = tech.description.as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| get_technology_doc(&tech_name));
            quote! {
                #[doc = #tech_doc]
                impl crate::technologies::#tech_path for #ident {}
            }
        }).collect();

        let role_impl = if let Some(role_ident) = role {
            // Generate achievements list for trait impl docstring
            let role_doc = if !achievements.is_empty() {
                let mut doc = String::from("Main responsibilities and achievements:\n");
                for achievement in &achievements {
                    doc.push_str(&format!("* {}\n", achievement));
                }
                doc
            } else {
                String::new()
            };

            // Derive the language bound from the declared `technology(languages::*)`
            // entries instead of hard-coding `Rust`. A project tagged with
            // `languages::Python` + `languages::Erlang` yields `Language: Python + Erlang`,
            // matching the hand-written impls.
            let language_bound = {
                let language_paths: Vec<&syn::Path> = technologies
                    .iter()
                    .map(|t| &t.path)
                    .filter(|p| {
                        p.segments
                            .first()
                            .map(|s| s.ident == "languages")
                            .unwrap_or(false)
                    })
                    .collect();
                if language_paths.is_empty() {
                    None
                } else {
                    let bounds = language_paths
                        .iter()
                        .map(|p| quote! { crate::technologies::#p });
                    Some(quote! { Language: #(#bounds)+* })
                }
            };

            // Impl generics: a `Language` type parameter plus one named lifetime per
            // declared period (e.g. `<'period, 'f2017, 't2018, Language>`).
            let generics = if periods.is_empty() {
                quote! { <'period, Language> }
            } else {
                let lifetimes = periods.iter().map(|p| &p.0);
                quote! { <'period, #(#lifetimes,)* Language> }
            };

            // Where-clause: the derived language bound (if any) followed by one
            // `'period: 'fXXXX` bound per declared period lifetime.
            let mut predicates: Vec<proc_macro2::TokenStream> = Vec::new();
            if let Some(bound) = language_bound {
                predicates.push(bound);
            }
            for period in periods {
                let lifetime = &period.0;
                predicates.push(quote! { 'period: #lifetime });
            }
            let where_clause = if predicates.is_empty() {
                quote! {}
            } else {
                quote! { where #(#predicates),* }
            };

            // Only add doc attribute if there's actual content
            let doc_attr = if !role_doc.is_empty() {
                quote! { #[doc = #role_doc] }
            } else {
                quote! {}
            };

            quote! {
                #doc_attr
                #[allow(clippy::extra_unused_lifetimes)]
                impl #generics crate::roles::#role_ident<Language> for &'period #ident
                #where_clause
                {
                }
            }
        } else {
            quote! {}
        };

        quote! {
            #(#tech_traits)*
            #role_impl
        }
    };

    // Convert to string literals for inventory (which creates statics)
    let tech_lit_strings: Vec<_> = tech_strings.iter().collect();

    let expanded = quote! {
        // Submit to inventory for template generation
        inventory::submit! {
            crate::resume_data::Project {
                name: #name,
                description: #docs,
                achievements: &[#(#achievements),*],
                technologies: &[#(#tech_lit_strings),*],
            }
        }

        // Also submit a marker to link to parent
        inventory::submit! {
            crate::resume_data::ProjectParent {
                project_name: #name,
                parent_company: #parent,
            }
        }

        // Generate trait implementations
        #trait_impls

        // Clean up auto-traits from documentation
        #[doc(hidden)]
        impl !Send for #ident {}
        #[doc(hidden)]
        impl !Sync for #ident {}
        #[doc(hidden)]
        impl !Unpin for #ident {}
        #[doc(hidden)]
        impl !core::panic::RefUnwindSafe for #ident {}
        #[doc(hidden)]
        impl !core::panic::UnwindSafe for #ident {}
        #[doc(hidden)]
        impl !std::marker::Freeze for #ident {}
    };

    TokenStream::from(expanded)
}

/// Derive macro for opensource projects
///
/// Usage:
/// ```ignore
/// #[derive(ResumeOpenSource)]
/// #[resume(
///     role = "Author",
///     url = "https://github.com/user/project",
///     language = "Rust"
/// )]
/// struct ProjectName<R: Role = Author>(pub R);
/// ```
#[proc_macro_derive(ResumeOpenSource, attributes(resume))]
pub fn derive_resume_opensource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match ResumeOpenSourceOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let name = opts.ident.to_string();
    let docs = extract_docs(&input.attrs);
    let role = &opts.role;
    let url = &opts.url;
    let language = &opts.language;

    let expanded = quote! {
        // Submit to inventory for template generation
        inventory::submit! {
            crate::resume_data::OpenSourceProject {
                name: #name,
                url: #url,
                role: #role,
                language: #language,
                description: #docs,
            }
        }
    };

    TokenStream::from(expanded)
}

// Helper functions

fn extract_docs(attrs: &[syn::Attribute]) -> String {
    attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let syn::Meta::NameValue(nv) = &attr.meta {
                    if let syn::Expr::Lit(expr_lit) = &nv.value {
                        if let syn::Lit::Str(lit) = &expr_lit.lit {
                            return Some(lit.value().trim().to_string());
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_achievements(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut achievements = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("achievements") {
            // Parse as a parenthesized list
            if let Ok(content) = attr.meta.require_list() {
                // Parse the token stream properly to handle both regular and raw string literals
                let parser = syn::punctuated::Punctuated::<syn::Lit, syn::Token![,]>::parse_terminated;
                if let Ok(literals) = parser.parse2(content.tokens.clone()) {
                    for lit in literals {
                        if let syn::Lit::Str(lit_str) = lit {
                            let value = lit_str.value();

                            // Check if this is a multiline string (contains newlines)
                            if value.contains('\n') {
                                // Split by newlines and treat each non-empty line as a separate achievement
                                for line in value.lines() {
                                    let trimmed = line.trim();
                                    if !trimmed.is_empty() {
                                        achievements.push(trimmed.to_string());
                                    }
                                }
                            } else {
                                // Regular single achievement
                                achievements.push(value);
                            }
                        }
                    }
                }
            }
        }
    }

    achievements
}

fn get_technology_doc(tech_name: &str) -> &'static str {
    match tech_name {
        "Tokio" => "Async runtime for concurrent operations.",
        "Axum" => "Web framework for HTTP server.",
        "Juniper" => "GraphQL server implementation.",
        "AsyncGraphQL" => "GraphQL server implementation with async support.",
        "Tracing" => "Observability and distributed tracing.",
        "DataFusion" => "SQL query engine and execution framework.",
        "Actix" => "Actor framework and web server.",
        "ActixWeb" => "Web framework built on Actix.",
        "Serde" => "Serialization and deserialization framework.",
        "Diesel" => "ORM and query builder for databases.",
        "SQLx" => "Async SQL toolkit.",
        "Tonic" => "gRPC framework.",
        "Rocket" => "Web framework for Rust.",
        "Warp" => "Web server framework.",
        "Tower" => "Library of modular and reusable components for building networking clients and servers.",
        "Hyper" => "HTTP library.",
        "Reqwest" => "HTTP client.",
        "Poem" => "Web framework for HTTP services.",
        "Kafka" => "Message broker for event streaming.",
        "Criterion" => "Performance benchmarking framework.",
        "Cucumber" => "BDD testing framework.",
        "AsyncIO" => "Asynchronous I/O framework for Python.",
        "AioHTTP" => "Async HTTP client/server framework for asyncio.",
        "Celery" => "Distributed task queue for Python.",
        "Django" => "High-level Python web framework.",
        "FastAPI" => "Modern, fast web framework for building APIs with Python.",
        "Flask" => "Lightweight WSGI web application framework.",
        "Qt" => "GUI framework for desktop applications.",
        "OpenGL" => "Graphics API for 3D rendering.",
        _ => "Technology used in this project.",
    }
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
//     #[project(parent = Rambler)]
//     #[role(Developer, span(2017, 2018), langs(Python), doc = "…")]
//     #[role(TeamLead,  span(2018, 2022), langs(Rust, Python), doc = "…")]
//     #[tech(frameworks::Actix, "Actor & web framework.")]
//     pub struct News;
// ============================================================================

/// Parsed `#[project(parent = Company)]`.
#[derive(FromDeriveInput)]
#[darling(attributes(project), supports(struct_any))]
struct ProjectOpts {
    ident: syn::Ident,
    parent: syn::Path,
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

/// The two `inventory::submit!` blocks feeding the template/binary views.
struct ProjectInventory {
    name: String,
    parent: String,
    docs: String,
    /// Per-role prose (the `doc` of each `#[role(...)]`), rendered as the
    /// project's highlights in the markdown/HTML views.
    highlights: Vec<String>,
    tech_names: Vec<String>,
}

impl ToTokens for ProjectInventory {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name, parent, docs, highlights, tech_names } = self;
        let highlights = highlights.iter();
        let techs = tech_names.iter();
        tokens.extend(quote! {
            inventory::submit! {
                crate::resume_data::Project {
                    name: #name,
                    description: #docs,
                    achievements: &[#(#highlights),*],
                    technologies: &[#(#techs),*],
                }
            }
            inventory::submit! {
                crate::resume_data::ProjectParent {
                    project_name: #name,
                    parent_company: #parent,
                }
            }
        });
    }
}

/// Derive macro for projects within work sections.
///
/// Builds the rustdoc trait impls (technologies + one impl per `#[role(...)]`)
/// and the inventory submissions from attributes. See the module comment above
/// for the attribute syntax.
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
    let name = ident.to_string();
    let docs = extract_docs(&input.attrs);
    let parent = opts
        .parent
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();

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

    let tech_names: Vec<String> = techs
        .iter()
        .map(|tech| {
            tech.path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap_or_default()
        })
        .collect();
    let highlights: Vec<String> = roles
        .iter()
        .map(|spec| spec.doc.clone())
        .filter(|doc| !doc.is_empty())
        .collect();
    let inventory = ProjectInventory { name, parent, docs, highlights, tech_names };
    let cleanup = AutoTraitCleanup { ident };

    TokenStream::from(quote! {
        #inventory
        #(#tech_impls)*
        #(#role_impls)*
        #cleanup
    })
}

/// Parsed `#[company(key = Ident, name = "...", url = "...", role = "...", period = "...")]`.
#[derive(FromDeriveInput)]
#[darling(attributes(company), supports(struct_any))]
struct CompanyOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    key: syn::Path,
    name: String,
    url: String,
    role: String,
    period: String,
}

/// Derive macro for a company / work section.
///
/// Submits a [`crate::resume_data::WorkSection`] to inventory. `key` is the join
/// key matched against each project's `parent`; the struct's doc comment becomes
/// the section description. Apply it to a private marker struct.
///
/// ```ignore
/// #[derive(Company)]
/// #[company(key = Rambler, name = "Rambler&Co", url = "https://rambler.ru",
///           role = "Team Lead & Software Engineer", period = "2015-2022")]
/// /// One of the biggest media holdings in Russia.
/// struct Rambler;
/// ```
#[proc_macro_derive(Company, attributes(company))]
pub fn derive_company(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let opts = match CompanyOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let key = opts
        .key
        .segments
        .last()
        .map(|s| s.ident.to_string())
        .unwrap_or_default();
    let name = &opts.name;
    let url = &opts.url;
    let role = &opts.role;
    let period = &opts.period;
    let docs = extract_docs(&input.attrs);

    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::WorkSection {
                key: #key,
                company: #name,
                url: #url,
                role: #role,
                period: #period,
                description: #docs,
                projects: Vec::new(),
            }
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

/// Drop a leading Markdown link such as `[name](url) - ` from an open-source doc
/// comment, since the rendered heading already shows the linked project name.
fn strip_leading_link(doc: &str) -> String {
    let trimmed = doc.trim_start();
    if !trimmed.starts_with('[') {
        return doc.to_string();
    }
    let Some(text_end) = trimmed.find("](") else {
        return doc.to_string();
    };
    let after_text = &trimmed[text_end + 2..];
    let Some(url_end) = after_text.find(')') else {
        return doc.to_string();
    };
    after_text[url_end + 1..]
        .trim_start()
        .trim_start_matches(|c: char| c == '-' || c == '—' || c == ':')
        .trim_start()
        .to_string()
}

/// Derive for an open-source project. Submits an [`crate::resume_data::OpenSourceProject`]
/// to inventory; the project name is the struct ident and the description is its doc comment.
#[proc_macro_derive(Oss, attributes(oss))]
pub fn derive_oss(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = match OssOpts::from_derive_input(&input) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    let name = opts.ident.to_string();
    // The heading already shows the linked name; drop a leading `[name](url) - `.
    let description = strip_leading_link(&extract_docs(&input.attrs));
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

/// Parsed `#[profile(name = "...", tagline = "...", email = "...", ...)]`.
#[derive(FromDeriveInput)]
#[darling(attributes(profile), supports(struct_any))]
struct ProfileOpts {
    #[allow(dead_code)]
    ident: syn::Ident,
    name: String,
    tagline: String,
    email: String,
    phone: String,
    telegram: String,
    github: String,
    linkedin: String,
    birthdate: String,
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
    let email = &opts.email;
    let phone = &opts.phone;
    let telegram = &opts.telegram;
    let github = &opts.github;
    let linkedin = &opts.linkedin;
    let birthdate = &opts.birthdate;
    let skills = skills.iter();
    TokenStream::from(quote! {
        inventory::submit! {
            crate::resume_data::Profile {
                name: #name,
                tagline: #tagline,
                email: #email,
                phone: #phone,
                telegram: #telegram,
                github: #github,
                linkedin: #linkedin,
                birthdate: #birthdate,
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
