//! Database connection and schema management for Postboy
//!
//! This module handles SQLite database initialization, migrations,
//! and connection pooling for local-only operation with future cloud sync support.

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, sqlite::Sqlite, Pool};
use sqlx::migrate::MigrateDatabase;
use std::path::Path;
use std::str::FromStr;
use anyhow::{Context, Result};

/// Database manager for Postboy
#[derive(Clone)]
pub struct Db {
    pool: SqlitePool,
}

impl Db {
    /// Initialize a new database connection
    ///
    /// Creates the database file if it doesn't exist and runs migrations.
    pub async fn new(db_path: impl AsRef<Path>) -> Result<Self> {
        let path = db_path.as_ref();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await
                .with_context(|| format!("Failed to create database directory: {:?}", parent))?;
        }

        // Create database if it doesn't exist
        let db_url = format!("sqlite:{}", path.display());
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await
                .context("Failed to create database")?;
        }

        // Configure connection options for optimal performance
        let options = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            // Enable WAL mode for better concurrency
            .pragma("journal_mode", "WAL")
            // Normal sync mode (safe but faster)
            .pragma("synchronous", "NORMAL")
            // Store temporary tables in memory
            .pragma("temp_store", "MEMORY")
            // Enable memory-mapped I/O for better performance
            .pragma("mmap_size", "30000000000")
            // Page size for better performance
            .pragma("page_size", "4096")
            // Cache size (negative value = KB)
            .pragma("cache_size", "-64000") // 64MB cache
            // Busy timeout for concurrent access
            .busy_timeout(std::time::Duration::from_secs(30))
            // Foreign key constraints
            .pragma("foreign_keys", "true");

        // Create connection pool
        let pool = SqlitePool::connect_with(options)
            .await
            .context("Failed to connect to database")?;

        // Run migrations
        Self::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    /// Create an in-memory database for testing
    pub async fn in_memory() -> Result<Self> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?
            .pragma("foreign_keys", "true");

        let pool = SqlitePool::connect_with(options)
            .await
            .context("Failed to create in-memory database")?;

        Self::run_migrations(&pool).await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .context("Failed to run database migrations")?;
        Ok(())
    }

    /// Get a reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Close the database connection pool
    pub async fn close(self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }

    /// Health check - verify database is accessible
    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .context("Database ping failed")?;
        Ok(())
    }

    /// Begin a new transaction
    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, Sqlite>> {
        self.pool.begin().await
            .context("Failed to begin transaction")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_db() {
        let db = Db::in_memory().await.unwrap();
        assert!(db.ping().await.is_ok());
    }

    #[tokio::test]
    async fn test_temp_db() {
        let db = Db::new(":memory:").await.unwrap();
        assert!(db.ping().await.is_ok());
    }
}
