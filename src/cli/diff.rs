use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::BTreeSet;

use crate::crypto;
use crate::vault;

pub fn handle_diff(project1: &str, project2: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let proj1 = vault::project::get_project(&conn, project1)?;
    let proj2 = vault::project::get_project(&conn, project2)?;

    let vars1 = vault::variable::get_variables(&conn, &proj1.id)?;
    let vars2 = vault::variable::get_variables(&conn, &proj2.id)?;

    // Collect keys from both projects
    let keys1: BTreeSet<String> = vars1.iter().map(|v| v.key.clone()).collect();
    let keys2: BTreeSet<String> = vars2.iter().map(|v| v.key.clone()).collect();
    let all_keys: BTreeSet<&String> = keys1.iter().chain(keys2.iter()).collect();

    println!(
        "Diff: {} vs {}",
        project1.cyan().bold(),
        project2.cyan().bold()
    );
    println!();

    let mut only_in_1 = Vec::new();
    let mut only_in_2 = Vec::new();
    let mut different = Vec::new();
    let mut same = Vec::new();

    for key in &all_keys {
        let in_1 = keys1.contains(*key);
        let in_2 = keys2.contains(*key);

        match (in_1, in_2) {
            (true, false) => only_in_1.push(key.to_string()),
            (false, true) => only_in_2.push(key.to_string()),
            (true, true) => {
                // Compare values
                let v1 = vars1.iter().find(|v| &v.key == *key).unwrap();
                let v2 = vars2.iter().find(|v| &v.key == *key).unwrap();

                let dec1 = crypto::decrypt_value(&enc_key, &v1.encrypted_value)?;
                let dec2 = crypto::decrypt_value(&enc_key, &v2.encrypted_value)?;

                if dec1 == dec2 {
                    same.push(key.to_string());
                } else {
                    different.push(key.to_string());
                }
            }
            (false, false) => unreachable!(),
        }
    }

    // Print results
    if !only_in_1.is_empty() {
        println!("  Only in {}:", project1.cyan());
        for key in &only_in_1 {
            println!("    {} {}", "+".green(), key);
        }
        println!();
    }

    if !only_in_2.is_empty() {
        println!("  Only in {}:", project2.cyan());
        for key in &only_in_2 {
            println!("    {} {}", "+".green(), key);
        }
        println!();
    }

    if !different.is_empty() {
        println!("  {} (same key, different value):", "Different".yellow());
        for key in &different {
            println!("    {} {}", "~".yellow(), key);
        }
        println!();
    }

    println!(
        "  Summary: {} same, {} different, {} only in {}, {} only in {}",
        same.len(),
        different.len(),
        only_in_1.len(),
        project1,
        only_in_2.len(),
        project2
    );

    Ok(())
}