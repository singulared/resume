// #![no_std]
#![feature(freeze)]
#![allow(internal_features)]
#![feature(doc_cfg)]
#![feature(freeze_impls)]
#![feature(negative_impls)]
#![feature(associated_const_equality)]
#![allow(deprecated)]
#![allow(unused)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/414425?v=4")]
#![doc = include_str!("../README.md")]

use conferences::{
    roles::ProgramDirector, DevConf2016, MoscowPythonMeetup2016, MoscowPythonPodcast2020,
    MoscowRustMeetup2021, PyCon2021, RamblerDevOps2021, RamblerPython2021, RustCon2021,
    RustCon2022, RustForPythonDevelopers,
};
use opensource::{
    ActixWebValidator, AioHTTP, AioRiak, Author, BashoDocs, ChromebookKernel, Conflow, Contributor,
    DataFusionPostgres, FlaskHmacAuth, FlaskRestful, Hitbox, Hitboxd, Poetry, PyJasperClient,
    Shortland, SlogStdlog, Swayboard, SwayipcRs, BB8,
};
#[doc(hidden)]
pub use resume_macro::CleanupDocs;

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
#[derive(CleanupDocs)]
pub struct Me;
/// My technical interests
impl TechnicalInterests for Me {}
/// Started getting interested in Rust in 2018.
///
/// From small cli tools and playground experiments to big opensource projects like
/// [Hitbox](https://github.com/hit-box/hitbox/) and
/// production applications like [News](crate::work::Rambler::News) and [MediaUp](crate::work::Rambler::MediaUp).
impl Rust for Me {}
/// Started to use Linux as main system since 2006.
///
/// Debian one Love (:
impl Linux for Me {}
/// In 2015 with Riak I opened the distributed systems world for myself. I continue exploring it nowadays, of course.
impl DistributedSystems for Me {}
/// CRDT, as I think, is one of the most interesting technologies of recent years.
impl CRDT for Me {}
/// One of my hobbies is microcontrollers and embedded development.
///
/// I started from porting Python3 and building kernel for Motorola a1200.
/// Porting linux on Toshiba ac100 arm smartbook.
/// Samsung ARM chromeebook hacks and development.
/// And now I develop for some PINE64 devices such as PinePhone, PinePhone PRO, PineTime and other
/// SBC platforms like NVidia Jetson Nano.
impl EmbeddedSystems for Me {}
/// I was really interested in Cryptography as a Ph.D student and wrote Ph.D dissertation about SMT
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
/// I am a tennis player since 2004.
impl Tennis for Me {}
/// Participant of Honeypot team in:
/// * RuCTF/RuCTFe
/// * iCTF
/// * Defcon
///
/// And other information security competitions.
impl CTF for Me {}

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
    /// *2021, 2022*: Biggest [Rust conference](https://rustcon.ru/) in Russia.
    RustCon(RustCon2021, RustCon2022),
    /// *2021*: Biggest [Python conference](https://pycon.ru/) in Russia.
    PyCon(PyCon2021),
    /// *2021*: Local [Moscow Rust community](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/279291922/).
    MoscowRustMeetup(MoscowRustMeetup2021),
    /// *2021, 2022, 2023*: Rust lecturer in Yandex School of Data Analysis.
    SHAD,
    /// *2016*: [Conference](https://devconf.ru) of professional web developers.
    DevConf(DevConf2016),
    /// *2016*: Local [Moscow python community](https://moscowpython.ru).
    MoscowPythonMeetup(MoscowPythonMeetup2016),
    /// *2015 - 2022*: Rambler company meetups.
    RamblerMeetup(
        RamblerDevOps2021,
        RamblerPython2021,
        RustForPythonDevelopers,
    ),
    /// *2020*: Podcast from founders of the biggest Russian python community [MoscowPython](https://www.facebook.com/groups/MoscowDjango/)
    MoscowPythonPodcast(MoscowPythonPodcast2020),
}

pub mod conferences;
pub mod hobbies;
pub mod opensource;
pub mod publications;
pub mod roles;
pub mod technologies;
pub mod work;
