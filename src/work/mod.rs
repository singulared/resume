//! Information about previous works
use resume_macro::CleanupDocs;
use AnjLab::{CloudCube, FlightMap, FlightMonitor, Satago};
use KayCom::{CyberRead, PosterShop};
use Rambler::{AdBlock, MediaUp, News, Paywall, Video, Weather};

/// Work timeline since 2008
#[derive(CleanupDocs)]
pub enum WorkHistory {
    /// [Rambler:](self::Rambler) *2015 - 2022*  
    /// Team-lead, Rust and Python developer
    Rambler(News, MediaUp, Weather, Video, Paywall, AdBlock),
    /// [AnjLab:](self::AnjLab) *2010 - 2015*  
    /// Software engineer
    Anjlab(FlightMonitor, FlightMap, CloudCube, Satago),
    /// [Vladimir State University:](https://www.vlsu.ru/) *2012 - 2014*  
    /// Lecturer in computer science and cryptography
    VLSU,
    /// [KayCom:](self::KayCom) *2008 - 2010*  
    /// PHP and JS full-stack developer
    KayCom(PosterShop, CyberRead),
}

#[allow(non_snake_case)]
pub mod AnjLab;
#[allow(non_snake_case)]
pub mod KayCom;
#[allow(non_snake_case)]
pub mod Rambler;
