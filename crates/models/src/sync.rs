//! Synchronization models for cloud support
//!
//! This module defines types for cloud synchronization.
//! The offline-first design allows seamless integration with cloud sync later.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{Id, Timestamp, new_id, now};

/// Synchronization mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncMode {
    /// Fully offline - no cloud sync
    Offline,

    /// Auto sync when online
    OnlineAuto,

    /// Manual sync only
    OnlineManual,

    /// Hybrid - local first, periodic sync
    Hybrid,
}

impl Default for SyncMode {
    fn default() -> Self {
        SyncMode::Offline
    }
}

/// Synchronization status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Idle - no sync activity
    Idle,

    /// Sync in progress
    Syncing,

    /// Last successful sync
    Success { timestamp: Timestamp },

    /// Sync error
    Error { message: String },

    /// Conflicts detected and waiting for resolution
    Conflict { conflicts: Vec<ConflictInfo> },
}

/// Sync configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConfig {
    /// Current sync mode
    pub mode: SyncMode,

    /// Server URL (for online modes)
    pub server_url: Option<String>,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Device ID (unique per installation)
    pub device_id: Id,

    /// Last successful sync timestamp
    pub last_sync: Option<Timestamp>,

    /// Auto-sync interval in seconds (0 = disabled)
    pub auto_sync_interval: u64,

    /// Conflict resolution strategy
    pub conflict_strategy: ConflictStrategy,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            mode: SyncMode::Offline,
            server_url: None,
            api_key: None,
            device_id: new_id(),
            last_sync: None,
            auto_sync_interval: 0,
            conflict_strategy: ConflictStrategy::LastWriteWins,
        }
    }
}

impl SyncConfig {
    /// Create a new offline config
    pub fn offline() -> Self {
        Self::default()
    }

    /// Create an online config with server URL
    pub fn online(server_url: String, api_key: String) -> Self {
        Self {
            mode: SyncMode::OnlineAuto,
            server_url: Some(server_url),
            api_key: Some(api_key),
            device_id: new_id(),
            last_sync: None,
            auto_sync_interval: 300, // 5 minutes
            conflict_strategy: ConflictStrategy::LastWriteWins,
        }
    }

    /// Check if currently in online mode
    pub fn is_online(&self) -> bool {
        matches!(
            self.mode,
            SyncMode::OnlineAuto | SyncMode::OnlineManual | SyncMode::Hybrid
        )
    }

    /// Check if auto-sync is enabled
    pub fn auto_sync_enabled(&self) -> bool {
        self.is_online() && self.auto_sync_interval > 0
    }

    /// Update last sync timestamp
    pub fn mark_synced(&mut self) {
        self.last_sync = Some(now());
    }

    /// Clear server credentials (switch to offline)
    pub fn go_offline(&mut self) {
        self.mode = SyncMode::Offline;
        self.server_url = None;
        self.api_key = None;
    }
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// Local version wins (overwrites remote)
    LocalWins,

    /// Remote version wins (overwrites local)
    RemoteWins,

    /// Last write wins based on timestamp
    LastWriteWins,

    /// Manual resolution required
    Manual,
}

/// Information about a sync conflict
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictInfo {
    pub conflict_id: Id,
    pub item_type: SyncItemType,
    pub item_id: Id,
    pub item_name: String,
    pub local_version: i64,
    pub remote_version: i64,
    pub local_value: serde_json::Value,
    pub remote_value: serde_json::Value,
    pub created_at: Timestamp,
}

/// Types of items that can be synced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncItemType {
    Collection,
    Folder,
    Request,
    Environment,
}

impl SyncItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncItemType::Collection => "collection",
            SyncItemType::Folder => "folder",
            SyncItemType::Request => "request",
            SyncItemType::Environment => "environment",
        }
    }
}

/// A sync change operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncChange {
    pub change_id: Id,
    pub item_type: SyncItemType,
    pub item_id: Id,
    pub operation: SyncOperation,
    pub version: i64,
    pub data: serde_json::Value,
    pub timestamp: Timestamp,
    pub synced: bool,
}

impl SyncChange {
    pub fn create(item_type: SyncItemType, item_id: Id, data: serde_json::Value) -> Self {
        Self {
            change_id: new_id(),
            item_type,
            item_id,
            operation: SyncOperation::Create,
            version: 1,
            data,
            timestamp: now(),
            synced: false,
        }
    }

    pub fn update(item_type: SyncItemType, item_id: Id, version: i64, data: serde_json::Value) -> Self {
        Self {
            change_id: new_id(),
            item_type,
            item_id,
            operation: SyncOperation::Update,
            version,
            data,
            timestamp: now(),
            synced: false,
        }
    }

    pub fn delete(item_type: SyncItemType, item_id: Id, version: i64) -> Self {
        Self {
            change_id: new_id(),
            item_type,
            item_id,
            operation: SyncOperation::Delete,
            version,
            data: serde_json::Value::Null,
            timestamp: now(),
            synced: false,
        }
    }

    pub fn mark_synced(&mut self) {
        self.synced = true;
    }
}

/// Sync operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncOperation {
    Create,
    Update,
    Delete,
}

impl SyncOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyncOperation::Create => "create",
            SyncOperation::Update => "update",
            SyncOperation::Delete => "delete",
        }
    }
}

/// Result of a sync operation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncResult {
    /// Offline mode - nothing to sync
    Offline,

    /// Successful sync
    Success {
        timestamp: Timestamp,
        changes_pushed: usize,
        changes_pulled: usize,
    },

    /// Conflicts detected
    Conflict {
        conflicts: Vec<ConflictInfo>,
    },
}

/// Device information for sync
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_id: Id,
    pub name: String,
    pub device_type: DeviceType,
    pub os_info: Option<String>,
    pub last_seen: Timestamp,
    pub is_online: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Web,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "desktop",
            DeviceType::Mobile => "mobile",
            DeviceType::Web => "web",
        }
    }
}

/// Sync session state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSession {
    pub session_id: Id,
    pub started_at: Timestamp,
    pub changes_pushed: Vec<SyncChange>,
    pub changes_pulled: Vec<SyncChange>,
    pub conflicts: Vec<ConflictInfo>,
    pub completed_at: Option<Timestamp>,
}

impl SyncSession {
    pub fn new() -> Self {
        Self {
            session_id: new_id(),
            started_at: now(),
            changes_pushed: Vec::new(),
            changes_pulled: Vec::new(),
            conflicts: Vec::new(),
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed_at = Some(now());
    }

    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }

    pub fn duration(&self) -> Option<i64> {
        self.completed_at.map(|end| end - self.started_at)
    }
}

/// Pending changes queue for offline-first sync
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PendingChanges {
    pub changes: Vec<SyncChange>,
    pub max_size: usize,
}

impl PendingChanges {
    pub fn new(max_size: usize) -> Self {
        Self {
            changes: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, change: SyncChange) -> Result<(), SyncError> {
        // Remove existing change for the same item
        self.changes.retain(|c| {
            !(c.item_id == change.item_id && c.item_type == change.item_type)
        });

        // Check capacity
        if self.changes.len() >= self.max_size {
            return Err(SyncError::QueueFull);
        }

        self.changes.push(change);
        Ok(())
    }

    pub fn drain(&mut self) -> Vec<SyncChange> {
        std::mem::take(&mut self.changes)
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// Get changes for a specific item type
    pub fn for_item_type(&self, item_type: SyncItemType) -> Vec<&SyncChange> {
        self.changes
            .iter()
            .filter(|c| c.item_type == item_type)
            .collect()
    }

    /// Remove synced changes
    pub fn remove_synced(&mut self) {
        self.changes.retain(|c| !c.synced);
    }
}

/// Sync-related errors
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SyncError {
    #[error("Sync not configured")]
    NotConfigured,

    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed")]
    AuthenticationFailed,

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Conflict detected for item {item_type}:{item_id}")]
    Conflict { item_type: String, item_id: String },

    #[error("Queue is full")]
    QueueFull,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Cloud sync provider (placeholder for future implementation)
pub trait SyncProvider: Send + Sync {
    /// Authenticate with the sync server
    fn authenticate(&self, api_key: &str) -> Result<bool, SyncError>;

    /// Push local changes to server
    fn push_changes(&self, changes: Vec<SyncChange>) -> Result<SyncResult, SyncError>;

    /// Pull remote changes from server
    fn pull_changes(&self, since: Option<Timestamp>) -> Result<Vec<SyncChange>, SyncError>;

    /// Resolve conflicts on server
    fn resolve_conflicts(&self, resolutions: Vec<ConflictResolution>) -> Result<(), SyncError>;
}

/// Conflict resolution choice
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub conflict_id: Id,
    pub resolution: ConflictChoice,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictChoice {
    Local,
    Remote,
    Merged { value: serde_json::Value },
}

/// Local sync provider for offline mode
pub struct LocalSyncProvider;

impl SyncProvider for LocalSyncProvider {
    fn authenticate(&self, _api_key: &str) -> Result<bool, SyncError> {
        // Local mode - always succeeds
        Ok(true)
    }

    fn push_changes(&self, _changes: Vec<SyncChange>) -> Result<SyncResult, SyncError> {
        // Local mode - nothing to push
        Ok(SyncResult::Offline)
    }

    fn pull_changes(&self, _since: Option<Timestamp>) -> Result<Vec<SyncChange>, SyncError> {
        // Local mode - nothing to pull
        Ok(Vec::new())
    }

    fn resolve_conflicts(&self, _resolutions: Vec<ConflictResolution>) -> Result<(), SyncError> {
        // Local mode - nothing to resolve
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_config_offline() {
        let config = SyncConfig::offline();
        assert!(!config.is_online());
        assert!(!config.auto_sync_enabled());
        assert!(config.server_url.is_none());
    }

    #[test]
    fn test_sync_config_online() {
        let config = SyncConfig::online(
            "https://api.postboy.app".to_string(),
            "test-key".to_string(),
        );
        assert!(config.is_online());
        assert!(config.auto_sync_enabled());
        assert_eq!(config.server_url, Some("https://api.postboy.app".to_string()));
    }

    #[test]
    fn test_pending_changes() {
        let mut pending = PendingChanges::new(10);

        let change = SyncChange::create(
            SyncItemType::Request,
            new_id(),
            serde_json::json!({"name": "Test"}),
        );

        assert!(pending.push(change.clone()).is_ok());
        assert_eq!(pending.len(), 1);
        assert!(!pending.is_empty());

        let drained = pending.drain();
        assert_eq!(drained.len(), 1);
        assert!(pending.is_empty());
    }

    #[test]
    fn test_pending_changes_dedup() {
        let mut pending = PendingChanges::new(10);

        let id = new_id();
        let change1 = SyncChange::create(
            SyncItemType::Request,
            id,
            serde_json::json!({"v": 1}),
        );
        let change2 = SyncChange::update(
            SyncItemType::Request,
            id,
            2,
            serde_json::json!({"v": 2}),
        );

        pending.push(change1).unwrap();
        pending.push(change2).unwrap();

        // Should only have the latest change
        assert_eq!(pending.len(), 1);
        assert_eq!(pending.changes[0].version, 2);
    }

    #[test]
    fn test_sync_session() {
        let mut session = SyncSession::new();
        assert!(!session.is_complete());
        assert!(session.duration().is_none());

        session.complete();
        assert!(session.is_complete());
        assert!(session.duration().is_some());
    }

    #[test]
    fn test_sync_item_type() {
        assert_eq!(SyncItemType::Collection.as_str(), "collection");
        assert_eq!(SyncItemType::Request.as_str(), "request");
        assert_eq!(SyncItemType::Environment.as_str(), "environment");
    }

    #[test]
    fn test_device_type() {
        assert_eq!(DeviceType::Desktop.as_str(), "desktop");
        assert_eq!(DeviceType::Mobile.as_str(), "mobile");
        assert_eq!(DeviceType::Web.as_str(), "web");
    }
}
