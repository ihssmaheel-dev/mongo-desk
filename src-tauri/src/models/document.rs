use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPage {
    pub documents: Vec<serde_json::Value>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindParams {
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertParams {
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub document: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateParams {
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub id: String,
    pub update: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteParams {
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertResult {
    pub inserted_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub matched_count: u64,
    pub modified_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResult {
    pub deleted_count: u64,
}
