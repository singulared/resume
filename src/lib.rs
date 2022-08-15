// #![no_std]
#![feature(doc_cfg)]
#![feature(negative_impls)]
#![feature(associated_const_equality)]
#![allow(deprecated)]
#![allow(unused)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/414425?v=4")]
//! # Maxim Belousov
//! Rustacean, Crypto-anarchist and Techno-punk.

use conferences::{roles::ProgramDirector, PyCon2021, RustCon2021, RustCon2022, DevConf2016, MoscowRustMeetup2021, MoscowPythonMeetup2016, RamblerDevOps2021, RamblerPython2021, RustForPythonDevelopers, MoscowPythonPodcast2020};
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

use hobbies::{Biking, EmbeddedSystems, Music, Piano, Tennis, CTF};
use technologies::{
    languages::Rust, tools::Linux, DistributedSystems, Hobbies, TechnicalInterests, CRDT, Cryptography,
};
#[doc(inline)]
pub use work::WorkHistory;

/// Information about previous works
pub mod work {
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

    /// 2015-2022 [Rambler&Co](https://rambler.ru) is one of the biggiest media holding in Russia.
    ///
    /// I start work in Rambler as Middle Python developer in 2015 and end as Head of Media
    /// Development team in 2022.
    #[allow(non_snake_case)]
    pub mod Rambler {
        use crate::roles::{Developer, TeamLead};
        use crate::technologies::databases::{Cassandra, ClickHouse, Postgresql, Riak};
        use crate::technologies::frameworks::{Actix, AioHTTP, AsyncIO, Celery, Django, Tokio};
        use crate::technologies::languages::{Erlang, Python, Rust};
        use crate::technologies::messages::GRPC;
        use crate::CleanupDocs;

        /// System for collecting and processing media portal’s users such as visit information, unique material, and others.
        ///
        /// The media portal using this collected information can set up some access limits.
        /// These restrictions can be overcome by users after successful payment.
        #[derive(CleanupDocs)]
        #[deprecated(since = "2017.1.1", note = "Closed")]
        pub struct Paywall;
        /// Python developer in Paywall
        ///
        /// The project uses Riak as the main storage and is based on event architecture built on top of RabbitMQ.
        /// Btw this project uses Cassandra and python 3.5 with asyncio as backend.
        impl<'period, 'f2015, 't2017, Languages> Developer<Languages> for &'period Paywall
        where
            Languages: Python + Erlang,
            'period: 'f2015 + 't2017,
        {
        }
        /// Python was a main programming language on this project.
        impl Python for Paywall {}
        /// Main storage.
        impl Riak for Paywall {}
        /// Secondary storage for analytics
        impl Cassandra for Paywall {}
        /// Project was built on top of asyncio.
        impl AsyncIO for Paywall {}
        /// Project was built on top of aiohttp.
        impl AioHTTP for Paywall {}

        /// The system allows mobile users to disable advertisements for some small payments.
        #[derive(CleanupDocs)]
        #[deprecated(since = "2017.1.1", note = "Closed")]
        pub struct AdBlock;
        /// This project uses [Paywall] architecture and shares some codebases.
        impl<'period, 'f2016, 't2017, Language> Developer<Language> for &'period AdBlock
        where
            Language: Python,
            'period: 'f2016 + 't2017,
        {
        }
        /// Python was a main programming language on this project.
        impl Python for AdBlock {}
        /// Main storage.
        impl Riak for AdBlock {}
        /// Secondary storage.
        impl ClickHouse for AdBlock {}

        /// Video recommendation [platform](https://web.archive.org/web/20180119134125/https://video.rambler.ru/).
        #[derive(CleanupDocs)]
        #[deprecated(since = "2018.1.1", note = "Closed")]
        pub struct Video;
        /// Migration to Python 3.6 bugfix and codebase support.
        impl<'period, 'in2017, Language> Developer<Language> for &'period Video
        where
            Language: Python,
            'period: 'in2017,
        {
        }
        /// Python was a main programming language on this project.
        impl Python for Video {}

        /// One of the biggest [news aggregator and news platform](https://news.rambler.ru) in Russia.
        ///
        /// When I started on this project (2017) it was mostly python applications with classical legacy project problems:
        /// * High costs of development and support
        /// * High latency on web components (API and front-end)
        /// * The low actuality of news (mostly by long caching)
        #[derive(CleanupDocs)]
        pub struct News;
        /// Python developer in Rambler News
        ///
        /// Infrastructure migration to k8s and refactoring deployment process.
        /// Migration legacy codebase to python 3.
        impl<'period, 'f2017, 't2018, Language> Developer<Language> for &'period News
        where
            Language: Python,
            'period: 'f2017 + 't2018,
        {
        }
        /// Head of Media development team.
        ///
        /// Since 2018, I have become the head of the development team. With a team we were able to mostly solve all the problems described:
        /// * We are rewriting all Aggregator parts with actual ML algorithms and technical stack
        /// * Rewrite core API with Rust.
        /// * Rewrite parsing component with Rust
        /// * Remove all caching layers instead of one
        /// * Latency has decreased by about 10 times
        /// * Incidents count has decreased by about 10 times
        /// * Resource costs have decreased by about 20 times
        /// * Re-select and rewrite all DS workers  
        /// Btw in the development team’s area of responsibility includes such projects as [MediaUp], [Horoscopes], [Weather], and some others.
        impl<'period, 'f2018, 't2022, Team> TeamLead<Team> for &'period News
        where
            Team: Rust + Python,
            'period: 'f2018 + 't2022,
        {
        }
        /// Current realisation of the core API rewritten completely in Rust.
        impl Rust for News {}
        /// Project was built on top of Actix actor and web framework.
        impl Actix for News {}
        /// Project was built on top of tokio and Tonic for RPC and Event Sourcing services.
        impl Tokio for News {}
        /// Main language for Rambler/News up to 2020.
        ///
        /// DS tasks such as classification, clusterisation and etc. are written in Python with
        /// self-made actor framework built on top of RabbitMQ and Protobuf.
        impl Python for News {}
        /// Main data storage.
        impl Postgresql for News {}
        /// Old DS workers based on Celery and Python 2.
        impl Celery for News {}
        /// RPC implementation based on Tonic for Event Sourcing.
        impl GRPC for News {}

        /// [MediaUp](https://play.google.com/store/apps/details?id=ru.rambler.media_app&gl=US) is a news mobile application for Android and iOS based on recommendations and editor’s choices.
        ///
        /// With the team, I designed the entire API for the application and the Administrative part (for news editors).
        #[derive(CleanupDocs)]
        pub struct MediaUp;
        /// As a Head of the development team, I chose all technical decisions about application architecture and infrastructure.
        ///
        /// Backend architecture closest to News rust core API and had the same technology stack.
        impl<'period, 'f2020, 't2021, Team> TeamLead<Team> for &'period MediaUp
        where
            Team: Rust,
            'period: 'f2020 + 't2021,
        {
        }
        /// Rust is a main programming language on this project.
        impl Rust for MediaUp {}
        /// Project was built on top of Actix actor and web framework.
        impl Actix for MediaUp {}
        /// Main data storage.
        impl Postgresql for MediaUp {}
        /// RPC implementation based on Tonic for Event Sourcing.
        impl GRPC for MediaUp {}

        /// Rambler&Co service with [esoteric content](http://horoscopes.rambler.ru). (:
        #[derive(CleanupDocs)]
        pub struct Horoscopes;
        impl<'period, 'f2018, 't2022, Team> TeamLead<Team> for &'period Horoscopes
        where
            Team: Python,
            'period: 'f2018 + 't2022,
        {
        }
        /// Main data storage.
        impl Postgresql for Horoscopes {}
        /// Project was built on top of Django web framework.
        impl Django for Horoscopes {}

        /// Rambler&Co weather [forecast service](https://weather.rambler.ru).
        #[derive(CleanupDocs)]
        pub struct Weather;
        impl<'period, 'f2018, 't2022, Team> TeamLead<Team> for &'period Weather
        where
            Team: Python,
            'period: 'f2018 + 't2022,
        {
        }
        /// Main data storage.
        impl Postgresql for Weather {}
        /// Project was built on top of Django web framework.
        impl Django for Weather {}
    }

    /// 2010-2015 [Anjlab](https://anjlab.com) is outsource development company in Vladimir city.
    ///
    /// AnjLab is a team of skilled software development professionals.
    /// We are experienced in the development of performant, scalable and reliable solutions for companies around the world.
    /// We also maintain and contribute to many open source projects.
    #[allow(non_snake_case)]
    pub mod AnjLab {
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
    }

    /// 2008-2010 [KayCom](https://kaycom.ru) is a web-studio in Vladimir city.
    ///
    /// KayCom is a web-studio with mobile development team and many projects in Russia and around
    /// the world.
    #[allow(non_snake_case)]
    pub mod KayCom {
        use crate::{
            roles::Developer,
            technologies::{
                databases::MySQL,
                languages::{JS, PHP},
            },
            CleanupDocs,
        };

        /// Russian online [poster shop](https://web.archive.org/web/20100416090318/http://www.postershop.ru/).
        #[deprecated]
        #[derive(CleanupDocs)]
        pub struct PosterShop;
        impl<Language> Developer<Language> for PosterShop where Language: PHP + JS {}
        impl PHP for PosterShop {}
        impl JS for PosterShop {}
        impl MySQL for PosterShop {}

        /// US online [bookstore](https://web.archive.org/web/20101225030926/http://www.cyberread.com/).
        #[deprecated]
        #[derive(CleanupDocs)]
        pub struct CyberRead;
        impl<Language> Developer<Language> for CyberRead where Language: PHP + JS {}
        impl PHP for CyberRead {}
        impl JS for CyberRead {}
        impl MySQL for CyberRead {}
    }
}

/// My technology stack
pub mod technologies {
    use crate::hobbies::{Biking, EmbeddedSystems, Music, Piano, Tennis, CTF};

    use self::{languages::Rust, tools::Linux};

    /// My technical interests
    pub trait TechnicalInterests
    where
        Self: Rust + DistributedSystems + Linux + CRDT + EmbeddedSystems + Cryptography,
    {
    }

    pub trait Hobbies
    where
        Self: Music + Piano + Tennis + EmbeddedSystems + Biking + CTF,
    {
    }

    pub trait CRDT {}
    pub trait Cryptography {}
    pub trait DistributedSystems {}

    /// Programming languages
    pub mod languages {
        pub trait Languages: Rust + Python {}
        pub trait Rust {}
        pub trait Python {}
        pub trait CPP {}
        pub trait PHP {}
        pub trait Erlang {}
        pub trait CS {}
        pub trait JS {}
    }

    /// Databases
    pub mod databases {
        pub trait Postgresql {}
        pub trait Riak {}
        pub trait Cassandra {}
        pub trait MySQL {}
        pub trait ClickHouse {}
    }

    /// Frameworks
    pub mod frameworks {
        pub trait Actix {}
        pub trait Tokio {}
        pub trait AsyncIO {}
        pub trait AioHTTP {}
        pub trait Flask {}
        pub trait Celery {}
        pub trait Django {}
        pub trait OpenGL {}
        pub trait Qt {}
    }

    /// Message brokers, event handling systems and RPCs.
    pub mod messages {
        pub trait RabbitMQ {}
        pub trait GRPC {}
    }

    /// Working tools
    pub mod tools {
        pub trait Linux {}
        pub trait Neovim {}
    }
}

/// Some personal hobbies and pet-projects
pub mod hobbies {
    pub trait Piano {}
    pub trait Music {}
    pub trait Tennis {}
    pub trait EmbeddedSystems {}
    pub trait Science {}
    pub trait GameDev {}
    pub trait Biking {}
    pub trait CTF {}
}

/// Work roles
pub mod roles {
    pub trait Developer<Language> {}
    pub trait TeamLead<Team> {}
}

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
/// Started use Linux as main system since 2006.
///
/// Debian one Love (:
impl Linux for Me {}
/// In 2015 with Riak I opened distributed systems world for myself. I continue opening it up nowadays of course.
impl DistributedSystems for Me {}
/// CRDT as i think one of the most interesting technology of the last years.
impl CRDT for Me {}
/// One of my hobby is microcontrollers and embedded development.
///
/// I started from porting Python3 and building kernel for Motorola a1200.  
/// Porting linux on Toshiba ac100 arm smartbook.  
/// Samsung ARM chromeebook hacks and development.  
/// And now i develop for some PINE64 devices such as PinePhone, PinePhone PRO, PineTime and other
/// SBC platforms like NVidia Jetson Nano.
impl EmbeddedSystems for Me {}
/// As Ph.D student i really interested in Cryptography and wrote Ph.D dissertation about SMT
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
    RamblerMeetup(RamblerDevOps2021, RamblerPython2021, RustForPythonDevelopers),
    MoscowPythonPodcast(MoscowPythonPodcast2020),
}

/// Publications and articles
pub mod publications {

}

/// Conferences and meet-up
pub mod conferences {
    use resume_macro::CleanupDocs;

    use crate::opensource::Author;

    use self::roles::{Speaker, ProgramDirector, Moderator};

    pub mod roles {
        /// Program director of the conference.
        pub trait ProgramDirector {}
        /// Speaker on the conference.
        pub trait Speaker {}
        /// Moderator or presenter of the event.
        pub trait Moderator {}
    }
    /// Video of the speech at the conference.
    pub trait Video<Language> {}

    /// RustCon Russia 2021. All videos you can see on [YouTube](https://www.youtube.com/playlist?list=PLRdS-n5seLRroZ480sDtes06hn6_M7N_i)
    #[derive(CleanupDocs)]
    pub struct RustCon2021;
    /// One of the Program directors of RustCon Russia 2021
    impl ProgramDirector for RustCon2021 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/bEP0YcOyuyE?list=PLRdS-n5seLRroZ480sDtes06hn6_M7N_i" title="Rust и Python - как в небольшой команде переписать узкие места на Rust. Максим Акинин, assi.ai" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for RustCon2021 {}

    #[derive(CleanupDocs)]
    /// RustCon Russia 2022. TBD.
    pub struct RustCon2022;
    /// One of the Program directors of RustCon Russia 2021
    impl ProgramDirector for RustCon2022 {}

    /// PyCon Russia 2021.
    ///
    /// Biggest Python conference in Russia.
    #[derive(CleanupDocs)]
    pub struct PyCon2021;
    /// Many articles have been written on the topic of using optional developed typing in Python.
    ///
    /// In this speech, we will not try to repeat common truths but will try to answer the question of why we need to use it.
    /// Using examples, we will figure out how this can be done, touch on the complexities, and demonstrate how to solve them.
    impl Speaker for PyCon2021 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/Z6sUShcirsU?list=PLRdS-n5seLRrgvgaGcx6S7e3zsyLpY5iO" title="Андрей Ермилов, Максим Белоусов. Советы по использованию опциональной статической типизации" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for PyCon2021 {}

    /// [Conference](https://devconf.ru) of professional web developers.
    #[derive(CleanupDocs)]
    pub struct DevConf2016;
    /// This speech discusses the problem of the lack of an asynchronous Python driver for Riak and the reasons for writing its own implementation.
    /// Review of possible solutions and their comparsion.  
    /// [Presentation](https://s.conf.guru/data/devconf2016/ppt/155.pdf)
    impl Speaker for DevConf2016 {}

    /// Local [Moscow Rust community](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/279291922/).
    #[derive(CleanupDocs)]
    pub struct MoscowRustMeetup2021;
    /// The experience of switching to Rust or how we got to open source.
    impl Speaker for MoscowRustMeetup2021 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/mZqFJwrySbI" title="Moscow Rust Meetup в офисе UnitedTraders (16.07.2021)" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for MoscowRustMeetup2021 {}

    /// Local [Moscow python community](https://moscowpython.ru).
    #[derive(CleanupDocs)]
    pub struct MoscowPythonMeetup2016;
    /// This speech discusses the problem of the lack of an asynchronous Python driver for Riak and the reasons for writing its own implementation.
    /// Review of possible solutions and their comparison.  
    ///
    /// - Domain theory (riak, asyncio), problem statement.
    /// - Overview of existing solutions.
    /// - Implemented solutions comparsion:
    ///   - blocking calls
    ///   - executor
    ///   - aioriak
    /// - performance comparison.
    /// - Why was it necessary?
    ///
    /// [Presentation](https://speakerdeck.com/moscowdjango/asyncio-kliient-dlia-riak-zachiem)
    impl Speaker for MoscowPythonMeetup2016 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/q-hPXZZzx9k" title="Asyncio клиент для Riak. Зачем?" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for MoscowPythonMeetup2016 {}

    /// First cooperative DevOps meetup of two companies: Rambler&Co and Okko.
    #[derive(CleanupDocs)]
    pub struct RamblerDevOps2021;
    /// Me as moderator and presenter.
    impl Moderator for RamblerDevOps2021 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/78vpPXuwxL0" title="Rambler&Okko DevOps Meetup" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for RamblerDevOps2021 {}

    /// Rambler Python meetup.
    /// - Best practice in user embedding with PyTourch and Docker
    /// - Rust bindings for Python applications
    /// - Spark: 20 minutes adventure
    #[derive(CleanupDocs)]
    pub struct RamblerPython2021;
    /// Me as moderator and presenter.
    impl Moderator for RamblerPython2021 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/gpf_KOAmgzY" title="RamblerMeetup&Python" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for RamblerPython2021 {}

    /// Video version of articles "[Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)" created with collaboration with Andrey Ermolov.
    ///
    /// In this video we will talk about why we move away from the usual technology stack, and show what advantages Rust has compared to Python.
    /// * First part ([Text version](https://habr.com/ru/company/rambler_group/blog/533268/))
    ///     * Types
    ///     * User types and polymorphism
    ///     * Enums
    ///     * Option & Result
    ///     * Pattern matching
    ///     * Traits and Protocols
    ///     * Generic programming
    /// * Second part ([Text version](https://habr.com/ru/company/rambler_group/blog/535234/))
    ///     * Multithreading
    ///     * Asynchronous
    ///     * Functional programming
    ///     * Conclusion
    #[derive(CleanupDocs)]
    pub struct RustForPythonDevelopers;
    /// Author and speaker with Andrey Ermilov.
    impl Speaker for RustForPythonDevelopers {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/mXrgzTkDSGs" title="Rust глазами Python-иста" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for RustForPythonDevelopers {}

    #[derive(CleanupDocs)]
    /// Podcast from founders of the biggest Russian python community [MoscowPython](https://www.facebook.com/groups/MoscowDjango/)
    pub struct MoscowPythonPodcast2020;
    /// Guests with Andrey Ermilov.
    ///
    /// We talked about the winding path of Python (and not only) developers. 
    /// Btw we talked about refactoring and rewriting big projects (on Rust).
    impl Speaker for MoscowPythonPodcast2020 {}
    /// <iframe width="1279" height="719" src="https://www.youtube.com/embed/d9suAx7A6VM" title="Moscow Python Podcast. Рефакторинг проектов по-взрослому (level: middle)" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
    impl<Ru> Video<Ru> for MoscowPythonPodcast2020 {}
}

/// Opensource projects and contributions
pub mod opensource {
    use resume_macro::CleanupDocs;
    
    /// Author of the opensource project
    #[derive(CleanupDocs)]
    pub struct Author;

    /// Contributor to the opensource project
    #[derive(CleanupDocs)]
    pub struct Contributor;
}
