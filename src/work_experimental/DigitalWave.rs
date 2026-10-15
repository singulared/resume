//! 2024-present [Digital Wave Technology](https://www.digitalwavetechnology.com/) is an AI-native platform for consumer brands.
//!
//! I work as a Core Software Engineer on OLAP database and Domain Engine solutions in a team of 4-5 developers.
//! The company provides the ONE Platform - an enterprise AI-native platform with master data management at its core,
//! serving retail, consumer brands, healthcare, and distribution industries.
use resume_macro::Project;

/// OLAP database engine for real-time analytical processing.
///
/// High-performance columnar OLAP database (olap-rust) with block-based storage, GraphQL API, and multi-user access control.
/// Designed for real-time analytical queries on large datasets with caching, external data connector framework (spouts),
/// and full-text search integration. Built with Rust, Tokio, Juniper/Async-GraphQL, Axum, and OpenTelemetry tracing.
#[derive(Project)]
#[project(parent = DigitalWave)]
#[role(
    Developer,
    span(2024),
    langs(Rust),
    doc = r#"Core Software Engineer on OLAP

Main responsibilities and achievements:
* Development of columnar storage engine with block-based data structures
* GraphQL API implementation and query optimization, including schema generation fixes and async resolver improvements
* Caching strategies and performance improvements, implemented multi-level caching with configurable TTL policies
* Multi-user access control and row-level security enhancements
* External data connector framework (spouts) for data loading with comprehensive error handling
* Optimized query processing with parallel execution capabilities
* Observability: OpenTelemetry tracing with proper context propagation, Prometheus metrics
* Build system and Docker containerization optimizations"#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Async runtime for concurrent operations.")]
#[tech(frameworks::Axum, "Web framework for HTTP server.")]
#[tech(frameworks::Tracing, "Observability and distributed tracing.")]
#[tech(frameworks::Juniper, "GraphQL server implementation.")]
pub struct OLAP;

/// Metadata-driven domain engine for complex data management.
///
/// Enterprise-grade domain engine providing GraphQL API for managing domains, entities, and attributes with dynamic
/// expression evaluation and multi-source data aggregation. Features fine-grained permissions, multi-dimensional attributes,
/// and connectivity to data sources (OLAP, PostgreSQL, ClickHouse). Built with Rust, Tokio, async-graphql, Axum,
/// DataFusion (SQL frontend), and OpenTelemetry.
#[derive(Project)]
#[project(parent = DigitalWave)]
#[role(
    Developer,
    span(2024),
    langs(Rust),
    doc = r#"Core Software Engineer on Domain Engine

Main responsibilities and achievements:
* Development of metadata-driven schema system and storage layer
* Implemented schema versioning and diff system for tracking schema evolution over time
* Expression language implementation for dynamic calculations
* GraphQL API design with federation support
* Multi-source persistent layer integration (OLAP, PostgreSQL, ClickHouse) with unified query interface
* Permissions system and access control implementation
* SQL frontend integration using DataFusion
* Fixed critical issues in API layer
* Observability: structured logging, tracing, Prometheus metrics"#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Async runtime for concurrent operations.")]
#[tech(frameworks::Axum, "Web framework for HTTP server.")]
#[tech(frameworks::AsyncGraphQL, "GraphQL server implementation.")]
#[tech(frameworks::DataFusion, "Query engine built on Apache Arrow.")]
#[tech(databases::Postgresql, "Relational database for structured data.")]
#[tech(databases::ClickHouse, "Columnar database for analytical workloads.")]
pub struct DomainEngine;
