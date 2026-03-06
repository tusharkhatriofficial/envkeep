use anyhow::{Context, Result};
use colored::Colorize;
use chrono::Utc;
use std::fs;

use crate::vault;

pub fn handle_backup() -> Result<()> {
    let vault_path = vault::vault_path();

    if !vault_path.exists() {
        println!("{} No vault found. Run {} first.", "Error:".red(), "envkeep init".cyan());
        return Ok(());
    }

    // Generate backup filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("envkeep_backup_{}.db", timestamp);

    // Copy the entire vault file
    // Since the database is already encrypted with SQLCipher, the backup
    // is also encrypted. The user needs their master password to restore.
    fs::copy(&vault_path, &backup_name)
        .context("Failed to create backup")?;

    let file_size = fs::metadata(&backup_name)
        .map(|m| m.len())
        .unwrap_or(0);

    println!(
        "{} Vault backed up to {}",
        "Done.".green().bold(),
        backup_name.bold()
    );
    println!("  Size: {} KB", file_size / 1024);
    println!("  {}", "Keep this file safe. It is encrypted with your master password.".dimmed());

    Ok(())
}