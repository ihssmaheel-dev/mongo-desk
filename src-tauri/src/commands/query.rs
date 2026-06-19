use crate::error::AppError;
use crate::models::query::{QueryParams, QueryResult, QueryValidation};
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
pub async fn execute_query(
    state: State<'_, AppState>,
    params: QueryParams,
) -> Result<QueryResult, AppError> {
    let client = get_client(&state, &params.connection_id).await?;
    crate::services::query::QueryService::execute_query(
        &client,
        &params.database,
        &params.collection,
        &params.query_text,
    ).await
}

#[tauri::command]
pub async fn validate_query(
    query_text: String,
) -> Result<QueryValidation, AppError> {
    Ok(crate::services::query::QueryService::validate_query(&query_text))
}
