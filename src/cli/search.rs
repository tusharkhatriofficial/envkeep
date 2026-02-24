use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

pub fn handle_search(key: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let results = vault::variable::search_key(&conn, key)?;

    if results.is_empty() {
        println!("No projects use the key {}.", key.cyan().bold());
        return Ok(());
    }

    println!(
        "Found {} in {} projects:",
        key.cyan().bold(),
        results.len()
    );

    for (project_name, encrypted_value) in &results {
        let decrypted = crypto::decrypt_value(&enc_key, encrypted_value)?;

        // Show a short preview (mask if sensitive)
        let preview = if decrypted.len() > 40 {
            format!("{}...", &decrypted[..40])
        } else {
            decrypted
        };

        println!("  {} {}: {}", "|--".dimmed(), project_name.cyan(), preview);
    }

    Ok(())
}