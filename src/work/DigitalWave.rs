//! 2024-present [Digital Wave Technology](https://www.digitalwavetechnology.com/) is an AI-native platform for consumer brands.
//!
//! I work as a Core Software Engineer on OLAP database and Domain Engine solutions in a team of 4-5 developers.
//! The company provides the ONE Platform - an enterprise AI-native platform with master data management at its core,
//! serving retail, consumer brands, healthcare, and distribution industries.
use crate::roles::Developer;
use crate::technologies::databases::{ClickHouse, Postgresql};
use crate::technologies::frameworks::{AsyncGraphQL, Axum, DataFusion, Juniper, Tokio, Tracing};
use crate::technologies::languages::Rust;
use crate::CleanupDocs;

/// OLAP database engine for real-time analytical processing.
///
/// High-performance columnar OLAP database (olap-rust) with block-based storage, GraphQL API, and multi-user access control.
/// Designed for real-time analytical queries on large datasets with caching, external data connector framework (spouts),
/// and full-text search integration. Built with Rust, Tokio, Juniper/Async-GraphQL, Axum, and OpenTelemetry tracing.
#[derive(CleanupDocs)]
pub struct OLAP;
/// Core Software Engineer on OLAP
///
/// Main responsibilities and achievements:
/// * Development of columnar storage engine with block-based data structures
/// * GraphQL API implementation and query optimization, including schema generation fixes and async resolver improvements
/// * Caching strategies and performance improvements, implemented multi-level caching with configurable TTL policies
/// * Multi-user access control and row-level security enhancements
/// * External data connector framework (spouts) for data loading with comprehensive error handling
/// * Optimized query processing with parallel execution capabilities
/// * Observability: OpenTelemetry tracing with proper context propagation, Prometheus metrics
/// * Build system and Docker containerization optimizations
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2024, Language> Developer<Language> for &'period OLAP
where
    Language: Rust,
    'period: 'f2024,
{
}
/// Rust is the main programming language for this project.
impl Rust for OLAP {}
/// Async runtime for concurrent operations.
impl Tokio for OLAP {}
/// Web framework for HTTP server.
impl Axum for OLAP {}
/// Observability and distributed tracing.
impl Tracing for OLAP {}
/// GraphQL server implementation.
impl Juniper for OLAP {}

/// Metadata-driven domain engine for complex data management.
///
/// Enterprise-grade domain engine providing GraphQL API for managing domains, entities, and attributes with dynamic
/// expression evaluation and multi-source data aggregation. Features fine-grained permissions, multi-dimensional attributes,
/// and connectivity to data sources (OLAP, PostgreSQL, ClickHouse). Built with Rust, Tokio, async-graphql, Axum,
/// DataFusion (SQL frontend), and OpenTelemetry.
#[derive(CleanupDocs)]
pub struct DomainEngine;
/// Core Software Engineer on Domain Engine
///
/// Main responsibilities and achievements:
/// * Development of metadata-driven schema system and storage layer
/// * Implemented schema versioning and diff system for tracking schema evolution over time
/// * Expression language implementation for dynamic calculations
/// * GraphQL API design with federation support
/// * Multi-source persistent layer integration (OLAP, PostgreSQL, ClickHouse) with unified query interface
/// * Permissions system and access control implementation
/// * SQL frontend integration using DataFusion
/// * Fixed critical issues in API layer
/// * Observability: structured logging, tracing, Prometheus metrics
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2024, Language> Developer<Language> for &'period DomainEngine
where
    Language: Rust,
    'period: 'f2024,
{
}
/// Rust is the main programming language for this project.
impl Rust for DomainEngine {}
/// Async runtime for concurrent operations.
impl Tokio for DomainEngine {}
/// Web framework for HTTP server.
impl Axum for DomainEngine {}
/// GraphQL server implementation.
impl AsyncGraphQL for DomainEngine {}
/// Query engine built on Apache Arrow.
impl DataFusion for DomainEngine {}
/// Relational database for structured data.
impl Postgresql for DomainEngine {}
/// Columnar database for analytical workloads.
impl ClickHouse for DomainEngine {}
