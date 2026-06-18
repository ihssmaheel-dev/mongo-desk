use crate::error::AppError;
use crate::models::document::{DocumentPage, FindParams, InsertParams, UpdateParams, DeleteParams, InsertResult, UpdateResult, DeleteResult};
use crate::state::AppState;
use tauri::State;

async fn get_client(state: &AppState, connection_id: &str) -> Result<mongodb::Client, AppError> {
    let connection_string = {
        let sqlite = state.sqlite.lock();
        crate::services::connection_manager::ConnectionManager::get_connection_string(connection_id, &sqlite)?
    };

    let options = mongodb::options::ClientOptions::parse(&connection_string)
        .await
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))?;
    mongodb::Client::with_options(options)
        .map_err(|e| AppError::connection_refused(Some(e.to_string())))
}

#[tauri::command]
pub async fn find_documents(
    state: State<'_, AppState>,
    params: FindParams,
) -> Result<DocumentPage, AppError> {
    let client = get_client(&state, &params.connection_id).await?;
    crate::services::document::DocumentService::find_documents(
        &client,
        &params.database,
        &params.collection,
        params.filter.as_deref(),
        params.sort.as_deref(),
        params.page.unwrap_or(0),
        params.page_size.unwrap_or(50),
    ).await
}

#[tauri::command]
pub async fn insert_document(
    state: State<'_, AppState>,
    params: InsertParams,
) -> Result<InsertResult, AppError> {
    let client = get_client(&state, &params.connection_id).await?;
    crate::services::document::DocumentService::insert_document(
        &client,
        &params.database,
        &params.collection,
        &params.document,
    ).await
}

#[tauri::command]
pub async fn update_document(
    state: State<'_, AppState>,
    params: UpdateParams,
) -> Result<UpdateResult, AppError> {
    let client = get_client(&state, &params.connection_id).await?;
    crate::services::document::DocumentService::update_document(
        &client,
        &params.database,
        &params.collection,
        &params.id,
        &params.update,
    ).await
}

#[tauri::command]
pub async fn delete_document(
    state: State<'_, AppState>,
    params: DeleteParams,
) -> Result<DeleteResult, AppError> {
    let client = get_client(&state, &params.connection_id).await?;
    crate::services::document::DocumentService::delete_document(
        &client,
        &params.database,
        &params.collection,
        &params.id,
    ).await
}
