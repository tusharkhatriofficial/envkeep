use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Table, ContentArrangement, presets::UTF8_FULL_CONDENSED};
use chrono::{DateTime, Utc};

use crate::crypto;
use crate::vault;

pub fn handle_list() -> Result<()> {
    let password = crypto::prompt_existing_password()?;
    let conn = vault::open_vault(&password)
        .context("Failed to open vault")?;

    let projects = vault::project::list_projects(&conn)?;

    if projects.is_empty() {
        println!("No projects in vault.");
        println!("  Run {} in a project directory.", "envkeep add <name>".cyan());
        return Ok(());
    }

    println!(
        "{} envkeep projects ({})",
        "".bold(),  // or use an ascii icon
        projects.len()
    );
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Project", "Vars", "Modified"]);

    for project in &projects {
        let var_count = vault::project::count_variables(&conn, &project.id)?;
        let modified = format_relative_time(&project.updated_at);

        table.add_row(vec![
            project.name.clone(),
            var_count.to_string(),
            modified,
        ]);
    }

    println!("{table}");

    Ok(())
}

/// Format a timestamp as a human-readable relative time.
fn format_relative_time(iso_time: &str) -> String {
    let parsed = match DateTime::parse_from_rfc3339(iso_time) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => return iso_time.to_string(),
    };

    let now = Utc::now();
    let duration = now.signed_duration_since(parsed);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() < 30 {
        format!("{}d ago", duration.num_days())
    } else {
        format!("{}mo ago", duration.num_days() / 30)
    }
}
