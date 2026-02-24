//! Collection service implementation
//!
//! Provides business logic for managing HTTP request collections,
//! including CRUD operations and hierarchical organization.

use anyhow::Result;
use postboy_models::{Collection, Folder, Id, new_id, now};
use postboy_store::Database;
use tracing::{info, debug, instrument};

/// Collection service for managing collections
#[derive(Clone)]
pub struct CollectionService {
    db: Database,
}

impl CollectionService {
    /// Create a new collection service
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a new collection
    #[instrument(skip(self))]
    pub async fn create_collection(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<Collection> {
        let mut collection = Collection::new(name);

        if let Some(desc) = description {
            collection = collection.with_description(desc);
        }

        // Save to database
        let collection_json = serde_json::to_string(&collection)?;

        sqlx::query(
            r#"
            INSERT INTO collections (id, name, description, data, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(collection.id.to_string())
        .bind(&collection.name)
        .bind(&collection.description)
        .bind(&collection_json)
        .bind(collection.created_at as i64)
        .bind(collection.updated_at as i64)
        .execute(self.db.pool())
        .await?;

        info!("Created collection: {} ({})", collection.name, collection.id);
        Ok(collection)
    }

    /// Get a collection by ID
    #[instrument(skip(self))]
    pub async fn get_collection(&self, id: Id) -> Result<Option<Collection>> {
        let result = sqlx::query_as::<_, (String,)>(
            "SELECT data FROM collections WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(self.db.pool())
        .await?;

        if let Some((data,)) = result {
            let collection: Collection = serde_json::from_str(&data)?;
            Ok(Some(collection))
        } else {
            Ok(None)
        }
    }

    /// List all collections
    #[instrument(skip(self))]
    pub async fn list_collections(&self) -> Result<Vec<Collection>> {
        let rows = sqlx::query_as::<_, (String,)>(
            "SELECT data FROM collections ORDER BY name ASC"
        )
        .fetch_all(self.db.pool())
        .await?;

        let mut collections = Vec::new();
        for (data,) in rows {
            let collection: Collection = serde_json::from_str(&data)?;
            collections.push(collection);
        }

        debug!("Listed {} collections", collections.len());
        Ok(collections)
    }

    /// Update a collection
    #[instrument(skip(self))]
    pub async fn update_collection(
        &self,
        id: Id,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Option<Collection>> {
        // First, get the existing collection
        let existing = match self.get_collection(id).await? {
            Some(c) => c,
            None => return Ok(None),
        };

        // Update fields
        let mut updated = existing.clone();
        if let Some(name) = name {
            updated.name = name;
        }
        if let Some(description) = description {
            updated.description = Some(description);
        }
        updated.updated_at = now();

        // Save to database
        let collection_json = serde_json::to_string(&updated)?;

        sqlx::query(
            r#"
            UPDATE collections
            SET name = ?, description = ?, data = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&updated.name)
        .bind(&updated.description)
        .bind(&collection_json)
        .bind(updated.updated_at as i64)
        .bind(id.to_string())
        .execute(self.db.pool())
        .await?;

        info!("Updated collection: {} ({})", updated.name, id);
        Ok(Some(updated))
    }

    /// Delete a collection
    #[instrument(skip(self))]
    pub async fn delete_collection(&self, id: Id) -> Result<bool> {
        let result = sqlx::query("DELETE FROM collections WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!("Deleted collection: {}", id);
        }

        Ok(deleted)
    }

    /// Add a folder to a collection
    #[instrument(skip(self))]
    pub async fn create_folder(
        &self,
        collection_id: Id,
        name: String,
        description: Option<String>,
        parent_id: Option<Id>,
    ) -> Result<Folder> {
        let mut folder = Folder::new(name);

        if let Some(desc) = description {
            folder = folder.with_description(desc);
        }

        if let Some(parent) = parent_id {
            folder = folder.with_parent(parent);
        }

        // Get the collection and add the folder
        let mut collection = self.get_collection(collection_id).await?
            .ok_or_else(|| anyhow::anyhow!("Collection not found: {}", collection_id))?;

        collection.folders.push(folder.clone());
        collection.updated_at = now();

        // Update collection
        let collection_json = serde_json::to_string(&collection)?;

        sqlx::query(
            r#"
            UPDATE collections
            SET data = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&collection_json)
        .bind(collection.updated_at as i64)
        .bind(collection_id.to_string())
        .execute(self.db.pool())
        .await?;

        info!("Created folder '{}' in collection {}", folder.name, collection_id);
        Ok(folder)
    }

    /// Update a folder
    #[instrument(skip(self))]
    pub async fn update_folder(
        &self,
        collection_id: Id,
        folder_id: Id,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Option<Folder>> {
        let mut collection = self.get_collection(collection_id).await?
            .ok_or_else(|| anyhow::anyhow!("Collection not found: {}", collection_id))?;

        // Find and update the folder
        let folder = collection.folders.iter_mut()
            .find(|f| f.id == folder_id);

        if let Some(folder) = folder {
            if let Some(name) = name {
                folder.name = name;
            }
            if let Some(description) = description {
                folder.description = Some(description);
            }
            folder.updated_at = now();
            collection.updated_at = now();

            // Clone the folder before we need to serialize the collection
            let updated_folder = folder.clone();

            // Save to database
            let collection_json = serde_json::to_string(&collection)?;

            sqlx::query(
                "UPDATE collections SET data = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&collection_json)
            .bind(collection.updated_at as i64)
            .bind(collection_id.to_string())
            .execute(self.db.pool())
            .await?;

            info!("Updated folder {} in collection {}", folder_id, collection_id);
            Ok(Some(updated_folder))
        } else {
            Ok(None)
        }
    }

    /// Delete a folder
    #[instrument(skip(self))]
    pub async fn delete_folder(
        &self,
        collection_id: Id,
        folder_id: Id,
    ) -> Result<bool> {
        let mut collection = self.get_collection(collection_id).await?
            .ok_or_else(|| anyhow::anyhow!("Collection not found: {}", collection_id))?;

        let original_len = collection.folders.len();
        collection.folders.retain(|f| f.id != folder_id);

        let deleted = collection.folders.len() < original_len;

        if deleted {
            collection.updated_at = now();

            // Save to database
            let collection_json = serde_json::to_string(&collection)?;

            sqlx::query(
                "UPDATE collections SET data = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&collection_json)
            .bind(collection.updated_at as i64)
            .bind(collection_id.to_string())
            .execute(self.db.pool())
            .await?;

            info!("Deleted folder {} from collection {}", folder_id, collection_id);
        }

        Ok(deleted)
    }

    /// Search collections by name or description
    #[instrument(skip(self))]
    pub async fn search_collections(&self, query: &str) -> Result<Vec<Collection>> {
        let search_pattern = format!("%{}%", query);

        let rows = sqlx::query_as::<_, (String,)>(
            r#"
            SELECT data FROM collections
            WHERE name LIKE ? OR description LIKE ?
            ORDER BY name ASC
            "#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(self.db.pool())
        .await?;

        let mut collections = Vec::new();
        for (data,) in rows {
            let collection: Collection = serde_json::from_str(&data)?;
            collections.push(collection);
        }

        debug!("Found {} collections matching '{}'", collections.len(), query);
        Ok(collections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postboy_store::StoreConfig;

    async fn setup_test_service() -> CollectionService {
        let db = postboy_store::open_store(StoreConfig {
            db_path: ":memory:".to_string(),
            ..Default::default()
        }).await.unwrap();

        CollectionService::new(db)
    }

    #[tokio::test]
    async fn test_create_and_get_collection() {
        let service = setup_test_service().await;

        let collection = service.create_collection(
            "Test Collection".to_string(),
            Some("A test collection".to_string()),
        ).await.unwrap();

        assert_eq!(collection.name, "Test Collection");
        assert_eq!(collection.description, Some("A test collection".to_string()));

        let retrieved = service.get_collection(collection.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Test Collection");
    }

    #[tokio::test]
    async fn test_list_collections() {
        let service = setup_test_service().await;

        service.create_collection("Collection 1".to_string(), None).await.unwrap();
        service.create_collection("Collection 2".to_string(), None).await.unwrap();

        let collections = service.list_collections().await.unwrap();
        assert_eq!(collections.len(), 2);
    }

    #[tokio::test]
    async fn test_update_collection() {
        let service = setup_test_service().await;

        let collection = service.create_collection(
            "Original Name".to_string(),
            None,
        ).await.unwrap();

        let updated = service.update_collection(
            collection.id,
            Some("Updated Name".to_string()),
            Some("Updated description".to_string()),
        ).await.unwrap();

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.description, Some("Updated description".to_string()));
    }

    #[tokio::test]
    async fn test_delete_collection() {
        let service = setup_test_service().await;

        let collection = service.create_collection(
            "To Delete".to_string(),
            None,
        ).await.unwrap();

        let deleted = service.delete_collection(collection.id).await.unwrap();
        assert!(deleted);

        let retrieved = service.get_collection(collection.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_create_folder() {
        let service = setup_test_service().await;

        let collection = service.create_collection(
            "Test Collection".to_string(),
            None,
        ).await.unwrap();

        let folder = service.create_folder(
            collection.id,
            "Test Folder".to_string(),
            Some("A test folder".to_string()),
            None,
        ).await.unwrap();

        assert_eq!(folder.name, "Test Folder");

        let updated_collection = service.get_collection(collection.id).await.unwrap().unwrap();
        assert_eq!(updated_collection.folders.len(), 1);
        assert_eq!(updated_collection.folders[0].name, "Test Folder");
    }

    #[tokio::test]
    async fn test_search_collections() {
        let service = setup_test_service().await;

        service.create_collection(
            "API Tests".to_string(),
            Some("Collection for API testing".to_string()),
        ).await.unwrap();

        service.create_collection(
            "UI Tests".to_string(),
            None,
        ).await.unwrap();

        let results = service.search_collections("API").await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "API Tests");
    }
}
