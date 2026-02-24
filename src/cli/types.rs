use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};

use crate::crypto;
use crate::vault;

pub fn handle_types(project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let project = vault::project::get_project(&conn, project_name)?;
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!("Project {} has no variables.", project_name.cyan());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_header(vec!["Key", "Inferred Type", "Example"]);

    for var in &variables {
        let value = crypto::decrypt_value(&enc_key, &var.encrypted_value)?;
        let (var_type, example) = infer_type(&value);

        table.add_row(vec![var.key.clone(), var_type.to_string(), example]);
    }

    println!(
        "Type inference for {} ({} variables):",
        project_name.cyan().bold(),
        variables.len()
    );
    println!();
    println!("{table}");

    Ok(())
}

#[derive(Debug)]
enum VarType {
    Boolean,
    Integer,
    Float,
    Url,
    Email,
    FilePath,
    IpAddress,
    Json,
    String,
}

impl std::fmt::Display for VarType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarType::Boolean => write!(f, "boolean"),
            VarType::Integer => write!(f, "integer"),
            VarType::Float => write!(f, "float"),
            VarType::Url => write!(f, "url"),
            VarType::Email => write!(f, "email"),
            VarType::FilePath => write!(f, "file_path"),
            VarType::IpAddress => write!(f, "ip_address"),
            VarType::Json => write!(f, "json"),
            VarType::String => write!(f, "string"),
        }
    }
}

fn infer_type(value: &str) -> (VarType, String) {
    // Boolean
    let booleans = ["true", "false", "yes", "no", "1", "0"];
    if booleans.contains(&value.to_lowercase().as_str()) {
        return (VarType::Boolean, value.to_string());
    }

    // Integer
    if value.parse::<i64>().is_ok() && !value.contains('.') {
        return (VarType::Integer, value.to_string());
    }

    // Float
    if value.parse::<f64>().is_ok() {
        return (VarType::Float, value.to_string());
    }

    // URL
    if value.contains("://") {
        let short = if value.len() > 30 {
            format!("{}...", &value[..30])
        } else {
            value.to_string()
        };
        return (VarType::Url, short);
    }

    // Email
    if value.contains('@') && value.contains('.') && !value.contains(' ') {
        return (VarType::Email, value.to_string());
    }

    // File path
    if value.starts_with('/') || value.starts_with("./") || value.starts_with("~/") {
        return (VarType::FilePath, value.to_string());
    }

    // IP address (simple check)
    let parts: Vec<&str> = value.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return (VarType::IpAddress, value.to_string());
    }

    // JSON
    if (value.starts_with('{') && value.ends_with('}'))
        || (value.starts_with('[') && value.ends_with(']'))
    {
        return (VarType::Json, "(json object)".to_string());
    }

    // Default: String
    let short = if value.len() > 20 {
        format!("{}...", &value[..20])
    } else {
        value.to_string()
    };
    (VarType::String, short)
}