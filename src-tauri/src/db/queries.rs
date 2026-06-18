use rusqlite::{Connection as SqliteConn, params, Result};
use uuid::Uuid;
use chrono::Utc;

use crate::error::AppError;
use crate::models::connection::{Connection, UpdateConnection, ConnectionGroup, NewConnectionGroup, ConnectionFavorite};

#[allow(dead_code)]
pub struct Queries;

impl Queries {
    pub fn insert_connection(conn: &Connection, sqlite: &SqliteConn) -> Result<Connection, AppError> {
        sqlite.execute(
            "INSERT INTO connections (id, name, connection_string, type, color, read_only, ssl_enabled, ssl_config, group_id, tags, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                conn.id,
                conn.name,
                conn.connection_string,
                conn.conn_type,
                conn.color,
                conn.read_only as i32,
                conn.ssl_enabled as i32,
                conn.ssl_config,
                conn.group_id,
                conn.tags,
                conn.created_at,
                conn.updated_at,
            ],
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to insert connection.".into(),
        })?;

        Ok(conn.clone())
    }

    pub fn get_connection(id: &str, sqlite: &SqliteConn) -> Result<Connection, AppError> {
        sqlite.query_row(
            "SELECT id, name, connection_string, type, color, read_only, ssl_enabled, ssl_config, group_id, tags, created_at, updated_at
             FROM connections WHERE id = ?1",
            params![id],
            |row| {
                Ok(Connection {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    connection_string: row.get(2)?,
                    conn_type: row.get(3)?,
                    color: row.get(4)?,
                    read_only: row.get::<_, i32>(5)? != 0,
                    ssl_enabled: row.get::<_, i32>(6)? != 0,
                    ssl_config: row.get(7)?,
                    group_id: row.get(8)?,
                    tags: row.get(9)?,
                    created_at: row.get(10)?,
                    updated_at: row.get(11)?,
                })
            },
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Connection not found.".into(),
        })
    }

    pub fn list_connections(sqlite: &SqliteConn) -> Result<Vec<Connection>, AppError> {
        let mut stmt = sqlite.prepare(
            "SELECT id, name, connection_string, type, color, read_only, ssl_enabled, ssl_config, group_id, tags, created_at, updated_at
             FROM connections ORDER BY name"
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list connections.".into(),
        })?;

        let connections = stmt.query_map([], |row| {
            Ok(Connection {
                id: row.get(0)?,
                name: row.get(1)?,
                connection_string: row.get(2)?,
                conn_type: row.get(3)?,
                color: row.get(4)?,
                read_only: row.get::<_, i32>(5)? != 0,
                ssl_enabled: row.get::<_, i32>(6)? != 0,
                ssl_config: row.get(7)?,
                group_id: row.get(8)?,
                tags: row.get(9)?,
                created_at: row.get(10)?,
                updated_at: row.get(11)?,
            })
        }).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list connections.".into(),
        })?.collect::<Result<Vec<_>>>().map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to collect connections.".into(),
        })?;

        Ok(connections)
    }

    pub fn update_connection(id: &str, update: &UpdateConnection, sqlite: &SqliteConn) -> Result<Connection, AppError> {
        let mut conn = Self::get_connection(id, sqlite)?;

        if let Some(name) = &update.name {
            conn.name = name.clone();
        }
        if let Some(cs) = &update.connection_string {
            conn.connection_string = cs.clone();
        }
        if let Some(ct) = &update.conn_type {
            conn.conn_type = ct.clone();
        }
        if let Some(color) = &update.color {
            conn.color = Some(color.clone());
        }
        if let Some(ro) = update.read_only {
            conn.read_only = ro;
        }
        if let Some(ssl) = update.ssl_enabled {
            conn.ssl_enabled = ssl;
        }
        if let Some(ssl_config) = &update.ssl_config {
            conn.ssl_config = Some(ssl_config.clone());
        }
        if let Some(group_id) = &update.group_id {
            conn.group_id = Some(group_id.clone());
        }
        if let Some(tags) = &update.tags {
            conn.tags = Some(tags.clone());
        }

        conn.updated_at = Utc::now().to_rfc3339();

        sqlite.execute(
            "UPDATE connections SET name = ?1, connection_string = ?2, type = ?3, color = ?4, read_only = ?5, ssl_enabled = ?6, ssl_config = ?7, group_id = ?8, tags = ?9, updated_at = ?10
             WHERE id = ?11",
            params![
                conn.name,
                conn.connection_string,
                conn.conn_type,
                conn.color,
                conn.read_only as i32,
                conn.ssl_enabled as i32,
                conn.ssl_config,
                conn.group_id,
                conn.tags,
                conn.updated_at,
                conn.id,
            ],
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to update connection.".into(),
        })?;

        Ok(conn)
    }

    pub fn delete_connection(id: &str, sqlite: &SqliteConn) -> Result<(), AppError> {
        sqlite.execute("DELETE FROM connections WHERE id = ?1", params![id])
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to delete connection.".into(),
            })?;
        Ok(())
    }

    pub fn list_groups(sqlite: &SqliteConn) -> Result<Vec<ConnectionGroup>, AppError> {
        let mut stmt = sqlite.prepare(
            "SELECT id, name, parent_id, sort_order, created_at FROM connection_groups ORDER BY sort_order, name"
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list groups.".into(),
        })?;

        let groups = stmt.query_map([], |row| {
            Ok(ConnectionGroup {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
                sort_order: row.get(3)?,
                created_at: row.get(4)?,
            })
        }).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list groups.".into(),
        })?.collect::<Result<Vec<_>>>().map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to collect groups.".into(),
        })?;

        Ok(groups)
    }

    pub fn insert_group(group: &NewConnectionGroup, sqlite: &SqliteConn) -> Result<ConnectionGroup, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlite.execute(
            "INSERT INTO connection_groups (id, name, parent_id, sort_order, created_at) VALUES (?1, ?2, ?3, 0, ?4)",
            params![id, group.name, group.parent_id, now],
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to insert group.".into(),
        })?;

        Ok(ConnectionGroup {
            id,
            name: group.name.clone(),
            parent_id: group.parent_id.clone(),
            sort_order: 0,
            created_at: now,
        })
    }

    pub fn delete_group(id: &str, sqlite: &SqliteConn) -> Result<(), AppError> {
        sqlite.execute("DELETE FROM connection_groups WHERE id = ?1", params![id])
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to delete group.".into(),
            })?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn list_favorites(sqlite: &SqliteConn) -> Result<Vec<ConnectionFavorite>, AppError> {
        let mut stmt = sqlite.prepare(
            "SELECT id, connection_id, sort_order, created_at FROM connection_favorites ORDER BY sort_order"
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list favorites.".into(),
        })?;

        let favorites = stmt.query_map([], |row| {
            Ok(ConnectionFavorite {
                id: row.get(0)?,
                connection_id: row.get(1)?,
                sort_order: row.get(2)?,
                created_at: row.get(3)?,
            })
        }).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to list favorites.".into(),
        })?.collect::<Result<Vec<_>>>().map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to collect favorites.".into(),
        })?;

        Ok(favorites)
    }

    #[allow(dead_code)]
    pub fn add_favorite(connection_id: &str, sqlite: &SqliteConn) -> Result<ConnectionFavorite, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        sqlite.execute(
            "INSERT INTO connection_favorites (id, connection_id, sort_order, created_at) VALUES (?1, ?2, 0, ?3)",
            params![id, connection_id, now],
        ).map_err(|_e| AppError::Internal {
            code: "ERR_INTERNAL".into(),
            message: "Failed to add favorite.".into(),
        })?;

        Ok(ConnectionFavorite {
            id,
            connection_id: connection_id.to_string(),
            sort_order: 0,
            created_at: now,
        })
    }

    #[allow(dead_code)]
    pub fn remove_favorite(id: &str, sqlite: &SqliteConn) -> Result<(), AppError> {
        sqlite.execute("DELETE FROM connection_favorites WHERE id = ?1", params![id])
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to remove favorite.".into(),
            })?;
        Ok(())
    }
}
