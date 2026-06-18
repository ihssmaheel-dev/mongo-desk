use crate::error::AppError;
use crate::models::connection::{Connection, NewConnection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionTestResult};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn add_connection(
    state: State<'_, AppState>,
    connection: NewConnection,
) -> Result<Connection, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.add_connection(connection, &sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn update_connection(
    state: State<'_, AppState>,
    id: String,
    connection: UpdateConnection,
) -> Result<Connection, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.update_connection(&id, connection, &sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn delete_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    {
        let sqlite = state.sqlite.lock();
        let mut conn_mgr = state.connection_manager.lock();
        conn_mgr.delete_connection(&id, &sqlite)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn test_connection(
    state: State<'_, AppState>,
    id: String,
) -> Result<ConnectionTestResult, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        let _conn_mgr = state.connection_manager.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&id, &sqlite)?
    };
    // SQLite guard dropped, now do async MongoDB test
    crate::services::connection_manager::ConnectionManager::test_connection_string(&connection_string).await
}

#[tauri::command]
pub async fn list_connections(
    state: State<'_, AppState>,
) -> Result<Vec<Connection>, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.list_connections(&sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn list_connection_groups(
    state: State<'_, AppState>,
) -> Result<Vec<ConnectionGroup>, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.list_groups(&sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn add_connection_group(
    state: State<'_, AppState>,
    group: NewConnectionGroup,
) -> Result<ConnectionGroup, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.add_group(group, &sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn import_connections(
    state: State<'_, AppState>,
    data: String,
) -> Result<Vec<Connection>, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.import_connections(&data, &sqlite)?
    };
    Ok(result)
}

#[tauri::command]
pub async fn export_connections(
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let result = {
        let sqlite = state.sqlite.lock();
        let conn_mgr = state.connection_manager.lock();
        conn_mgr.export_connections(&sqlite)?
    };
    Ok(result)
}
