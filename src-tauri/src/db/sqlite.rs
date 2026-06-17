use crate::error::AppError;
use rusqlite::{Connection, params};

pub fn initialize_database(db_path: &std::path::Path) -> Result<Connection, AppError> {
    let conn = Connection::open(db_path)
        .map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to open database.".into(),
        })?;

    conn.execute_batch("PRAGMA journal_mode=WAL;")
        .map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to set journal mode.".into(),
        })?;

    run_migrations(&conn)?;

    Ok(conn)
}

fn run_migrations(conn: &Connection) -> Result<(), AppError> {
    conn.execute_batch(include_str!("../../migrations/001_initial.sql"))
        .map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to run initial migration.".into(),
        })?;

    conn.execute_batch(include_str!("../../migrations/002_history.sql"))
        .map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to run history migration.".into(),
        })?;

    conn.execute_batch(include_str!("../../migrations/003_backups.sql"))
        .map_err(|e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to run backups migration.".into(),
        })?;

    Ok(())
}
