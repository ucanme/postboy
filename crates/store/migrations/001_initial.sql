-- Initial database schema for Postboy
-- Designed for offline-first operation with future cloud sync compatibility

-- Enable foreign keys (redundant with app-level setting, but safe to have)
PRAGMA foreign_keys = ON;

-- ============================================================================
-- Collections
-- ============================================================================
CREATE TABLE IF NOT EXISTS collections (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,

    -- Collection metadata (JSON)
    info TEXT NOT NULL DEFAULT '{}',

    -- Authentication config (JSON)
    auth TEXT,

    -- Sync state (JSON, for future cloud sync)
    sync_state TEXT NOT NULL DEFAULT '{}',

    -- UI state (JSON)
    ui_state TEXT NOT NULL DEFAULT '{}',

    -- Timestamps
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    -- Full-text search
    fts_name TEXT GENERATED ALWAYS AS (name) STORED
);

-- Index for collection lookups
CREATE INDEX IF NOT EXISTS idx_collections_name
    ON collections(name COLLATE NOCASE);

CREATE INDEX IF NOT EXISTS idx_collections_updated
    ON collections(updated_at DESC);

-- Full-text search virtual table
CREATE VIRTUAL TABLE IF NOT EXISTS collections_fts USING fts5(
    name,
    content='collections',
    content_rowid='rowid'
);

-- FTS triggers
CREATE TRIGGER IF NOT EXISTS collections_fts_insert AFTER INSERT ON collections BEGIN
    INSERT INTO collections_fts(rowid, name)
    VALUES (new.rowid, new.name);
END;

CREATE TRIGGER IF NOT EXISTS collections_fts_delete AFTER DELETE ON collections BEGIN
    DELETE FROM collections_fts WHERE rowid = old.rowid;
END;

CREATE TRIGGER IF NOT EXISTS collections_fts_update AFTER UPDATE ON collections BEGIN
    UPDATE collections_fts SET name = new.name WHERE rowid = new.rowid;
END;

-- ============================================================================
-- Folders
-- ============================================================================
CREATE TABLE IF NOT EXISTS folders (
    id TEXT PRIMARY KEY NOT NULL,
    collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    parent_id TEXT REFERENCES folders(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,

    -- UI state (JSON)
    ui_state TEXT NOT NULL DEFAULT '{}',

    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_folders_collection
    ON folders(collection_id);

CREATE INDEX IF NOT EXISTS idx_folders_parent
    ON folders(parent_id);

-- ============================================================================
-- Requests
-- ============================================================================
CREATE TABLE IF NOT EXISTS requests (
    id TEXT PRIMARY KEY NOT NULL,
    collection_id TEXT REFERENCES collections(id) ON DELETE SET NULL,
    folder_id TEXT REFERENCES folders(id) ON DELETE SET NULL,
    name TEXT NOT NULL,
    description TEXT,

    -- HTTP method
    method TEXT NOT NULL CHECK(method IN ('GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'HEAD', 'OPTIONS')),

    -- URL (stored as both raw and parsed for efficient queries)
    url_raw TEXT NOT NULL,
    url_protocol TEXT,
    url_host TEXT,
    url_path TEXT,

    -- Request data (JSON)
    headers TEXT NOT NULL DEFAULT '[]',
    query_params TEXT NOT NULL DEFAULT '[]',
    body TEXT NOT NULL DEFAULT '{}',
    auth TEXT,

    -- Script configuration (JSON)
    script TEXT NOT NULL DEFAULT '{}',

    -- UI state (JSON)
    ui_state TEXT NOT NULL DEFAULT '{}',

    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,

    -- Full-text search
    fts_name TEXT GENERATED ALWAYS AS (name) STORED,
    fts_url TEXT GENERATED ALWAYS AS (url_raw) STORED
);

CREATE INDEX IF NOT EXISTS idx_requests_collection
    ON requests(collection_id);

CREATE INDEX IF NOT EXISTS idx_requests_folder
    ON requests(folder_id);

CREATE INDEX IF NOT EXISTS idx_requests_method
    ON requests(method);

CREATE INDEX IF NOT EXISTS idx_requests_updated
    ON requests(updated_at DESC);

-- Full-text search for requests
CREATE VIRTUAL TABLE IF NOT EXISTS requests_fts USING fts5(
    name,
    url,
    content='requests',
    content_rowid='rowid'
);

CREATE TRIGGER IF NOT EXISTS requests_fts_insert AFTER INSERT ON requests BEGIN
    INSERT INTO requests_fts(rowid, name, url)
    VALUES (new.rowid, new.name, new.url_raw);
END;

CREATE TRIGGER IF NOT EXISTS requests_fts_delete AFTER DELETE ON requests BEGIN
    DELETE FROM requests_fts WHERE rowid = old.rowid;
END;

CREATE TRIGGER IF NOT EXISTS requests_fts_update AFTER UPDATE ON requests BEGIN
    UPDATE requests_fts SET name = new.name, url = new.url_raw WHERE rowid = new.rowid;
END;

-- ============================================================================
-- Environments
-- ============================================================================
CREATE TABLE IF NOT EXISTS environments (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,

    -- Variables (JSON array)
    variables TEXT NOT NULL DEFAULT '[]',

    -- Is this the currently active environment?
    is_active INTEGER NOT NULL DEFAULT 0 CHECK(is_active IN (0, 1)),

    -- Sync state (JSON, for future cloud sync)
    sync_state TEXT NOT NULL DEFAULT '{}',

    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_environments_active
    ON environments(is_active);

CREATE INDEX IF NOT EXISTS idx_environments_name
    ON environments(name COLLATE NOCASE);

-- Ensure only one environment can be active at a time
CREATE TRIGGER IF NOT EXISTS enforce_single_active_env
    AFTER UPDATE OF is_active ON environments
    WHEN NEW.is_active = 1
BEGIN
    UPDATE environments SET is_active = 0
    WHERE id != NEW.id AND is_active = 1;
END;

-- ============================================================================
-- Global Variables
-- ============================================================================
CREATE TABLE IF NOT EXISTS globals (
    id TEXT PRIMARY KEY NOT NULL,

    -- Variables (JSON array)
    variables TEXT NOT NULL DEFAULT '[]',

    -- Last update timestamp
    updated_at INTEGER NOT NULL
);

-- Initialize globals row
INSERT OR IGNORE INTO globals (id, variables, updated_at)
    VALUES ('default', '[]', 0);

-- ============================================================================
-- Request History
-- ============================================================================
CREATE TABLE IF NOT EXISTS request_history (
    id TEXT PRIMARY KEY NOT NULL,
    request_id TEXT REFERENCES requests(id) ON DELETE SET NULL,

    -- Request snapshot at time of execution
    method TEXT NOT NULL,
    url TEXT NOT NULL,
    headers TEXT,
    body_preview TEXT,

    -- Response data
    status_code INTEGER,
    status_text TEXT,
    response_headers TEXT,

    -- Response body stored separately if large
    response_body_size INTEGER,
    response_body_id TEXT,

    -- Timing
    duration_ms INTEGER,
    timestamp INTEGER NOT NULL,

    -- Test results (JSON)
    test_results TEXT NOT NULL DEFAULT '[]',

    -- Errors (JSON array)
    errors TEXT NOT NULL DEFAULT '[]'
);

CREATE INDEX IF NOT EXISTS idx_history_request
    ON request_history(request_id, timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_history_timestamp
    ON request_history(timestamp DESC);

CREATE INDEX IF NOT EXISTS idx_history_status
    ON request_history(status_code);

-- Auto-cleanup old history (keep last 1000 per request)
CREATE TRIGGER IF NOT EXISTS cleanup_old_history
    AFTER INSERT ON request_history
    WHEN NEW.request_id IS NOT NULL
BEGIN
    DELETE FROM request_history
    WHERE id IN (
        SELECT id FROM request_history
        WHERE request_id = NEW.request_id
        ORDER BY timestamp DESC
        LIMIT -1 OFFSET 1000
    );
END;

-- ============================================================================
-- Response Bodies (stored separately to save space)
-- ============================================================================
CREATE TABLE IF NOT EXISTS response_bodies (
    id TEXT PRIMARY KEY NOT NULL,
    body BLOB,

    -- Metadata
    content_type TEXT,
    encoding TEXT DEFAULT 'utf-8',
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_response_bodies_created
    ON response_bodies(created_at DESC);

-- ============================================================================
-- Sync Changes (for future cloud sync)
-- ============================================================================
CREATE TABLE IF NOT EXISTS sync_changes (
    id TEXT PRIMARY KEY NOT NULL,

    -- Item identification
    item_type TEXT NOT NULL CHECK(item_type IN ('collection', 'folder', 'request', 'environment')),
    item_id TEXT NOT NULL,

    -- Operation
    operation TEXT NOT NULL CHECK(operation IN ('create', 'update', 'delete')),
    version INTEGER NOT NULL,

    -- Data snapshot (JSON)
    data TEXT NOT NULL,

    -- Sync status
    synced INTEGER NOT NULL DEFAULT 0 CHECK(synced IN (0, 1)),
    error TEXT,

    -- Timestamp
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sync_changes_unsynced
    ON sync_changes(synced, created_at) WHERE synced = 0;

CREATE INDEX IF NOT EXISTS idx_sync_changes_item
    ON sync_changes(item_type, item_id, created_at DESC);

-- ============================================================================
-- Sync State (for future cloud sync)
-- ============================================================================
CREATE TABLE IF NOT EXISTS sync_state (
    id TEXT PRIMARY KEY NOT NULL DEFAULT 'config',

    -- Sync mode
    mode TEXT NOT NULL DEFAULT 'offline' CHECK(mode IN ('offline', 'online_auto', 'online_manual', 'hybrid')),

    -- Server configuration (JSON, encrypted if needed)
    server_config TEXT,

    -- Device ID (unique per installation)
    device_id TEXT NOT NULL,

    -- Last sync timestamp
    last_sync_at INTEGER,

    -- Sync status (JSON)
    status TEXT NOT NULL DEFAULT '{}',

    -- Pending changes count
    pending_changes INTEGER NOT NULL DEFAULT 0,

    -- Auto-sync interval (seconds, 0 = disabled)
    auto_sync_interval INTEGER NOT NULL DEFAULT 0,

    -- Conflict resolution strategy
    conflict_strategy TEXT NOT NULL DEFAULT 'last_write_wins'
        CHECK(conflict_strategy IN ('local_wins', 'remote_wins', 'last_write_wins', 'manual')),

    updated_at INTEGER NOT NULL
);

-- Initialize sync state (offline by default)
INSERT OR IGNORE INTO sync_state (id, mode, device_id, updated_at)
    VALUES ('config', 'offline', lower(hex(randomblob(16))), strftime('%s', 'now') * 1000);

-- ============================================================================
-- User Settings (local application settings)
-- ============================================================================
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY NOT NULL,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Insert default settings
INSERT OR IGNORE INTO settings (key, value, updated_at) VALUES
    ('theme', '"system"', strftime('%s', 'now') * 1000),
    ('language', '"en"', strftime('%s', 'now') * 1000),
    ('auto_save', 'true', strftime('%s', 'now') * 1000),
    ('follow_redirects', 'true', strftime('%s', 'now') * 1000),
    ('validate_ssl', 'true', strftime('%s', 'now') * 1000),
    ('editor_font_size', '14', strftime('%s', 'now') * 1000),
    ('editor_tab_size', '4', strftime('%s', 'now') * 1000);

-- ============================================================================
-- Collection Variables
-- ============================================================================
CREATE TABLE IF NOT EXISTS collection_variables (
    id TEXT PRIMARY KEY NOT NULL,
    collection_id TEXT NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    variable_type TEXT NOT NULL DEFAULT 'normal' CHECK(variable_type IN ('normal', 'secret', 'json')),
    enabled INTEGER NOT NULL DEFAULT 1,
    description TEXT,

    UNIQUE(collection_id, key)
);

CREATE INDEX IF NOT EXISTS idx_collection_vars_collection
    ON collection_variables(collection_id);

-- ============================================================================
-- Audit Log (for tracking changes, useful for debugging and sync)
-- ============================================================================
CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY NOT NULL,

    -- Event type
    event_type TEXT NOT NULL,

    -- Target entity
    entity_type TEXT NOT NULL,
    entity_id TEXT,

    -- Event details (JSON)
    details TEXT,

    -- Timestamp
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_log_entity
    ON audit_log(entity_type, entity_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_log_timestamp
    ON audit_log(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_audit_log_type
    ON audit_log(event_type, created_at DESC);

-- ============================================================================
-- Helper views
-- ============================================================================

-- View: Collections with request counts
CREATE VIEW IF NOT EXISTS v_collections_with_stats AS
SELECT
    c.id,
    c.name,
    c.description,
    c.info,
    c.auth,
    c.sync_state,
    c.created_at,
    c.updated_at,
    COUNT(DISTINCT r.id) as request_count,
    COUNT(DISTINCT CASE WHEN r.folder_id IS NULL THEN r.id END) as root_request_count
FROM collections c
LEFT JOIN requests r ON r.collection_id = c.id
GROUP BY c.id;

-- View: Recent request history
CREATE VIEW IF NOT EXISTS v_recent_history AS
SELECT
    rh.id,
    rh.request_id,
    r.name as request_name,
    rh.method,
    rh.url,
    rh.status_code,
    rh.status_text,
    rh.duration_ms,
    rh.timestamp,
    rh.test_results
FROM request_history rh
LEFT JOIN requests r ON rh.request_id = r.id
ORDER BY rh.timestamp DESC
LIMIT 100;

-- ============================================================================
-- Triggers for automatic timestamp updates
-- ============================================================================

-- Collections
CREATE TRIGGER IF NOT EXISTS collections_updated_at
    AFTER UPDATE ON collections
BEGIN
    UPDATE collections SET updated_at = strftime('%s', 'now') * 1000
    WHERE id = NEW.id;
END;

-- Folders
CREATE TRIGGER IF NOT EXISTS folders_updated_at
    AFTER UPDATE ON folders
BEGIN
    UPDATE folders SET updated_at = strftime('%s', 'now') * 1000
    WHERE id = NEW.id;
END;

-- Requests
CREATE TRIGGER IF NOT EXISTS requests_updated_at
    AFTER UPDATE ON requests
BEGIN
    UPDATE requests SET updated_at = strftime('%s', 'now') * 1000
    WHERE id = NEW.id;
END;

-- Environments
CREATE TRIGGER IF NOT EXISTS environments_updated_at
    AFTER UPDATE ON environments
BEGIN
    UPDATE environments SET updated_at = strftime('%s', 'now') * 1000
    WHERE id = NEW.id;
END;

-- Sync state
CREATE TRIGGER IF NOT EXISTS sync_state_updated_at
    AFTER UPDATE ON sync_state
BEGIN
    UPDATE sync_state SET updated_at = strftime('%s', 'now') * 1000
    WHERE id = NEW.id;
END;

-- ============================================================================
-- Triggers for cascade collection deletion
-- ============================================================================

-- Delete collection variables when collection is deleted
CREATE TRIGGER IF NOT EXISTS delete_collection_vars
    AFTER DELETE ON collections
BEGIN
    DELETE FROM collection_variables WHERE collection_id = OLD.id;
END;

-- ============================================================================
-- Application Version
-- ============================================================================
CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER PRIMARY KEY,
    applied_at INTEGER NOT NULL
);

INSERT INTO schema_version (version, applied_at) VALUES (1, strftime('%s', 'now') * 1000);
