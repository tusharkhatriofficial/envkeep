use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};
use ring::rand::{SecureRandom, SystemRandom};

use crate::cli::SecretsAction;
use crate::crypto;
use crate::errors::EnvkeepError;
use crate::vault;
use crate::vault::secret;

pub fn handle_secrets(action: SecretsAction) -> Result<()> {
    match action {
        SecretsAction::Set { pair } => handle_set(&pair),
        SecretsAction::List => handle_list(),
        SecretsAction::Link { secret, project } => handle_link(&secret, &project),
        SecretsAction::Unlink { secret, project } => handle_unlink(&secret, &project),
        SecretsAction::Rotate { secret } => handle_rotate(&secret),
    }
}

fn handle_set(pair: &str) -> Result<()> {
    let (key, value) = pair
        .split_once('=')
        .ok_or_else(|| EnvkeepError::InvalidKeyValue(pair.to_string()))?;

    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let encrypted = crypto::encrypt_value(&enc_key, value)?;
    secret::create_secret(&conn, key, &encrypted)?;

    println!("{} Secret {} stored.", "Done.".green().bold(), (*key).cyan());

    Ok(())
}

fn handle_list() -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    let secrets = secret::list_secrets(&conn)?;

    if secrets.is_empty() {
        println!("No secrets stored.");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_header(vec!["Key", "Linked Projects", "Updated"]);

    for s in &secrets {
        let projects = secret::get_linked_projects(&conn, &s.key)?;
        let project_list = if projects.is_empty() {
            "(none)".to_string()
        } else {
            projects.join(", ")
        };

        table.add_row(vec![s.key.clone(), project_list, s.updated_at.clone()]);
    }

    println!("{table}");

    Ok(())
}

fn handle_link(secret_key: &str, project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    secret::link_secret(&conn, secret_key, project_name)?;

    println!(
        "{} Linked {} to {}",
        "Done.".green().bold(),
        secret_key.cyan(),
        project_name.cyan()
    );

    Ok(())
}

fn handle_unlink(secret_key: &str, project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;

    secret::unlink_secret(&conn, secret_key, project_name)?;

    println!(
        "{} Unlinked {} from {}",
        "Done.".green().bold(),
        secret_key.cyan(),
        project_name.cyan()
    );

    Ok(())
}

fn handle_rotate(secret_key: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    // Verify the secret exists
    secret::get_secret(&conn, secret_key)?;

    // Generate a new random value (32 bytes, hex-encoded)
    let rng = SystemRandom::new();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes)
        .map_err(|_| EnvkeepError::EncryptionError("RNG failed".to_string()))?;
    let new_value: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();

    let encrypted = crypto::encrypt_value(&enc_key, &new_value)?;
    secret::create_secret(&conn, secret_key, &encrypted)?;

    println!(
        "{} Rotated {}. New value: {}",
        "Done.".green().bold(),
        secret_key.cyan(),
        new_value.dimmed()
    );
    println!("{}", "Copy this value now -- it will not be shown again.".yellow());

    Ok(())
}