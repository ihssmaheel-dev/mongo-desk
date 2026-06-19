use tauri::Manager;

mod commands;
mod db;
mod error;
mod models;
mod services;
mod state;

pub use error::AppError;
pub use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_state = AppState::new()?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connection::add_connection,
            commands::connection::update_connection,
            commands::connection::delete_connection,
            commands::connection::test_connection,
            commands::connection::list_connections,
            commands::connection::list_connection_groups,
            commands::connection::add_connection_group,
            commands::connection::import_connections,
            commands::connection::export_connections,
            commands::database::list_databases,
            commands::database::get_database_stats,
            commands::database::drop_database,
            commands::collection::list_collections,
            commands::collection::get_collection_stats,
            commands::collection::create_collection,
            commands::collection::drop_collection,
            commands::collection::rename_collection,
            commands::document::find_documents,
            commands::document::insert_document,
            commands::document::update_document,
            commands::document::delete_document,
            commands::document::get_collection_fields,
            commands::query::execute_query,
            commands::query::validate_query,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
