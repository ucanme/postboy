//! Postboy local storage layer
//!
//! This crate provides local persistence for all Postboy data.
//! Designed with an offline-first approach that can be extended for cloud sync.

pub mod database;
pub mod collections;
pub mod requests;
pub mod environments;
pub mod settings;
pub mod migrations;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions};
use std::path::Path;
use std::str::FromStr;
use anyhow::Result;

pub use database::Database;

/// Re-export commonly used types
pub use models::{Id, Timestamp, new_id, now};

/// Storage configuration
#[derive(Debug, Clone)]
pub struct StoreConfig {
    /// Database file path
    pub db_path: String,

    /// Maximum pool size
    pub max_connections: u32,

    /// Enable WAL mode (better concurrent access)
    pub enable_wal: bool,

    /// Enable foreign key constraints
    pub enable_foreign_keys: bool,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            db_path: "postboy.db".to_string(),
            max_connections: 5,
            enable_wal: true,
            enable_foreign_keys: true,
        }
    }
}

impl StoreConfig {
    /// Create a new store config with a custom database path
    pub fn with_db_path(mut self, path: impl Into<String>) -> Self {
        self.db_path = path.into();
        self
    }

    /// Set the maximum number of database connections
    pub fn with_max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Enable or disable WAL mode
    pub fn with_wal(mut self, enable: bool) -> Self {
        self.enable_wal = enable;
        self
    }
}

/// Initialize and open the database
pub async fn open_store(config: StoreConfig) -> Result<Database> {
    let db_path = &config.db_path;

    // Ensure parent directory exists
    if let Some(parent) = Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }

    // Configure connection options
    let mut options = SqliteConnectOptions::from_str(db_path)?;

    if config.enable_wal {
        options = options.pragma("journal_mode", "WAL");
        options = options.pragma("synchronous", "NORMAL");
    }

    if config.enable_foreign_keys {
        options = options.pragma("foreign_keys", "true");
    }

    // Performance optimizations
    options = options.pragma("cache_size", "-64000"); // 64MB cache
    options = options.pragma("temp_store", "memory");

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .connect_with(options)
        .await?;

    // Run migrations
    migrations::run_migrations(&pool).await?;

    Ok(Database::new(pool))
}

/// Result type alias for store operations
pub type StoreResult<T> = Result<T, StoreError>;

/// Store operation errors
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Item not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Migration error: {0}")]
    Migration(String),
}

/// Transaction wrapper for atomic operations
pub struct Transaction<'a> {
    inner: sqlx::Transaction<'a, sqlx::Sqlite>,
}

impl<'a> Transaction<'a> {
    /// Create a new transaction wrapper
    pub fn new(tx: sqlx::Transaction<'a, sqlx::Sqlite>) -> Self {
        Self { inner: tx }
    }

    /// Commit the transaction
    pub async fn commit(self) -> Result<()> {
        self.inner.commit().await?;
        Ok(())
    }

    /// Rollback the transaction
    pub async fn rollback(self) -> Result<()> {
        self.inner.rollback().await?;
        Ok(())
    }

    /// Get access to the inner transaction
    pub fn as_mut(&mut self) -> &mut sqlx::Transaction<'a, sqlx::Sqlite> {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_store_in_memory() {
        let config = StoreConfig {
            db_path: ":memory:".to_string(),
            ..Default::default()
        };

        let db = open_store(config).await.unwrap();
        drop(db);
    }

    #[tokio::test]
    async fn test_open_store_with_wal() {
        let config = StoreConfig {
            db_path: ":memory:".to_string(),
            enable_wal: true,
            ..Default::default()
        };

        let db = open_store(config).await.unwrap();

        // Verify WAL mode is enabled
        let journal_mode: (String,) = sqlx::query_as(
            "PRAGMA journal_mode"
        )
        .fetch_one(db.pool())
        .await
        .unwrap();

        assert_eq!(journal_mode.0, "wal");
    }
}
