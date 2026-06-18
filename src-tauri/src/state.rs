use parking_lot::Mutex;
use std::sync::Arc;
use std::path::PathBuf;
use tokio_util::sync::CancellationToken;
use rusqlite::Connection as SqliteConn;

use crate::db::sqlite::initialize_database;
use crate::services::connection_manager::ConnectionManager;

pub struct AppState {
    pub sqlite: Arc<Mutex<SqliteConn>>,
    pub connection_manager: Arc<Mutex<ConnectionManager>>,
    pub cancellation_tokens: Arc<Mutex<CancellationTokenRegistry>>,
}

pub struct CancellationTokenRegistry {
    tokens: std::collections::HashMap<String, CancellationToken>,
}

impl CancellationTokenRegistry {
    pub fn new() -> Self {
        Self {
            tokens: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, token: CancellationToken) {
        self.tokens.insert(id, token);
    }

    pub fn get(&self, id: &str) -> Option<CancellationToken> {
        self.tokens.get(id).cloned()
    }

    pub fn remove(&mut self, id: &str) -> Option<CancellationToken> {
        self.tokens.remove(id)
    }

    pub fn cancel(&mut self, id: &str) -> bool {
        if let Some(token) = self.tokens.get(id) {
            token.cancel();
            true
        } else {
            false
        }
    }
}

impl AppState {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_db_path();
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create data directory: {}", e))?;
        }
        let sqlite = initialize_database(&db_path)
            .map_err(|e| format!("Failed to initialize database: {:?}", e))?;

        Ok(Self {
            sqlite: Arc::new(Mutex::new(sqlite)),
            connection_manager: Arc::new(Mutex::new(ConnectionManager::new())),
            cancellation_tokens: Arc::new(Mutex::new(CancellationTokenRegistry::new())),
        })
    }

    fn get_db_path() -> PathBuf {
        let app_data = dirs::data_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".local").join("share")))
            .unwrap_or_else(|| PathBuf::from("."));
        app_data.join("MongoDesk").join("mongodesk.db")
    }
}
