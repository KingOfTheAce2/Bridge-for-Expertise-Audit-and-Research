mod renderer;
mod validator;

pub use validator::validate_template;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use walkdir::WalkDir;

use crate::prompts::{parse_prompt_file, substitute_variables};

/// Document template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub variables: Vec<String>,
    pub output_format: OutputFormat,
    pub language: String,
    pub tags: Vec<String>,
    pub version: String,
    pub author: Option<String>,
    pub created: Option<String>,
    pub is_builtin: bool,
    pub file_path: Option<PathBuf>,
}

/// Output format for rendered templates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OutputFormat {
    Markdown,
    PlainText,
    Html,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Markdown
    }
}

impl DocumentTemplate {
    /// Create a new template
    pub fn new(name: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description: String::new(),
            category: "general".to_string(),
            content,
            variables: Vec::new(),
            output_format: OutputFormat::Markdown,
            language: "en".to_string(),
            tags: Vec::new(),
            version: "1.0".to_string(),
            author: None,
            created: Some(chrono::Utc::now().to_rfc3339()),
            is_builtin: false,
            file_path: None,
        }
    }

    /// Extract variables from content
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

    /// Render template with variables
    pub fn render(&self, values: &HashMap<String, String>) -> Result<String> {
        substitute_variables(&self.content, values)
    }

    /// Validate template syntax
    pub fn validate(&self) -> Result<()> {
        validate_template(&self.content)
    }
}

/// Template library manager
pub struct TemplateLibrary {
    #[allow(dead_code)]
    templates_dir: PathBuf,
    builtin_dir: PathBuf,
    user_dir: PathBuf,
}

impl TemplateLibrary {
    /// Create a new template library
    pub fn new(base_dir: PathBuf) -> Result<Self> {
        let templates_dir = base_dir.join("prompts").join("templates");
        let builtin_dir = templates_dir.join("builtin");
        let user_dir = templates_dir.join("user");

        fs::create_dir_all(&builtin_dir)?;
        fs::create_dir_all(&user_dir)?;

        Ok(Self {
            templates_dir,
            builtin_dir,
            user_dir,
        })
    }

    /// Load all templates
    pub fn load_all(&self) -> Result<Vec<DocumentTemplate>> {
        let mut templates = Vec::new();

        // Load built-in templates
        templates.extend(self.load_from_dir(&self.builtin_dir, true)?);

        // Load user templates
        templates.extend(self.load_from_dir(&self.user_dir, false)?);

        Ok(templates)
    }

    /// Load templates from directory
    fn load_from_dir(&self, dir: &Path, is_builtin: bool) -> Result<Vec<DocumentTemplate>> {
        let mut templates = Vec::new();

        for entry in WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "md" || ext == "txt" {
                        match self.load_template_from_file(path, is_builtin) {
                            Ok(template) => templates.push(template),
                            Err(e) => {
                                log::warn!("Failed to load template from {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(templates)
    }

    /// Load a single template from file
    fn load_template_from_file(&self, path: &Path, is_builtin: bool) -> Result<DocumentTemplate> {
        // Reuse the prompt parser for templates
        let prompt = parse_prompt_file(path)?;

        let mut template = DocumentTemplate {
            id: prompt.id,
            name: prompt.name,
            description: prompt.description,
            category: prompt.category,
            content: prompt.content,
            variables: prompt.variables,
            output_format: OutputFormat::Markdown,
            language: prompt.language,
            tags: prompt.tags,
            version: prompt.version,
            author: prompt.author,
            created: prompt.created,
            is_builtin,
            file_path: Some(path.to_path_buf()),
        };

        template.extract_variables();
        Ok(template)
    }

    /// Save template to file
    pub fn save_template(&self, template: &DocumentTemplate) -> Result<PathBuf> {
        let target_dir = if template.is_builtin {
            &self.builtin_dir
        } else {
            &self.user_dir
        };

        let filename = format!("{}.md", template.id);
        let file_path = target_dir.join(filename);

        self.write_template_to_file(template, &file_path)?;

        Ok(file_path)
    }

    /// Write template to file with frontmatter
    fn write_template_to_file(&self, template: &DocumentTemplate, path: &Path) -> Result<()> {
        let mut content = String::new();
        content.push_str("---\n");
        content.push_str(&format!("name: {}\n", template.name));
        content.push_str(&format!("description: {}\n", template.description));
        content.push_str(&format!("category: {}\n", template.category));
        content.push_str(&format!("language: {}\n", template.language));

        if !template.tags.is_empty() {
            content.push_str(&format!("tags: {:?}\n", template.tags));
        }

        content.push_str(&format!("version: {}\n", template.version));

        if let Some(ref created) = template.created {
            content.push_str(&format!("created: {}\n", created));
        }

        if let Some(ref author) = template.author {
            content.push_str(&format!("author: {}\n", author));
        }

        content.push_str("---\n\n");
        content.push_str(&template.content);

        fs::write(path, content).context("Failed to write template file")?;

        Ok(())
    }

    /// Import template from external file
    pub fn import_template(&self, source_path: &Path) -> Result<DocumentTemplate> {
        let template = self.load_template_from_file(source_path, false)?;
        self.save_template(&template)?;
        Ok(template)
    }

    /// Delete template
    pub fn delete_template(&self, template_id: &str) -> Result<()> {
        let templates = self.load_all()?;

        if let Some(template) = templates.iter().find(|t| t.id == template_id) {
            if template.is_builtin {
                anyhow::bail!("Cannot delete built-in templates");
            }

            if let Some(ref path) = template.file_path {
                fs::remove_file(path).context("Failed to delete template file")?;
            }
        }

        Ok(())
    }

    /// Get template by ID
    pub fn get_template(&self, template_id: &str) -> Result<Option<DocumentTemplate>> {
        let templates = self.load_all()?;
        Ok(templates.into_iter().find(|t| t.id == template_id))
    }

    /// Get templates by category
    pub fn get_by_category(&self, category: &str) -> Result<Vec<DocumentTemplate>> {
        let all_templates = self.load_all()?;
        Ok(all_templates
            .into_iter()
            .filter(|t| t.category.eq_ignore_ascii_case(category))
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_creation() {
        let mut template = DocumentTemplate::new(
            "NDA Template".to_string(),
            "NDA between {PARTY_A} and {PARTY_B}".to_string(),
        );
        template.extract_variables();

        assert_eq!(template.variables.len(), 2);
        assert!(template.variables.contains(&"PARTY_A".to_string()));
        assert!(template.variables.contains(&"PARTY_B".to_string()));
    }

    #[test]
    fn test_template_render() {
        let template = DocumentTemplate::new(
            "Test".to_string(),
            "Agreement dated {DATE}".to_string(),
        );

        let mut values = HashMap::new();
        values.insert("DATE".to_string(), "2025-01-26".to_string());

        let result = template.render(&values).unwrap();
        assert_eq!(result, "Agreement dated 2025-01-26");
    }
}
