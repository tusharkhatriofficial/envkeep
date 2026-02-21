use rusqlite::{params, Connection};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::DotkeepError;

#[derive(Debug, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub directory: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_used_at: Option<String>,
}

impl Project {
    pub fn new(name: &str, directory: Option<&str>) -> Self {
        let now = Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            directory: directory.map(|d| d.to_string()),
            created_at: now.clone(),
            updated_at: now,
            last_used_at: None,
        }
    }
}

/// Insert a new project into the vault.
pub fn create_project(conn: &Connection, project: &Project) -> Result<(), DotkeepError> {
    conn.execute(
        "INSERT INTO projects (id, name, directory, created_at, updated_at, last_used_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            project.id,
            project.name,
            project.directory,
            project.created_at,
            project.updated_at,
            project.last_used_at,
        ],
    )
    .map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            DotkeepError::ProjectAlreadyExists(project.name.clone())
        } else {
            DotkeepError::DatabaseError(e)
        }
    })?;
    Ok(())
}

/// Get a project by name.
pub fn get_project(conn: &Connection, name: &str) -> Result<Project, DotkeepError> {
    conn.query_row(
        "SELECT id, name, directory, created_at, updated_at, last_used_at
         FROM projects WHERE name = ?1",
        [name],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                directory: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                last_used_at: row.get(5)?,
            })
        },
    )
    .map_err(|_| DotkeepError::ProjectNotFound(name.to_string()))
}

/// List all projects, ordered by last used (most recent first).
pub fn list_projects(conn: &Connection) -> Result<Vec<Project>, DotkeepError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, directory, created_at, updated_at, last_used_at
         FROM projects ORDER BY COALESCE(last_used_at, updated_at) DESC",
    )?;

    let projects = stmt
        .query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                directory: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                last_used_at: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(projects)
}

/// Delete a project and all its variables (CASCADE).
pub fn delete_project(conn: &Connection, name: &str) -> Result<(), DotkeepError> {
    let project = get_project(conn, name)?;

    conn.execute("DELETE FROM variables WHERE project_id = ?1", [&project.id])?;
    conn.execute("DELETE FROM secret_links WHERE project_id = ?1", [&project.id])?;
    conn.execute("DELETE FROM projects WHERE id = ?1", [&project.id])?;

    Ok(())
}

/// Update the last_used_at timestamp for a project.
pub fn touch_project(conn: &Connection, name: &str) -> Result<(), DotkeepError> {
    let now = Utc::now().to_rfc3339();
    let rows = conn.execute(
        "UPDATE projects SET last_used_at = ?1, updated_at = ?1 WHERE name = ?2",
        params![now, name],
    )?;

    if rows == 0 {
        return Err(DotkeepError::ProjectNotFound(name.to_string()));
    }

    Ok(())
}

/// Count the number of variables in a project.
pub fn count_variables(conn: &Connection, project_id: &str) -> Result<u32, DotkeepError> {
    let count: u32 = conn.query_row(
        "SELECT COUNT(*) FROM variables WHERE project_id = ?1",
        [project_id],
        |row| row.get(0),
    )?;
    Ok(count)
}
