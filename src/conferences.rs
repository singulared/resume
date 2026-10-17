//! Conferences and meet-up
use resume_macro::CleanupDocs;

use crate::opensource::Author;

use self::roles::{Moderator, ProgramDirector, Speaker};

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

/// RustCon Russia 2021. All talks are available on [YouTube](https://www.youtube.com/playlist?list=PLRdS-n5seLRroZ480sDtes06hn6_M7N_i).
#[derive(CleanupDocs)]
pub struct RustCon2021;
/// One of the program directors of RustCon Russia 2021.
impl ProgramDirector for RustCon2021 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/bEP0YcOyuyE?list=PLRdS-n5seLRroZ480sDtes06hn6_M7N_i" title="Rust и Python - как в небольшой команде переписать узкие места на Rust. Максим Акинин, assi.ai" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for RustCon2021 {}

#[derive(CleanupDocs)]
/// RustCon Russia 2022.
pub struct RustCon2022;
/// One of the program directors of RustCon Russia 2022.
impl ProgramDirector for RustCon2022 {}

/// PyCon Russia 2021.
///
/// The biggest Python conference in Russia.
#[derive(CleanupDocs)]
pub struct PyCon2021;
/// Many articles have been written about using optional static typing in Python.
///
/// In this talk, we do not repeat common truths but try to answer the question of why we need it.
/// Using examples, we show how to do it, discuss the difficulties, and demonstrate how to solve them.
impl Speaker for PyCon2021 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/Z6sUShcirsU?list=PLRdS-n5seLRrgvgaGcx6S7e3zsyLpY5iO" title="Андрей Ермилов, Максим Белоусов. Советы по использованию опциональной статической типизации" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for PyCon2021 {}

/// [Conference](https://devconf.ru) of professional web developers.
#[derive(CleanupDocs)]
pub struct DevConf2016;
/// This talk covers the lack of an asynchronous Python driver for Riak and the reasons for writing our own.
/// A review and comparison of possible solutions.
/// [Presentation](https://s.conf.guru/data/devconf2016/ppt/155.pdf)
impl Speaker for DevConf2016 {}

/// Local [Moscow Rust community](https://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/279291922/).
#[derive(CleanupDocs)]
pub struct MoscowRustMeetup2021;
/// The experience of switching to Rust, or how we got into open source.
impl Speaker for MoscowRustMeetup2021 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/mZqFJwrySbI" title="Moscow Rust Meetup в офисе UnitedTraders (16.07.2021)" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for MoscowRustMeetup2021 {}

/// Local [Moscow python community](https://moscowpython.ru).
#[derive(CleanupDocs)]
pub struct MoscowPythonMeetup2016;
/// This talk covers the lack of an asynchronous Python driver for Riak and the reasons for writing our own.
/// A review and comparison of possible solutions.
///
/// - Domain theory (Riak, asyncio), problem statement.
/// - Overview of existing solutions.
/// - Comparison of the implemented solutions:
///   - blocking calls
///   - executor
///   - aioriak
/// - Performance comparison.
/// - Why was it necessary?
///
/// [Presentation](https://speakerdeck.com/moscowdjango/asyncio-kliient-dlia-riak-zachiem)
impl Speaker for MoscowPythonMeetup2016 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/q-hPXZZzx9k" title="Asyncio клиент для Riak. Зачем?" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for MoscowPythonMeetup2016 {}

/// First cooperative DevOps meetup of two companies: Rambler&Co and Okko.
#[derive(CleanupDocs)]
pub struct RamblerDevOps2021;
/// Moderator and presenter.
impl Moderator for RamblerDevOps2021 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/78vpPXuwxL0" title="Rambler&Okko DevOps Meetup" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for RamblerDevOps2021 {}

/// Rambler Python meetup.
/// - Best practice in user embedding with PyTorch and Docker
/// - Rust bindings for Python applications
/// - Spark: 20 minutes adventure
#[derive(CleanupDocs)]
pub struct RamblerPython2021;
/// Moderator and presenter.
impl Moderator for RamblerPython2021 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/gpf_KOAmgzY" title="RamblerMeetup&Python" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for RamblerPython2021 {}

/// Video version of the article series "[Rust for Python developers](https://habr.com/ru/company/rambler_group/blog/533268/)" created in collaboration with Andrey Ermilov.
///
/// In this video, we talk about why we moved away from our usual technology stack and show the advantages Rust has over Python.
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
/// A podcast by the founders of the biggest Russian Python community, [MoscowPython](https://www.facebook.com/groups/MoscowDjango/).
pub struct MoscowPythonPodcast2020;
/// Guest, together with Andrey Ermilov.
///
/// We talked about the winding career paths of Python (and other) developers,
/// and about refactoring and rewriting big projects in Rust.
impl Speaker for MoscowPythonPodcast2020 {}
/// <iframe width="1279" height="719" src="https://www.youtube.com/embed/d9suAx7A6VM" title="Moscow Python Podcast. Рефакторинг проектов по-взрослому (level: middle)" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>
impl<Ru> Video<Ru> for MoscowPythonPodcast2020 {}

/// Conference / speaking entries submitted to inventory for the template/markdown
/// views. Private (kept out of rustdoc), mirroring [`crate::ConferencesHistory`].
mod conferences_inventory {
    use resume_macro::Conference;

    #[derive(Conference)]
    #[conference(name = "RustCon Russia", year = "2021, 2022", role = "Program Director", url = "https://rustcon.ru/")]
    /// One of the program directors of the biggest Rust conference in Russia.
    struct RustCon;

    #[derive(Conference)]
    #[conference(name = "PyCon Russia", year = "2021", role = "Speaker", url = "https://pycon.ru/")]
    /// Talk on the practical use of optional static typing in Python.
    struct PyCon;

    #[derive(Conference)]
    #[conference(name = "Moscow Rust Meetup", year = "2021", role = "Speaker")]
    /// "The experience of switching to Rust, or how we got into open source."
    struct MoscowRustMeetup;

    #[derive(Conference)]
    #[conference(name = "Yandex School of Data Analysis (SHAD)", year = "2021-2023", role = "Lecturer")]
    /// Rust lecturer at the Yandex School of Data Analysis.
    struct Shad;

    #[derive(Conference)]
    #[conference(name = "DevConf", year = "2016", role = "Speaker", url = "https://devconf.ru")]
    /// Talk on writing an asynchronous Python driver for Riak (aioriak).
    struct DevConf;

    #[derive(Conference)]
    #[conference(name = "Moscow Python Meetup", year = "2016", role = "Speaker", url = "https://moscowpython.ru")]
    /// Talk on the asyncio client for Riak and a comparison of approaches.
    struct MoscowPythonMeetup;

    #[derive(Conference)]
    #[conference(name = "Rambler&Co Meetups", year = "2015-2022", role = "Moderator & Speaker")]
    /// Moderator and presenter at Rambler DevOps and Python meetups, including the
    /// "Rust for Python developers" series.
    struct RamblerMeetup;

    #[derive(Conference)]
    #[conference(name = "Moscow Python Podcast", year = "2020", role = "Guest")]
    /// Guest episode on refactoring and rewriting large projects in Rust.
    struct MoscowPythonPodcast;
}
