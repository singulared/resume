//! 2022-2023 [Workato](https://www.workato.com) is an enterprise integration and automation platform (iPaaS).
//!
//! I worked as a Senior Rust Engineer in a team of 4-5 developers focused on performance optimization, benchmarking,
//! and observability integration for core infrastructure services that support the main Workato product (written in Ruby).
use resume_macro::Project;

/// WorkatoDB is a high-performance core infrastructure table storage/provider for the Workato platform.
///
/// Table storage provider used in automation processes (main product). Based on PostgreSQL or MySQL engines
/// (PostgreSQL is primary) with custom API and query planning for performance and predictable behavior.
#[derive(Project)]
#[role(
    Developer,
    span(2022, 2023),
    langs(Rust),
    doc = r#"Senior Rust Engineer on WorkatoDB

Main responsibilities and achievements:
* Performance benchmarking and optimization using Criterion framework
* Identified and resolved performance bottlenecks through systematic profiling
* Integrated comprehensive observability stack (metrics, tracing, monitoring)
* Implemented predictable query planning for consistent performance characteristics
* Contributed to design decisions for custom API layer over PostgreSQL/MySQL"#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Project was built on top of Tokio async runtime.")]
#[tech(frameworks::Poem, "Web framework used for HTTP services.")]
#[tech(frameworks::Kafka, "Message broker for event streaming.")]
#[tech(frameworks::Criterion, "Performance benchmarking framework.")]
#[tech(frameworks::Cucumber, "BDD testing framework.")]
#[tech(databases::Postgresql, "Primary database engine.")]
#[tech(databases::MySQL, "Secondary database engine option.")]
#[tech(databases::Redis, "Caching layer.")]
pub struct WorkatoDB;

/// File storage and management core infrastructure service for the Workato platform.
///
/// Hybrid file storage built on S3 and PostgreSQL for optimizing costs and performance. Stores small files/blobs
/// in PostgreSQL and larger files in S3. Provides a unified interface for the Workato platform including file storage
/// operations, streaming, billing, and storage usage statistics.
#[derive(Project)]
#[role(
    Developer,
    span(2022, 2023),
    langs(Rust),
    doc = r#"Senior Rust Engineer on WorkatoFiles

Key contributions:
* Improved API layer for hybrid storage system (PostgreSQL for small files, S3 for large files)
* Enhanced unified interface for file storage operations
* Optimized file streaming capabilities for better performance
* Integrated billing and storage usage statistics tracking"#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Tokio, "Project was built on top of Tokio async runtime.")]
#[tech(frameworks::Poem, "Web framework used for HTTP services.")]
#[tech(databases::Postgresql, "Main data storage.")]
pub struct WorkatoFiles;

/// Event exchange and subscription platform for the Workato ecosystem.
///
/// Event exchange and subscription platform built on Kafka, with a simplified interface implemented in Poem.
/// Provides event streaming capabilities for integration workflows.
#[derive(Project)]
#[role(
    Developer,
    span(2022, 2023),
    langs(Rust),
    doc = r#"Senior Rust Engineer on WorkatoStreams

Limited contributions focused on:
* API layer improvements for event streaming operations
* Configuration enhancements for better maintainability"#
)]
#[tech(languages::Rust, "Rust is the main programming language for this project.")]
#[tech(frameworks::Kafka, "Message broker for event streaming.")]
#[tech(frameworks::Tokio, "Project was built on top of Tokio async runtime.")]
#[tech(frameworks::Poem, "Web framework used to build the simplified interface.")]
pub struct WorkatoStreams;
