// #![no_std]
#![feature(freeze)]
#![allow(internal_features)]
#![feature(doc_cfg)]
#![feature(freeze_impls)]
#![feature(negative_impls)]
#![allow(deprecated)]
#![allow(unused)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/414425?v=4")]
#![doc = include_str!("../README.md")]

use conferences::{
    DevConf2016, MoscowPythonMeetup2016, MoscowPythonPodcast2020, MoscowRustMeetup2021, PyCon2021,
    RamblerDevOps2021, RamblerPython2021, RustCon2021, RustCon2022, RustForPythonDevelopers,
    roles::ProgramDirector,
};
use opensource::{
    ActixWebValidator, AioHTTP, AioRiak, Author, BB8, BashoDocs, ChromebookKernel, Conflow,
    Contributor, DataFusionPostgres, FlaskHmacAuth, FlaskRestful, Hitbox, Hitboxd, Moka, Poetry,
    PyJasperClient, Shortland, SlogStdlog, Swayboard, SwayipcRs,
};
#[doc(hidden)]
pub use resume_macro::CleanupDocs;
use resume_macro::Profile;

// Data model behind the markdown/HTML/PDF views; public for the binary, hidden
// from the rustdoc resume.
#[doc(hidden)]
pub mod resume_data;

/// [@singulared](https://t.me/singulared)
pub const TELEGRAM: &str = "@singulared";

/// [https://github.com/singulared](https://github.com/singulared)
pub const GITHUB: &str = "https://github.com/singulared";

/// [https://www.linkedin.com/in/singulared/](https://www.linkedin.com/in/singulared/)
pub const LINKEDIN: &str = "https://www.linkedin.com/in/singulared/";

/// [work@singulared.space](mailto:work@singulared.space)
pub const EMAIL: &str = "work@singulared.space";

/// [+79190007134](tel:+79190007134)
pub const PHONE: u64 = 79190007134;

/// 28.12.1988
pub const BIRTHDATE: &str = "28-12-1988";

/// [Source code](https://github.com/singulared/resume/) of this resume
pub const SOURCES: &str = "https://github.com/singulared/resume/";

/// [Boring version](https://docs.google.com/document/d/1O2OFZn_dK-gHszWc85DdHQtlAM3IX7Vf4_72tqOLD6I/edit?usp=sharing) of this resume
pub const RESUME: &str = "https://docs.google.com/document/d/1O2OFZn_dK-gHszWc85DdHQtlAM3IX7Vf4_72tqOLD6I/edit?usp=sharing";

use hobbies::{Biking, EmbeddedSystems, Music, Piano, Tennis, CTF};
use technologies::{
    languages::Rust, tools::Linux, Cryptography, DistributedSystems, Hobbies, TechnicalInterests,
    CRDT,
};
#[doc(inline)]
pub use work::WorkHistory;

/// Information about me and my personal interests.
#[derive(CleanupDocs, Profile)]
#[profile(
    name = "Maxim Belousov",
    tagline = "Rustacean, Crypto-anarchist and Techno-punk",
    // Keep in sync with the intro of README.md (`resume check` verifies).
    summary = "Systems and backend engineer since 2008, working mostly in Rust (since 2018) and Python. I build databases, caching layers and high-load services. I led the rewrite of Rambler News, moving its core API to Rust and cutting latency by about 10× and resource costs by about 20×. Author of the Hitbox caching framework and one of the program directors of RustCon Russia. Now working on the OLAP database and Domain Engine at Digital Wave Technology.",
    email = "work@singulared.space",
    phone = "+7 919 000 7134",
    telegram = "@singulared",
    github = "https://github.com/singulared",
    linkedin = "https://www.linkedin.com/in/singulared/"
)]
#[skill("Rust")]
#[skill("Python")]
#[skill("Distributed Systems")]
#[skill("Linux")]
#[skill("PostgreSQL")]
#[skill("ClickHouse")]
pub struct Me;
/// My technical interests
impl TechnicalInterests for Me {}
/// I got interested in Rust in 2018.
///
/// From small CLI tools and playground experiments to big open-source projects like
/// [Hitbox](https://github.com/hit-box/hitbox/) and
/// production applications like [News](crate::work::Rambler::News) and [MediaUp](crate::work::Rambler::MediaUp).
impl Rust for Me {}
/// I have been using Linux as my main system since 2006.
///
/// Debian, one love (:
impl Linux for Me {}
/// In 2015, Riak opened up the world of distributed systems for me, and I keep exploring it to this day.
impl DistributedSystems for Me {}
/// I think CRDTs are one of the most interesting technologies of recent years.
impl CRDT for Me {}
/// One of my hobbies is microcontrollers and embedded development.
///
/// I started by porting Python 3 and building a kernel for the Motorola A1200,
/// then ported Linux to the Toshiba AC100 ARM smartbook and hacked on the Samsung ARM Chromebook.
/// Now I develop for PINE64 devices such as the PinePhone, PinePhone Pro and PineTime, and for
/// SBC platforms like the NVIDIA Jetson Nano.
impl EmbeddedSystems for Me {}
/// I was really interested in cryptography as a Ph.D. student and wrote my Ph.D. dissertation on SMT
/// algorithms.
impl Cryptography for Me {}

/// My hobbies
impl Hobbies for Me {}

/// I really like cross-country biking.
impl Biking for Me {}
/// I am a musician.
impl Music for Me {}
/// I am a piano player.
impl Piano for Me {}
/// I have been playing tennis since 2004.
impl Tennis for Me {}
/// Member of the Honeypot team at:
/// * RuCTF/RuCTFe
/// * iCTF
/// * Defcon
///
/// And other information security competitions.
impl CTF for Me {}

/// Personal interests and hobbies submitted to inventory for the template/markdown
/// views. Private (kept out of rustdoc); mirrors the trait-impl docs on [`Me`].
mod interests {
    use resume_macro::Interest;

    #[derive(Interest)]
    #[interest(kind = "tech", name = "Rust", desc = "Into Rust since 2018 — from CLI tools and playground experiments to open-source projects like Hitbox and production systems.")]
    #[interest(kind = "tech", name = "Distributed systems", desc = "Exploring distributed systems since 2015 (starting with Riak); CRDTs are a particular interest.")]
    #[interest(kind = "tech", name = "Cryptography", desc = "Studied cryptography as a Ph.D. student; dissertation on SMT algorithms.")]
    #[interest(kind = "tech", name = "Embedded systems", desc = "Microcontrollers and embedded development — kernels for Motorola A1200 and Toshiba AC100, Samsung ARM Chromebook, and PINE64 devices.")]
    #[interest(kind = "hobby", name = "Cross-country biking", desc = "I really like cross-country biking.")]
    #[interest(kind = "hobby", name = "Music & piano", desc = "Musician and piano player.")]
    #[interest(kind = "hobby", name = "Tennis", desc = "Tennis player since 2004.")]
    #[interest(kind = "hobby", name = "CTF", desc = "Honeypot team — RuCTF/RuCTFe, iCTF, Defcon and other information-security competitions.")]
    struct Interests;
}

/// Education history
#[derive(CleanupDocs)]
pub enum EducationHistory {
    /// *2011 - 2014* Ph.D. Student (System analysis, management, and information processing)  
    /// Vladimir State University named after Alexander and Nikolay Stoletovs
    PhDStudent,
    /// *2006 - 2011* Information Security Specialist (Complex security of information objects)  
    /// Vladimir State University named after Alexander and Nikolay Stoletovs
    Specialist,
}

/// Education entries submitted to inventory for the template/markdown views.
/// Private (kept out of rustdoc), mirroring [`EducationHistory`].
mod education_inventory {
    use resume_macro::Education;

    #[derive(Education)]
    #[education(
        institution = "Vladimir State University",
        degree = "Ph.D. Student",
        field = "System analysis, management, and information processing",
        period = "2011-2014"
    )]
    /// Ph.D. research on SMT algorithms and applied cryptography.
    struct PhdStudent;

    #[derive(Education)]
    #[education(
        institution = "Vladimir State University",
        degree = "Information Security Specialist",
        field = "Complex security of information objects",
        period = "2006-2011"
    )]
    /// Specialist degree in complex security of information objects.
    struct Specialist;
}

/// Opensource projects
#[derive(CleanupDocs)]
pub enum OpenSource {
    /// Hitbox caching framework
    Hitbox(Hitbox<Author>),
    /// Hitboxd caching reverse proxy
    Hitboxd(Hitboxd<Author>),
    /// Actix-web validator library
    ActixWebValidator(ActixWebValidator<Author>),
    /// AioRiak async client
    AioRiak(AioRiak<Author>),
    /// AioHTTP framework contribution
    AioHTTP(AioHTTP<Contributor>),
    /// Conflow configuration manager
    Conflow(Conflow<Author>),
    /// Shortland URL shortener
    Shortland(Shortland<Author>),
    /// Swayboard layout manager
    Swayboard(Swayboard<Author>),
    /// Chromebook kernel fork
    ChromebookKernel(ChromebookKernel<Author>),
    /// DataFusion-Postgres contribution
    DataFusionPostgres(DataFusionPostgres<Contributor>),
    /// Swayipc-rs contribution
    SwayipcRs(SwayipcRs<Contributor>),
    /// Poetry contribution
    Poetry(Poetry<Contributor>),
    /// Slog-stdlog contribution
    SlogStdlog(SlogStdlog<Contributor>),
    /// Basho docs contribution
    BashoDocs(BashoDocs<Contributor>),
    /// BB8 connection pool contribution
    BB8(BB8<Contributor>),
    /// Moka concurrent cache contribution
    Moka(Moka<Contributor>),
    /// Flask-RESTful contribution
    FlaskRestful(FlaskRestful<Contributor>),
    /// PyJasperClient contribution
    PyJasperClient(PyJasperClient<Contributor>),
    /// Flask-HMACAuth contribution
    FlaskHmacAuth(FlaskHmacAuth<Contributor>),
}

/// Conferences & meet-up
#[derive(CleanupDocs)]
pub enum ConferencesHistory {
    /// *2021, 2022*: The biggest [Rust conference](https://rustcon.ru/) in Russia.
    RustCon(RustCon2021, RustCon2022),
    /// *2021*: The biggest [Python conference](https://pycon.ru/) in Russia.
    PyCon(PyCon2021),
    /// *2021*: Local [Moscow Rust community](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/279291922/).
    MoscowRustMeetup(MoscowRustMeetup2021),
    /// *2021, 2022, 2023*: Rust lecturer at the Yandex School of Data Analysis.
    SHAD,
    /// *2016*: [Conference](https://devconf.ru) of professional web developers.
    DevConf(DevConf2016),
    /// *2016*: Local [Moscow Python community](https://moscowpython.ru).
    MoscowPythonMeetup(MoscowPythonMeetup2016),
    /// *2015 - 2022*: Rambler company meetups.
    RamblerMeetup(
        RamblerDevOps2021,
        RamblerPython2021,
        RustForPythonDevelopers,
    ),
    /// *2020*: A podcast by the founders of the biggest Russian Python community, [MoscowPython](https://www.facebook.com/groups/MoscowDjango/).
    MoscowPythonPodcast(MoscowPythonPodcast2020),
}

pub mod conferences;
pub mod hobbies;
pub mod opensource;
pub mod publications;
pub mod roles;
pub mod technologies;
pub mod work;
