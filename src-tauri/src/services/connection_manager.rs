use std::collections::HashMap;
use mongodb::{Client, options::ClientOptions};
use rusqlite::Connection as SqliteConn;

use crate::error::AppError;
use crate::models::connection::{Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult};
use crate::db::queries::Queries;

pub struct ConnectionManager {
    clients: HashMap<String, Client>,
    max_pool_size: usize,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            max_pool_size: 10,
        }
    }

    pub fn add_connection(&self, new_conn: NewConnection, sqlite: &SqliteConn) -> Result<Connection, AppError> {
        use uuid::Uuid;
        use chrono::Utc;

        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        let conn = Connection {
            id,
            name: new_conn.name,
            connection_string: new_conn.connection_string,
            conn_type: new_conn.conn_type,
            color: new_conn.color,
            read_only: new_conn.read_only,
            ssl_enabled: new_conn.ssl_enabled,
            ssl_config: new_conn.ssl_config,
            group_id: new_conn.group_id,
            tags: new_conn.tags,
            created_at: now.clone(),
            updated_at: now,
        };

        Queries::insert_connection(&conn, sqlite)
    }

    pub fn update_connection(&self, id: &str, update: UpdateConnection, sqlite: &SqliteConn) -> Result<Connection, AppError> {
        Queries::update_connection(id, &update, sqlite)
    }

    pub fn delete_connection(&mut self, id: &str, sqlite: &SqliteConn) -> Result<(), AppError> {
        self.clients.remove(id);
        Queries::delete_connection(id, sqlite)
    }

    pub fn list_connections(&self, sqlite: &SqliteConn) -> Result<Vec<Connection>, AppError> {
        Queries::list_connections(sqlite)
    }

    pub async fn test_connection_string(connection_string: &str) -> Result<ConnectionTestResult, AppError> {
        Self::validate_connection_string(connection_string)?;

        let options = ClientOptions::parse(connection_string)
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let client = Client::with_options(options)
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let db_names = client.list_database_names()
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        Ok(ConnectionTestResult {
            success: true,
            message: "Connection successful".to_string(),
            databases: Some(db_names),
        })
    }

    pub fn get_connection_string(id: &str, sqlite: &SqliteConn) -> Result<String, AppError> {
        let conn = Queries::get_connection(id, sqlite)?;
        Ok(conn.connection_string)
    }

    pub async fn get_or_create_client(&mut self, id: &str, connection_string: &str) -> Result<Client, AppError> {
        if let Some(client) = self.clients.get(id) {
            return Ok(client.clone());
        }

        if self.clients.len() >= self.max_pool_size {
            return Err(AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Connection pool is full (max 10). Disconnect unused connections.".into(),
            });
        }

        let options = ClientOptions::parse(connection_string)
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let client = Client::with_options(options)
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        self.clients.insert(id.to_string(), client.clone());
        Ok(client)
    }

    pub fn disconnect(&mut self, id: &str) {
        self.clients.remove(id);
    }

    pub fn is_connected(&self, id: &str) -> bool {
        self.clients.contains_key(id)
    }

    pub fn validate_connection_string(connection_string: &str) -> Result<(), AppError> {
        if connection_string.is_empty() {
            return Err(AppError::validation("connection_string", "Connection string is required"));
        }

        if !connection_string.starts_with("mongodb://") && !connection_string.starts_with("mongodb+srv://") {
            return Err(AppError::validation("connection_string", "Connection string must start with mongodb:// or mongodb+srv://"));
        }

        Ok(())
    }

    pub fn list_groups(&self, sqlite: &SqliteConn) -> Result<Vec<ConnectionGroup>, AppError> {
        Queries::list_groups(sqlite)
    }

    pub fn add_group(&self, group: NewConnectionGroup, sqlite: &SqliteConn) -> Result<ConnectionGroup, AppError> {
        Queries::insert_group(&group, sqlite)
    }

    pub fn delete_group(&self, id: &str, sqlite: &SqliteConn) -> Result<(), AppError> {
        Queries::delete_group(id, sqlite)
    }

    pub fn import_connections(&self, data: &str, sqlite: &SqliteConn) -> Result<Vec<Connection>, AppError> {
        let conns: Vec<NewConnection> = serde_json::from_str(data)
            .map_err(|e| AppError::Validation {
                code: "ERR_VALIDATION".into(),
                message: format!("Invalid JSON: {}", e),
                field: None,
            })?;

        let mut results = Vec::new();
        for conn in conns {
            let created = self.add_connection(conn, sqlite)?;
            results.push(created);
        }
        Ok(results)
    }

    pub fn export_connections(&self, sqlite: &SqliteConn) -> Result<String, AppError> {
        let connections = Queries::list_connections(sqlite)?;
        serde_json::to_string_pretty(&connections)
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to serialize connections: {}", e),
            })
    }
}
