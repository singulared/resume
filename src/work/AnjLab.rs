//! 2010-2015 [AnjLab](https://anjlab.com) is an outsourcing development company in Vladimir.
//!
//! AnjLab is a team of skilled software development professionals.
//! We are experienced in the development of performant, scalable and reliable solutions for companies around the world.
//! We also maintain and contribute to many open source projects.
use resume_macro::Project;

/// A [service](http://anjlab.com/en/portfolio-item/satago/) that lets companies submit data about how quickly their business customers pay them.
///
/// Other companies can pay for access to aggregated data on a company's payment behavior before doing business with it, as an alternative to a traditional business credit report.
#[deprecated(note = "Rewritten in Java")]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(Python),
    doc = "As a Python developer, I was responsible for the Sage integration (both as a service and as a standalone Windows desktop application)."
)]
#[tech(languages::Python)]
#[tech(frameworks::Qt)]
pub struct Satago;

/// Tools for analyzing meteorological data from different sources (satellite, radar).
///
/// The tool captures data from text files, binary files, and images.
/// The data is presented as layers that can be stacked on top of each other and shown as flat figures or isolines.
#[deprecated]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CPP),
    doc = "The whole development process, including choosing the technology stack."
)]
#[tech(languages::CPP)]
#[tech(frameworks::Qt)]
pub struct FlightMap;

/// CloudCube is a 3D OpenGL system for modeling and visualizing cloud data.
///
/// The tool can render cloud models in different visual styles, predict reagent dispersion, and save data for further analysis.
#[deprecated]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CPP),
    doc = "The whole development process, including choosing the technology stack."
)]
#[tech(languages::CPP)]
#[tech(frameworks::OpenGL)]
pub struct CloudCube;

/// [Hardware and software system](http://anjlab.com/en/portfolio-item/anjlab-flightmonitor/) for aircraft dispatching.
///
/// It transfers weather data for fog, hail and frost suppression and snow avalanche protection,
/// and handles data communication between aircraft and the base.
/// The system supports the exchange of special data such as digital maps, statistics, and meteorological reports.
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CS),
    doc = r#"* Integrated an error correction algorithm
* Infrastructure and installation support"#
)]
#[tech(languages::CS)]
pub struct FlightMonitor;
