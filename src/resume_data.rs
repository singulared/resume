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

/// Work experience section
#[derive(Debug, Clone, Serialize)]
pub struct WorkSection {
    /// Stable join key (the company ident, e.g. `Rambler`) matching
    /// [`ProjectParent::parent_company`]; not shown in output.
    pub key: &'static str,
    /// Display name, e.g. `Rambler&Co`.
    pub company: &'static str,
    pub url: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub description: &'static str,
    pub projects: Vec<Project>,
}

/// Project within a company
#[derive(Debug, Clone, Serialize)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub achievements: &'static [&'static str],
    pub technologies: &'static [&'static str],
}

/// Links a project to its parent company
#[derive(Debug, Clone)]
pub struct ProjectParent {
    pub project_name: &'static str,
    pub parent_company: &'static str,
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

// Inventory collections
inventory::collect!(WorkSection);
inventory::collect!(Project);
inventory::collect!(ProjectParent);
inventory::collect!(OpenSourceProject);
inventory::collect!(Conference);
inventory::collect!(Education);
inventory::collect!(Publication);
inventory::collect!(Interest);
inventory::collect!(Profile);
