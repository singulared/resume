//! 2023-2024 [Rask AI](https://www.rask.ai) is an AI-powered video localization and dubbing platform.
//!
//! I worked as a Senior Python Engineer on the main backend product and the dedicated API for enterprise clients.
//! The company serves over 2 million users with automatic translation and dubbing into 130+ languages.
use resume_macro::Project;

/// Main backend product for the Rask AI video localization platform.
///
/// Core backend service handling video and audio processing, AI-powered translation and dubbing workflows.
/// Built with Python, FastAPI, SQS, and DynamoDB.
#[derive(Project)]
#[project(parent = RaskAI)]
#[role(
    Developer,
    span(2023, 2024),
    langs(Python),
    doc = r#"Senior Python Engineer on Backend

Main responsibilities and achievements:
* Designed and implemented new features for video and audio processing workflows
* Optimized distributed task processing with SQS, improving throughput and reliability
* Integrated AI services for translation and dubbing into the main processing pipeline
* Refactored core backend components for better maintainability and performance
* Implemented monitoring and observability for critical processing workflows"#
)]
#[tech(languages::Python, "Python is the main programming language for this project.")]
#[tech(frameworks::FastAPI, "Web framework used for HTTP services.")]
#[tech(databases::DynamoDB, "NoSQL database for data storage.")]
#[tech(messages::SQS, "Message queue for distributed task processing.")]
pub struct Backend;

/// Dedicated API for enterprise clients.
///
/// Enterprise API for processing video and audio content at scale. Fully designed and implemented
/// to provide programmatic access to Rask AI's localization capabilities. Built with Python, FastAPI,
/// PostgreSQL, RabbitMQ, and DynamoDB.
#[derive(Project)]
#[project(parent = RaskAI)]
#[role(
    Developer,
    span(2023, 2024),
    langs(Python),
    doc = r#"Senior Python Engineer on API

Key achievements:
* Fully designed and implemented dedicated enterprise API from scratch
* Architected REST API with FastAPI for programmatic access to video/audio localization
* Designed database schema and data models for PostgreSQL
* Implemented async task processing with RabbitMQ for background jobs
* Built comprehensive API documentation and client integration guides
* Integrated with the main backend via DynamoDB to synchronize billing and usage limit statistics
* Established API versioning and backward compatibility strategy"#
)]
#[tech(languages::Python, "Python is the main programming language for this project.")]
#[tech(frameworks::FastAPI, "Web framework used for HTTP services.")]
#[tech(databases::Postgresql, "Relational database for API data.")]
#[tech(databases::DynamoDB, "NoSQL database for data storage.")]
#[tech(messages::RabbitMQ, "Message broker for async task processing.")]
pub struct API;
