use tauri::{AppHandle, Emitter, State};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::database::DatabaseManager;
use crate::models::{
    DownloadProgress, DownloadStatus, ModelDownloader, ModelInfo, ModelRegistry, ModelValidator,
};
use entity::models;

/// Response for listing models
#[derive(Debug, Serialize, Deserialize)]
pub struct ModelListItem {
    pub id: Option<i32>,
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub size: String,
    pub parameters: String,
    pub quantization: Option<String>,
    pub format: String,
    pub status: String,
    pub file_size: i64,
    pub is_active: bool,
    pub is_downloaded: bool,
    pub download_url: String,
    pub tags: Vec<String>,
}

/// Request for downloading a model
#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadModelRequest {
    pub model_id: String,
}

// Global state for download tracking
type DownloadState = Arc<Mutex<Option<String>>>;

/// List all available models from registry and database
#[tauri::command]
pub async fn list_models(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<ModelListItem>, String> {
    let registry = ModelRegistry::new();
    let conn = db
        .get_connection()
        .await
        .ok_or("Database not initialized")?;

    let mut result = Vec::new();

    for model_info in registry.list_models() {
        // Check if model is in database
        let db_model = models::Entity::find()
            .filter(models::Column::ModelId.eq(&model_info.model_id))
            .one(&conn)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let item = if let Some(db_record) = db_model {
            ModelListItem {
                id: Some(db_record.id),
                model_id: db_record.model_id.clone(),
                name: db_record.name.clone(),
                description: db_record.description.unwrap_or_default(),
                provider: db_record.provider.clone(),
                size: db_record.size.clone(),
                parameters: db_record.parameters.clone(),
                quantization: db_record.quantization.clone(),
                format: db_record.format.clone(),
                status: db_record.status.clone(),
                file_size: db_record.file_size.unwrap_or(model_info.file_size),
                is_active: db_record.is_active,
                is_downloaded: db_record.status == "downloaded",
                download_url: model_info.download_url.clone(),
                tags: serde_json::from_str(&db_record.tags.unwrap_or_else(|| "[]".to_string()))
                    .unwrap_or_default(),
            }
        } else {
            // Model not in database, show as available
            ModelListItem {
                id: None,
                model_id: model_info.model_id.clone(),
                name: model_info.name.clone(),
                description: model_info.description.clone(),
                provider: model_info.provider.clone(),
                size: model_info.size.clone(),
                parameters: model_info.parameters.clone(),
                quantization: model_info.quantization.clone(),
                format: model_info.format.clone(),
                status: "available".to_string(),
                file_size: model_info.file_size,
                is_active: false,
                is_downloaded: false,
                download_url: model_info.download_url.clone(),
                tags: model_info.tags.clone(),
            }
        };

        result.push(item);
    }

    Ok(result)
}

/// Download a model
#[tauri::command]
pub async fn download_model(
    model_id: String,
    app: AppHandle,
    db: State<'_, DatabaseManager>,
    download_state: State<'_, DownloadState>,
) -> Result<String, String> {
    // Check if already downloading
    let mut state = download_state.lock().await;
    if state.is_some() {
        return Err("A download is already in progress".to_string());
    }
    *state = Some(model_id.clone());
    drop(state);

    let registry = ModelRegistry::new();
    let model_info = registry
        .get_model(&model_id)
        .ok_or_else(|| format!("Model not found: {}", model_id))?
        .clone();

    let conn = db
        .get_connection()
        .await
        .ok_or("Database not initialized")?;

    // Create or update database record
    let existing = models::Entity::find()
        .filter(models::Column::ModelId.eq(&model_id))
        .one(&conn)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let db_id = if let Some(existing_model) = existing {
        let mut active_model: models::ActiveModel = existing_model.into();
        active_model.status = Set("downloading".to_string());
        active_model.download_started_at = Set(Some(chrono::Utc::now().naive_utc()));
        let updated = active_model
            .update(&conn)
            .await
            .map_err(|e| format!("Failed to update model: {}", e))?;
        updated.id
    } else {
        let new_model = models::ActiveModel {
            model_id: Set(model_info.model_id.clone()),
            name: Set(model_info.name.clone()),
            description: Set(Some(model_info.description.clone())),
            provider: Set(model_info.provider.clone()),
            size: Set(model_info.size.clone()),
            parameters: Set(model_info.parameters.clone()),
            quantization: Set(model_info.quantization.clone()),
            format: Set(model_info.format.clone()),
            status: Set("downloading".to_string()),
            download_url: Set(Some(model_info.download_url.clone())),
            file_size: Set(Some(model_info.file_size)),
            checksum: Set(Some(model_info.checksum.clone())),
            license: Set(Some(model_info.license.clone())),
            tags: Set(Some(serde_json::to_string(&model_info.tags).unwrap())),
            download_started_at: Set(Some(chrono::Utc::now().naive_utc())),
            ..Default::default()
        };
        let inserted = new_model
            .insert(&conn)
            .await
            .map_err(|e| format!("Failed to create model record: {}", e))?;
        inserted.id
    };

    // Start download in background
    let models_dir = ModelDownloader::default_models_dir()
        .map_err(|e| format!("Failed to get models directory: {}", e))?;

    let downloader = ModelDownloader::new(models_dir)
        .map_err(|e| format!("Failed to create downloader: {}", e))?;

    let download_url = model_info.download_url.clone();
    let model_id_clone = model_id.clone();
    let app_clone = app.clone();
    let db_clone = db.inner().clone();
    let download_state_clone = download_state.inner().clone();

    tokio::spawn(async move {
        let result = downloader
            .download_model(&model_id_clone, &download_url, move |progress| {
                // Emit progress event to frontend
                let _ = app_clone.emit("model-download-progress", &progress);
            })
            .await;

        // Update database based on result
        if let Some(conn) = db_clone.get_connection().await {
            match result {
                Ok(file_path) => {
                    // Verify checksum
                    let checksum_valid = if let Some(expected_checksum) =
                        models::Entity::find_by_id(db_id)
                            .one(&conn)
                            .await
                            .ok()
                            .flatten()
                            .and_then(|m| m.checksum)
                    {
                        ModelValidator::verify_checksum(&file_path, &expected_checksum)
                            .await
                            .unwrap_or(false)
                    } else {
                        true // No checksum to verify
                    };

                    if let Ok(Some(model)) = models::Entity::find_by_id(db_id).one(&conn).await {
                        let mut active: models::ActiveModel = model.into();
                        active.status = Set("downloaded".to_string());
                        active.file_path = Set(Some(file_path.to_string_lossy().to_string()));
                        active.checksum_verified = Set(checksum_valid);
                        active.download_completed_at =
                            Set(Some(chrono::Utc::now().naive_utc()));
                        let _ = active.update(&conn).await;
                    }

                    // Emit completion event
                    let _ = app_clone.emit(
                        "model-download-progress",
                        &DownloadProgress {
                            model_id: model_id_clone.clone(),
                            downloaded_bytes: 0,
                            total_bytes: 0,
                            percentage: 100.0,
                            speed_mbps: 0.0,
                            status: DownloadStatus::Completed,
                        },
                    );
                }
                Err(e) => {
                    // Update status to failed
                    if let Ok(Some(model)) = models::Entity::find_by_id(db_id).one(&conn).await {
                        let mut active: models::ActiveModel = model.into();
                        active.status = Set("failed".to_string());
                        let _ = active.update(&conn).await;
                    }

                    // Emit failure event
                    let _ = app_clone.emit(
                        "model-download-progress",
                        &DownloadProgress {
                            model_id: model_id_clone.clone(),
                            downloaded_bytes: 0,
                            total_bytes: 0,
                            percentage: 0.0,
                            speed_mbps: 0.0,
                            status: DownloadStatus::Failed,
                        },
                    );

                    log::error!("Download failed: {}", e);
                }
            }
        }

        // Clear download state
        let mut state = download_state_clone.lock().await;
        *state = None;
    });

    Ok(format!("Download started for model: {}", model_id))
}

/// Delete a downloaded model
#[tauri::command]
pub async fn delete_model(
    model_id: String,
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let conn = db
        .get_connection()
        .await
        .ok_or("Database not initialized")?;

    let model = models::Entity::find()
        .filter(models::Column::ModelId.eq(&model_id))
        .one(&conn)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| format!("Model not found: {}", model_id))?;

    // Delete file if it exists
    if let Some(file_path) = &model.file_path {
        let path = PathBuf::from(file_path);
        if path.exists() {
            tokio::fs::remove_file(&path)
                .await
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
    }

    // Update database record
    let mut active: models::ActiveModel = model.into();
    active.status = Set("available".to_string());
    active.file_path = Set(None);
    active.downloaded_size = Set(None);
    active.checksum_verified = Set(false);
    active
        .update(&conn)
        .await
        .map_err(|e| format!("Failed to update model: {}", e))?;

    Ok(format!("Model deleted: {}", model_id))
}

/// Set the active model
#[tauri::command]
pub async fn set_active_model(
    model_id: String,
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let conn = db
        .get_connection()
        .await
        .ok_or("Database not initialized")?;

    // Deactivate all models
    let all_models = models::Entity::find()
        .all(&conn)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    for model in all_models {
        let mut active: models::ActiveModel = model.into();
        active.is_active = Set(false);
        active
            .update(&conn)
            .await
            .map_err(|e| format!("Failed to update model: {}", e))?;
    }

    // Activate selected model
    let model = models::Entity::find()
        .filter(models::Column::ModelId.eq(&model_id))
        .one(&conn)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| format!("Model not found: {}", model_id))?;

    if model.status != "downloaded" {
        return Err("Model must be downloaded before activation".to_string());
    }

    let mut active: models::ActiveModel = model.into();
    active.is_active = Set(true);
    active.last_used_at = Set(Some(chrono::Utc::now().naive_utc()));
    active
        .update(&conn)
        .await
        .map_err(|e| format!("Failed to activate model: {}", e))?;

    Ok(format!("Model activated: {}", model_id))
}

/// Get the currently active model
#[tauri::command]
pub async fn get_active_model(
    db: State<'_, DatabaseManager>,
) -> Result<Option<ModelListItem>, String> {
    let conn = db
        .get_connection()
        .await
        .ok_or("Database not initialized")?;

    let active_model = models::Entity::find()
        .filter(models::Column::IsActive.eq(true))
        .one(&conn)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    if let Some(model) = active_model {
        Ok(Some(ModelListItem {
            id: Some(model.id),
            model_id: model.model_id.clone(),
            name: model.name.clone(),
            description: model.description.unwrap_or_default(),
            provider: model.provider.clone(),
            size: model.size.clone(),
            parameters: model.parameters.clone(),
            quantization: model.quantization.clone(),
            format: model.format.clone(),
            status: model.status.clone(),
            file_size: model.file_size.unwrap_or(0),
            is_active: model.is_active,
            is_downloaded: model.status == "downloaded",
            download_url: model.download_url.unwrap_or_default(),
            tags: serde_json::from_str(&model.tags.unwrap_or_else(|| "[]".to_string()))
                .unwrap_or_default(),
        }))
    } else {
        Ok(None)
    }
}
