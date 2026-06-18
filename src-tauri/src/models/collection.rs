use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub coll_type: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub ns: Option<String>,
    pub count: Option<i64>,
    pub size: Option<i64>,
    pub avg_obj_size: Option<f64>,
    pub storage_size: Option<i64>,
    pub total_index_size: Option<i64>,
    pub total_size: Option<i64>,
    pub indexes: Option<i64>,
}
