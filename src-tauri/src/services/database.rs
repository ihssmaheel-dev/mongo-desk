use mongodb::{Client, bson::doc};

use crate::error::AppError;
use crate::models::database::{DatabaseInfo, DatabaseStats};

pub struct DatabaseService;

impl DatabaseService {
    pub async fn list_databases(client: &Client) -> Result<Vec<DatabaseInfo>, AppError> {
        let names = client.list_database_names()
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to list databases: {}", e),
            })?;

        let mut databases = Vec::new();
        for name in names {
            databases.push(DatabaseInfo {
                name,
                size_on_disk: None,
                empty: None,
            });
        }
        Ok(databases)
    }

    pub async fn get_database_stats(client: &Client, db_name: &str) -> Result<DatabaseStats, AppError> {
        let db = client.database(db_name);
        let stats = db.run_command(
            doc! { "dbStats": 1 },
        ).await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to get database stats: {}", e),
        })?;

        Ok(DatabaseStats {
            name: stats.get_str("db").unwrap_or("").to_string(),
            collections: stats.get_i64("collections").ok(),
            views: stats.get_i64("views").ok(),
            objects: stats.get_i64("objects").ok(),
            avg_obj_size: stats.get_f64("avgObjSize").ok(),
            data_size: stats.get_i64("dataSize").ok(),
            storage_size: stats.get_i64("storageSize").ok(),
            indexes: stats.get_i64("indexes").ok(),
            index_size: stats.get_i64("indexSize").ok(),
        })
    }

    pub async fn drop_database(client: &Client, db_name: &str) -> Result<(), AppError> {
        let db = client.database(db_name);
        db.drop().await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to drop database: {}", e),
        })?;
        Ok(())
    }
}
