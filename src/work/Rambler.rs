//! 2015-2022 [Rambler&Co](https://rambler.ru) is one of the biggest media holdings in Russia.
//!
//! I started work at Rambler as a Middle Python developer in 2015 and ended as Head of Media
//! Development team in 2022.
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
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2017, 't2018, Language> Developer<Language> for &'period News
where
    Language: Python,
    'period: 'f2017 + 't2018,
{
}
/// Head of Media development team.
///
/// Since 2018, I have become the head of the development team. With the team, we were able to mostly solve all the problems described:
/// * We are rewriting all Aggregator parts with actual ML algorithms and technical stack
/// * Rewrite core API with Rust.
/// * Rewrite parsing component with Rust
/// * Remove all caching layers instead of one
/// * Latency has decreased by about 10 times
/// * Incidents count has decreased by about 10 times
/// * Resource costs have decreased by about 20 times
/// * Re-select and rewrite all DS workers
///
///   By the way, the development team's area of responsibility includes such projects as [MediaUp], [Horoscopes], [Weather], and some others.
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
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
#[allow(clippy::extra_unused_lifetimes)]
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
