use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

pub fn handle_sync(from_name: &str, to_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let from_project = vault::project::get_project(&conn, from_name)?;
    let to_project = vault::project::get_project(&conn, to_name)?;

    let from_vars = vault::variable::get_variables(&conn, &from_project.id)?;
    let to_vars = vault::variable::get_variables(&conn, &to_project.id)?;

    // Find common keys that exist in both projects
    let to_keys: std::collections::HashSet<String> =
        to_vars.iter().map(|v| v.key.clone()).collect();

    // Common env var prefixes often shared between projects
    let common_prefixes = [
        "DATABASE", "DB_", "REDIS", "SMTP", "MAIL", "AWS_", "S3_",
        "STRIPE", "SENTRY", "LOG_", "APP_",
    ];

    let mut synced = 0;
    let mut skipped = 0;

    for var in &from_vars {
        // Only sync vars with common prefixes OR that already exist in target
        let is_common = common_prefixes
            .iter()
            .any(|prefix| var.key.starts_with(prefix));

        if !is_common && !to_keys.contains(&var.key) {
            continue;
        }

        // Copy the variable (preserving encryption)
        vault::variable::upsert_variable(
            &conn,
            &to_project.id,
            &var.key,
            &var.encrypted_value,
        )?;

        if to_keys.contains(&var.key) {
            skipped += 1; // Updated existing
        } else {
            synced += 1; // Added new
        }
    }

    println!(
        "{} Synced {} -> {}: {} new, {} updated",
        "Done.".green().bold(),
        from_name.cyan(),
        to_name.cyan(),
        synced,
        skipped
    );

    Ok(())
}