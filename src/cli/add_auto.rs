use anyhow::{Context, Result};
use std::env;
use std::process::Command;

/// Detect project name from git remote or directory name.
pub fn detect_project_name() -> Result<String> {
    // Try git remote first
    if let Ok(output) = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
    {
        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Extract repo name from URL
            // https://github.com/user/repo.git -> repo
            // git@github.com:user/repo.git -> repo
            if let Some(name) = url
                .rsplit('/')
                .next()
                .or_else(|| url.rsplit(':').next())
                .map(|s| s.trim_end_matches(".git").to_string())
            {
                if !name.is_empty() {
                    return Ok(name);
                }
            }
        }
    }

    // Fall back to directory name
    let cwd = env::current_dir().context("Could not determine current directory")?;
    let name = cwd
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unnamed-project".to_string());

    Ok(name)
}

pub fn handle_add_auto() -> Result<()> {
    let name = detect_project_name()?;
    println!("Detected project name: {}", name);
    super::add::handle_add(&name)
}