use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::errors::DotkeepError;

/// Parse a .env file into key-value pairs.
///
/// Handles:
/// - KEY=VALUE
/// - KEY="VALUE WITH SPACES"
/// - KEY='VALUE WITH SPACES'
/// - # comments (ignored)
/// - Empty lines (ignored)
/// - Inline comments after values
pub fn parse_env_file(path: &Path) -> Result<BTreeMap<String, String>, DotkeepError> {
    let contents = fs::read_to_string(path)
        .map_err(|e| DotkeepError::FileReadError(path.display().to_string(), e))?;

    let mut vars = BTreeMap::new();

    for line in contents.lines() {
        let trimmed = line.trim();

        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Find the first = sign
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let raw_value = trimmed[eq_pos + 1..].trim();

            // Strip surrounding quotes if present
            let value = if (raw_value.starts_with('"') && raw_value.ends_with('"'))
                || (raw_value.starts_with('\'') && raw_value.ends_with('\''))
            {
                raw_value[1..raw_value.len() - 1].to_string()
            } else {
                // Remove inline comments (space + #)
                match raw_value.find(" #") {
                    Some(pos) => raw_value[..pos].trim().to_string(),
                    None => raw_value.to_string(),
                }
            };

            if !key.is_empty() {
                vars.insert(key, value);
            }
        }
    }

    Ok(vars)
}

/// Write key-value pairs to a .env file.
pub fn write_env_file(
    path: &Path,
    vars: &BTreeMap<String, String>,
) -> Result<(), DotkeepError> {
    let mut content = String::new();

    for (key, value) in vars {
        // Quote values with spaces or special characters
        if value.contains(' ') || value.contains('#') || value.contains('=') {
            content.push_str(&format!("{}=\"{}\"\n", key, value));
        } else {
            content.push_str(&format!("{}={}\n", key, value));
        }
    }

    fs::write(path, content)
        .map_err(|e| DotkeepError::FileWriteError(path.display().to_string(), e))?;

    Ok(())
}