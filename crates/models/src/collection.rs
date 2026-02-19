//! Collection and folder models for organizing API requests

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{Id, Timestamp, new_id, now, Temporal, Identifiable};

/// Collection - a container for organizing API requests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,

    /// Collection metadata
    #[serde(default)]
    pub info: CollectionInfo,

    /// Folders in this collection
    #[serde(default)]
    pub folders: Vec<Folder>,

    /// Requests at the root level (not in a folder)
    #[serde(default)]
    pub requests: Vec<Id>,

    /// Variables available in this collection
    #[serde(default)]
    pub variables: Vec<Variable>,

    /// Authentication configuration for the collection
    pub auth: Option<crate::request::AuthConfig>,

    /// Sync state for cloud support
    #[serde(default)]
    pub sync_state: SyncState,

    /// UI-specific state
    #[serde(default)]
    pub ui_state: CollectionUiState,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Collection metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionInfo {
    /// Schema version for compatibility
    pub schema: String,

    /// Postboy collection identifier
    #[serde(default = "default_collection_id")]
    pub postboy_id: String,

    /// Optional custom icon
    pub icon: Option<String>,

    /// Optional color theme
    pub color: Option<String>,
}

fn default_collection_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

impl Default for CollectionInfo {
    fn default() -> Self {
        Self {
            schema: "https://schema.getpostboy.com/json/collection/v2.1.0/collection.json".to_string(),
            postboy_id: default_collection_id(),
            icon: None,
            color: None,
        }
    }
}

/// Folder - a hierarchical container within a collection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Folder {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,

    /// Parent folder ID (for nested folders)
    pub parent_id: Option<Id>,

    /// Child folders
    #[serde(default)]
    pub children: Vec<Folder>,

    /// Request IDs in this folder
    #[serde(default)]
    pub requests: Vec<Id>,

    /// UI-specific state
    #[serde(default)]
    pub ui_state: FolderUiState,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Collection variable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub value: String,

    /// Variable type for display/hinting
    #[serde(rename = "type")]
    pub variable_type: VariableType,

    /// Whether this variable is currently enabled
    pub enabled: bool,

    /// Optional hint for the value
    pub hint: Option<String>,

    /// Initial value (for secret variables)
    pub initial_value: Option<String>,
}

/// Variable type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    /// Default string variable
    String,
    /// Boolean variable
    Boolean,
    /// Secret/sensitive variable (passwords, keys)
    Secret,
    /// JSON variable
    Json,
    /// Number variable
    Number,
}

impl Default for VariableType {
    fn default() -> Self {
        VariableType::String
    }
}

/// Sync state for cloud synchronization
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SyncState {
    /// Last sync timestamp
    pub last_synced_at: Option<Timestamp>,

    /// Sync status
    pub status: SyncStatus,

    /// Remote collection ID (if synced)
    pub remote_id: Option<Id>,

    /// Version for conflict resolution
    pub version: Option<i64>,

    /// Pending changes count
    pub pending_changes: usize,
}

/// Current sync status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Not synced, local only
    NotSynced,
    /// Sync in progress
    Syncing,
    /// Successfully synced
    Synced,
    /// Sync failed
    Failed,
    /// Conflict detected
    Conflict,
    /// Pending changes
    Pending,
}

impl Default for SyncStatus {
    fn default() -> Self {
        SyncStatus::NotSynced
    }
}

/// UI-specific state for collections
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CollectionUiState {
    /// Whether the collection is expanded in the sidebar
    pub is_expanded: bool,

    /// Currently selected item ID
    pub selected_item: Option<Id>,

    /// Scroll position
    pub scroll_position: Option<f32>,

    /// View mode (list, grid, etc.)
    pub view_mode: CollectionViewMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectionViewMode {
    List,
    Grid,
    Tree,
}

impl Default for CollectionViewMode {
    fn default() -> Self {
        CollectionViewMode::Tree
    }
}

/// UI-specific state for folders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FolderUiState {
    /// Whether the folder is expanded
    pub is_expanded: bool,

    /// Whether the folder is selected
    pub is_selected: bool,

    /// Depth level for display
    pub depth: usize,
}

impl Collection {
    /// Create a new collection
    pub fn new(name: String) -> Self {
        let now = now();
        Self {
            id: new_id(),
            name,
            description: None,
            info: CollectionInfo::default(),
            folders: Vec::new(),
            requests: Vec::new(),
            variables: Vec::new(),
            auth: None,
            sync_state: SyncState::default(),
            ui_state: CollectionUiState::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new collection with a description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a variable to the collection
    pub fn with_variable(mut self, key: String, value: String) -> Self {
        self.variables.push(Variable {
            key,
            value,
            variable_type: VariableType::String,
            enabled: true,
            hint: None,
            initial_value: None,
        });
        self
    }

    /// Add authentication to the collection
    pub fn with_auth(mut self, auth: crate::request::AuthConfig) -> Self {
        self.auth = Some(auth);
        self
    }

    /// Add a folder to the collection
    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
        self.updated_at = now();
    }

    /// Add a request ID to the collection
    pub fn add_request(&mut self, request_id: Id) {
        self.requests.push(request_id);
        self.updated_at = now();
    }

    /// Remove a folder by ID
    pub fn remove_folder(&mut self, folder_id: Id) -> bool {
        let original_len = self.folders.len();
        self.folders.retain(|f| f.id != folder_id);
        let removed = self.folders.len() < original_len;
        if removed {
            self.updated_at = now();
        }
        removed
    }

    /// Remove a request by ID
    pub fn remove_request(&mut self, request_id: Id) -> bool {
        let original_len = self.requests.len();
        self.requests.retain(|id| id != &request_id);
        let removed = self.requests.len() < original_len;
        if removed {
            self.updated_at = now();
        }
        removed
    }

    /// Get all request IDs (including those in folders)
    pub fn all_request_ids(&self) -> Vec<Id> {
        let mut ids = self.requests.clone();
        for folder in &self.folders {
            ids.extend(folder.all_request_ids());
        }
        ids
    }

    /// Find a folder by ID (recursive)
    pub fn find_folder(&self, folder_id: Id) -> Option<&Folder> {
        for folder in &self.folders {
            if folder.id == folder_id {
                return Some(folder);
            }
            if let Some(found) = folder.find_folder(folder_id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a mutable folder by ID (recursive)
    pub fn find_folder_mut(&mut self, folder_id: Id) -> Option<&mut Folder> {
        for folder in &mut self.folders {
            if folder.id == folder_id {
                return Some(folder);
            }
            if let Some(found) = folder.find_folder_mut(folder_id) {
                return Some(found);
            }
        }
        None
    }

    /// Check if collection is synced
    pub fn is_synced(&self) -> bool {
        matches!(self.sync_state.status, SyncStatus::Synced)
    }

    /// Check if collection has pending changes
    pub fn has_pending_changes(&self) -> bool {
        self.sync_state.pending_changes > 0
            || matches!(self.sync_state.status, SyncStatus::Pending)
    }

    /// Get a variable value by key
    pub fn get_variable(&self, key: &str) -> Option<&Variable> {
        self.variables.iter()
            .find(|v| v.enabled && v.key == key)
    }

    /// Get all enabled variables as a map
    pub fn enabled_variables_map(&self) -> HashMap<String, String> {
        self.variables.iter()
            .filter(|v| v.enabled)
            .map(|v| (v.key.clone(), v.value.clone()))
            .collect()
    }

    /// Mark collection as syncing
    pub fn mark_syncing(&mut self) {
        self.sync_state.status = SyncStatus::Syncing;
    }

    /// Mark collection as synced
    pub fn mark_synced(&mut self, remote_id: Id, version: i64) {
        self.sync_state.status = SyncStatus::Synced;
        self.sync_state.last_synced_at = Some(now());
        self.sync_state.remote_id = Some(remote_id);
        self.sync_state.version = Some(version);
        self.sync_state.pending_changes = 0;
    }

    /// Mark collection as failed to sync
    pub fn mark_sync_failed(&mut self) {
        self.sync_state.status = SyncStatus::Failed;
    }

    /// Increment pending changes counter
    pub fn increment_pending_changes(&mut self) {
        self.sync_state.pending_changes += 1;
        if !matches!(self.sync_state.status, SyncStatus::Syncing) {
            self.sync_state.status = SyncStatus::Pending;
        }
    }

    /// Export to Postman collection format (v2.1)
    pub fn to_postman(&self) -> serde_json::Value {
        serde_json::json!({
            "info": {
                "name": self.name,
                "description": self.description,
                "schema": self.info.schema,
                "_postman_id": self.info.postboy_id,
            },
            "item": self.to_postman_items(),
            "variable": self.variables.iter()
                .filter(|v| v.enabled)
                .map(|v| serde_json::json!({
                    "key": v.key,
                    "value": v.value,
                    "type": variable_type_to_postman(v.variable_type),
                }))
                .collect::<Vec<_>>(),
        })
    }

    fn to_postman_items(&self) -> Vec<serde_json::Value> {
        let mut items = Vec::new();

        // Add root-level folders
        for folder in &self.folders {
            items.push(folder.to_postman());
        }

        items
    }

    /// Import from Postman collection format (v2.1)
    pub fn from_postman(value: serde_json::Value) -> Result<Self, String> {
        let info = value.get("info")
            .and_then(|v| v.as_object())
            .ok_or("Missing info object")?;

        let name = info.get("name")
            .and_then(|v| v.as_str())
            .ok_or("Missing collection name")?
            .to_string();

        let mut collection = Self::new(name);

        if let Some(description) = info.get("description").and_then(|v| v.as_str()) {
            collection.description = Some(description.to_string());
        }

        if let Some(postman_id) = info.get("_postman_id").and_then(|v| v.as_str()) {
            collection.info.postboy_id = postman_id.to_string();
        }

        // Parse items
        if let Some(items) = value.get("item").and_then(|v| v.as_array()) {
            for item in items {
                if let Some(folder) = Folder::from_postman_item(item) {
                    collection.add_folder(folder);
                }
            }
        }

        // Parse variables
        if let Some(variables) = value.get("variable").and_then(|v| v.as_array()) {
            for var in variables {
                if let Ok(variable) = Variable::from_postman(var) {
                    collection.variables.push(variable);
                }
            }
        }

        Ok(collection)
    }

    /// Duplicate the collection
    pub fn duplicate(&self) -> Self {
        let mut dup = self.clone();
        dup.id = new_id();
        dup.name = format!("{} (Copy)", dup.name);
        dup.sync_state = SyncState::default();
        dup.created_at = now();
        dup.updated_at = now();
        dup
    }
}

impl Temporal for Collection {
    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

impl Identifiable for Collection {
    fn id(&self) -> Id {
        self.id
    }
}

impl Folder {
    /// Create a new folder
    pub fn new(name: String) -> Self {
        let now = now();
        Self {
            id: new_id(),
            name,
            description: None,
            parent_id: None,
            children: Vec::new(),
            requests: Vec::new(),
            ui_state: FolderUiState::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a new folder with a parent
    pub fn with_parent(mut self, parent_id: Id) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Create a new folder with a description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a child folder
    pub fn add_child(&mut self, folder: Folder) {
        self.children.push(folder);
        self.updated_at = now();
    }

    /// Add a request ID
    pub fn add_request(&mut self, request_id: Id) {
        self.requests.push(request_id);
        self.updated_at = now();
    }

    /// Get all request IDs (recursive)
    pub fn all_request_ids(&self) -> Vec<Id> {
        let mut ids = self.requests.clone();
        for child in &self.children {
            ids.extend(child.all_request_ids());
        }
        ids
    }

    /// Find a folder by ID (recursive)
    pub fn find_folder(&self, folder_id: Id) -> Option<&Folder> {
        if self.id == folder_id {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_folder(folder_id) {
                return Some(found);
            }
        }
        None
    }

    /// Find a mutable folder by ID (recursive)
    pub fn find_folder_mut(&mut self, folder_id: Id) -> Option<&mut Folder> {
        if self.id == folder_id {
            return Some(self);
        }
        for child in &mut self.children {
            if let Some(found) = child.find_folder_mut(folder_id) {
                return Some(found);
            }
        }
        None
    }

    /// Get the depth of this folder in the hierarchy
    pub fn depth(&self) -> usize {
        self.ui_state.depth
    }

    /// Set the depth for this folder and all children
    pub fn set_depth(&mut self, depth: usize) {
        self.ui_state.depth = depth;
        for child in &mut self.children {
            child.set_depth(depth + 1);
        }
    }

    /// Convert to Postman format
    pub fn to_postman(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "description": self.description,
            "item": self.children.iter()
                .map(|f| f.to_postman())
                .collect::<Vec<_>>(),
        })
    }

    /// Parse from Postman item
    pub fn from_postman_item(value: &serde_json::Value) -> Option<Self> {
        let name = value.get("name")?.as_str()?;
        let mut folder = Self::new(name.to_string());

        if let Some(description) = value.get("description").and_then(|v| v.as_str()) {
            folder.description = Some(description.to_string());
        }

        // Parse nested items
        if let Some(items) = value.get("item").and_then(|v| v.as_array()) {
            for item in items {
                // Check if this is a folder (has nested items) or a request
                if let Some(nested) = item.get("item").and_then(|v| v.as_array()) {
                    // This is a folder
                    if let Some(child_folder) = Folder::from_postman_item(item) {
                        folder.add_child(child_folder);
                    }
                }
                // Request handling would be done at the store level
            }
        }

        Some(folder)
    }

    /// Duplicate the folder
    pub fn duplicate(&self) -> Self {
        let mut dup = self.clone();
        dup.id = new_id();
        dup.name = format!("{} (Copy)", dup.name);
        dup.created_at = now();
        dup.updated_at = now();
        dup
    }
}

impl Temporal for Folder {
    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

impl Identifiable for Folder {
    fn id(&self) -> Id {
        self.id
    }
}

impl Variable {
    /// Create a new variable
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            variable_type: VariableType::String,
            enabled: true,
            hint: None,
            initial_value: None,
        }
    }

    /// Create a secret variable
    pub fn secret(key: String, value: String) -> Self {
        Self {
            key,
            value,
            variable_type: VariableType::Secret,
            enabled: true,
            hint: None,
            initial_value: None,
        }
    }

    /// Create a JSON variable
    pub fn json(key: String, value: String) -> Self {
        Self {
            key,
            value,
            variable_type: VariableType::Json,
            enabled: true,
            hint: None,
            initial_value: None,
        }
    }

    /// Set the hint
    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Disable the variable
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Parse from Postman variable format
    pub fn from_postman(value: &serde_json::Value) -> Result<Self, String> {
        let key = value.get("key")
            .and_then(|v| v.as_str())
            .ok_or("Missing variable key")?
            .to_string();

        let value_str = value.get("value")
            .and_then(|v| v.as_str())
            .ok_or("Missing variable value")?
            .to_string();

        let variable_type = value.get("type")
            .and_then(|v| v.as_str())
            .and_then(|t| postman_variable_type(t))
            .unwrap_or(VariableType::String);

        Ok(Self {
            key,
            value: value_str,
            variable_type,
            enabled: value.get("enabled").and_then(|v| v.as_bool()).unwrap_or(true),
            hint: value.get("hint").and_then(|v| v.as_str()).map(String::from),
            initial_value: None,
        })
    }
}

fn variable_type_to_postman(var_type: VariableType) -> &'static str {
    match var_type {
        VariableType::String => "string",
        VariableType::Boolean => "boolean",
        VariableType::Secret => "secret",
        VariableType::Json => "json",
        VariableType::Number => "number",
    }
}

fn postman_variable_type(s: &str) -> Option<VariableType> {
    match s {
        "string" => Some(VariableType::String),
        "boolean" => Some(VariableType::Boolean),
        "secret" => Some(VariableType::Secret),
        "json" => Some(VariableType::Json),
        "number" => Some(VariableType::Number),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new("My API".to_string());

        assert_eq!(collection.name, "My API");
        assert_eq!(collection.folders.len(), 0);
        assert_eq!(collection.requests.len(), 0);
        assert!(!collection.is_synced());
    }

    #[test]
    fn test_collection_with_variable() {
        let collection = Collection::new("My API".to_string())
            .with_variable("base_url".to_string(), "https://api.example.com".to_string());

        assert_eq!(collection.variables.len(), 1);
        assert_eq!(collection.variables[0].key, "base_url");

        let base_url = collection.get_variable("base_url");
        assert!(base_url.is_some());
        assert_eq!(base_url.unwrap().value, "https://api.example.com");
    }

    #[test]
    fn test_folder_hierarchy() {
        let mut collection = Collection::new("My API".to_string());

        let mut parent_folder = Folder::new("Parent".to_string());
        let child_folder = Folder::new("Child".to_string());

        parent_folder.add_child(child_folder);
        collection.add_folder(parent_folder);

        assert_eq!(collection.folders.len(), 1);
        assert_eq!(collection.folders[0].children.len(), 1);

        let all_ids = collection.all_request_ids();
        assert_eq!(all_ids.len(), 0); // No requests added
    }

    #[test]
    fn test_folder_find() {
        let mut collection = Collection::new("My API".to_string());

        let folder = Folder::new("Test Folder".to_string());
        let folder_id = folder.id;

        collection.add_folder(folder);

        let found = collection.find_folder(folder_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Folder");
    }

    #[test]
    fn test_variable_types() {
        let string_var = Variable::new("key".to_string(), "value".to_string());
        assert_eq!(string_var.variable_type, VariableType::String);

        let secret_var = Variable::secret("password".to_string(), "secret123".to_string());
        assert_eq!(secret_var.variable_type, VariableType::Secret);

        let json_var = Variable::json("config".to_string(), r#"{"key":"value"}"#.to_string());
        assert_eq!(json_var.variable_type, VariableType::Json);
    }

    #[test]
    fn test_sync_state() {
        let mut collection = Collection::new("My API".to_string());
        assert_eq!(collection.sync_state.status, SyncStatus::NotSynced);

        collection.mark_syncing();
        assert_eq!(collection.sync_state.status, SyncStatus::Syncing);

        collection.mark_synced(new_id(), 1);
        assert!(collection.is_synced());
    }

    #[test]
    fn test_pending_changes() {
        let mut collection = Collection::new("My API".to_string());

        collection.increment_pending_changes();
        assert!(collection.has_pending_changes());
        assert_eq!(collection.sync_state.pending_changes, 1);

        collection.increment_pending_changes();
        assert_eq!(collection.sync_state.pending_changes, 2);
    }

    #[test]
    fn test_enabled_variables_map() {
        let collection = Collection::new("My API".to_string())
            .with_variable("key1".to_string(), "value1".to_string())
            .with_variable("key2".to_string(), "value2".to_string());

        // Disable one variable
        collection.variables[1].enabled = false;

        let map = collection.enabled_variables_map();
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("key1"), Some(&"value1".to_string()));
        assert_eq!(map.get("key2"), None);
    }

    #[test]
    fn test_folder_depth() {
        let mut folder = Folder::new("Parent".to_string());
        let mut child = Folder::new("Child".to_string());
        let mut grandchild = Folder::new("Grandchild".to_string());

        folder.set_depth(0);
        assert_eq!(folder.depth(), 0);

        child.set_depth(1);
        grandchild.set_depth(2);

        child.add_child(grandchild);
        folder.add_child(child);

        assert_eq!(folder.depth(), 0);
        assert_eq!(folder.children[0].depth(), 1);
        assert_eq!(folder.children[0].children[0].depth(), 2);
    }
}
