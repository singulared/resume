//! Publications and articles
use resume_macro::CleanupDocs;

/// Link to article or publication
pub trait Link<Language> {}

/// Series of articles "[Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)" created in collaboration with Andrey Ermolov.
///
/// In this articles we will talk about why we move away from the usual technology stack, and show what advantages Rust has compared to Python.
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
/// You can see video version: [crate::conferences::RustForPythonDevelopers]
#[derive(CleanupDocs)]
pub struct RustForPythonDevelopers;
/// [Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)
impl<Ru> Link<Ru> for RustForPythonDevelopers {}
