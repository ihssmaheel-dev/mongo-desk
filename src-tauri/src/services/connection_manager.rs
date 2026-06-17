use crate::error::AppError;
use crate::models::connection::{Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult};
use mongodb::{Client, options::ClientOptions};
use std::str::FromStr;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn test_connection(connection_string: &str) -> Result<ConnectionTestResult, AppError> {
        let options = ClientOptions::parse(connection_string)
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let client = Client::with_options(options)
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        let db_names = client.list_database_names(None, None)
            .await
            .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

        Ok(ConnectionTestResult {
            success: true,
            message: "Connection successful".to_string(),
            databases: Some(db_names),
        })
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
