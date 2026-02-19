//! Postboy core data models
//!
//! This crate contains all the data types used throughout the Postboy application.
//! All models are designed to be serializable for storage and network transmission.

pub mod collection;
pub mod request;
pub mod response;
pub mod environment;
pub mod user;
pub mod sync;

pub use collection::*;
pub use request::*;
pub use response::*;
pub use environment::*;
pub use user::*;
pub use sync::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Re-export commonly used types
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

/// Unique identifier type alias
pub type Id = Uuid;

/// Timestamp type (milliseconds since epoch)
pub type Timestamp = i64;

/// Generate a new unique ID
pub fn new_id() -> Id {
    Uuid::new_v4()
}

/// Get current timestamp
pub fn now() -> Timestamp {
    use chrono::Utc;
    Utc::now().timestamp_millis()
}

/// Trait for entities that can be created and updated
pub trait Temporal {
    fn created_at(&self) -> Timestamp;
    fn updated_at(&self) -> Timestamp;
}

/// Trait for entities with unique identifier
pub trait Identifiable {
    fn id(&self) -> Id;
}
