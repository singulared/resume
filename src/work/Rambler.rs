//! 2015-2022 [Rambler&Co](https://rambler.ru) is one of the biggest media holdings in Russia.
//!
//! I joined Rambler as a Middle Python developer in 2015 and left in 2022 as Head of the Media
//! Development team.
use resume_macro::Project;

/// System for collecting and processing data about the media portal's users, such as visits and the unique articles each user read.
///
/// Using the collected data, the media portal can set access limits.
/// Users can lift these restrictions by paying.
#[deprecated(since = "2017.1.1", note = "Closed")]
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2015, 2017),
    langs(Python, Erlang),
    doc = r#"Python developer in Paywall.

* The project used Riak as the main storage and an event-driven architecture built on top of RabbitMQ
* The backend used Cassandra and Python 3.5 with asyncio"#
)]
#[tech(languages::Python, "Python was the main programming language for this project.")]
#[tech(languages::Erlang, "Used as part of the Riak database.")]
#[tech(databases::Riak, "Main storage.")]
#[tech(databases::Cassandra, "Secondary storage for analytics.")]
#[tech(frameworks::AsyncIO, "The project was built on top of asyncio.")]
#[tech(frameworks::AioHTTP, "The project was built on top of aiohttp.")]
pub struct Paywall;

/// A system that let mobile users disable advertisements for a small payment.
#[deprecated(since = "2017.1.1", note = "Closed")]
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2016, 2017),
    langs(Python),
    doc = "The project reused the [Paywall] architecture and shared part of its codebase."
)]
#[tech(languages::Python, "Python was the main programming language for this project.")]
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
#[tech(languages::Python, "Python was the main programming language for this project.")]
pub struct Video;

/// One of the biggest [news aggregators and news platforms](https://news.rambler.ru) in Russia.
///
/// When I joined the project in 2017, it was mostly Python applications with the classic problems of a legacy project:
/// * High development and support costs
/// * High latency of web components (API and front-end)
/// * Stale news (mostly due to long caching)
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    Developer,
    span(2017, 2018),
    langs(Python),
    doc = "Python developer in Rambler News.\n\nMigrated the infrastructure to k8s and refactored the deployment process. Migrated the legacy codebase to Python 3."
)]
#[role(
    TeamLead,
    span(2018, 2022),
    langs(Rust, Python),
    doc = r#"Head of the Media Development team.

In 2018, I became the head of the development team. Together with the team, we largely solved the problems described above:
* Rewrote all Aggregator parts with up-to-date ML algorithms and tech stack
* Rewrote the core API in Rust
* Rewrote the parsing component in Rust
* Removed all caching layers but one
* Reduced latency by about 10 times
* Reduced the number of incidents by about 10 times
* Reduced resource costs by about 20 times
* Re-selected and rewrote all DS workers

The team was also responsible for [MediaUp], [Horoscopes], [Weather], and other projects."#
)]
#[tech(languages::Python, "Main language of Rambler/News until 2020.\n\nDS tasks such as classification and clustering were written in Python with an in-house actor framework built on top of RabbitMQ and Protobuf.")]
#[tech(languages::Rust, "The core API was completely rewritten in Rust.")]
#[tech(frameworks::Actix, "The project was built on top of the Actix actor and web framework.")]
#[tech(frameworks::Tokio, "RPC and Event Sourcing services were built on top of Tokio and Tonic.")]
#[tech(frameworks::Celery, "Old DS workers based on Celery and Python 2.")]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(messages::GRPC, "RPC implementation based on Tonic for Event Sourcing.")]
pub struct News;

/// [MediaUp](https://play.google.com/store/apps/details?id=ru.rambler.media_app&gl=US) is a mobile news application for Android and iOS based on recommendations and editors' choices.
///
/// Together with the team, I designed the entire API for the application and its admin part for news editors.
#[derive(Project)]
#[project(parent = Rambler)]
#[role(
    TeamLead,
    span(2020, 2021),
    langs(Rust),
    doc = "As head of the development team, I made all technical decisions on the application architecture and infrastructure. The backend architecture closely followed the News Rust core API and shared its technology stack."
)]
#[tech(languages::Rust, "Rust was the main programming language for this project.")]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Actix, "The project was built on top of the Actix actor and web framework.")]
#[tech(messages::GRPC, "RPC implementation based on Tonic for Event Sourcing.")]
pub struct MediaUp;

/// Rambler&Co service with [esoteric content](http://horoscopes.rambler.ru). (:
#[derive(Project)]
#[project(parent = Rambler)]
#[role(TeamLead, span(2018, 2022), langs(Python))]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Django, "The project was built on top of the Django web framework.")]
pub struct Horoscopes;

/// Rambler&Co weather [forecast service](https://weather.rambler.ru).
#[derive(Project)]
#[project(parent = Rambler)]
#[role(TeamLead, span(2018, 2022), langs(Python))]
#[tech(databases::Postgresql, "Main data storage.")]
#[tech(frameworks::Django, "The project was built on top of the Django web framework.")]
pub struct Weather;
