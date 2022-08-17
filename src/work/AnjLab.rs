//! 2010-2015 [Anjlab](https://anjlab.com) is outsource development company in Vladimir city.
//!
//! AnjLab is a team of skilled software development professionals.
//! We are experienced in the development of performant, scalable and reliable solutions for companies around the world.
//! We also maintain and contribute to many open source projects.
use crate::{
    roles::Developer,
    technologies::{
        frameworks::{OpenGL, Qt},
        languages::{Python, CPP, CS},
    },
    CleanupDocs,
};

/// Is a [service](http://anjlab.com/en/portfolio-item/satago/) that allows companies to submit data about how quickly their business customers pay them.
///
/// Other companies can pay for access to aggregated data about companies, payment behavior before doing business with them, as an alternative to a traditional business credit report.  
#[derive(CleanupDocs)]
#[deprecated(note = "Rewritten on Java")]
pub struct Satago;
/// As a python developer, I was the conduct of Sage integration process (like a service and standalone desktop windows application).
impl<Language> Developer<Language> for Satago where Language: Python {}
impl Python for Satago {}
impl Qt for Satago {}

/// Tools for analysis of meteorological data taken from different sources (satellite, radar).
///
/// The tool allows capturing data from text, binary files, and images.
/// Different data present in the program as layers, which can be put into each other, showed as flat figures or isolines.
#[deprecated]
#[derive(CleanupDocs)]
pub struct FlightMap;
/// Whole development process and stack choosing.
impl<Language> Developer<Language> for FlightMap where Language: CPP {}
impl CPP for FlightMap {}
impl Qt for FlightMap {}

/// CloudCube is 3D OpenGL cloud data modeling and representation system.
///
/// The tool can represent cloud models in different visual types, predict reagent transmission and save data for the next analysis steps.
#[deprecated]
#[derive(CleanupDocs)]
pub struct CloudCube;
/// Whole development process and stack choosing.
impl<Language> Developer<Language> for CloudCube where Language: CPP {}
impl CPP for CloudCube {}
impl OpenGL for CloudCube {}

/// [Hardware-software complex](http://anjlab.com/en/portfolio-item/anjlab-flightmonitor/) for dispatching aircrafts.
///
/// Transferring data about weather state such as while fog, hail, frost suppression, protection against snow avalanches,
/// and for data communication between boards and the base.
/// The complex supports the exchange of special data such as digital maps, statistical information, and meteorological reports.
#[derive(CleanupDocs)]
pub struct FlightMonitor;
/// Integration of error correction algorithm.
/// Infrastructure and installation support.
impl<Language> Developer<Language> for FlightMonitor where Language: CS {}
