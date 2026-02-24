//! Environment service implementation
//!
//! Provides business logic for managing environments and variables,
//! including CRUD operations and active environment management.

use anyhow::Result;
use postboy_models::{Environment, Id, now};
use postboy_models::environment::{Variable, VariableType};
use postboy_store::Database;
use tracing::{info, debug, instrument};

/// Environment service for managing environments
#[derive(Clone)]
pub struct EnvironmentService {
    db: Database,
}

impl EnvironmentService {
    /// Create a new environment service
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// Create a new environment
    #[instrument(skip(self))]
    pub async fn create_environment(
        &self,
        name: String,
        description: Option<String>,
    ) -> Result<Environment> {
        let environment = Environment::new(name);

        // Save to database
        let variables_json = serde_json::to_string(&environment.values)?;

        sqlx::query(
            r#"
            INSERT INTO environments (id, name, variables, is_active, sync_state, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(environment.id.to_string())
        .bind(&environment.name)
        .bind(&variables_json)
        .bind(environment.is_active as i64)
        .bind("{}")
        .bind(environment.created_at as i64)
        .bind(environment.updated_at as i64)
        .execute(self.db.pool())
        .await?;

        info!("Created environment: {} ({})", environment.name, environment.id);
        Ok(environment)
    }

    /// Get an environment by ID
    #[instrument(skip(self))]
    pub async fn get_environment(&self, id: Id) -> Result<Option<Environment>> {
        let result = sqlx::query_as::<_, (String, String, String, bool, i64, i64)>(
            "SELECT id, name, variables, is_active, created_at, updated_at FROM environments WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(self.db.pool())
        .await?;

        if let Some((id_str, name, variables, is_active, created_at, updated_at)) = result {
            let values: Vec<Variable> = serde_json::from_str(&variables)?;
            Ok(Some(Environment {
                id: id_str.parse()?,
                name,
                values,
                is_active,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// List all environments
    #[instrument(skip(self))]
    pub async fn list_environments(&self) -> Result<Vec<Environment>> {
        let rows = sqlx::query_as::<_, (String, String, String, bool, i64, i64)>(
            "SELECT id, name, variables, is_active, created_at, updated_at FROM environments ORDER BY name ASC"
        )
        .fetch_all(self.db.pool())
        .await?;

        let mut environments = Vec::new();
        for (id_str, name, variables, is_active, created_at, updated_at) in rows {
            let values: Vec<Variable> = serde_json::from_str(&variables)?;
            environments.push(Environment {
                id: id_str.parse()?,
                name,
                values,
                is_active,
                created_at,
                updated_at,
            });
        }

        debug!("Listed {} environments", environments.len());
        Ok(environments)
    }

    /// Update an environment
    #[instrument(skip(self))]
    pub async fn update_environment(
        &self,
        id: Id,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Option<Environment>> {
        // First, get the existing environment
        let existing = match self.get_environment(id).await? {
            Some(e) => e,
            None => return Ok(None),
        };

        // Update fields
        let mut updated = existing.clone();
        if let Some(name) = name {
            updated.name = name;
        }
        // Note: description is not stored in the environment model, so we ignore it
        updated.updated_at = now();

        // Save to database
        let variables_json = serde_json::to_string(&updated.values)?;

        sqlx::query(
            r#"
            UPDATE environments
            SET name = ?, variables = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&updated.name)
        .bind(&variables_json)
        .bind(updated.updated_at as i64)
        .bind(id.to_string())
        .execute(self.db.pool())
        .await?;

        info!("Updated environment: {} ({})", updated.name, id);
        Ok(Some(updated))
    }

    /// Delete an environment
    #[instrument(skip(self))]
    pub async fn delete_environment(&self, id: Id) -> Result<bool> {
        let result = sqlx::query("DELETE FROM environments WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;

        let deleted = result.rows_affected() > 0;

        if deleted {
            info!("Deleted environment: {}", id);
        }

        Ok(deleted)
    }

    /// Set the active environment
    #[instrument(skip(self))]
    pub async fn set_active_environment(&self, id: Id) -> Result<()> {
        // First verify the environment exists
        let env = self.get_environment(id).await?
            .ok_or_else(|| anyhow::anyhow!("Environment not found: {}", id))?;

        // Set all environments to inactive, then set the target to active
        sqlx::query("UPDATE environments SET is_active = 0")
            .execute(self.db.pool())
            .await?;

        sqlx::query("UPDATE environments SET is_active = 1 WHERE id = ?")
            .bind(id.to_string())
            .execute(self.db.pool())
            .await?;

        info!("Set active environment: {} ({})", env.name, id);
        Ok(())
    }

    /// Get the currently active environment
    #[instrument(skip(self))]
    pub async fn get_active_environment(&self) -> Result<Option<Environment>> {
        let result = sqlx::query_as::<_, (String, String, String, bool, i64, i64)>(
            "SELECT id, name, variables, is_active, created_at, updated_at FROM environments WHERE is_active = 1"
        )
        .fetch_optional(self.db.pool())
        .await?;

        if let Some((id_str, name, variables, is_active, created_at, updated_at)) = result {
            let values: Vec<Variable> = serde_json::from_str(&variables)?;
            Ok(Some(Environment {
                id: id_str.parse()?,
                name,
                values,
                is_active,
                created_at,
                updated_at,
            }))
        } else {
            Ok(None)
        }
    }

    /// Add a variable to an environment
    #[instrument(skip(self))]
    pub async fn add_variable(
        &self,
        environment_id: Id,
        key: String,
        value: String,
        variable_type: VariableType,
    ) -> Result<Variable> {
        let mut environment = self.get_environment(environment_id).await?
            .ok_or_else(|| anyhow::anyhow!("Environment not found: {}", environment_id))?;

        let variable = Variable::new(key.clone(), value.clone())
            .with_type(variable_type);

        environment.values.push(variable.clone());
        environment.updated_at = now();

        // Update environment
        let variables_json = serde_json::to_string(&environment.values)?;

        sqlx::query(
            "UPDATE environments SET variables = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&variables_json)
        .bind(environment.updated_at as i64)
        .bind(environment_id.to_string())
        .execute(self.db.pool())
        .await?;

        info!("Added variable '{}' to environment {}", key, environment_id);
        Ok(variable)
    }

    /// Update a variable in an environment
    #[instrument(skip(self))]
    pub async fn update_variable(
        &self,
        environment_id: Id,
        variable_id: Id,
        key: String,
        value: String,
    ) -> Result<Option<Variable>> {
        let mut environment = self.get_environment(environment_id).await?
            .ok_or_else(|| anyhow::anyhow!("Environment not found: {}", environment_id))?;

        // Find and update the variable by ID (not index, as we need to track by ID)
        let variable = environment.values.iter_mut()
            .find(|v| v.key == key); // Variables don't have IDs in the model, so we use key

        if let Some(variable) = variable {
            let key_clone = key.clone();
            variable.value = value;
            variable.key = key;
            environment.updated_at = now();

            // Clone the variable before we need to serialize the environment
            let updated_variable = variable.clone();

            // Save to database
            let variables_json = serde_json::to_string(&environment.values)?;

            sqlx::query(
                "UPDATE environments SET variables = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&variables_json)
            .bind(environment.updated_at as i64)
            .bind(environment_id.to_string())
            .execute(self.db.pool())
            .await?;

            info!("Updated variable '{}' in environment {}", key_clone, environment_id);
            Ok(Some(updated_variable))
        } else {
            Ok(None)
        }
    }

    /// Delete a variable from an environment
    #[instrument(skip(self))]
    pub async fn delete_variable(
        &self,
        environment_id: Id,
        variable_id: Id,
    ) -> Result<bool> {
        let mut environment = self.get_environment(environment_id).await?
            .ok_or_else(|| anyhow::anyhow!("Environment not found: {}", environment_id))?;

        // Variables don't have IDs, so we'll treat variable_id as a string for the key
        let key = variable_id.to_string();
        let original_len = environment.values.len();
        environment.values.retain(|v| v.key != key);

        let deleted = environment.values.len() < original_len;

        if deleted {
            environment.updated_at = now();

            // Save to database
            let variables_json = serde_json::to_string(&environment.values)?;

            sqlx::query(
                "UPDATE environments SET variables = ?, updated_at = ? WHERE id = ?"
            )
            .bind(&variables_json)
            .bind(environment.updated_at as i64)
            .bind(environment_id.to_string())
            .execute(self.db.pool())
            .await?;

            info!("Deleted variable '{}' from environment {}", key, environment_id);
        }

        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postboy_store::StoreConfig;

    async fn setup_test_service() -> EnvironmentService {
        let db = postboy_store::open_store(StoreConfig {
            db_path: ":memory:".to_string(),
            ..Default::default()
        }).await.unwrap();

        EnvironmentService::new(db)
    }

    #[tokio::test]
    async fn test_create_and_get_environment() {
        let service = setup_test_service().await;

        let environment = service.create_environment(
            "Production".to_string(),
            None,
        ).await.unwrap();

        assert_eq!(environment.name, "Production");
        assert!(!environment.is_active);

        let retrieved = service.get_environment(environment.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Production");
    }

    #[tokio::test]
    async fn test_list_environments() {
        let service = setup_test_service().await;

        service.create_environment("Development".to_string(), None).await.unwrap();
        service.create_environment("Staging".to_string(), None).await.unwrap();

        let environments = service.list_environments().await.unwrap();
        assert_eq!(environments.len(), 2);
    }

    #[tokio::test]
    async fn test_update_environment() {
        let service = setup_test_service().await;

        let environment = service.create_environment(
            "Original Name".to_string(),
            None,
        ).await.unwrap();

        let updated = service.update_environment(
            environment.id,
            Some("Updated Name".to_string()),
            None,
        ).await.unwrap();

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Updated Name");
    }

    #[tokio::test]
    async fn test_delete_environment() {
        let service = setup_test_service().await;

        let environment = service.create_environment(
            "To Delete".to_string(),
            None,
        ).await.unwrap();

        let deleted = service.delete_environment(environment.id).await.unwrap();
        assert!(deleted);

        let retrieved = service.get_environment(environment.id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_active_environment() {
        let service = setup_test_service().await;

        let env1 = service.create_environment("Development".to_string(), None).await.unwrap();
        let env2 = service.create_environment("Production".to_string(), None).await.unwrap();

        // No active environment initially
        let active = service.get_active_environment().await.unwrap();
        assert!(active.is_none());

        // Set env1 as active
        service.set_active_environment(env1.id).await.unwrap();
        let active = service.get_active_environment().await.unwrap();
        assert!(active.is_some());
        assert_eq!(active.unwrap().id, env1.id);

        // Switch to env2
        service.set_active_environment(env2.id).await.unwrap();
        let active = service.get_active_environment().await.unwrap();
        assert!(active.is_some());
        assert_eq!(active.unwrap().id, env2.id);
    }

    #[tokio::test]
    async fn test_add_variable() {
        let service = setup_test_service().await;

        let environment = service.create_environment("Test".to_string(), None).await.unwrap();

        let variable = service.add_variable(
            environment.id,
            "api_key".to_string(),
            "secret123".to_string(),
            VariableType::Secret,
        ).await.unwrap();

        assert_eq!(variable.key, "api_key");
        assert_eq!(variable.value, "secret123");

        let updated_env = service.get_environment(environment.id).await.unwrap().unwrap();
        assert_eq!(updated_env.values.len(), 1);
        assert_eq!(updated_env.values[0].key, "api_key");
    }

    #[tokio::test]
    async fn test_update_variable() {
        let service = setup_test_service().await;

        let environment = service.create_environment("Test".to_string(), None).await.unwrap();

        service.add_variable(
            environment.id,
            "base_url".to_string(),
            "https://api.dev.com".to_string(),
            VariableType::Normal,
        ).await.unwrap();

        let updated = service.update_variable(
            environment.id,
            "base_url".to_string().parse().unwrap(), // Using key as ID
            "base_url".to_string(),
            "https://api.prod.com".to_string(),
        ).await.unwrap();

        assert!(updated.is_some());
        assert_eq!(updated.unwrap().value, "https://api.prod.com");

        let env = service.get_environment(environment.id).await.unwrap().unwrap();
        assert_eq!(env.values[0].value, "https://api.prod.com");
    }

    #[tokio::test]
    async fn test_delete_variable() {
        let service = setup_test_service().await;

        let environment = service.create_environment("Test".to_string(), None).await.unwrap();

        service.add_variable(
            environment.id,
            "temp_key".to_string(),
            "temp_value".to_string(),
            VariableType::Normal,
        ).await.unwrap();

        let deleted = service.delete_variable(
            environment.id,
            "temp_key".to_string().parse().unwrap(), // Using key as ID
        ).await.unwrap();

        assert!(deleted);

        let env = service.get_environment(environment.id).await.unwrap().unwrap();
        assert_eq!(env.values.len(), 0);
    }
}
