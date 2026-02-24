use rusqlite::{params, Connection};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::EnvkeepError;

#[derive(Debug, Clone)]
pub struct Secret {
    pub id: String,
    pub key: String,
    pub encrypted_value: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn create_secret(
    conn: &Connection,
    key: &str,
    encrypted_value: &str,
) -> Result<Secret, EnvkeepError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO secrets (id, key, encrypted_value, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?4)
         ON CONFLICT(key) DO UPDATE SET
           encrypted_value = excluded.encrypted_value,
           updated_at = excluded.updated_at",
        params![id, key, encrypted_value, now],
    )?;

    get_secret(conn, key)
}

pub fn get_secret(conn: &Connection, key: &str) -> Result<Secret, EnvkeepError> {
    conn.query_row(
        "SELECT id, key, encrypted_value, created_at, updated_at
         FROM secrets WHERE key = ?1",
        [key],
        |row| {
            Ok(Secret {
                id: row.get(0)?,
                key: row.get(1)?,
                encrypted_value: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        },
    )
    .map_err(|_| EnvkeepError::SecretNotFound(key.to_string()))
}

pub fn list_secrets(conn: &Connection) -> Result<Vec<Secret>, EnvkeepError> {
    let mut stmt = conn.prepare(
        "SELECT id, key, encrypted_value, created_at, updated_at
         FROM secrets ORDER BY key",
    )?;

    let secrets = stmt
        .query_map([], |row| {
            Ok(Secret {
                id: row.get(0)?,
                key: row.get(1)?,
                encrypted_value: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(secrets)
}

pub fn link_secret(
    conn: &Connection,
    secret_key: &str,
    project_name: &str,
) -> Result<(), EnvkeepError> {
    let secret = get_secret(conn, secret_key)?;
    let project = crate::vault::project::get_project(conn, project_name)?;

    conn.execute(
        "INSERT OR IGNORE INTO secret_links (secret_id, project_id)
         VALUES (?1, ?2)",
        params![secret.id, project.id],
    )?;

    Ok(())
}

pub fn unlink_secret(
    conn: &Connection,
    secret_key: &str,
    project_name: &str,
) -> Result<(), EnvkeepError> {
    let secret = get_secret(conn, secret_key)?;
    let project = crate::vault::project::get_project(conn, project_name)?;

    conn.execute(
        "DELETE FROM secret_links WHERE secret_id = ?1 AND project_id = ?2",
        params![secret.id, project.id],
    )?;

    Ok(())
}

pub fn get_linked_projects(
    conn: &Connection,
    secret_key: &str,
) -> Result<Vec<String>, EnvkeepError> {
    let secret = get_secret(conn, secret_key)?;

    let mut stmt = conn.prepare(
        "SELECT p.name FROM projects p
         JOIN secret_links sl ON p.id = sl.project_id
         WHERE sl.secret_id = ?1
         ORDER BY p.name",
    )?;

    let names = stmt
        .query_map([&secret.id], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    Ok(names)
}