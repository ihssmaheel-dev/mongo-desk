use mongodb::{Client, bson::Document};
use std::time::Instant;

use crate::error::AppError;
use crate::models::query::{QueryResult, QueryValidation};

pub struct QueryService;

impl QueryService {
    pub async fn execute_query(
        client: &Client,
        db_name: &str,
        coll_name: &str,
        query_text: &str,
    ) -> Result<QueryResult, AppError> {
        let start = Instant::now();
        let coll = client.database(db_name).collection::<Document>(coll_name);

        let query_doc: Document = serde_json::from_str(query_text)
            .map_err(|e| AppError::query_syntax(Some(e.to_string())))?;

        let filter = query_doc.get_document("filter").cloned().unwrap_or_default();
        let sort = query_doc.get_document("sort").cloned().unwrap_or_default();
        let projection = query_doc.get_document("projection").ok().cloned();
        let limit = query_doc.get_i64("limit").unwrap_or(100);
        let skip = query_doc.get_i64("skip").unwrap_or(0);

        let total_count = coll.count_documents(filter.clone())
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to count: {}", e),
            })?;

        let mut opts = mongodb::options::FindOptions::default();
        opts.limit = Some(limit);
        opts.skip = Some(skip as u64);
        if !sort.is_empty() {
            opts.sort = Some(sort);
        }
        if let Some(proj) = projection {
            opts.projection = Some(proj);
        }

        let mut cursor = coll.find(filter)
            .await
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: format!("Failed to execute query: {}", e),
            })?;

        let mut documents = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: format!("Failed to iterate results: {}", e),
        })? {
            let raw = cursor.current();
            let doc: Document = bson::from_slice(raw.as_bytes())
                .map_err(|e| AppError::Internal {
                    code: "ERR_INTERNAL".into(),
                    message: format!("Failed to deserialize: {}", e),
                })?;
            let json_val: serde_json::Value = mongodb::bson::from_bson(
                mongodb::bson::Bson::Document(doc)
            ).unwrap_or(serde_json::Value::Null);
            documents.push(json_val);
        }

        let execution_time_ms = start.elapsed().as_millis() as u64;

        Ok(QueryResult {
            documents,
            total_count,
            execution_time_ms,
        })
    }

    pub fn validate_query(query_text: &str) -> QueryValidation {
        if query_text.trim().is_empty() {
            return QueryValidation {
                is_valid: false,
                error_message: Some("Query cannot be empty".to_string()),
            };
        }

        match serde_json::from_str::<Document>(query_text) {
            Ok(_) => QueryValidation {
                is_valid: true,
                error_message: None,
            },
            Err(e) => QueryValidation {
                is_valid: false,
                error_message: Some(format!("Invalid JSON: {}", e)),
            },
        }
    }
}
