use mongodb::{Client, bson::doc};

use crate::error::AppError;
use crate::models::collection::{CollectionInfo, CollectionStats};

pub struct CollectionService;

impl CollectionService {
    pub async fn list_collections(client: &Client, db_name: &str) -> Result<Vec<CollectionInfo>, AppError> {
        let db = client.database(db_name);
        let mut cursor = db.list_collections()
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to list collections: {}", e),
            })?;

        let mut collections = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to iterate collections: {}", e),
        })? {
            let result = cursor.current();
            let name = result.get_str("name").unwrap_or("").to_string();
            let coll_type = result.get_str("type").ok().map(|s| s.to_string());
            let options = result.get_document("options").ok().and_then(|d| {
                serde_json::to_value(d).ok()
            });

            collections.push(CollectionInfo {
                name,
                coll_type,
                options,
            });
        }

        Ok(collections)
    }

    pub async fn get_collection_stats(client: &Client, db_name: &str, coll_name: &str) -> Result<CollectionStats, AppError> {
        let db = client.database(db_name);
        let stats = db.run_command(
            doc! { "collStats": coll_name },
        ).await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to get collection stats: {}", e),
        })?;

        Ok(CollectionStats {
            ns: stats.get_str("ns").ok().map(|s| s.to_string()),
            count: stats.get_i64("count").ok(),
            size: stats.get_i64("size").ok(),
            avg_obj_size: stats.get_f64("avgObjSize").ok(),
            storage_size: stats.get_i64("storageSize").ok(),
            total_index_size: stats.get_i64("totalIndexSize").ok(),
            total_size: stats.get_i64("totalSize").ok(),
            indexes: stats.get_i64("nindexes").ok(),
        })
    }

    pub async fn create_collection(client: &Client, db_name: &str, coll_name: &str) -> Result<(), AppError> {
        let db = client.database(db_name);
        db.create_collection(coll_name)
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to create collection: {}", e),
            })?;
        Ok(())
    }

    pub async fn drop_collection(client: &Client, db_name: &str, coll_name: &str) -> Result<(), AppError> {
        let db = client.database(db_name);
        let coll = db.collection::<mongodb::bson::Document>(coll_name);
        coll.drop().await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to drop collection: {}", e),
        })?;
        Ok(())
    }

    pub async fn rename_collection(client: &Client, db_name: &str, old_name: &str, new_name: &str) -> Result<(), AppError> {
        let db = client.database(db_name);
        let from = format!("{}.{}", db_name, old_name);
        let to = format!("{}.{}", db_name, new_name);

        db.run_command(
            doc! {
                "renameCollection": from,
                "to": to,
            },
        ).await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to rename collection: {}", e),
        })?;
        Ok(())
    }
}
