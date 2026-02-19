-- Migration: 003_add_history.sql
-- Description: Add request history tracking for offline functionality
-- This table stores a log of all executed requests with their responses

-- Request history table
CREATE TABLE IF NOT EXISTS request_history (
    id TEXT PRIMARY KEY,
    request_id TEXT,                          -- Reference to the saved request (if any)
    collection_id TEXT,                       -- Reference to collection for context
    folder_id TEXT,                           -- Reference to folder for context

    -- Request details (snapshot at time of execution)
    request_name TEXT NOT NULL,               -- Name of the request
    method TEXT NOT NULL,                     -- HTTP method used
    url TEXT NOT NULL,                        -- Full URL sent

    -- Response details
    status_code INTEGER,                      -- HTTP status code received
    status_text TEXT,                         -- Status text (e.g., "OK", "Not Found")
    response_size INTEGER DEFAULT 0,          -- Response size in bytes
    duration_ms INTEGER DEFAULT 0,            -- Request duration in milliseconds
    started_at INTEGER NOT NULL,              -- When the request started
    completed_at INTEGER NOT NULL,            -- When the request completed

    -- Response data (stored separately for size management)
    response_headers TEXT,                    -- Response headers as JSON
    response_body_path TEXT,                  -- Path to response body file (for large responses)

    -- Script execution results
    pre_request_script_passed INTEGER DEFAULT 1,  -- Whether pre-request script passed
    post_response_script_passed INTEGER DEFAULT 1, -- Whether post-response script passed
    test_results TEXT,                        -- Test results as JSON (array of test results)

    -- Error information
    error_message TEXT,                       -- Error message if request failed
    error_type TEXT,                          -- Type of error (network, timeout, script, etc.)

    -- Metadata
    created_at INTEGER NOT NULL,              -- When this history entry was created

    FOREIGN KEY (request_id) REFERENCES requests(id) ON DELETE SET NULL,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE SET NULL,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE SET NULL
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_history_request_id ON request_history(request_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_history_collection_id ON request_history(collection_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_history_folder_id ON request_history(folder_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_history_created_at ON request_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_history_method ON request_history(method, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_history_status_code ON request_history(status_code, created_at DESC);

-- Trigger to automatically set completed_at if not provided
CREATE TRIGGER IF NOT EXISTS trg_history_set_completed_at
    BEFORE INSERT ON request_history
    WHEN NEW.completed_at IS NULL OR NEW.completed_at = 0
BEGIN
    SET NEW.completed_at = (CAST(strftime('%s', 'now') AS INTEGER) * 1000);
END;

-- View for recent request history with summary
CREATE VIEW IF NOT EXISTS v_recent_history AS
SELECT
    id,
    request_name,
    method,
    url,
    status_code,
    status_text,
    duration_ms,
    response_size,
    started_at,
    CASE
        WHEN error_message IS NOT NULL THEN 'error'
        WHEN post_response_script_passed = 0 THEN 'failed'
        ELSE 'success'
    END as status,
    created_at
FROM request_history
ORDER BY started_at DESC
LIMIT 1000;
