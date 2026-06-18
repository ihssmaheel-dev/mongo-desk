use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub connection_string: String,
    #[serde(rename = "type")]
    pub conn_type: String,
    pub color: Option<String>,
    pub read_only: bool,
    pub ssl_enabled: bool,
    pub ssl_config: Option<String>,
    pub group_id: Option<String>,
    pub tags: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConnection {
    pub name: String,
    pub connection_string: String,
    #[serde(rename = "type")]
    pub conn_type: String,
    pub color: Option<String>,
    pub read_only: bool,
    pub ssl_enabled: bool,
    pub ssl_config: Option<String>,
    pub group_id: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConnection {
    pub name: Option<String>,
    pub connection_string: Option<String>,
    #[serde(rename = "type")]
    pub conn_type: Option<String>,
    pub color: Option<String>,
    pub read_only: Option<bool>,
    pub ssl_enabled: Option<bool>,
    pub ssl_config: Option<String>,
    pub group_id: Option<String>,
    pub tags: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionGroup {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConnectionGroup {
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionTestResult {
    pub success: bool,
    pub message: String,
    pub databases: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionFavorite {
    pub id: String,
    pub connection_id: String,
    pub sort_order: i32,
    pub created_at: String,
}
