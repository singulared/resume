//! 2015-2022 [Rambler&Co](https://rambler.ru) is one of the biggest media holdings in Russia.
//!
//! I started work at Rambler as a Middle Python developer in 2015 and ended as Head of Media
//! Development team in 2022.
use resume_macro::Project;

/// System for collecting and processing media portal's users such as visit information, unique material, and others.
///
/// The media portal using this collected information can set up some access limits.
/// These restrictions can be overcome by users after successful payment.
#[deprecated(since = "2017.1.1", note = "Closed")]
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2015, 2017),
    langs(Python, Erlang),
    doc = r#"Python developer in Paywall.

* The project uses Riak as the main storage and is based on event architecture built on top of RabbitMQ
* Project uses Cassandra and Python 3.5 with asyncio as backend"#
)]
#[tech(languages::Python, "Python was a main programming language on this project.")]
#[tech(languages::Erlang, "Used as part of the Riak database.")]
#[tech(databases::Riak, "Main storage.")]
#[tech(databases::Cassandra, "Secondary storage for analytics.")]
#[tech(frameworks::AsyncIO, "Project was built on top of asyncio.")]
#[tech(frameworks::AioHTTP, "Project was built on top of aiohttp.")]
pub struct Paywall;

/// The system allows mobile users to disable advertisements for some small payments.
#[deprecated(since = "2017.1.1", note = "Closed")]
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2016, 2017),
    langs(Python),
    doc = "This project uses [Paywall] architecture and shares some codebases."
)]
#[tech(languages::Python, "Python was a main programming language on this project.")]
#[tech(databases::Riak, "Main storage.")]
#[tech(databases::ClickHouse, "Secondary storage.")]
pub struct AdBlock;

/// Video recommendation [platform](https://web.archive.org/web/20180119134125/https://video.rambler.ru/).
#[deprecated(since = "2018.1.1", note = "Closed")]
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2017, 2017),
    langs(Python),
    doc = "Migration to Python 3.6, bugfixes and codebase support."
)]
#[tech(languages::Python, "Python was a main programming language on this project.")]
pub struct Video;

/// One of the biggest [news aggregator and news platform](https://news.rambler.ru) in Russia.
///
/// When I started on this project (2017) it was mostly python applications with classical legacy project problems:
/// * High costs of development and support
/// * High latency on web components (API and front-end)
/// * The low actuality of news (mostly by long caching)
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2017, 2018),
    langs(Python),
    doc = "Python developer in Rambler News.\n\nInfrastructure migration to k8s and refactoring deployment process. Migration legacy codebase to python 3."
)]
#[role(
    TeamLead,
    span(2018, 2022),
    langs(Rust, Python),
    doc = r#"Head of Media development team.

Since 2018, I have become the head of the development team. With the team, we were able to mostly solve all the problems described:
* Rewriting all Aggregator parts with actual ML algorithms and technical stack
* Rewrite core API with Rust
* Rewrite parsing component with Rust
* Remove all caching layers instead of one
* Latency has decreased by about 10 times
* Incidents count has decreased by about 10 times
* Resource costs have decreased by about 20 times
* Re-select and rewrite all DS workers

By the way, the development team’s area of responsibility includes such projects as [MediaUp], [Horoscopes], [Weather], and some others."#
)]
#[tech(languages::Python, "Main language for Rambler/News up to 2020.\n\nDS tasks such as classification, clusterisation and etc. are written in Python with self-made actor framework built on top of RabbitMQ and Protobuf.")]
#[tech(languages::Rust, "Current realisation of the core API rewritten completely in Rust.")]
#[tech(frameworks::Actix, "Project was built on top of Actix actor and web framework.")]
#[tech(frameworks::Tokio, "Project was built on top of tokio and Tonic for RPC and Event Sourcing services.")]
#[tech(frameworks::Celery, "Old DS workers based on Celery and Python 2.")]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(messages::GRPC, "RPC implementation based on Tonic for Event Sourcing.")]
pub struct News;

/// [MediaUp](https://play.google.com/store/apps/details?id=ru.rambler.media_app&gl=US) is a news mobile application for Android and iOS based on recommendations and editor's choices.
///
/// With the team, I designed the entire API for the application and the Administrative part (for news editors).
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    TeamLead,
    span(2020, 2021),
    langs(Rust),
    doc = "As Head of the development team, I chose all technical decisions about application architecture and infrastructure. Backend architecture was closest to the News Rust core API and shared the same technology stack."
)]
#[tech(languages::Rust, "Rust is a main programming language on this project.")]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Actix, "Project was built on top of Actix actor and web framework.")]
#[tech(messages::GRPC, "RPC implementation based on Tonic for Event Sourcing.")]
pub struct MediaUp;

/// Rambler&Co service with [esoteric content](http://horoscopes.rambler.ru). (:
#[derive(Project)]
#[project(parent = Rambler)]
#[role(TeamLead, span(2018, 2022), langs(Python))]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Django, "Project was built on top of Django web framework.")]
pub struct Horoscopes;

/// Rambler&Co weather [forecast service](https://weather.rambler.ru).
#[derive(Project)]
#[project(parent = Rambler)]
#[role(TeamLead, span(2018, 2022), langs(Python))]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Django, "Project was built on top of Django web framework.")]
pub struct Weather;
