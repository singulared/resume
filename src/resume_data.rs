//! Data structures for resume generation

use serde::Serialize;

/// Complete resume data structure
#[derive(Debug, Clone, Serialize)]
pub struct Resume {
    pub personal: PersonalInfo,
    pub work: Vec<WorkSection>,
    pub opensource: Vec<OpenSourceProject>,
    pub conferences: Vec<Conference>,
    pub education: Vec<Education>,
}

/// Personal information
#[derive(Debug, Clone, Serialize)]
pub struct PersonalInfo {
    pub name: &'static str,
    pub tagline: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub telegram: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub birthdate: &'static str,
    pub skills: Vec<&'static str>,
}

/// Work experience section: one `WorkHistory` variant.
#[derive(Debug, Clone, Serialize)]
pub struct WorkSection {
    /// Display name, e.g. `Rambler&Co`.
    pub company: &'static str,
    pub url: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub description: &'static str,
    /// In `WorkHistory` tuple order.
    pub projects: &'static [Project],
}

/// Project within a company
#[derive(Debug, Clone, Serialize)]
pub struct Project {
    /// The struct ident, e.g. `DomainEngine`.
    pub key: &'static str,
    /// Display name, e.g. `Domain Engine`.
    pub name: &'static str,
    pub description: &'static str,
    /// Some role is still running (an open-ended span).
    pub ongoing: bool,
    pub roles: &'static [ProjectRole],
    pub technologies: &'static [&'static str],
}

/// One documented `#[role(...)]` of a project.
#[derive(Debug, Clone, Serialize)]
pub struct ProjectRole {
    /// `2017-2018`, `2024-present`, `2017`, or empty without a span.
    pub period: &'static str,
    pub doc: &'static str,
}

/// Implemented by `#[derive(Project)]`.
pub trait WorkProject {
    const PROJECT: Project;
}

/// Implemented by `#[derive(Work)]` on the work-history enum: one section per
/// variant, in declaration order.
pub trait WorkRoot {
    const SECTIONS: &'static [WorkSection];
}

/// Display name of a technology trait, implemented for `dyn Trait` by the
/// `technologies!` macro.
pub trait Technology {
    const NAME: &'static str;
}

/// Open source project
#[derive(Debug, Clone, Serialize)]
pub struct OpenSourceProject {
    pub name: &'static str,
    pub url: &'static str,
    pub role: &'static str, // "Author" or "Contributor"
    /// Manual display order within the role group (lower first).
    pub rank: u32,
    pub language: &'static str,
    pub description: &'static str,
}

/// Conference or speaking engagement
#[derive(Debug, Clone, Serialize)]
pub struct Conference {
    pub name: &'static str,
    pub year: &'static str,
    pub role: &'static str, // "Speaker", "Attendee", "Organizer"
    pub description: &'static str,
    pub url: &'static str, // Use empty string instead of Option
}

/// Education entry
#[derive(Debug, Clone, Serialize)]
pub struct Education {
    pub institution: &'static str,
    pub degree: &'static str,
    pub field: &'static str,
    pub period: &'static str,
    pub description: &'static str,
}

/// Publication or article
#[derive(Debug, Clone, Serialize)]
pub struct Publication {
    pub title: &'static str,
    pub url: &'static str,
    pub year: &'static str,
    pub description: &'static str,
}

/// Personal interest or hobby
#[derive(Debug, Clone, Serialize)]
pub struct Interest {
    pub kind: &'static str, // "tech" or "hobby"
    pub name: &'static str,
    pub description: &'static str,
}

/// Profile / contact information submitted to inventory. `skills` is a static
/// slice (not a `Vec`) so the whole record is const-constructible in `submit!`.
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: &'static str,
    pub tagline: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub telegram: &'static str,
    pub github: &'static str,
    pub linkedin: &'static str,
    pub birthdate: &'static str,
    pub skills: &'static [&'static str],
}

// Inventory collections (work sections come from `WorkRoot` instead)
inventory::collect!(OpenSourceProject);
inventory::collect!(Conference);
inventory::collect!(Education);
inventory::collect!(Publication);
inventory::collect!(Interest);
inventory::collect!(Profile);
