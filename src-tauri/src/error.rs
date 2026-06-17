use serde::Serialize;
use tauri::ipc::InvokeError;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AppError {
    #[serde(rename = "connection")]
    Connection {
        code: String,
        message: String,
        detail: Option<String>,
    },
    #[serde(rename = "query")]
    Query {
        code: String,
        message: String,
        detail: Option<String>,
    },
    #[serde(rename = "auth")]
    Auth {
        code: String,
        message: String,
    },
    #[serde(rename = "validation")]
    Validation {
        code: String,
        message: String,
        field: Option<String>,
    },
    #[serde(rename = "io")]
    Io {
        code: String,
        message: String,
    },
    #[serde(rename = "internal")]
    Internal {
        code: String,
        message: String,
    },
}

impl From<AppError> for InvokeError {
    fn from(err: AppError) -> Self {
        InvokeError::from(serde_json::to_string(&err).unwrap_or_default())
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        if err.is_auth_error() {
            AppError::Auth {
                code: "ERR_CONNECTION_AUTH".into(),
                message: "Authentication failed. Check your username and password.".into(),
            }
        } else if err.is_network_error() {
            AppError::Connection {
                code: "ERR_CONNECTION_REFUSED".into(),
                message: "Connection refused. Verify MongoDB is running.".into(),
                detail: Some(err.to_string()),
            }
        } else {
            AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "An internal error occurred.".into(),
            }
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Database error.".into(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Validation {
            code: "ERR_VALIDATION".into(),
            message: "Invalid JSON.".into(),
            field: None,
        }
    }
}

impl AppError {
    pub fn connection_timeout(detail: Option<String>) -> Self {
        AppError::Connection {
            code: "ERR_CONNECTION_TIMEOUT".into(),
            message: "Connection timed out. Check your network and connection settings.".into(),
            detail,
        }
    }

    pub fn connection_refused(detail: Option<String>) -> Self {
        AppError::Connection {
            code: "ERR_CONNECTION_REFUSED".into(),
            message: "Connection refused. Verify MongoDB is running on localhost:27017.".into(),
            detail,
        }
    }

    pub fn query_syntax(detail: Option<String>) -> Self {
        AppError::Query {
            code: "ERR_QUERY_SYNTAX".into(),
            message: "Query syntax error. Check your query and try again.".into(),
            detail,
        }
    }

    pub fn query_timeout(detail: Option<String>) -> Self {
        AppError::Query {
            code: "ERR_QUERY_TIMEOUT".into(),
            message: "Query timed out. Try adding an index or narrowing your filter.".into(),
            detail,
        }
    }

    pub fn validation(field: &str, message: &str) -> Self {
        AppError::Validation {
            code: "ERR_VALIDATION".into(),
            message: message.into(),
            field: Some(field.into()),
        }
    }
}
