use tauri::State;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use crate::database::DatabaseManager;
use crate::entity::settings;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeSettings {
    pub theme: String, // "light", "dark", or "system"
}

// ---------- THEME COMMANDS ----------

#[tauri::command]
pub async fn get_theme_setting(
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let conn = db.get_connection().await
        .ok_or("Database not initialized")?;

    match settings::Entity::find()
        .filter(settings::Column::Key.eq("theme"))
        .one(&conn)
        .await
    {
        Ok(Some(setting)) => Ok(setting.value),
        Ok(None) => Ok("system".to_string()),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
pub async fn save_theme_setting(
    theme: String,
    db: State<'_, DatabaseManager>,
) -> Result<(), String> {
    if !["light", "dark", "system"].contains(&theme.as_str()) {
        return Err("Invalid theme value".to_string());
    }

    let conn = db.get_connection().await
        .ok_or("Database not initialized")?;

    let existing = settings::Entity::find()
        .filter(settings::Column::Key.eq("theme"))
        .one(&conn)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if let Some(mut record) = existing {
        let mut model: settings::ActiveModel = record.into();
        model.value = Set(theme);
        model.update(&conn)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;
    } else {
        let new_setting = settings::ActiveModel {
            key: Set("theme".to_string()),
            value: Set(theme),
            ..Default::default()
        };
        new_setting
            .insert(&conn)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;
    }
    Ok(())
}

// ---------- GENERIC SETTINGS COMMANDS ----------

#[tauri::command]
pub async fn get_setting(
    key: String,
    db: State<'_, DatabaseManager>,
) -> Result<Option<String>, String> {
    let conn = db.get_connection().await
        .ok_or("Database not initialized")?;

    match settings::Entity::find()
        .filter(settings::Column::Key.eq(key.clone()))
        .one(&conn)
        .await
    {
        Ok(Some(setting)) => Ok(Some(setting.value)),
        Ok(None) => Ok(None),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}

#[tauri::command]
pub async fn set_setting(
    key: String,
    value: String,
    db: State<'_, DatabaseManager>,
) -> Result<(), String> {
    let conn = db.get_connection().await
        .ok_or("Database not initialized")?;

    let existing = settings::Entity::find()
        .filter(settings::Column::Key.eq(key.clone()))
        .one(&conn)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    if let Some(mut record) = existing {
        let mut model: settings::ActiveModel = record.into();
        model.value = Set(value);
        model.update(&conn)
            .await
            .map_err(|e| format!("Update failed: {}", e))?;
    } else {
        let new_setting = settings::ActiveModel {
            key: Set(key),
            value: Set(value),
            ..Default::default()
        };
        new_setting
            .insert(&conn)
            .await
            .map_err(|e| format!("Insert failed: {}", e))?;
    }
    Ok(())
}

// ---------- APP VERSION COMMAND ----------

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
