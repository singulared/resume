//! Opensource projects and contributions
use resume_macro::CleanupDocs;

/// Marker trait for project role
pub trait Role {}

/// Author of the opensource project
#[derive(CleanupDocs)]
pub struct Author;
impl Role for Author {}

/// Contributor to the opensource project
#[derive(CleanupDocs)]
pub struct Contributor;
impl Role for Contributor {}

/// [Hitbox](https://github.com/hit-box/hitbox) - Protocol-agnostic asynchronous caching framework for Rust.
///
/// Built around a Finite State Machine architecture, Hitbox provides a generic caching engine that works with any protocol
/// (HTTP, gRPC, GraphQL, custom) through extensible predicate and extractor traits. These protocol-agnostic traits enable
/// predicates to determine cacheability of requests/responses with composable logical operators, while extractors generate
/// cache keys from request data - both working independently of the underlying protocol. Features include multiple backend
/// implementations (Redis, Moka in-memory), Tower middleware integration, YAML/JSON configuration, three-tier cache
/// freshness model (actual/stale/expired), and comprehensive BDD test suite. The project consists of 11 modular crates
/// with ~700+ commits and focuses on sophisticated FSM-based design enabling complete separation between core caching
/// logic and protocol implementations.
#[derive(CleanupDocs)]
pub struct Hitbox<R: Role = Author>(pub R);

/// [Hitboxd](https://github.com/hit-box/hitboxd) - Planned caching reverse proxy daemon for HTTP (Varnish-like).
///
/// Standalone caching reverse proxy daemon built on top of the Hitbox framework, designed to provide high-performance
/// HTTP caching with YAML configuration, hot reload, and production-grade observability. Currently in planning/pre-release phase.
#[derive(CleanupDocs)]
pub struct Hitboxd<R: Role = Author>(pub R);

/// [actix-web-validator](https://github.com/rambler-digital-solutions/actix-web-validator) - Rust library for providing validation mechanism to actix-web with Validator crate.
#[derive(CleanupDocs)]
pub struct ActixWebValidator<R: Role = Author>(pub R);

/// [aioriak](https://github.com/rambler-digital-solutions/aioriak) - Python asyncio client for RiakKV database.
#[derive(CleanupDocs)]
pub struct AioRiak<R: Role = Author>(pub R);

/// [aiohttp](https://github.com/aio-libs/aiohttp) - Asynchronous HTTP client/server framework for asyncio and Python.
#[derive(CleanupDocs)]
pub struct AioHTTP<R: Role = Contributor>(pub R);

/// [conflow](https://github.com/singulared/conflow) - Python configuration manager with YAML/JSON support.
#[derive(CleanupDocs)]
pub struct Conflow<R: Role = Author>(pub R);

/// [shortland](https://github.com/singulared/shortland) - URL shortener service written in Rust with configurable storage backends (in-memory, Redis).
#[derive(CleanupDocs)]
pub struct Shortland<R: Role = Author>(pub R);

/// [swayboard](https://github.com/singulared/swayboard) - Keyboard layout manager for Sway window manager written in Rust.
#[derive(CleanupDocs)]
pub struct Swayboard<R: Role = Author>(pub R);

/// [chromebook-kernel](https://github.com/singulared/chromebook-kernel) - Fork of Linux kernel for Samsung ARM Chromebook (Snow).
#[derive(CleanupDocs)]
pub struct ChromebookKernel<R: Role = Author>(pub R);

/// [datafusion-postgres](https://github.com/datafusion-contrib/datafusion-postgres) - Postgres protocol frontend for DataFusion SQL engine.
#[derive(CleanupDocs)]
pub struct DataFusionPostgres<R: Role = Contributor>(pub R);

/// [swayipc-rs](https://github.com/JayceFayne/swayipc-rs) - Library for controlling Sway window manager through IPC interface.
#[derive(CleanupDocs)]
pub struct SwayipcRs<R: Role = Contributor>(pub R);

/// [poetry](https://github.com/python-poetry/poetry) - Python packaging and dependency management.
#[derive(CleanupDocs)]
pub struct Poetry<R: Role = Contributor>(pub R);

/// [slog-stdlog](https://github.com/slog-rs/stdlog) - Standard Rust log crate adapter to slog-rs.
#[derive(CleanupDocs)]
pub struct SlogStdlog<R: Role = Contributor>(pub R);

/// [basho_docs](https://github.com/basho/basho_docs) - Basho products documentation (Riak ecosystem).
#[derive(CleanupDocs)]
pub struct BashoDocs<R: Role = Contributor>(pub R);

/// [bb8](https://github.com/djc/bb8) - Full-featured async (tokio-based) connection pool (like r2d2).
#[derive(CleanupDocs)]
pub struct BB8<R: Role = Contributor>(pub R);

/// [flask-restful](https://github.com/flask-restful/flask-restful) - REST API framework for Flask.
#[derive(CleanupDocs)]
pub struct FlaskRestful<R: Role = Contributor>(pub R);

/// [pyjasperclient](https://github.com/agaoglu/pyjasperclient) - SOAP client for JasperReports.
#[derive(CleanupDocs)]
pub struct PyJasperClient<R: Role = Contributor>(pub R);

/// [flask-hmacauth](https://github.com/Phillipmartin/flask-hmacauth/) - Flask HMAC auth module.
#[derive(CleanupDocs)]
pub struct FlaskHmacAuth<R: Role = Contributor>(pub R);
