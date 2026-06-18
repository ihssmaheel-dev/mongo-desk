use std::sync::Arc;
use parking_lot::Mutex;
use mongodb::{Client, options::ClientOptions};
use std::collections::HashMap;

use crate::error::AppError;
use crate::models::connection::{Connection, ConnectionTestResult};

pub struct ConnectionManager {
    connections: Arc<Mutex<HashMap<String, Client>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn test_connection(connection_string: &str) -> Result<ConnectionTestResult, AppError> {
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

    pub async fn get_or_create_client(&self, connection: &Connection) -> Result<Client, AppError> {
        {
            let connections = self.connections.lock();
            if let Some(client) = connections.get(&connection.id) {
                return Ok(client.clone());
            }
        }

        let options = ClientOptions::parse(&connection.connection_string)
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let client = Client::with_options(options)
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        {
            let mut connections = self.connections.lock();
            connections.insert(connection.id.clone(), client.clone());
        }

        Ok(client)
    }

    pub fn disconnect(&self, id: &str) {
        let mut connections = self.connections.lock();
        connections.remove(id);
    }

    pub fn is_connected(&self, id: &str) -> bool {
        let connections = self.connections.lock();
        connections.contains_key(id)
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
}
