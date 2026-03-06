use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::crypto;
use crate::vault;

pub fn handle_restore(file_path: &str) -> Result<()> {
    let backup_path = Path::new(file_path);

    if !backup_path.exists() {
        println!("{} Backup file not found: {}", "Error:".red(), file_path);
        return Ok(());
    }

    // Verify the backup is a valid SQLCipher database by trying to open it
    println!("Verifying backup file...");
    let password = crypto::prompt_password("Master password for backup: ")?;

    // Try opening the backup with the given password
    let backup_conn = rusqlite::Connection::open(backup_path)?;
    backup_conn.pragma_update(None, "key", &password)?;
    backup_conn
        .execute_batch("SELECT count(*) FROM sqlite_master;")
        .map_err(|_| {
            anyhow::anyhow!("Wrong password or invalid backup file")
        })?;

    // Count projects in backup
    let project_count: u32 = backup_conn
        .query_row("SELECT COUNT(*) FROM projects", [], |row| row.get(0))
        .unwrap_or(0);

    drop(backup_conn);

    // Confirm overwrite
    let vault_path = vault::vault_path();
    if vault_path.exists() {
        println!();
        println!(
            "{} This will REPLACE your current vault with the backup.",
            "WARNING:".red().bold()
        );
        println!("  Backup contains {} projects.", project_count);
        print!("  Continue? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            println!("Cancelled.");
            return Ok(());
        }

        // Create a safety backup of the current vault
        let safety = vault_path.with_extension("db.bak");
        fs::copy(&vault_path, &safety)
            .context("Failed to create safety backup")?;
        println!("  Current vault saved to {}", safety.display().to_string().dimmed());
    }

    // Ensure the directory exists
    let dir = vault::data_dir();
    fs::create_dir_all(&dir)?;

    // Copy backup to vault location
    fs::copy(backup_path, &vault_path)
        .context("Failed to restore backup")?;

    println!(
        "{} Vault restored from {} ({} projects)",
        "Done.".green().bold(),
        file_path.bold(),
        project_count
    );

    Ok(())
}