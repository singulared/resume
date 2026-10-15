//! Publications and articles
use resume_macro::{CleanupDocs, Publication};

/// Link to article or publication
pub trait Link<Language> {}

/// Series of articles "[Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)" created in collaboration with Andrey Ermilov.
///
/// In these articles, we talk about why we moved away from our usual technology stack and show the advantages Rust has over Python.
/// * First part ([Text version](https://habr.com/ru/company/rambler_group/blog/533268/))
///     * Types
///     * User types and polymorphism
///     * Enums
///     * Option & Result
///     * Pattern matching
///     * Traits and Protocols
///     * Generic programming
/// * Second part ([Text version](https://habr.com/ru/company/rambler_group/blog/535234/))
///     * Multithreading
///     * Asynchronous
///     * Functional programming
///     * Conclusion
///
/// There is also a [video version](crate::conferences::RustForPythonDevelopers).
#[derive(CleanupDocs, Publication)]
#[publication(
    title = "Rust for Python developers",
    url = "https://habr.com/ru/company/rambler_group/blog/533268/",
    year = "2020"
)]
pub struct RustForPythonDevelopers;
/// [Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)
impl<Ru> Link<Ru> for RustForPythonDevelopers {}
