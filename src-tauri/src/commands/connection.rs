use crate::error::AppError;
use crate::models::connection::{Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult};
use crate::state::AppState;
use tauri::State;
use uuid::Uuid;
use chrono::Utc;

#[tauri::command]
pub async fn add_connection(
    state: State<'_, AppState>,
    connection: NewConnection,
) -> Result<Connection, AppError> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let conn = Connection {
        id,
        name: connection.name,
        connection_string: connection.connection_string,
        conn_type: connection.conn_type,
        color: connection.color,
        read_only: connection.read_only,
        ssl_enabled: connection.ssl_enabled,
        ssl_config: connection.ssl_config,
        group_id: connection.group_id,
        tags: connection.tags,
        created_at: now.clone(),
        updated_at: now,
    };

    Ok(conn)
}

#[tauri::command]
pub async fn update_connection(
    state: State<'_, AppState>,
    id: String,
    connection: UpdateConnection,
) -> Result<Connection, AppError> {
    Err(AppError::Internal {
        code: "ERR_INTERNAL".into(),
        message: "Not implemented yet.".into(),
    })
}

#[tauri::command]
pub async fn delete_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<ConnectionTestResult, AppError> {
    Err(AppError::Internal {
        code: "ERR_INTERNAL".into(),
        message: "Not implemented yet.".into(),
    })
}

#[tauri::command]
pub async fn list_connections(
    state: State<'_, AppState>,
) -> Result<Vec<Connection>, AppError> {
    Ok(vec![])
}

#[tauri::command]
pub async fn list_connection_groups(
    state: State<'_, AppState>,
) -> Result<Vec<ConnectionGroup>, AppError> {
    Ok(vec![])
}

#[tauri::command]
pub async fn add_connection_group(
    state: State<'_, AppState>,
    group: NewConnectionGroup,
) -> Result<ConnectionGroup, AppError> {
    Err(AppError::Internal {
        code: "ERR_INTERNAL".into(),
        message: "Not implemented yet.".into(),
    })
}

#[tauri::command]
pub async fn import_connections(
    state: State<'_, AppState>,
    data: String,
) -> Result<Vec<Connection>, AppError> {
    Ok(vec![])
}

#[tauri::command]
pub async fn export_connections(
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    Ok("[]".to_string())
}
