use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

pub fn handle_migrate() -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    // Check current schema version
    let version: String = conn
        .query_row(
            "SELECT value FROM metadata WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| "0".to_string());

    let current: u32 = version.parse().unwrap_or(0);

    println!("Current schema version: {}", current);

    // Apply migrations based on version
    // For now, version 1 is the only version. Future versions will be added here.
    if current >= 1 {
        println!("{} Schema is up to date (version {}).", "Done.".green().bold(), current);
        return Ok(());
    }

    // Example of a future migration:
    // if current < 2 {
    //     conn.execute_batch("ALTER TABLE projects ADD COLUMN tags TEXT;")?;
    //     conn.execute("UPDATE metadata SET value = '2' WHERE key = 'schema_version'", [])?;
    //     println!("  Applied migration to version 2");
    // }

    Ok(())
}