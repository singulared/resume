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

/// Programming languages
pub mod languages {
    pub trait Languages: Rust + Python {}
    pub trait Rust {}
    pub trait Python {}
    pub trait CPP {}
    pub trait PHP {}
    pub trait Erlang {}
    pub trait CS {}
    pub trait JS {}
}

/// Databases
pub mod databases {
    pub trait Postgresql {}
    pub trait Riak {}
    pub trait Cassandra {}
    pub trait MySQL {}
    pub trait ClickHouse {}
}

/// Frameworks
pub mod frameworks {
    pub trait Actix {}
    pub trait Tokio {}
    pub trait AsyncIO {}
    pub trait AioHTTP {}
    pub trait Flask {}
    pub trait Celery {}
    pub trait Django {}
    pub trait OpenGL {}
    pub trait Qt {}
}

/// Message brokers, event handling systems and RPCs.
pub mod messages {
    pub trait RabbitMQ {}
    pub trait GRPC {}
}

/// Working tools
pub mod tools {
    pub trait Linux {}
    pub trait Neovim {}
}
