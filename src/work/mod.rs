//! Information about previous works
use resume_macro::CleanupDocs;
use AnjLab::{CloudCube, FlightMap, FlightMonitor, Satago};
use DigitalWave::{DomainEngine, OLAP};
use KayCom::{CyberRead, PosterShop};
use Rambler::{AdBlock, MediaUp, News, Paywall, Video, Weather};
use RaskAI::{API, Backend};
use Workato::{WorkatoDB, WorkatoFiles, WorkatoStreams};

/// Work timeline since 2008
#[derive(CleanupDocs)]
pub enum WorkHistory {
    /// [Digital Wave Technology:](self::DigitalWave) *2024 - present*
    /// Core Software Engineer
    DigitalWave(OLAP, DomainEngine),
    /// [Rask AI:](self::RaskAI) *2023 - 2024*
    /// Senior Python Engineer
    RaskAI(Backend, API),
    /// [Workato:](self::Workato) *2022 - 2023*
    /// Senior Rust Engineer
    Workato(WorkatoDB, WorkatoFiles, WorkatoStreams),
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
pub mod DigitalWave;
#[allow(non_snake_case)]
pub mod KayCom;
#[allow(non_snake_case)]
pub mod Rambler;
#[allow(non_snake_case)]
pub mod RaskAI;
#[allow(non_snake_case)]
pub mod Workato;
