use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Table, ContentArrangement, presets::UTF8_FULL_CONDENSED};

use crate::crypto;
use crate::vault;

pub fn handle_inspect(name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let project = vault::project::get_project(&conn, name)?;
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!("Project {} has no variables.", name.cyan());
        return Ok(());
    }

    println!(
        "Project: {} ({} variables)",
        name.cyan().bold(),
        variables.len()
    );
    if let Some(dir) = &project.directory {
        println!("Directory: {}", dir);
    }
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Key", "Value"]);

    for var in &variables {
        let decrypted = crypto::decrypt_value(&enc_key, &var.encrypted_value)
            .context(format!("Failed to decrypt: {}", var.key))?;

        let masked = mask_value(&var.key, &decrypted);
        table.add_row(vec![var.key.clone(), masked]);
    }

    println!("{table}");

    Ok(())
}

/// Mask sensitive values while keeping non-sensitive ones readable.
///
/// Rules:
/// - If the key contains SECRET, KEY, PASSWORD, TOKEN, or AUTH: fully masked.
/// - If the value looks like a URL: show protocol + host, mask the rest.
/// - If the value is short (<=4 chars): show as-is (likely booleans/ports).
/// - Otherwise: show first 4 chars + mask.
fn mask_value(key: &str, value: &str) -> String {
    let key_upper = key.to_uppercase();
    let sensitive_keywords = ["SECRET", "KEY", "PASSWORD", "TOKEN", "AUTH", "PRIVATE"];

    // Check if the key suggests a sensitive value
    let is_sensitive = sensitive_keywords
        .iter()
        .any(|kw| key_upper.contains(kw));

    if is_sensitive {
        return "********".to_string();
    }

    // Short values (booleans, ports) -- show as-is
    if value.len() <= 4 {
        return value.to_string();
    }

    // URLs -- show protocol and host
    if value.contains("://") {
        if let Some(protocol_end) = value.find("://") {
            let after_protocol = &value[protocol_end + 3..];
            if let Some(host_end) = after_protocol.find('/') {
                let host = &after_protocol[..host_end];
                return format!("{}://{}/*****", &value[..protocol_end], host);
            } else {
                return format!("{}://{}/*****", &value[..protocol_end], after_protocol);
            }
        }
    }

    // Default: show first 4 characters
    format!("{}****", &value[..4])
}