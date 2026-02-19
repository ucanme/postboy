-- Migration 002: Add sync support tables
-- This migration adds tables for cloud synchronization capability
-- The app works fully offline with these tables storing pending changes

-- Sync configuration
CREATE TABLE IF NOT EXISTS sync_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    mode TEXT NOT NULL DEFAULT 'offline', -- 'offline', 'online_auto', 'online_manual', 'hybrid'
    server_url TEXT,
    api_key TEXT,
    device_id TEXT NOT NULL,
    last_sync_at INTEGER,
    auto_sync_interval INTEGER DEFAULT 0,
    conflict_strategy TEXT DEFAULT 'last_write_wins', -- 'local_wins', 'remote_wins', 'last_write_wins', 'manual'
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000)
);

-- Insert default config
INSERT OR IGNORE INTO sync_config (id, mode, device_id)
VALUES (1, 'offline', lower(hex(randomblob(16))));

-- Pending sync changes queue
CREATE TABLE IF NOT EXISTS sync_changes (
    change_id TEXT PRIMARY KEY,
    item_type TEXT NOT NULL, -- 'collection', 'folder', 'request', 'environment'
    item_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    version INTEGER NOT NULL DEFAULT 1,
    data JSON NOT NULL,
    synced INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000),
    synced_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_sync_changes_item ON sync_changes(item_type, item_id);
CREATE INDEX IF NOT EXISTS idx_sync_changes_synced ON sync_changes(synced, created_at);

-- Conflict records
CREATE TABLE IF NOT EXISTS sync_conflicts (
    conflict_id TEXT PRIMARY KEY,
    item_type TEXT NOT NULL,
    item_id TEXT NOT NULL,
    item_name TEXT,
    local_version INTEGER NOT NULL,
    remote_version INTEGER NOT NULL,
    local_value JSON NOT NULL,
    remote_value JSON NOT NULL,
    resolved INTEGER NOT NULL DEFAULT 0,
    resolution TEXT, -- 'local', 'remote', 'merged'
    resolved_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000)
);

CREATE INDEX IF NOT EXISTS idx_sync_conflicts_resolved ON sync_conflicts(resolved, created_at);

-- Sync session history
CREATE TABLE IF NOT EXISTS sync_sessions (
    session_id TEXT PRIMARY KEY,
    started_at INTEGER NOT NULL,
    completed_at INTEGER,
    changes_pushed INTEGER DEFAULT 0,
    changes_pulled INTEGER DEFAULT 0,
    conflicts_count INTEGER DEFAULT 0,
    status TEXT NOT NULL, -- 'running', 'success', 'failed', 'partial'
    error_message TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_sessions_started ON sync_sessions(started_at DESC);

-- Devices tracking (for multi-device sync)
CREATE TABLE IF NOT EXISTS devices (
    device_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    device_type TEXT NOT NULL, -- 'desktop', 'mobile', 'web'
    os_info TEXT,
    last_seen INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000),
    is_online INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000)
);

-- Remote item tracking (maps local IDs to remote IDs and versions)
CREATE TABLE IF NOT EXISTS remote_items (
    local_id TEXT NOT NULL,
    remote_id TEXT NOT NULL,
    item_type TEXT NOT NULL,
    remote_version INTEGER NOT NULL,
    last_synced_at INTEGER NOT NULL DEFAULT (strftime('%s', 'subpath('now')) * 1000),
    PRIMARY KEY (local_id, item_type)
);

CREATE INDEX IF NOT EXISTS idx_remote_items_remote ON remote_items(remote_id, item_type);

-- Sync state for collections (inline version tracking)
ALTER TABLE collections ADD COLUMN sync_state TEXT DEFAULT '{"status":"not_synced","version":null,"remote_id":null,"last_synced_at":null,"pending_changes":0}';

-- Sync state for individual requests
ALTER TABLE requests ADD COLUMN sync_state TEXT DEFAULT '{"status":"not_synced","version":null,"remote_id":null,"last_synced_at":null}';

-- Add updated_at trigger to sync_config
CREATE TRIGGER IF NOT EXISTS update_sync_config_timestamp
AFTER UPDATE ON sync_config
BEGIN
    UPDATE sync_config SET updated_at = (strftime('%s', 'subpath('now')) * 1000) WHERE id = NEW.id;
END;
