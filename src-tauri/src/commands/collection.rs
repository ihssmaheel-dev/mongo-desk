use crate::error::AppError;
use crate::models::collection::{CollectionInfo, CollectionStats};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_collections(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
) -> Result<Vec<CollectionInfo>, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    let client = mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

    crate::services::collection::CollectionService::list_collections(&client, &database).await
}

#[tauri::command]
pub async fn get_collection_stats(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
    collection: String,
) -> Result<CollectionStats, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(&connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    let client = mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;

    crate::services::collection::CollectionService::get_collection_stats(&client, &database, &collection).await
}

#[tauri::command]
pub async fn create_collection(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
    name: String,
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

    crate::services::collection::CollectionService::create_collection(&client, &database, &name).await
}

#[tauri::command]
pub async fn drop_collection(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
    collection: String,
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

    crate::services::collection::CollectionService::drop_collection(&client, &database, &collection).await
}

#[tauri::command]
pub async fn rename_collection(
    state: State<'_, AppState>,
    connection_id: String,
    database: String,
    collection: String,
    new_name: String,
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

    crate::services::collection::CollectionService::rename_collection(&client, &database, &collection, &new_name).await
}
