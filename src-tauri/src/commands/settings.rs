use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub theme: String, // "light", "dark", or "system"
}

#[tauri::command]
pub async fn get_theme_setting() -> Result<String, String> {
    // Load from database or config file
    // For now, return default
    Ok("system".to_string())
}

#[tauri::command]
pub async fn save_theme_setting(theme: String) -> Result<(), String> {
    if !["light", "dark", "system"].contains(&theme.as_str()) {
        return Err("Invalid theme value".to_string());
    }
    // TODO: Save theme to DB or config
    Ok(())
}
