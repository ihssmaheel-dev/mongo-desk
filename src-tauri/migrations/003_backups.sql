CREATE TABLE IF NOT EXISTS backups (
    id TEXT PRIMARY KEY,
    connection_id TEXT NOT NULL,
    database TEXT,
    collection TEXT,
    scope TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_size INTEGER,
    document_count INTEGER,
    duration_ms INTEGER,
    status TEXT NOT NULL,
    error_message TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_backups_connection ON backups(connection_id);
CREATE INDEX IF NOT EXISTS idx_backups_created ON backups(created_at DESC);

CREATE TABLE IF NOT EXISTS window_state (
    id INTEGER PRIMARY KEY DEFAULT 1,
    x INTEGER,
    y INTEGER,
    width INTEGER,
    height INTEGER,
    is_maximized INTEGER DEFAULT 0,
    sidebar_width INTEGER DEFAULT 260,
    active_tab TEXT,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS tab_snapshots (
    id TEXT PRIMARY KEY,
    tab_type TEXT NOT NULL,
    query_text TEXT,
    scroll_position INTEGER DEFAULT 0,
    metadata TEXT,
    created_at TEXT NOT NULL,
    accessed_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_tab_snapshots_accessed ON tab_snapshots(accessed_at DESC);

CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    action TEXT NOT NULL,
    entity_type TEXT,
    entity_id TEXT,
    details TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_log_created ON audit_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_action ON audit_log(action);
