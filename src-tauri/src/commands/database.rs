use crate::error::AppError;
use crate::models::database::{DatabaseInfo, DatabaseStats};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_databases(
    state: State<'_, AppState>,
    connection_id: String,
) -> Result<Vec<DatabaseInfo>, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    let client = mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

    crate::services::database::DatabaseService::list_databases(&client).await
}

#[tauri::command]
pub async fn get_database_stats(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
) -> Result<DatabaseStats, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    let client = mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

    crate::services::database::DatabaseService::get_database_stats(&client, &database).await
}

#[tauri::command]
pub async fn drop_database(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
) -> Result<(), AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    let client = mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

    crate::services::database::DatabaseService::drop_database(&client, &database).await
}
