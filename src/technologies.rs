//! My technology stack
use crate::hobbies::{Biking, EmbeddedSystems, Music, Piano, Tennis, CTF};

use self::{languages::Rust, tools::Linux};

/// My technical interests
pub trait TechnicalInterests
where
    Self: Rust + DistributedSystems + Linux + CRDT + EmbeddedSystems + Cryptography,
{
}

pub trait Hobbies
where
    Self: Music + Piano + Tennis + EmbeddedSystems + Biking + CTF,
{
}

pub trait CRDT {}
pub trait Cryptography {}
pub trait DistributedSystems {}

/// Declares technology traits, each with a display name for the template views
/// (`Postgresql = "PostgreSQL"`; the ident when omitted). The name lives on a
/// hidden `Technology` impl for `dyn Trait`, so rustdoc shows plain traits.
macro_rules! technologies {
    (@name $tech:ident $name:literal) => { $name };
    (@name $tech:ident) => { stringify!($tech) };
    ($($tech:ident $(= $name:literal)?),* $(,)?) => {$(
        pub trait $tech {}
        #[doc(hidden)]
        impl crate::resume_data::Technology for dyn $tech {
            const NAME: &'static str = technologies!(@name $tech $($name)?);
        }
    )*};
}

/// Programming languages
pub mod languages {
    pub trait Languages: Rust + Python {}
    technologies!(Rust, Python, CPP = "C++", PHP, Erlang, CS = "C#", JS = "JavaScript");
}

/// Databases
pub mod databases {
    technologies!(
        Postgresql = "PostgreSQL",
        Riak,
        Cassandra,
        MySQL,
        ClickHouse,
        Redis,
        DynamoDB,
        MongoDB,
        Neo4j,
    );
}

/// Frameworks
pub mod frameworks {
    technologies!(
        Actix,
        Axum,
        Tokio,
        AsyncIO = "asyncio",
        AioHTTP = "aiohttp",
        FastAPI,
        Flask,
        Celery,
        Django,
        OpenGL,
        Qt,
        Poem,
        Kafka,
        Criterion,
        Cucumber,
        Tracing,
        Juniper,
        AsyncGraphQL = "async-graphql",
        DataFusion,
    );
}

/// Message brokers, event handling systems and RPCs.
pub mod messages {
    technologies!(RabbitMQ, GRPC = "gRPC", SQS);
}

/// Working tools
pub mod tools {
    technologies!(Linux, Neovim);
}
