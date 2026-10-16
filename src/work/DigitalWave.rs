//! 2024-present [Digital Wave Technology](https://www.digitalwavetechnology.com/) is an AI-native platform for consumer brands.
//!
//! I work as a Core Software Engineer on the OLAP database and Domain Engine solutions in a team of 4-5 developers.
//! The company provides the ONE Platform - an enterprise AI-native platform with master data management at its core,
//! serving retail, consumer brands, healthcare, and distribution industries.
use resume_macro::Project;

/// OLAP database engine for real-time analytical processing.
///
/// High-performance columnar OLAP database (olap-rust) with block-based storage, GraphQL API, and multi-user access control.
/// Designed for real-time analytical queries on terabytes of data with caching, external data connector framework (spouts),
/// and full-text search integration. Built with Rust, Tokio, Juniper/Async-GraphQL, Axum, and OpenTelemetry tracing.
#[derive(Project)]
#[role(
    Developer,
    span(2024),
    langs(Rust),
    doc = r#"Core Software Engineer on OLAP

My focus is the engine internals on terabytes of data:
* Internal data layouts of the block-based columnar storage engine
* Memory allocations and performance optimization
* Performance and instrumentation tooling: OpenTelemetry tracing with context propagation, Prometheus metrics
* Multi-level caching with configurable TTL policies
* Parallel query execution

Also contributed to the GraphQL API, row-level security for multi-user access, the external data connector framework (spouts), and build and Docker optimizations."#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Async runtime for concurrent operations.")]
#[tech(frameworks::Axum, "Web framework for HTTP server.")]
#[tech(frameworks::Tracing, "Observability and distributed tracing.")]
#[tech(frameworks::Juniper, "GraphQL server implementation.")]
pub struct OLAP;

/// Metadata-driven domain engine for complex data management.
///
/// Enterprise-grade domain engine providing a GraphQL API for managing domains, entities, and attributes with dynamic
/// expression evaluation and multi-source data aggregation. Features fine-grained permissions, multi-dimensional attributes,
/// and connectivity to data sources (OLAP, PostgreSQL, ClickHouse, Kafka). Built with Rust, Tokio, async-graphql, Axum,
/// DataFusion (SQL frontend), and OpenTelemetry.
#[derive(Project)]
#[project(name = "Domain Engine")]
#[role(
    Developer,
    span(2024),
    langs(Rust),
    doc = r#"Core Software Engineer on Domain Engine

My main areas:
* Storage layouts for the persistent backends (OLAP, PostgreSQL, ClickHouse, Kafka) behind a unified query interface
* SQL frontend built on DataFusion
* GraphQL API with federation support
* Snapshotting systems
* Versioning and diff system for tracking schema and data changes over time
* Observability: structured logging, tracing, Prometheus metrics

Also contributed to the metadata-driven schema system, the expression language for dynamic calculations, and permissions and access control."#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Async runtime for concurrent operations.")]
#[tech(frameworks::Axum, "Web framework for HTTP server.")]
#[tech(frameworks::AsyncGraphQL, "GraphQL server implementation.")]
#[tech(frameworks::DataFusion, "Query engine built on Apache Arrow.")]
#[tech(databases::Postgresql, "Relational database for structured data.")]
#[tech(databases::ClickHouse, "Columnar database for analytical workloads.")]
#[tech(frameworks::Kafka, "One of the persistent storage backends.")]
pub struct DomainEngine;
