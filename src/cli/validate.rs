use anyhow::{Context, Result};
use colored::Colorize;

use crate::crypto;
use crate::vault;

#[derive(Debug)]
struct ValidationIssue {
    key: String,
    severity: Severity,
    message: String,
}

#[derive(Debug)]
enum Severity {
    Error,
    Warning,
}

pub fn handle_validate(project_name: &str) -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)?;
    let enc_key = vault::get_encryption_key(&conn, &password)?;

    let project = vault::project::get_project(&conn, project_name)?;
    let variables = vault::variable::get_variables(&conn, &project.id)?;

    if variables.is_empty() {
        println!("Project {} has no variables.", project_name.cyan());
        return Ok(());
    }

    let mut issues = Vec::new();

    for var in &variables {
        let value = crypto::decrypt_value(&enc_key, &var.encrypted_value)?;
        validate_variable(&var.key, &value, &mut issues);
    }

    if issues.is_empty() {
        println!(
            "{} All {} variables passed validation.",
            "Done.".green().bold(),
            variables.len()
        );
        return Ok(());
    }

    println!(
        "Validation results for {} ({} issues):",
        project_name.cyan().bold(),
        issues.len()
    );
    println!();

    for issue in &issues {
        let icon = match issue.severity {
            Severity::Error => "ERROR".red().bold(),
            Severity::Warning => "WARN ".yellow().bold(),
        };
        println!("  [{}] {}: {}", icon, issue.key.cyan(), issue.message);
    }

    Ok(())
}

fn validate_variable(key: &str, value: &str, issues: &mut Vec<ValidationIssue>) {
    let key_upper = key.to_uppercase();

    // Check for empty values
    if value.is_empty() {
        issues.push(ValidationIssue {
            key: key.to_string(),
            severity: Severity::Warning,
            message: "Value is empty".to_string(),
        });
        return;
    }

    // Port validation
    if key_upper.contains("PORT") {
        if let Ok(port) = value.parse::<u32>() {
            if port == 0 || port > 65535 {
                issues.push(ValidationIssue {
                    key: key.to_string(),
                    severity: Severity::Error,
                    message: format!("Port {} is out of range (1-65535)", port),
                });
            }
        } else if value.parse::<f64>().is_ok() {
            issues.push(ValidationIssue {
                key: key.to_string(),
                severity: Severity::Error,
                message: "Port should be an integer".to_string(),
            });
        }
    }

    // URL validation
    if key_upper.contains("URL") || key_upper.contains("URI") || key_upper.contains("ENDPOINT") {
        if !value.contains("://") {
            issues.push(ValidationIssue {
                key: key.to_string(),
                severity: Severity::Error,
                message: "URL is missing protocol (e.g., https://)".to_string(),
            });
        } else if value.contains("localhost") && key_upper.contains("PROD") {
            issues.push(ValidationIssue {
                key: key.to_string(),
                severity: Severity::Warning,
                message: "Production URL points to localhost".to_string(),
            });
        }
    }

    // Boolean validation
    if key_upper.contains("DEBUG") || key_upper.contains("ENABLE") || key_upper.contains("DISABLE") {
        let valid_booleans = ["true", "false", "1", "0", "yes", "no"];
        if !valid_booleans.contains(&value.to_lowercase().as_str()) {
            issues.push(ValidationIssue {
                key: key.to_string(),
                severity: Severity::Warning,
                message: format!("Expected boolean value, got '{}'", value),
            });
        }
    }

    // Placeholder detection
    let placeholders = ["TODO", "CHANGEME", "FIXME", "xxx", "your-", "example"];
    for ph in &placeholders {
        if value.to_lowercase().contains(&ph.to_lowercase()) {
            issues.push(ValidationIssue {
                key: key.to_string(),
                severity: Severity::Warning,
                message: format!("Value looks like a placeholder (contains '{}')", ph),
            });
        }
    }

    // Whitespace issues
    if value != value.trim() {
        issues.push(ValidationIssue {
            key: key.to_string(),
            severity: Severity::Warning,
            message: "Value has leading or trailing whitespace".to_string(),
        });
    }
}