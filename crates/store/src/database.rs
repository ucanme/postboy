//! Database wrapper with high-level operations for Postboy
//!
//! Provides a clean API over raw SQL operations for all CRUD operations.
//! Designed for offline-first with future cloud sync compatibility.

use sqlx::{SqlitePool, sqlite::Sqlite};
use std::sync::Arc;
use anyhow::Result;

use crate::{StoreError, StoreResult};
use models::{Id, Timestamp, new_id, now};

/// Main database interface for Postboy
#[derive(Clone)]
pub struct Database {
    pool: Arc<SqlitePool>,
}

impl Database {
    /// Create a new database wrapper
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool: Arc::new(pool),
        }
    }

    /// Get reference to the connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Begin a new transaction
    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, Sqlite>> {
        self.pool
            .begin()
            .await
            .map_err(|e| StoreError::Database(e).into())
    }

    /// Health check - verify database is accessible
    pub async fn ping(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e).into())
    }

    /// Get database statistics
    pub async fn stats(&self) -> Result<DbStats> {
        let collections_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM collections")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let requests_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM requests")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let environments_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM environments")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let history_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM request_history")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let pending_sync: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sync_changes WHERE synced = 0")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        // Get database file size
        let page_count: i64 = sqlx::query_scalar("PRAGMA page_count")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let page_size: i64 = sqlx::query_scalar("PRAGMA page_size")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))?;

        let db_size_bytes = page_count * page_size;

        Ok(DbStats {
            collections_count: collections_count as usize,
            requests_count: requests_count as usize,
            environments_count: environments_count as usize,
            history_count: history_count as usize,
            pending_sync_changes: pending_sync as usize,
            db_size_bytes,
        })
    }

    /// Vacuum the database to reclaim space
    pub async fn vacuum(&self) -> Result<()> {
        sqlx::query("VACUUM")
            .execute(self.pool())
            .await
            .map_err(|e| StoreError::Database(e).into())
    }

    /// Analyze the database to update statistics
    pub async fn analyze(&self) -> Result<()> {
        sqlx::query("ANALYZE")
            .execute(self.pool())
            .await
            .map_err(|e| StoreError::Database(e).into())
    }

    /// Export all data as JSON (for backup/migration)
    pub async fn export_json(&self) -> Result<serde_json::Value> {
        let collections: Vec<serde_json::Value> = sqlx::query(
            "SELECT * FROM collections ORDER BY created_at"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| StoreError::Database(e))?
        .into_iter()
        .map(|row| {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: Option<String> = row.get("description");
            let info: String = row.get("info");
            let auth: Option<String> = row.get("auth");
            let sync_state: String = row.get("sync_state");
            let ui_state: String = row.get("ui_state");
            let created_at: i64 = row.get("created_at");
            let updated_at: i64 = row.get("updated_at");

            serde_json::json!({
                "id": id,
                "name": name,
                "description": description,
                "info": serde_json::from_str::<serde_json::Value>(&info).unwrap_or_default(),
                "auth": auth.and_then(|a| serde_json::from_str(&a).ok()),
                "sync_state": serde_json::from_str::<serde_json::Value>(&sync_state).unwrap_or_default(),
                "ui_state": serde_json::from_str::<serde_json::Value>(&ui_state).unwrap_or_default(),
                "created_at": created_at,
                "updated_at": updated_at,
            })
        })
        .collect();

        let requests: Vec<serde_json::Value> = sqlx::query(
            "SELECT * FROM requests ORDER BY created_at"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| StoreError::Database(e))?
        .into_iter()
        .map(|row| {
            let id: String = row.get("id");
            let collection_id: Option<String> = row.get("collection_id");
            let folder_id: Option<String> = row.get("folder_id");
            let name: String = row.get("name");
            let description: Option<String> = row.get("description");
            let method: String = row.get("method");
            let url_raw: String = row.get("url_raw");
            let headers: String = row.get("headers");
            let query_params: String = row.get("query_params");
            let body: String = row.get("body");
            let auth: Option<String> = row.get("auth");
            let script: String = row.get("script");
            let ui_state: String = row.get("ui_state");
            let created_at: i64 = row.get("created_at");
            let updated_at: i64 = row.get("updated_at");

            serde_json::json!({
                "id": id,
                "collection_id": collection_id,
                "folder_id": folder_id,
                "name": name,
                "description": description,
                "method": method,
                "url": {"raw": url_raw},
                "headers": serde_json::from_str::<Vec<serde_json::Value>>(&headers).unwrap_or_default(),
                "query_params": serde_json::from_str::<Vec<serde_json::Value>>(&query_params).unwrap_or_default(),
                "body": serde_json::from_str::<serde_json::Value>(&body).unwrap_or_default(),
                "auth": auth.and_then(|a| serde_json::from_str(&a).ok()),
                "script": serde_json::from_str::<serde_json::Value>(&script).unwrap_or_default(),
                "ui_state": serde_json::from_str::<serde_json::Value>(&ui_state).unwrap_or_default(),
                "created_at": created_at,
                "updated_at": updated_at,
            })
        })
        .collect();

        let environments: Vec<serde_json::Value> = sqlx::query(
            "SELECT * FROM environments ORDER BY created_at"
        )
        .fetch_all(self.pool())
        .await
        .map_err(|e| StoreError::Database(e))?
        .into_iter()
        .map(|row| {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let variables: String = row.get("variables");
            let is_active: bool = row.get("is_active");
            let sync_state: String = row.get("sync_state");
            let created_at: i64 = row.get("created_at");
            let updated_at: i64 = row.get("updated_at");

            serde_json::json!({
                "id": id,
                "name": name,
                "variables": serde_json::from_str::<Vec<serde_json::Value>>(&variables).unwrap_or_default(),
                "is_active": is_active,
                "sync_state": serde_json::from_str::<serde_json::Value>(&sync_state).unwrap_or_default(),
                "created_at": created_at,
                "updated_at": updated_at,
            })
        })
        .collect();

        let globals: serde_json::Value = sqlx::query("SELECT * FROM globals")
            .fetch_one(self.pool())
            .await
            .map_err(|e| StoreError::Database(e))
            .and_then(|row| {
                let variables: String = row.get("variables");
                serde_json::from_str::<serde_json::Value>(&variables)
                    .map_err(|e| StoreError::Deserialization(e.to_string()))
            })?;

        Ok(serde_json::json!({
            "version": 1,
            "exported_at": now(),
            "collections": collections,
            "requests": requests,
            "environments": environments,
            "globals": globals,
        }))
    }

    /// Import data from JSON export
    pub async fn import_json(&self, data: &serde_json::Value) -> Result<ImportResult> {
        let mut result = ImportResult::default();

        let mut tx = self.begin().await?;

        // Import globals first
        if let Some(globals) = data.get("globals") {
            let variables_json = serde_json::to_string(globals)
                .map_err(|e| StoreError::Serialization(e.to_string()))?;

            sqlx::query(
                "UPDATE globals SET variables = ?, updated_at = ?"
            )
            .bind(&variables_json)
            .bind(now())
            .execute(&mut *tx)
            .await
            .map_err(|e| StoreError::Database(e))?;

            result.globals_imported = 1;
        }

        // Import environments
        if let Some(envs) = data.get("environments").and_then(|v| v.as_array()) {
            for env in envs {
                let id = env.get("id").and_then(|v| v.as_str())
                    .unwrap_or_else(|| new_id().to_string());
                let name = env.get("name").and_then(|v| v.as_str())
                    .ok_or_else(|| StoreError::InvalidData("Environment name missing".into()))?;
                let variables = serde_json::to_string(env.get("variables").unwrap_or(&serde_json::json!([])))
                    .map_err(|e| StoreError::Serialization(e.to_string()))?;

                sqlx::query(
                    "INSERT OR REPLACE INTO environments (id, name, variables, is_active, sync_state, created_at, updated_at)
                    VALUES (?, ?, ?, 0, '{}', ?, ?)"
                )
                .bind(&id)
                .bind(name)
                .bind(&variables)
                .bind(now())
                .bind(now())
                .execute(&mut *tx)
                .await
                .map_err(|e| StoreError::Database(e))?;

                result.environments_imported += 1;
            }
        }

        // Import collections
        if let Some(collections) = data.get("collections").and_then(|v| v.as_array()) {
            for collection in collections {
                let id = collection.get("id").and_then(|v| v.as_str())
                    .unwrap_or_else(|| new_id().to_string());
                let name = collection.get("name").and_then(|v| v.as_str())
                    .ok_or_else(|| StoreError::InvalidData("Collection name missing".into()))?;
                let description = collection.get("description").and_then(|v| v.as_str());
                let info = serde_json::to_string(
                    collection.get("info").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let sync_state = serde_json::to_string(
                    collection.get("sync_state").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let ui_state = serde_json::to_string(
                    collection.get("ui_state").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let auth = collection.get("auth")
                    .and_then(|v| v.as_str())
                    .and_then(|s| serde_json::to_string(&serde_json::json!(s)).ok());

                sqlx::query(
                    "INSERT OR REPLACE INTO collections (id, name, description, info, auth, sync_state, ui_state, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&id)
                .bind(name)
                .bind(description)
                .bind(&info)
                .bind(&auth)
                .bind(&sync_state)
                .bind(&ui_state)
                .bind(now())
                .bind(now())
                .execute(&mut *tx)
                .await
                .map_err(|e| StoreError::Database(e))?;

                result.collections_imported += 1;
            }
        }

        // Import requests
        if let Some(requests) = data.get("requests").and_then(|v| v.as_array()) {
            for request in requests {
                let id = request.get("id").and_then(|v| v.as_str())
                    .unwrap_or_else(|| new_id().to_string());
                let collection_id = request.get("collection_id").and_then(|v| v.as_str());
                let folder_id = request.get("folder_id").and_then(|v| v.as_str());
                let name = request.get("name").and_then(|v| v.as_str())
                    .ok_or_else(|| StoreError::InvalidData("Request name missing".into()))?;
                let method = request.get("method").and_then(|v| v.as_str())
                    .ok_or_else(|| StoreError::InvalidData("Request method missing".into()))?;
                let url = request.get("url")
                    .and_then(|v| v.as_object())
                    .and_then(|o| o.get("raw"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| StoreError::InvalidData("Request URL missing".into()))?;
                let headers = serde_json::to_string(
                    request.get("headers").unwrap_or(&serde_json::json!([]))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let query_params = serde_json::to_string(
                    request.get("query_params").unwrap_or(&serde_json::json!([]))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let body = serde_json::to_string(
                    request.get("body").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let script = serde_json::to_string(
                    request.get("script").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let ui_state = serde_json::to_string(
                    request.get("ui_state").unwrap_or(&serde_json::json!({}))
                ).map_err(|e| StoreError::Serialization(e.to_string()))?;
                let auth = request.get("auth")
                    .and_then(|v| v.as_str())
                    .and_then(|s| serde_json::to_string(&serde_json::json!(s)).ok());

                sqlx::query(
                    "INSERT OR REPLACE INTO requests
                    (id, collection_id, folder_id, name, method, url_raw, headers, query_params, body, auth, script, ui_state, created_at, updated_at)
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                )
                .bind(&id)
                .bind(collection_id)
                .bind(folder_id)
                .bind(name)
                .bind(method)
                .bind(url)
                .bind(&headers)
                .bind(&query_params)
                .bind(&body)
                .bind(&auth)
                .bind(&script)
                .bind(&ui_state)
                .bind(now())
                .bind(now())
                .execute(&mut *tx)
                .await
                .map_err(|e| StoreError::Database(e))?;

                result.requests_imported += 1;
            }
        }

        tx.commit().await?;
        Ok(result)
    }
}

/// Database statistics
#[derive(Debug, Clone, serde::Serialize)]
pub struct DbStats {
    pub collections_count: usize,
    pub requests_count: usize,
    pub environments_count: usize,
    pub history_count: usize,
    pub pending_sync_changes: usize,
    pub db_size_bytes: i64,
}

/// Result of an import operation
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ImportResult {
    pub collections_imported: usize,
    pub requests_imported: usize,
    pub environments_imported: usize,
    pub globals_imported: usize,
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_ping() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .unwrap();
        let db = Database::new(pool);

        assert!(db.ping().await.is_ok());
    }
}
