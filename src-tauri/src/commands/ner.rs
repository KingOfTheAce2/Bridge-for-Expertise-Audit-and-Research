use crate::database::DatabaseManager;
use crate::ner::{
    DetectionMode, HybridDetector, NerModelDownloader, NerModelInfo, NerModelManager,
    NerModelRegistry, NerPipeline, NerResult,
};
use crate::pii::detector::PIIDetector;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Request to download NER model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadNerModelRequest {
    pub model_id: String,
}

/// NER model info for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerModelResponse {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub model_type: String,
    pub language: String,
    pub size: String,
    pub parameters: String,
    pub file_size: i64,
    pub accuracy: Option<f64>,
    pub is_downloaded: bool,
    pub is_active: bool,
}

/// NER inference request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerInferenceRequest {
    pub text: String,
    pub detection_mode: Option<String>, // "pattern", "ner", "hybrid"
}

/// List all available NER models
#[tauri::command]
pub async fn list_ner_models(
    db: State<'_, DatabaseManager>,
) -> Result<Vec<NerModelResponse>, String> {
    let registry = NerModelRegistry::new();
    let models = registry.list_models();

    // Check which models are downloaded
    let app_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("bear-llm-ai")
        .join("ner_models");

    let downloader = NerModelDownloader::new(app_dir)
        .map_err(|e| format!("Failed to create downloader: {}", e))?;

    let mut responses = Vec::new();
    for model in models {
        let is_downloaded = downloader.is_downloaded(&model.model_id).await;

        responses.push(NerModelResponse {
            model_id: model.model_id.clone(),
            name: model.name.clone(),
            description: model.description.clone(),
            provider: model.provider.clone(),
            model_type: model.model_type.clone(),
            language: model.language.clone(),
            size: model.size.clone(),
            parameters: model.parameters.clone(),
            file_size: model.file_size,
            accuracy: model.accuracy,
            is_downloaded,
            is_active: false, // TODO: Track active model in state
        });
    }

    Ok(responses)
}

/// Download NER model
#[tauri::command]
pub async fn download_ner_model(
    request: DownloadNerModelRequest,
    db: State<'_, DatabaseManager>,
    window: tauri::Window,
) -> Result<String, String> {
    let registry = NerModelRegistry::new();
    let model_info = registry
        .get_model(&request.model_id)
        .ok_or(format!("Model not found: {}", request.model_id))?
        .clone();

    // Get app directory for storing models
    let app_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("bear-llm-ai")
        .join("ner_models");

    let downloader = NerModelDownloader::new(app_dir)
        .map_err(|e| format!("Failed to create downloader: {}", e))?;

    // Download with progress updates
    let model_id = model_info.model_id.clone();
    let window_clone = window.clone();

    let result = downloader
        .download_model(&model_info, move |progress| {
            let _ = window_clone.emit(
                "ner-download-progress",
                serde_json::json!({
                    "model_id": &model_id,
                    "file_name": progress.file_name,
                    "downloaded": progress.downloaded_bytes,
                    "total": progress.total_bytes,
                    "progress": progress.progress_percent,
                    "speed": progress.speed_mbps,
                }),
            );
        })
        .await;

    match result {
        Ok(path) => Ok(format!("Model downloaded to: {:?}", path)),
        Err(e) => Err(format!("Download failed: {}", e)),
    }
}

/// Delete NER model
#[tauri::command]
pub async fn delete_ner_model(
    model_id: String,
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    let app_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("bear-llm-ai")
        .join("ner_models");

    let downloader = NerModelDownloader::new(app_dir)
        .map_err(|e| format!("Failed to create downloader: {}", e))?;

    downloader
        .delete_model(&model_id)
        .await
        .map_err(|e| format!("Failed to delete model: {}", e))?;

    Ok("Model deleted successfully".to_string())
}

/// Load NER model for inference
#[tauri::command]
pub async fn load_ner_model(
    model_id: String,
    ner_manager: State<'_, Arc<Mutex<Option<NerModelManager>>>>,
) -> Result<String, String> {
    let app_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("bear-llm-ai")
        .join("ner_models");

    let model_path = app_dir.join(model_id.replace('/', "_"));

    // Check if model is downloaded
    if !model_path.exists() {
        return Err(format!("Model not downloaded: {}", model_id));
    }

    // Create model manager and load model
    let manager = NerModelManager::new();
    let config = crate::ner::types::NerModelConfig::default();

    manager
        .load_model(model_path, config)
        .await
        .map_err(|e| format!("Failed to load model: {}", e))?;

    // Store in state
    let mut manager_lock = ner_manager.lock().await;
    *manager_lock = Some(manager);

    Ok(format!("Model loaded: {}", model_id))
}

/// Run NER inference on text
#[tauri::command]
pub async fn run_ner_inference(
    request: NerInferenceRequest,
    ner_manager: State<'_, Arc<Mutex<Option<NerModelManager>>>>,
    hybrid_detector: State<'_, Arc<Mutex<Option<HybridDetector>>>>,
) -> Result<NerResult, String> {
    // Get hybrid detector
    let detector_lock = hybrid_detector.lock().await;

    if detector_lock.is_none() {
        return Err("NER system not initialized".to_string());
    }

    let detector = detector_lock
        .as_ref()
        .ok_or("Hybrid detector not available")?;

    // Set detection mode
    let mode = match request.detection_mode.as_deref() {
        Some("pattern") => DetectionMode::PatternOnly,
        Some("ner") => DetectionMode::NerOnly,
        Some("hybrid") | None => DetectionMode::Hybrid,
        _ => DetectionMode::Hybrid,
    };

    detector.set_mode(mode).await;

    // Detect entities
    let entities = detector
        .detect(&request.text)
        .await
        .map_err(|e| format!("Detection failed: {}", e))?;

    // Convert to NER result format
    // For now, return a simplified result
    // In a real implementation, we'd convert PII entities back to NER format
    Ok(NerResult {
        text: request.text.clone(),
        entities: Vec::new(), // TODO: Convert from PII entities
        token_predictions: Vec::new(),
        inference_time_ms: 0,
    })
}

/// Get NER model recommendations
#[tauri::command]
pub async fn get_ner_recommendations() -> Result<serde_json::Value, String> {
    let registry = NerModelRegistry::new();

    Ok(serde_json::json!({
        "recommended": registry.get_recommended_model().map(|m| m.model_id.clone()),
        "fastest": registry.get_fastest_model().map(|m| m.model_id.clone()),
        "most_accurate": registry.get_most_accurate_model().map(|m| m.model_id.clone()),
        "multilingual": registry.get_multilingual_model().map(|m| m.model_id.clone()),
    }))
}

/// Cancel NER model download
#[tauri::command]
pub async fn cancel_ner_download() -> Result<String, String> {
    // TODO: Implement cancel functionality with shared state
    Ok("Download cancelled".to_string())
}

/// Get NER system status
#[tauri::command]
pub async fn get_ner_status(
    ner_manager: State<'_, Arc<Mutex<Option<NerModelManager>>>>,
) -> Result<serde_json::Value, String> {
    let manager_lock = ner_manager.lock().await;

    let model_loaded = if let Some(manager) = manager_lock.as_ref() {
        manager.is_loaded().await
    } else {
        false
    };

    let model_path = if let Some(manager) = manager_lock.as_ref() {
        manager.get_model_path().await
    } else {
        None
    };

    Ok(serde_json::json!({
        "model_loaded": model_loaded,
        "model_path": model_path,
        "system_ready": model_loaded,
    }))
}
