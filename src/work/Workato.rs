//! 2022-2023 [Workato](https://www.workato.com) is an enterprise integration and automation platform (iPaaS).
//!
//! I worked as a Senior Rust Engineer in a team of 4-5 developers focused on performance optimization, benchmarking,
//! and observability integration for core infrastructure services that support the main Workato product (written in Ruby).
use crate::roles::Developer;
use crate::technologies::databases::{MySQL, Postgresql, Redis};
use crate::technologies::frameworks::{Criterion, Cucumber, Kafka, Poem, Tokio};
use crate::technologies::languages::Rust;
use crate::CleanupDocs;

/// WorkatoDB is a high-performance core infrastructure table storage/provider for the Workato platform.
///
/// Table storage provider used in automation processes (main product). Based on PostgreSQL or MySQL engines
/// (PostgreSQL is primary) with custom API and query planning for performance and predictable behavior.
#[derive(CleanupDocs)]
pub struct WorkatoDB;

/// Senior Rust Engineer on WorkatoDB
///
/// Main responsibilities and achievements:
/// * Performance benchmarking and optimization using Criterion framework
/// * Identified and resolved performance bottlenecks through systematic profiling
/// * Code refactoring for improved maintainability and architecture improvements
/// * Integrated comprehensive observability stack (metrics, tracing, monitoring)
/// * Implemented predictable query planning for consistent performance characteristics
/// * Contributed to design decisions for custom API layer over PostgreSQL/MySQL
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2022, 't2023, Language> Developer<Language> for &'period WorkatoDB
where
    Language: Rust,
    'period: 'f2022 + 't2023,
{
}

/// Rust is the main programming language for this project.
impl Rust for WorkatoDB {}
/// Primary database engine.
impl Postgresql for WorkatoDB {}
/// Secondary database engine option.
impl MySQL for WorkatoDB {}
/// Caching layer.
impl Redis for WorkatoDB {}
/// Project was built on top of Tokio async runtime.
impl Tokio for WorkatoDB {}
/// Web framework used for HTTP services.
impl Poem for WorkatoDB {}
/// Message broker for event streaming.
impl Kafka for WorkatoDB {}
/// Performance benchmarking framework.
impl Criterion for WorkatoDB {}
/// BDD testing framework.
impl Cucumber for WorkatoDB {}

/// File storage and management core infrastructure service for the Workato platform.
///
/// Hybrid file storage built on S3 and PostgreSQL for optimizing costs and performance. Stores small files/blobs
/// in PostgreSQL and larger files in S3. Provides a unified interface for the Workato platform including file storage
/// operations, streaming, billing, and storage usage statistics.
#[derive(CleanupDocs)]
pub struct WorkatoFiles;

/// Senior Rust Engineer on WorkatoFiles
///
/// Key contributions:
/// * Improved API layer for hybrid storage system (PostgreSQL for small files, S3 for large files)
/// * Enhanced unified interface for file storage operations
/// * Optimized file streaming capabilities for better performance
/// * Integrated billing and storage usage statistics tracking
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2022, 't2023, Language> Developer<Language> for &'period WorkatoFiles
where
    Language: Rust,
    'period: 'f2022 + 't2023,
{
}

/// Rust is the main programming language for this project.
impl Rust for WorkatoFiles {}
/// Main data storage.
impl Postgresql for WorkatoFiles {}
/// Project was built on top of Tokio async runtime.
impl Tokio for WorkatoFiles {}
/// Web framework used for HTTP services.
impl Poem for WorkatoFiles {}

/// Event exchange and subscription platform for the Workato ecosystem.
///
/// Event exchange and subscription platform built on Kafka with a simplified interface built with Poem.
/// Provides event streaming capabilities for integration workflows.
#[derive(CleanupDocs)]
pub struct WorkatoStreams;

/// Senior Rust Engineer on WorkatoStreams
///
/// Limited contributions focused on:
/// * API layer improvements for event streaming operations
/// * Configuration enhancements for better maintainability
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2022, 't2023, Language> Developer<Language> for &'period WorkatoStreams
where
    Language: Rust,
    'period: 'f2022 + 't2023,
{
}

/// Rust is the main programming language for this project.
impl Rust for WorkatoStreams {}
/// Message broker for event streaming.
impl Kafka for WorkatoStreams {}
/// Project was built on top of Tokio async runtime.
impl Tokio for WorkatoStreams {}
/// Web framework used for building simplified interface.
impl Poem for WorkatoStreams {}
