use crate::prompts::{LicenseTier, Prompt, PromptLibrary};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Get all prompts from the library
#[tauri::command]
pub async fn get_all_prompts(
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<Prompt>, String> {
    let lib = library.lock().await;
    lib.load_all_prompts()
        .map_err(|e| format!("Failed to load prompts: {}", e))
}

/// Get prompt by ID
#[tauri::command]
pub async fn get_prompt_by_id(
    prompt_id: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Option<Prompt>, String> {
    let lib = library.lock().await;
    lib.get_prompt(&prompt_id)
        .map_err(|e| format!("Failed to get prompt: {}", e))
}

/// Search prompts
#[tauri::command]
pub async fn search_prompts(
    query: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<Prompt>, String> {
    let lib = library.lock().await;
    lib.search(&query)
        .map_err(|e| format!("Failed to search prompts: {}", e))
}

/// Get prompts by category
#[tauri::command]
pub async fn get_prompts_by_category(
    category: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<Prompt>, String> {
    let lib = library.lock().await;
    lib.get_by_category(&category)
        .map_err(|e| format!("Failed to get prompts by category: {}", e))
}

/// Get prompts by tag
#[tauri::command]
pub async fn get_prompts_by_tag(
    tag: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<Prompt>, String> {
    let lib = library.lock().await;
    lib.get_by_tag(&tag)
        .map_err(|e| format!("Failed to get prompts by tag: {}", e))
}

/// Get prompts accessible to a tier
#[tauri::command]
pub async fn get_prompts_by_tier(
    tier: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<Prompt>, String> {
    let lib = library.lock().await;

    let license_tier = match tier.to_lowercase().as_str() {
        "free" => LicenseTier::Free,
        "basic" => LicenseTier::Basic,
        "pro" => LicenseTier::Pro,
        "enterprise" => LicenseTier::Enterprise,
        _ => LicenseTier::Basic,
    };

    lib.get_by_tier(license_tier)
        .map_err(|e| format!("Failed to get prompts by tier: {}", e))
}

/// Get all available categories
#[tauri::command]
pub async fn get_prompt_categories(
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<String>, String> {
    let lib = library.lock().await;
    lib.get_categories()
        .map_err(|e| format!("Failed to get categories: {}", e))
}

/// Get all available tags
#[tauri::command]
pub async fn get_prompt_tags(
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Vec<String>, String> {
    let lib = library.lock().await;
    lib.get_tags()
        .map_err(|e| format!("Failed to get tags: {}", e))
}

/// Request to create/update a prompt
#[derive(Debug, Serialize, Deserialize)]
pub struct SavePromptRequest {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub category: String,
    pub content: String,
    pub tags: Vec<String>,
    pub language: String,
}

/// Save (create or update) a prompt
#[tauri::command]
pub async fn save_prompt(
    request: SavePromptRequest,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Prompt, String> {
    let lib = library.lock().await;

    let mut prompt = if let Some(id) = request.id {
        // Update existing prompt
        lib.get_prompt(&id)
            .map_err(|e| format!("Failed to get prompt: {}", e))?
            .ok_or_else(|| format!("Prompt not found: {}", id))?
    } else {
        // Create new prompt
        Prompt::new(request.name.clone(), request.content.clone())
    };

    // Update fields
    prompt.name = request.name;
    prompt.description = request.description;
    prompt.category = request.category;
    prompt.content = request.content;
    prompt.tags = request.tags;
    prompt.language = request.language;
    prompt.extract_variables();

    lib.save_prompt(&prompt)
        .map_err(|e| format!("Failed to save prompt: {}", e))?;

    Ok(prompt)
}

/// Delete a prompt
#[tauri::command]
pub async fn delete_prompt(
    prompt_id: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<String, String> {
    let lib = library.lock().await;

    lib.delete_prompt(&prompt_id)
        .map_err(|e| format!("Failed to delete prompt: {}", e))?;

    Ok(format!("Prompt {} deleted successfully", prompt_id))
}

/// Import a prompt from a file
#[tauri::command]
pub async fn import_prompt_file(
    file_path: String,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<Prompt, String> {
    let lib = library.lock().await;
    let path = PathBuf::from(file_path);

    lib.import_prompt(&path)
        .map_err(|e| format!("Failed to import prompt: {}", e))
}

/// Request to apply variables to a prompt
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyVariablesRequest {
    pub prompt_id: String,
    pub variables: HashMap<String, String>,
}

/// Apply variables to a prompt and return the rendered result
#[tauri::command]
pub async fn apply_prompt_variables(
    request: ApplyVariablesRequest,
    library: State<'_, Arc<Mutex<PromptLibrary>>>,
) -> Result<String, String> {
    let lib = library.lock().await;

    let prompt = lib
        .get_prompt(&request.prompt_id)
        .map_err(|e| format!("Failed to get prompt: {}", e))?
        .ok_or_else(|| format!("Prompt not found: {}", request.prompt_id))?;

    prompt
        .apply_variables(&request.variables)
        .map_err(|e| format!("Failed to apply variables: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_prompt_request_serialization() {
        let request = SavePromptRequest {
            id: Some("test-id".to_string()),
            name: "Test Prompt".to_string(),
            description: "Description".to_string(),
            category: "general".to_string(),
            content: "Content with {VARIABLE}".to_string(),
            tags: vec!["test".to_string()],
            language: "en".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: SavePromptRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Test Prompt");
    }
}
