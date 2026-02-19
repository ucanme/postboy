//! Database migration runner

use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateDatabase};
use std::path::Path;
use anyhow::Result;

/// Run all database migrations
pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // sqlx::migrate! will look for migrations in the migrations/ directory
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .with_context(|| "Failed to run database migrations")?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}

/// Create a new database file and run migrations
pub async fn create_database(db_path: impl AsRef<Path>) -> Result<SqlitePool> {
    let path = db_path.as_ref();
    let db_url = format!("sqlite:{}", path.display());

    // Create database if it doesn't exist
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
        tracing::info!("Created new database at: {}", path.display());
    }

    // Build connection pool
    let pool = SqlitePool::connect(&db_url)
        .await
        .with_context(|| format!("Failed to connect to database: {}", path.display()))?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Get the current schema version from the database
pub async fn get_schema_version(pool: &SqlitePool) -> Result<Option<i64>> {
    let result: Option<(i64,)> = sqlx::query_as(
        "SELECT version FROM schema_version ORDER BY version DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|(v,)| v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_migrations() {
        let pool = SqlitePool::connect(":memory:")
            .await
            .unwrap();

        assert!(run_migrations(&pool).await.is_ok());

        // Verify schema version was created
        let version = get_schema_version(&pool).await.unwrap();
        assert!(version.is_some());
        assert!(version.unwrap() >= 1);
    }
}
