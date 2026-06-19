use mongodb::{Client, bson::Document, bson::Bson, Collection};
use mongodb::options::FindOptions;

use crate::error::AppError;
use crate::models::document::{DocumentPage, InsertResult, UpdateResult, DeleteResult};

pub struct DocumentService;

impl DocumentService {
    fn get_collection(client: &Client, db_name: &str, coll_name: &str) -> Collection<Document> {
        client.database(db_name).collection(coll_name)
    }

    fn convert_oids(doc: &mut Document) {
        for (_, value) in doc.iter_mut() {
            match value {
                Bson::Document(sub_doc) => {
                    if let Some(oid_str) = sub_doc.get_str("$oid").ok() {
                        if let Ok(oid) = mongodb::bson::oid::ObjectId::parse_str(oid_str) {
                            *value = Bson::ObjectId(oid);
                        }
                    } else {
                        Self::convert_oids(sub_doc);
                    }
                }
                Bson::Array(arr) => {
                    for item in arr.iter_mut() {
                        if let Bson::Document(sub_doc) = item {
                            if let Some(oid_str) = sub_doc.get_str("$oid").ok() {
                                if let Ok(oid) = mongodb::bson::oid::ObjectId::parse_str(oid_str) {
                                    *item = Bson::ObjectId(oid);
                                }
                            } else {
                                Self::convert_oids(sub_doc);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub async fn find_documents(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        filter: Option<&str>,
        sort: Option<&str>,
        page: u64,
        page_size: u64,
    ) -> Result<DocumentPage, AppError> {
        let coll = Self::get_collection(client, db_name, coll_name);

        let mut filter_doc = match filter {
            Some(f) if !f.is_empty() => {
                serde_json::from_str::<Document>(f)
                    .map_err(|e| AppError::query_syntax(Some(e.to_string())))?
            }
            _ => Document::new(),
        };

        Self::convert_oids(&mut filter_doc);

        let sort_doc = match sort {
            Some(s) if !s.is_empty() => {
                serde_json::from_str::<Document>(s)
                    .map_err(|e| AppError::query_syntax(Some(e.to_string())))?
            }
            _ => Document::new(),
        };

        let total_count = coll.count_documents(filter_doc.clone())
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to count documents: {}", e),
            })?;

        let skip = page * page_size;
        let mut opts = FindOptions::default();
        opts.skip = Some(skip);
        opts.limit = Some(page_size as i64);
        if !sort_doc.is_empty() {
            opts.sort = Some(sort_doc);
        }

        let mut cursor = coll.find(filter_doc)
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to find documents: {}", e),
            })?;

        let mut documents = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to iterate documents: {}", e),
        })? {
            let raw = cursor.current();
            let doc: Document = bson::from_slice(raw.as_bytes())
                .map_err(|e| AppError::Internal {
                    code: "ERR_INTERNAL".into(),
                    message: format!("Failed to deserialize document: {}", e),
                })?;
            let json_val: serde_json::Value = mongodb::bson::from_bson(
                mongodb::bson::Bson::Document(doc)
            ).unwrap_or(serde_json::Value::Null);
            documents.push(json_val);
        }

        let has_more = skip + page_size < total_count;

        Ok(DocumentPage {
            documents,
            total_count,
            page,
            page_size,
            has_more,
        })
    }

    pub async fn insert_document(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        doc_json: &str,
    ) -> Result<InsertResult, AppError> {
        let coll = Self::get_collection(client, db_name, coll_name);
        let doc: Document = serde_json::from_str(doc_json)
            .map_err(|e| AppError::query_syntax(Some(e.to_string())))?;

        let result = coll.insert_one(doc)
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to insert document: {}", e),
            })?;

        let inserted_id = match result.inserted_id {
            mongodb::bson::Bson::ObjectId(oid) => oid.to_hex(),
            other => other.to_string(),
        };

        Ok(InsertResult { inserted_id })
    }

    pub async fn update_document(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        id: &str,
        update_json: &str,
    ) -> Result<UpdateResult, AppError> {
        let coll = Self::get_collection(client, db_name, coll_name);
        let oid = mongodb::bson::oid::ObjectId::parse_str(id)
            .map_err(|e| AppError::validation("id", &format!("Invalid ObjectId: {}", e)))?;
        let update_doc: Document = serde_json::from_str(update_json)
            .map_err(|e| AppError::query_syntax(Some(e.to_string())))?;

        let result = coll.update_one(
            mongodb::bson::doc! { "_id": oid },
            mongodb::bson::doc! { "$set": update_doc },
        ).await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to update document: {}", e),
        })?;

        Ok(UpdateResult {
            matched_count: result.matched_count,
            modified_count: result.modified_count,
        })
    }

    pub async fn delete_document(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        id: &str,
    ) -> Result<DeleteResult, AppError> {
        let coll = Self::get_collection(client, db_name, coll_name);
        let oid = mongodb::bson::oid::ObjectId::parse_str(id)
            .map_err(|e| AppError::validation("id", &format!("Invalid ObjectId: {}", e)))?;

        let result = coll.delete_one(mongodb::bson::doc! { "_id": oid })
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to delete document: {}", e),
            })?;

        Ok(DeleteResult {
            deleted_count: result.deleted_count,
        })
    }
}
