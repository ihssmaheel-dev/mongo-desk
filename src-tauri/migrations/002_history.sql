CREATE TABLE IF NOT EXISTS query_history (
    id TEXT PRIMARY KEY,
    connection_id TEXT NOT NULL,
    database TEXT NOT NULL,
    collection TEXT,
    query_text TEXT NOT NULL,
    query_type TEXT NOT NULL,
    execution_time_ms INTEGER,
    result_count INTEGER,
    is_favorite INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_query_history_connection ON query_history(connection_id);
CREATE INDEX IF NOT EXISTS idx_query_history_created ON query_history(created_at DESC);

CREATE TABLE IF NOT EXISTS saved_queries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    folder TEXT,
    connection_id TEXT,
    database TEXT,
    collection TEXT,
    query_text TEXT NOT NULL,
    query_type TEXT NOT NULL,
    sort_order INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS aggregation_history (
    id TEXT PRIMARY KEY,
    connection_id TEXT NOT NULL,
    database TEXT NOT NULL,
    collection TEXT NOT NULL,
    pipeline_json TEXT NOT NULL,
    execution_time_ms INTEGER,
    result_count INTEGER,
    is_favorite INTEGER DEFAULT 0,
    created_at TEXT NOT NULL,
    FOREIGN KEY (connection_id) REFERENCES connections(id) ON DELETE CASCADE
);
