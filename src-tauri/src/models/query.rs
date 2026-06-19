use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub query_text: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub documents: Vec<serde_json::Value>,
    pub total_count: u64,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryValidation {
    pub is_valid: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryHistoryEntry {
    pub id: String,
    pub connection_id: String,
    pub database: String,
    pub collection: String,
    pub query_text: String,
    pub execution_time_ms: Option<u64>,
    pub result_count: Option<u64>,
    pub is_favorite: bool,
    pub created_at: String,
}
