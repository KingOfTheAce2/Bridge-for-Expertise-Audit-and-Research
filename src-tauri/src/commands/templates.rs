use crate::templates::{DocumentTemplate, TemplateLibrary};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Get all templates
#[tauri::command]
pub async fn get_all_templates(
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<Vec<DocumentTemplate>, String> {
    let lib = library.lock().await;
    lib.load_all()
        .map_err(|e| format!("Failed to load templates: {}", e))
}

/// Get template by ID
#[tauri::command]
pub async fn get_template_by_id(
    template_id: String,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<Option<DocumentTemplate>, String> {
    let lib = library.lock().await;
    lib.get_template(&template_id)
        .map_err(|e| format!("Failed to get template: {}", e))
}

/// Get templates by category
#[tauri::command]
pub async fn get_templates_by_category(
    category: String,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<Vec<DocumentTemplate>, String> {
    let lib = library.lock().await;
    lib.get_by_category(&category)
        .map_err(|e| format!("Failed to get templates by category: {}", e))
}

/// Request to save a template
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTemplateRequest {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub tags: Vec<String>,
    pub language: String,
}

/// Save (create or update) a template
#[tauri::command]
pub async fn save_template(
    request: SaveTemplateRequest,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<DocumentTemplate, String> {
    let lib = library.lock().await;

    let mut template = if let Some(id) = request.id {
        // Update existing template
        lib.get_template(&id)
            .map_err(|e| format!("Failed to get template: {}", e))?
            .ok_or_else(|| format!("Template not found: {}", id))?
    } else {
        // Create new template
        DocumentTemplate::new(request.name.clone(), request.content.clone())
    };

    // Update fields
    template.name = request.name;
    template.description = request.description;
    template.category = request.category;
    template.content = request.content;
    template.tags = request.tags;
    template.language = request.language;
    template.extract_variables();

    // Validate template
    template
        .validate()
        .map_err(|e| format!("Template validation failed: {}", e))?;

    lib.save_template(&template)
        .map_err(|e| format!("Failed to save template: {}", e))?;

    Ok(template)
}

/// Delete a template
#[tauri::command]
pub async fn delete_template(
    template_id: String,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<String, String> {
    let lib = library.lock().await;

    lib.delete_template(&template_id)
        .map_err(|e| format!("Failed to delete template: {}", e))?;

    Ok(format!("Template {} deleted successfully", template_id))
}

/// Import a template from a file
#[tauri::command]
pub async fn import_template_file(
    file_path: String,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<DocumentTemplate, String> {
    let lib = library.lock().await;
    let path = PathBuf::from(file_path);

    lib.import_template(&path)
        .map_err(|e| format!("Failed to import template: {}", e))
}

/// Request to render a template
#[derive(Debug, Serialize, Deserialize)]
pub struct RenderTemplateRequest {
    pub template_id: String,
    pub variables: HashMap<String, String>,
}

/// Render a template with variables
#[tauri::command]
pub async fn render_template(
    request: RenderTemplateRequest,
    library: State<'_, Arc<Mutex<TemplateLibrary>>>,
) -> Result<String, String> {
    let lib = library.lock().await;

    let template = lib
        .get_template(&request.template_id)
        .map_err(|e| format!("Failed to get template: {}", e))?
        .ok_or_else(|| format!("Template not found: {}", request.template_id))?;

    template
        .render(&request.variables)
        .map_err(|e| format!("Failed to render template: {}", e))
}

/// Validate template syntax
#[tauri::command]
pub async fn validate_template_syntax(
    content: String,
) -> Result<String, String> {
    let template = DocumentTemplate::new("Validation".to_string(), content);

    template
        .validate()
        .map_err(|e| format!("Validation failed: {}", e))?;

    Ok("Template syntax is valid".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_template_request_serialization() {
        let request = SaveTemplateRequest {
            id: Some("test-id".to_string()),
            name: "NDA Template".to_string(),
            description: "Non-disclosure agreement".to_string(),
            category: "legal".to_string(),
            content: "NDA between {PARTY_A} and {PARTY_B}".to_string(),
            tags: vec!["nda".to_string(), "contract".to_string()],
            language: "en".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: SaveTemplateRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "NDA Template");
    }
}
