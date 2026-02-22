use rusqlite::{params, Connection};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::EnvkeepError;

#[derive(Debug, Clone)]
pub struct Variable {
    pub id: String,
    pub project_id: String,
    pub key: String,
    pub encrypted_value: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Insert or update a variable for a project.
pub fn upsert_variable(
    conn: &Connection,
    project_id: &str,
    key: &str,
    encrypted_value: &str,
) -> Result<(), EnvkeepError> {
    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO variables (id, project_id, key, encrypted_value, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?5)
         ON CONFLICT(project_id, key) DO UPDATE SET
           encrypted_value = excluded.encrypted_value,
           updated_at = excluded.updated_at",
        params![id, project_id, key, encrypted_value, now],
    )?;

    Ok(())
}

/// Get all variables for a project.
pub fn get_variables(
    conn: &Connection,
    project_id: &str,
) -> Result<Vec<Variable>, EnvkeepError> {
    let mut stmt = conn.prepare(
        "SELECT id, project_id, key, encrypted_value, created_at, updated_at
         FROM variables WHERE project_id = ?1 ORDER BY key",
    )?;

    let vars = stmt
        .query_map([project_id], |row| {
            Ok(Variable {
                id: row.get(0)?,
                project_id: row.get(1)?,
                key: row.get(2)?,
                encrypted_value: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(vars)
}

/// Get a single variable by project ID and key name.
pub fn get_variable(
    conn: &Connection,
    project_id: &str,
    key: &str,
) -> Result<Variable, EnvkeepError> {
    conn.query_row(
        "SELECT id, project_id, key, encrypted_value, created_at, updated_at
         FROM variables WHERE project_id = ?1 AND key = ?2",
        params![project_id, key],
        |row| {
            Ok(Variable {
                id: row.get(0)?,
                project_id: row.get(1)?,
                key: row.get(2)?,
                encrypted_value: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        },
    )
    .map_err(|_| EnvkeepError::SecretNotFound(format!("{}:{}", project_id, key)))
}

/// Delete a variable.
pub fn delete_variable(
    conn: &Connection,
    project_id: &str,
    key: &str,
) -> Result<(), EnvkeepError> {
    conn.execute(
        "DELETE FROM variables WHERE project_id = ?1 AND key = ?2",
        params![project_id, key],
    )?;
    Ok(())
}

/// Search for a key across all projects. Returns (project_name, encrypted_value).
pub fn search_key(
    conn: &Connection,
    key: &str,
) -> Result<Vec<(String, String)>, EnvkeepError> {
    let mut stmt = conn.prepare(
        "SELECT p.name, v.encrypted_value
         FROM variables v
         JOIN projects p ON v.project_id = p.id
         WHERE v.key = ?1
         ORDER BY p.name",
    )?;

    let results = stmt
        .query_map([key], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(results)
}