use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub size_on_disk: Option<i64>,
    pub empty: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub name: String,
    pub collections: Option<i64>,
    pub views: Option<i64>,
    pub objects: Option<i64>,
    pub avg_obj_size: Option<f64>,
    pub data_size: Option<i64>,
    pub storage_size: Option<i64>,
    pub indexes: Option<i64>,
    pub index_size: Option<i64>,
}
