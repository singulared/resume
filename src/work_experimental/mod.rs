//! Information about previous works - experimental version with new derive macros
//!
//! ## Career Timeline
//!
#![doc = simple_mermaid::mermaid!("timeline.mmd")]
//!
//! <script type="module">
//!   import mermaid from "https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs";
//!   const doc_theme = localStorage.getItem("rustdoc-theme");
//!   const theme = (doc_theme === "dark" || doc_theme === "ayu") ? "dark" : "default";
//!   mermaid.initialize({ startOnLoad: true, theme: theme });
//!   window.addEventListener('DOMContentLoaded', (event) => { mermaid.run(); });
//! </script>
use resume_macro::CleanupDocs;
use AnjLab::{CloudCube, FlightMap, FlightMonitor, Satago};
use DigitalWave::{DomainEngine, OLAP};
use KayCom::{CyberRead, PosterShop};
use Rambler::{AdBlock, Horoscopes, MediaUp, News, Paywall, Video, Weather};
use RaskAI::{Backend, API};
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
    Rambler(News, MediaUp, Weather, Video, Paywall, AdBlock, Horoscopes),
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

/// Company-level work sections submitted to inventory for the template/markdown
/// views. Private (kept out of rustdoc); the `key` matches each project's `parent`.
mod companies {
    use resume_macro::Company;

    #[derive(Company)]
    #[company(
        key = DigitalWave,
        name = "Digital Wave Technology",
        url = "https://www.digitalwavetechnology.com/",
        role = "Core Software Engineer",
        period = "2024-present"
    )]
    /// AI-native platform for consumer brands (the ONE Platform with master data
    /// management at its core). Core Software Engineer on the OLAP database and
    /// Domain Engine solutions.
    struct DigitalWave;

    #[derive(Company)]
    #[company(
        key = RaskAI,
        name = "Rask AI",
        url = "https://www.rask.ai",
        role = "Senior Python Engineer",
        period = "2023-2024"
    )]
    /// AI-powered video localization and dubbing platform serving 2M+ users with
    /// translation and dubbing into 130+ languages. Senior Python Engineer on the
    /// main backend product and the dedicated enterprise API.
    struct RaskAI;

    #[derive(Company)]
    #[company(
        key = Workato,
        name = "Workato",
        url = "https://www.workato.com",
        role = "Senior Rust Engineer",
        period = "2022-2023"
    )]
    /// Enterprise integration and automation platform (iPaaS). Senior Rust Engineer
    /// focused on performance, benchmarking, and observability for core
    /// infrastructure services backing the main product.
    struct Workato;

    #[derive(Company)]
    #[company(
        key = Rambler,
        name = "Rambler&Co",
        url = "https://rambler.ru",
        role = "Team Lead & Software Engineer",
        period = "2015-2022"
    )]
    /// One of the biggest media holdings in Russia. Started as a Middle Python
    /// developer in 2015 and finished as Head of the Media Development team in 2022.
    struct Rambler;

    #[derive(Company)]
    #[company(
        key = VLSU,
        name = "Vladimir State University",
        url = "https://www.vlsu.ru/",
        role = "Lecturer",
        period = "2012-2014"
    )]
    /// Lecturer in computer science and cryptography.
    struct VLSU;

    #[derive(Company)]
    #[company(
        key = AnjLab,
        name = "AnjLab",
        url = "https://anjlab.com",
        role = "Software Engineer",
        period = "2010-2015"
    )]
    /// Outsource development company in Vladimir, building performant and reliable
    /// solutions for companies around the world.
    struct AnjLab;

    #[derive(Company)]
    #[company(
        key = KayCom,
        name = "KayCom",
        url = "https://kaycom.ru",
        role = "PHP/JavaScript Developer",
        period = "2008-2010"
    )]
    /// Web studio in Vladimir with a mobile development team and many projects in
    /// Russia and around the world.
    struct KayCom;
}
