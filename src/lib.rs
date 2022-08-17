// #![no_std]
#![feature(doc_cfg)]
#![feature(negative_impls)]
#![feature(associated_const_equality)]
#![allow(deprecated)]
#![allow(unused)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/414425?v=4")]
//! # Maxim Belousov
//! Rustacean, Crypto-anarchist and Techno-punk.

use conferences::{
    roles::ProgramDirector, DevConf2016, MoscowPythonMeetup2016, MoscowPythonPodcast2020,
    MoscowRustMeetup2021, PyCon2021, RamblerDevOps2021, RamblerPython2021, RustCon2021,
    RustCon2022, RustForPythonDevelopers,
};
use opensource::{Author, Contributor};
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
/// Started interesting in Rust in 2018.
///
/// From small cli tools and playground experiments to big opensource projects like
/// [Hitbox](https://github.com/hit-box/hitbox/) and
/// production applications like [News](crate::work::Rambler::News) and [MediaUp](crate::work::Rambler::MediaUp).
impl Rust for Me {}
/// Started to use Linux as main system since 2006.
///
/// Debian one Love (:
impl Linux for Me {}
/// In 2015 with Riak I opened distributed systems world for myself. I continue opening it up nowadays of course.
impl DistributedSystems for Me {}
/// CRDT as i think is one of the most interesting technology of the last years.
impl CRDT for Me {}
/// One of my hobby is microcontrollers and embedded development.
///
/// I started from porting Python3 and building kernel for Motorola a1200.  
/// Porting linux on Toshiba ac100 arm smartbook.  
/// Samsung ARM chromeebook hacks and development.  
/// And now i develop for some PINE64 devices such as PinePhone, PinePhone PRO, PineTime and other
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
/// and other information security competitions.
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
    /// [Hitbox](https://github.com/hit-box/hitbox) is a high-performance caching framework suitable for single-machine and for distributed applications in Rust.
    Hitbox(Author),
    /// [Hitboxd](https://github.com/hit-box/hitboxd) is a caching reverse proxy for HTTP.
    Hitboxd(Author),
    /// [actix-web-validator](https://github.com/rambler-digital-solutions/actix-web-validator) is a Rust library for providing validation mechanism to actix-web with Validator crate.
    ActixWebValidator(Author),
    /// [aioriak](https://github.com/rambler-digital-solutions/aioriak) is a Python asyncio client for RiakKV database.
    AioRiak(Author),
    /// [aiohttp](https://github.com/aio-libs/aiohttp) - Asynchronous HTTP client/server framework for asyncio and Python.
    AioHTTP(Contributor),
    /// [conflow](https://github.com/singulared/conflow) - Python configuration manager.
    Conflow(Author),
    /// [bb8](https://github.com/djc/bb8) - Full-featured async (tokio-based) connection pool (like r2d2).
    BB8(Contributor),
    /// [flask-restfull](https://github.com/flask-restful/flask-restful) - REST API framework for Flask.
    FlaskRestful(Contributor),
    /// [pyjasperclient](https://github.com/agaoglu/pyjasperclient) - SOAP client for JasperReports.
    PyJasperClient(Contributor),
    /// [flask-hmacauth](https://github.com/Phillipmartin/flask-hmacauth/) - Flask hmac auth module.
    FlaskHmacAuth(Contributor),
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
    /// *2021, 2022*: Rust lecturer in Yandex School of Data Analysis.
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
