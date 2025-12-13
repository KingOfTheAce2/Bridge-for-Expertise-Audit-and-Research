use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use uuid::Uuid;

use super::{LicenseTier, Prompt};

/// Metadata extracted from YAML frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMetadata {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
    pub version: Option<String>,
    pub created: Option<String>,
    pub author: Option<String>,
    pub license_tier: Option<String>,
}

/// Parse a prompt file with YAML frontmatter
///
/// File format:
/// ```markdown
/// ---
/// name: Prompt Name
/// description: Description here
/// category: contract_analysis
/// language: en
/// tags: [contract, review]
/// version: 1.0
/// created: 2025-01-26
/// author: User Name
/// license_tier: basic
/// ---
///
/// Prompt content goes here...
/// ```
pub fn parse_prompt_file(path: &Path) -> Result<Prompt> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read prompt file: {:?}", path))?;

    // Check if file has YAML frontmatter
    if content.starts_with("---\n") || content.starts_with("---\r\n") {
        parse_with_frontmatter(&content, path)
    } else {
        // No frontmatter, create a basic prompt from content
        parse_without_frontmatter(&content, path)
    }
}

/// Parse file with YAML frontmatter
fn parse_with_frontmatter(content: &str, path: &Path) -> Result<Prompt> {
    // Split content into frontmatter and body
    let parts: Vec<&str> = content.splitn(3, "---").collect();

    if parts.len() < 3 {
        anyhow::bail!("Invalid YAML frontmatter format");
    }

    let frontmatter = parts[1].trim();
    let body = parts[2].trim();

    // Parse YAML metadata
    let metadata: PromptMetadata = serde_yaml::from_str(frontmatter)
        .context("Failed to parse YAML frontmatter")?;

    // Extract filename without extension for default name
    let default_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();

    // Build prompt
    let prompt = Prompt {
        id: Uuid::new_v4().to_string(),
        name: metadata.name.unwrap_or(default_name),
        description: metadata.description.unwrap_or_default(),
        category: metadata.category.unwrap_or_else(|| "general".to_string()),
        content: body.to_string(),
        variables: Vec::new(), // Will be extracted later
        tags: metadata.tags.unwrap_or_default(),
        language: metadata.language.unwrap_or_else(|| "en".to_string()),
        tier: parse_tier(&metadata.license_tier),
        version: metadata.version.unwrap_or_else(|| "1.0".to_string()),
        author: metadata.author,
        created: metadata.created,
        is_builtin: false,
        file_path: Some(path.to_path_buf()),
    };

    Ok(prompt)
}

/// Parse file without frontmatter (plain text)
fn parse_without_frontmatter(content: &str, path: &Path) -> Result<Prompt> {
    let default_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Untitled")
        .to_string();

    let prompt = Prompt {
        id: Uuid::new_v4().to_string(),
        name: default_name,
        description: String::new(),
        category: "general".to_string(),
        content: content.to_string(),
        variables: Vec::new(),
        tags: Vec::new(),
        language: "en".to_string(),
        tier: LicenseTier::Basic,
        version: "1.0".to_string(),
        author: None,
        created: Some(chrono::Utc::now().to_rfc3339()),
        is_builtin: false,
        file_path: Some(path.to_path_buf()),
    };

    Ok(prompt)
}

/// Parse license tier string to enum
fn parse_tier(tier_str: &Option<String>) -> LicenseTier {
    match tier_str.as_ref().map(|s| s.to_lowercase()) {
        Some(ref s) if s == "free" => LicenseTier::Free,
        Some(ref s) if s == "basic" => LicenseTier::Basic,
        Some(ref s) if s == "pro" => LicenseTier::Pro,
        Some(ref s) if s == "enterprise" => LicenseTier::Enterprise,
        _ => LicenseTier::Basic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_with_frontmatter() {
        let content = r#"---
name: Test Prompt
description: A test prompt
category: testing
language: en
tags: [test, example]
version: 1.0
license_tier: pro
---

This is the prompt content with {VARIABLE}."#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();

        let prompt = parse_prompt_file(temp_file.path()).unwrap();

        assert_eq!(prompt.name, "Test Prompt");
        assert_eq!(prompt.description, "A test prompt");
        assert_eq!(prompt.category, "testing");
        assert_eq!(prompt.tier, LicenseTier::Pro);
        assert!(prompt.content.contains("{VARIABLE}"));
    }

    #[test]
    fn test_parse_without_frontmatter() {
        let content = "This is a simple prompt without frontmatter.";

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();

        let prompt = parse_prompt_file(temp_file.path()).unwrap();

        assert_eq!(prompt.content, content);
        assert_eq!(prompt.tier, LicenseTier::Basic);
    }

    #[test]
    fn test_parse_tier() {
        assert_eq!(parse_tier(&Some("free".to_string())), LicenseTier::Free);
        assert_eq!(parse_tier(&Some("Basic".to_string())), LicenseTier::Basic);
        assert_eq!(parse_tier(&Some("PRO".to_string())), LicenseTier::Pro);
        assert_eq!(parse_tier(&None), LicenseTier::Basic);
    }
}
