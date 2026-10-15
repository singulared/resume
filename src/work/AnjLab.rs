//! 2010-2015 [Anjlab](https://anjlab.com) is outsource development company in Vladimir city.
//!
//! AnjLab is a team of skilled software development professionals.
//! We are experienced in the development of performant, scalable and reliable solutions for companies around the world.
//! We also maintain and contribute to many open source projects.
use resume_macro::Project;

/// Is a [service](http://anjlab.com/en/portfolio-item/satago/) that allows companies to submit data about how quickly their business customers pay them.
///
/// Other companies can pay for access to aggregated data about companies, payment behavior before doing business with them, as an alternative to a traditional business credit report.
#[deprecated(note = "Rewritten on Java")]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(Python),
    doc = "As a python developer, I was the conduct of Sage integration process (like a service and standalone desktop windows application)."
)]
#[tech(languages::Python)]
#[tech(frameworks::Qt)]
pub struct Satago;

/// Tools for analysis of meteorological data taken from different sources (satellite, radar).
///
/// The tool allows capturing data from text, binary files, and images.
/// Different data present in the program as layers, which can be put into each other, showed as flat figures or isolines.
#[deprecated]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CPP),
    doc = "Whole development process and stack choosing."
)]
#[tech(languages::CPP)]
#[tech(frameworks::Qt)]
pub struct FlightMap;

/// CloudCube is 3D OpenGL cloud data modeling and representation system.
///
/// The tool can represent cloud models in different visual types, predict reagent transmission and save data for the next analysis steps.
#[deprecated]
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CPP),
    doc = "Whole development process and stack choosing."
)]
#[tech(languages::CPP)]
#[tech(frameworks::OpenGL)]
pub struct CloudCube;

/// [Hardware-software complex](http://anjlab.com/en/portfolio-item/anjlab-flightmonitor/) for dispatching aircrafts.
///
/// Transferring data about weather state such as while fog, hail, frost suppression, protection against snow avalanches,
/// and for data communication between boards and the base.
/// The complex supports the exchange of special data such as digital maps, statistical information, and meteorological reports.
#[derive(Project)]
#[project(parent = AnjLab)]
#[role(
    Developer,
    langs(CS),
    doc = r#"* Integration of error correction algorithm
* Infrastructure and installation support"#
)]
#[tech(languages::CS)]
pub struct FlightMonitor;
