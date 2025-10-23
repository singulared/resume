//! 2023-2024 [Rask AI](https://www.rask.ai) is an AI-powered video localization and dubbing platform.
//!
//! I worked as a Senior Python Engineer on the main backend product and dedicated API for enterprise clients.
//! The company serves over 2 million users with automatic translation and dubbing into 130+ languages.
use crate::roles::Developer;
use crate::technologies::databases::{DynamoDB, Postgresql};
use crate::technologies::frameworks::FastAPI;
use crate::technologies::languages::Python;
use crate::technologies::messages::{RabbitMQ, SQS};
use crate::CleanupDocs;

/// Main backend product for Rask AI video localization platform.
///
/// Core backend service handling video and audio processing, AI-powered translation and dubbing workflows.
/// Built with Python, FastAPI, SQS, and DynamoDB.
#[derive(CleanupDocs)]
pub struct Backend;
/// Senior Python Engineer on Backend
///
/// Main responsibilities and achievements:
/// * Designed and implemented new features for video and audio processing workflows
/// * Optimized distributed task processing with SQS, improving throughput and reliability
/// * Integrated AI services for translation and dubbing into the main processing pipeline
/// * Refactored core backend components for better maintainability and performance
/// * Implemented monitoring and observability for critical processing workflows
/// * Contributed to architectural decisions for scalability improvements
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2023, 't2024, Language> Developer<Language> for &'period Backend
where
    Language: Python,
    'period: 'f2023 + 't2024,
{
}
/// Python is the main programming language for this project.
impl Python for Backend {}
/// Web framework used for HTTP services.
impl FastAPI for Backend {}
/// Message queue for distributed task processing.
impl SQS for Backend {}
/// NoSQL database for data storage.
impl DynamoDB for Backend {}

/// Dedicated API for enterprise clients.
///
/// Enterprise API for processing video and audio content at scale. Fully designed and implemented
/// to provide programmatic access to Rask AI's localization capabilities. Built with Python, FastAPI,
/// PostgreSQL, RabbitMQ, and DynamoDB.
#[derive(CleanupDocs)]
pub struct API;
/// Senior Python Engineer on API
///
/// Key achievements:
/// * Fully designed and implemented dedicated enterprise API from scratch
/// * Architected REST API with FastAPI for programmatic access to video/audio localization
/// * Designed database schema and data models for PostgreSQL
/// * Implemented async task processing with RabbitMQ for background jobs
/// * Built comprehensive API documentation and client integration guides
/// * Integrated with main backend via DynamoDB for synchronizing billing and usage limits statistics
/// * Established API versioning and backward compatibility strategy
/// * Delivered production-ready API serving enterprise clients at scale
#[allow(clippy::extra_unused_lifetimes)]
impl<'period, 'f2023, 't2024, Language> Developer<Language> for &'period API
where
    Language: Python,
    'period: 'f2023 + 't2024,
{
}
/// Python is the main programming language for this project.
impl Python for API {}
/// Web framework used for HTTP services.
impl FastAPI for API {}
/// Relational database for API data.
impl Postgresql for API {}
/// Message broker for async task processing.
impl RabbitMQ for API {}
/// NoSQL database for data storage.
impl DynamoDB for API {}
