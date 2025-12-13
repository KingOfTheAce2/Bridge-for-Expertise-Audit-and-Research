mod parser;
mod variables;
mod search;
mod categories;
mod system_prompts;

pub use parser::parse_prompt_file;
pub use variables::substitute_variables;
pub use search::search_prompts;
pub use system_prompts::get_builtin_prompts;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

/// License tier levels for prompt access control
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LicenseTier {
    Free,
    Basic,
    Pro,
    Enterprise,
}

impl Default for LicenseTier {
    fn default() -> Self {
        LicenseTier::Basic
    }
}

/// A prompt template with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub variables: Vec<String>,
    pub tags: Vec<String>,
    pub language: String,
    pub tier: LicenseTier,
    pub version: String,
    pub author: Option<String>,
    pub created: Option<String>,
    pub is_builtin: bool,
    pub file_path: Option<PathBuf>,
}

impl Prompt {
    /// Create a new prompt with default values
    pub fn new(name: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            category: "general".to_string(),
            content,
            variables: Vec::new(),
            tags: Vec::new(),
            language: "en".to_string(),
            tier: LicenseTier::Basic,
            version: "1.0".to_string(),
            author: None,
            created: Some(chrono::Utc::now().to_rfc3339()),
            is_builtin: false,
            file_path: None,
        }
    }

    /// Extract variables from content (anything in {VARIABLE_NAME} format)
    pub fn extract_variables(&mut self) {
        let re = regex::Regex::new(r"\{([A-Z_][A-Z0-9_]*)\}").unwrap();
        self.variables = re
            .captures_iter(&self.content)
            .map(|cap| cap[1].to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        self.variables.sort();
    }

    /// Substitute variables in the prompt content
    pub fn apply_variables(&self, values: &HashMap<String, String>) -> Result<String> {
        substitute_variables(&self.content, values)
    }

    /// Check if user has access to this prompt based on tier
    pub fn check_access(&self, user_tier: LicenseTier) -> bool {
        user_tier >= self.tier
    }
}

/// Prompt library manager
pub struct PromptLibrary {
    #[allow(dead_code)]
    prompts_dir: PathBuf,
    system_dir: PathBuf,
    user_dir: PathBuf,
    #[allow(dead_code)]
    templates_dir: PathBuf,
    #[allow(dead_code)]
    shared_dir: PathBuf,
}

impl PromptLibrary {
    /// Create a new prompt library with the specified base directory
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        let prompts_dir = base_dir.join("prompts");
        let system_dir = prompts_dir.join("system");
        let user_dir = prompts_dir.join("user");
        let templates_dir = prompts_dir.join("templates");
        let shared_dir = prompts_dir.join("shared");

        // Create directories if they don't exist
        fs::create_dir_all(&system_dir)?;
        fs::create_dir_all(&user_dir)?;
        fs::create_dir_all(&templates_dir)?;
        fs::create_dir_all(&shared_dir)?;

        Ok(Self {
            prompts_dir,
            system_dir,
            user_dir,
            templates_dir,
            shared_dir,
        })
    }

    /// Initialize the library with built-in system prompts
    pub fn initialize(&self) -> Result<()> {
        let builtin_prompts = get_builtin_prompts();

        for prompt in builtin_prompts {
            let file_path = self.system_dir.join(format!("{}.md", prompt.id));

            // Only write if file doesn't exist
            if !file_path.exists() {
                self.write_prompt_to_file(&prompt, &file_path)?;
            }
        }

        Ok(())
    }

    /// Load all prompts from the library
    pub fn load_all_prompts(&self) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();

        // Load system prompts
        prompts.extend(self.load_prompts_from_dir(&self.system_dir, true)?);

        // Load user prompts
        prompts.extend(self.load_prompts_from_dir(&self.user_dir, false)?);

        Ok(prompts)
    }

    /// Load prompts from a specific directory
    fn load_prompts_from_dir(&self, dir: &Path, is_builtin: bool) -> Result<Vec<Prompt>> {
        let mut prompts = Vec::new();

        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Only process .md and .txt files
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "txt" {
                        match self.load_prompt_from_file(path, is_builtin) {
                            Ok(prompt) => prompts.push(prompt),
                            Err(e) => {
                                log::warn!("Failed to load prompt from {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(prompts)
    }

    /// Load a single prompt from a file
    pub fn load_prompt_from_file(&self, path: &Path, is_builtin: bool) -> Result<Prompt> {
        let mut prompt = parse_prompt_file(path)?;
        prompt.is_builtin = is_builtin;
        prompt.file_path = Some(path.to_path_buf());
        prompt.extract_variables();
        Ok(prompt)
    }

    /// Save a prompt to a file
    pub fn save_prompt(&self, prompt: &Prompt) -> Result<PathBuf> {
        let target_dir = if prompt.is_builtin {
            &self.system_dir
        } else {
            &self.user_dir
        };

        let filename = format!("{}.md", prompt.id);
        let file_path = target_dir.join(filename);

        self.write_prompt_to_file(prompt, &file_path)?;

        Ok(file_path)
    }

    /// Write prompt to file with YAML frontmatter
    fn write_prompt_to_file(&self, prompt: &Prompt, path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str("---\n");
        content.push_str(&format!("name: {}\n", prompt.name));
        content.push_str(&format!("description: {}\n", prompt.description));
        content.push_str(&format!("category: {}\n", prompt.category));
        content.push_str(&format!("language: {}\n", prompt.language));

        if !prompt.tags.is_empty() {
            content.push_str(&format!("tags: {:?}\n", prompt.tags));
        }

        content.push_str(&format!("version: {}\n", prompt.version));

        if let Some(ref created) = prompt.created {
            content.push_str(&format!("created: {}\n", created));
        }

        if let Some(ref author) = prompt.author {
            content.push_str(&format!("author: {}\n", author));
        }

        content.push_str(&format!("license_tier: {:?}\n", prompt.tier));
        content.push_str("---\n\n");
        content.push_str(&prompt.content);

        fs::write(path, content).context("Failed to write prompt file")?;

        Ok(())
    }

    /// Import a prompt file from an external path
    pub fn import_prompt(&self, source_path: &Path) -> Result<Prompt> {
        let prompt = self.load_prompt_from_file(source_path, false)?;
        self.save_prompt(&prompt)?;
        Ok(prompt)
    }

    /// Delete a prompt
    pub fn delete_prompt(&self, prompt_id: &str) -> Result<()> {
        let prompts = self.load_all_prompts()?;

        if let Some(prompt) = prompts.iter().find(|p| p.id == prompt_id) {
            if prompt.is_builtin {
                anyhow::bail!("Cannot delete built-in system prompts");
            }

            if let Some(ref path) = prompt.file_path {
                fs::remove_file(path).context("Failed to delete prompt file")?;
            }
        }

        Ok(())
    }

    /// Get prompt by ID
    pub fn get_prompt(&self, prompt_id: &str) -> Result<Option<Prompt>> {
        let prompts = self.load_all_prompts()?;
        Ok(prompts.into_iter().find(|p| p.id == prompt_id))
    }

    /// Search prompts by query
    pub fn search(&self, query: &str) -> Result<Vec<Prompt>> {
        let all_prompts = self.load_all_prompts()?;
        Ok(search_prompts(&all_prompts, query))
    }

    /// Get prompts by category
    pub fn get_by_category(&self, category: &str) -> Result<Vec<Prompt>> {
        let all_prompts = self.load_all_prompts()?;
        Ok(all_prompts
            .into_iter()
            .filter(|p| p.category.eq_ignore_ascii_case(category))
            .collect())
    }

    /// Get prompts by tag
    pub fn get_by_tag(&self, tag: &str) -> Result<Vec<Prompt>> {
        let all_prompts = self.load_all_prompts()?;
        Ok(all_prompts
            .into_iter()
            .filter(|p| p.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)))
            .collect())
    }

    /// Get prompts accessible to a specific tier
    pub fn get_by_tier(&self, user_tier: LicenseTier) -> Result<Vec<Prompt>> {
        let all_prompts = self.load_all_prompts()?;
        Ok(all_prompts
            .into_iter()
            .filter(|p| p.check_access(user_tier))
            .collect())
    }

    /// Get all available categories
    pub fn get_categories(&self) -> Result<Vec<String>> {
        let all_prompts = self.load_all_prompts()?;
        let mut categories: Vec<String> = all_prompts
            .iter()
            .map(|p| p.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort();
        Ok(categories)
    }

    /// Get all available tags
    pub fn get_tags(&self) -> Result<Vec<String>> {
        let all_prompts = self.load_all_prompts()?;
        let mut tags: Vec<String> = all_prompts
            .iter()
            .flat_map(|p| p.tags.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        tags.sort();
        Ok(tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_prompt_creation() {
        let mut prompt = Prompt::new(
            "Test Prompt".to_string(),
            "Hello {NAME}, your email is {EMAIL}".to_string(),
        );
        prompt.extract_variables();

        assert_eq!(prompt.variables.len(), 2);
        assert!(prompt.variables.contains(&"NAME".to_string()));
        assert!(prompt.variables.contains(&"EMAIL".to_string()));
    }

    #[test]
    fn test_variable_substitution() {
        let prompt = Prompt::new(
            "Test".to_string(),
            "Hello {NAME}!".to_string(),
        );

        let mut values = HashMap::new();
        values.insert("NAME".to_string(), "World".to_string());

        let result = prompt.apply_variables(&values).unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_tier_access() {
        let mut prompt = Prompt::new("Test".to_string(), "Content".to_string());
        prompt.tier = LicenseTier::Pro;

        assert!(!prompt.check_access(LicenseTier::Free));
        assert!(!prompt.check_access(LicenseTier::Basic));
        assert!(prompt.check_access(LicenseTier::Pro));
        assert!(prompt.check_access(LicenseTier::Enterprise));
    }
}
