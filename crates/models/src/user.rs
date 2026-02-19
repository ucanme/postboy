//! User and authentication models
//!
//! This module defines user-related data structures.
//! Designed to work with both local-only and cloud-synced scenarios.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::{Id, Timestamp, new_id, now, Temporal, Identifiable};

/// User account
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,

    /// Whether the email has been verified (for cloud sync)
    pub is_verified: bool,

    /// Whether the account is active
    pub is_active: bool,

    /// Account tier/plan (for future cloud features)
    pub plan: UserPlan,

    /// Quota limits
    pub quota: UserQuota,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub last_login_at: Option<Timestamp>,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        let now = now();
        Self {
            id: new_id(),
            email,
            name,
            avatar_url: None,
            bio: None,
            is_verified: false,
            is_active: true,
            plan: UserPlan::Free,
            quota: UserQuota::default(),
            created_at: now,
            updated_at: now,
            last_login_at: Some(now),
        }
    }

    pub fn with_avatar(mut self, avatar_url: String) -> Self {
        self.avatar_url = Some(avatar_url);
        self
    }

    pub fn with_bio(mut self, bio: String) -> Self {
        self.bio = Some(bio);
        self
    }

    /// Check if user can create more collections
    pub fn can_create_collection(&self, current_count: usize) -> bool {
        current_count < self.quota.max_collections as usize
    }

    /// Check if user can add more requests to a collection
    pub fn can_add_requests(&self, current_count: usize) -> bool {
        current_count < self.quota.max_requests_per_collection as usize
    }
}

impl Temporal for User {
    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

impl Identifiable for User {
    fn id(&self) -> Id {
        self.id
    }
}

/// User subscription plan
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserPlan {
    Free,
    Pro,
    Team,
    Enterprise,
}

impl UserPlan {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserPlan::Free => "free",
            UserPlan::Pro => "pro",
            UserPlan::Team => "team",
            UserPlan::Enterprise => "enterprise",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "free" => Some(UserPlan::Free),
            "pro" => Some(UserPlan::Pro),
            "team" => Some(UserPlan::Team),
            "enterprise" => Some(UserPlan::Enterprise),
            _ => None,
        }
    }
}

impl Default for UserPlan {
    fn default() -> Self {
        UserPlan::Free
    }
}

/// User quota limits
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserQuota {
    /// Maximum number of collections
    pub max_collections: u32,

    /// Maximum requests per collection
    pub max_requests_per_collection: u32,

    /// Maximum storage in MB
    pub max_storage_mb: u32,

    /// Maximum collaborators per shared collection
    pub max_collaborators: u32,

    /// Current usage counters
    pub collections_count: u32,
    pub storage_used_mb: u32,
}

impl Default for UserQuota {
    fn default() -> Self {
        Self::free()
    }
}

impl UserQuota {
    pub fn free() -> Self {
        Self {
            max_collections: 10,
            max_requests_per_collection: 100,
            max_storage_mb: 100,
            max_collaborators: 0, // No sharing in free tier
            collections_count: 0,
            storage_used_mb: 0,
        }
    }

    pub fn pro() -> Self {
        Self {
            max_collections: 100,
            max_requests_per_collection: 1000,
            max_storage_mb: 1000,
            max_collaborators: 5,
            collections_count: 0,
            storage_used_mb: 0,
        }
    }

    pub fn team() -> Self {
        Self {
            max_collections: 1000,
            max_requests_per_collection: 10000,
            max_storage_mb: 10000,
            max_collaborators: 50,
            collections_count: 0,
            storage_used_mb: 0,
        }
    }

    pub fn enterprise() -> Self {
        Self {
            max_collections: u32::MAX,
            max_requests_per_collection: u32::MAX,
            max_storage_mb: u32::MAX,
            max_collaborators: u32::MAX,
            collections_count: 0,
            storage_used_mb: 0,
        }
    }

    /// Get quota for a given plan
    pub fn for_plan(plan: UserPlan) -> Self {
        match plan {
            UserPlan::Free => Self::free(),
            UserPlan::Pro => Self::pro(),
            UserPlan::Team => Self::team(),
            UserPlan::Enterprise => Self::enterprise(),
        }
    }
}

/// Device information (for multi-device sync in the future)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Device {
    pub id: Id,
    pub user_id: Id,
    pub name: String,
    pub device_type: DeviceType,
    pub os_info: Option<String>,
    pub last_seen: Timestamp,
    pub is_online: bool,
    pub created_at: Timestamp,
}

impl Device {
    pub fn new(user_id: Id, name: String, device_type: DeviceType) -> Self {
        let now = now();
        Self {
            id: new_id(),
            user_id,
            name,
            device_type,
            os_info: None,
            last_seen: now,
            is_online: false,
            created_at: now,
        }
    }

    pub fn with_os_info(mut self, os_info: String) -> Self {
        self.os_info = Some(os_info);
        self
    }
}

impl Identifiable for Device {
    fn id(&self) -> Id {
        self.id
    }
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Web,
    Cli,
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "desktop",
            DeviceType::Mobile => "mobile",
            DeviceType::Web => "web",
            DeviceType::Cli => "cli",
        }
    }
}

/// Session information (for future cloud authentication)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: Id,
    pub user_id: Id,
    pub device_id: Option<Id>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Timestamp,
    pub created_at: Timestamp,
    pub last_used_at: Timestamp,
}

impl Session {
    pub fn new(user_id: Id) -> Self {
        let now = now();
        // Default 30 day expiration
        let expires_at = now + (30 * 24 * 60 * 60 * 1000);

        Self {
            id: new_id(),
            user_id,
            device_id: None,
            access_token: None,
            refresh_token: None,
            expires_at,
            created_at: now,
            last_used_at: now,
        }
    }

    pub fn with_device(mut self, device_id: Id) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn with_tokens(mut self, access_token: String, refresh_token: String) -> Self {
        self.access_token = Some(access_token);
        self.refresh_token = Some(refresh_token);
        self
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        now() > self.expires_at
    }

    /// Check if session is valid (not expired)
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

impl Identifiable for Session {
    fn id(&self) -> Id {
        self.id
    }
}

/// Local user settings (persisted locally)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSettings {
    pub theme: Theme,
    pub language: String,
    pub auto_save: bool,
    pub send_anonymous_usage_data: bool,
    pub check_updates: bool,
    pub default_request_timeout_secs: u32,
    pub follow_redirects: bool,
    pub validate_ssl: bool,

    /// Editor settings
    pub editor: EditorSettings,

    /// Proxy settings
    pub proxy: Option<ProxySettings>,

    /// Cloud sync settings (for future use)
    pub cloud_sync: CloudSyncSettings,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: "en".to_string(),
            auto_save: true,
            send_anonymous_usage_data: false,
            check_updates: true,
            default_request_timeout_secs: 30,
            follow_redirects: true,
            validate_ssl: true,
            editor: EditorSettings::default(),
            proxy: None,
            cloud_sync: CloudSyncSettings::default(),
        }
    }
}

/// Application theme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// Editor settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorSettings {
    pub font_size: u32,
    pub font_family: String,
    pub tab_size: u32,
    pub insert_final_newline: bool,
    pub trim_trailing_whitespace: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub minimap: bool,
    pub format_on_paste: bool,
}

impl Default for EditorSettings {
    fn default() -> Self {
        Self {
            font_size: 14,
            font_family: "Monaco, Menlo, monospace".to_string(),
            tab_size: 4,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            word_wrap: false,
            line_numbers: true,
            minimap: true,
            format_on_paste: true,
        }
    }
}

/// Proxy settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxySettings {
    pub enabled: bool,
    pub protocol: ProxyProtocol,
    pub host: String,
    pub port: u16,
    pub auth: Option<ProxyAuth>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyProtocol {
    Http,
    Https,
    Socks5,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProxyAuth {
    pub username: String,
    pub password: String,
}

/// Cloud sync settings (designed for future cloud integration)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudSyncSettings {
    /// Sync mode
    pub mode: SyncMode,

    /// Auto-sync interval in seconds
    pub auto_sync_interval_secs: u32,

    /// Conflict resolution strategy
    pub conflict_strategy: ConflictStrategy,

    /// Configured server (if any)
    pub server_config: Option<CloudServerConfig>,

    /// Last successful sync timestamp
    pub last_sync_at: Option<Timestamp>,

    /// Pending changes count
    pub pending_changes: u32,
}

impl Default for CloudSyncSettings {
    fn default() -> Self {
        Self {
            mode: SyncMode::Offline,
            auto_sync_interval_secs: 300, // 5 minutes
            conflict_strategy: ConflictStrategy::LastWriteWins,
            server_config: None,
            last_sync_at: None,
            pending_changes: 0,
        }
    }
}

/// Sync mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncMode {
    /// Completely offline - no cloud sync
    Offline,

    /// Online with automatic sync
    OnlineAuto,

    /// Online with manual sync only
    OnlineManual,

    /// Hybrid - local first with periodic sync
    Hybrid,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConflictStrategy {
    /// Local version wins
    LocalWins,

    /// Remote version wins
    RemoteWins,

    /// Last write wins based on timestamp
    LastWriteWins,

    /// Require manual resolution
    Manual,
}

/// Cloud server configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudServerConfig {
    pub server_url: String,
    pub api_key: Option<String>,
    pub organization_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "user@example.com".to_string(),
            "John Doe".to_string(),
        );

        assert_eq!(user.email, "user@example.com");
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.plan, UserPlan::Free);
        assert!(user.is_active);
    }

    #[test]
    fn test_user_quota_default() {
        let quota = UserQuota::default();
        assert_eq!(quota.max_collections, 10);
        assert_eq!(quota.max_requests_per_collection, 100);
    }

    #[test]
    fn test_user_quota_for_plan() {
        let free_quota = UserQuota::for_plan(UserPlan::Free);
        assert_eq!(free_quota.max_collections, 10);

        let pro_quota = UserQuota::for_plan(UserPlan::Pro);
        assert_eq!(pro_quota.max_collections, 100);

        let enterprise_quota = UserQuota::for_plan(UserPlan::Enterprise);
        assert_eq!(enterprise_quota.max_collections, u32::MAX);
    }

    #[test]
    fn test_can_create_collection() {
        let user = User::new("test@example.com".to_string(), "Test".to_string());

        assert!(user.can_create_collection(0));
        assert!(user.can_create_collection(9));
        assert!(!user.can_create_collection(10));
    }

    #[test]
    fn test_session_expiration() {
        let mut session = Session::new(new_id());

        // Session should be valid initially
        assert!(session.is_valid());
        assert!(!session.is_expired());

        // Set expiration to past
        session.expires_at = now() - 1000;
        assert!(!session.is_valid());
        assert!(session.is_expired());
    }

    #[test]
    fn test_device_creation() {
        let device = Device::new(
            new_id(),
            "MacBook Pro".to_string(),
            DeviceType::Desktop,
        );

        assert_eq!(device.name, "MacBook Pro");
        assert_eq!(device.device_type, DeviceType::Desktop);
    }

    #[test]
    fn test_sync_mode_default() {
        let settings = CloudSyncSettings::default();
        assert_eq!(settings.mode, SyncMode::Offline);
        assert!(settings.server_config.is_none());
    }

    #[test]
    fn test_user_settings_default() {
        let settings = UserSettings::default();

        assert_eq!(settings.theme, Theme::System);
        assert_eq!(settings.language, "en");
        assert!(settings.auto_save);
        assert!(settings.validate_ssl);
    }
}
