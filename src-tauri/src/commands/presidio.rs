//! Tauri commands for Presidio integration (Layer 3 PII detection)
//!
//! These commands manage the Presidio Docker containers and provide
//! advanced PII detection capabilities.

// Allow dead code - these are API commands that will be used from frontend
#![allow(dead_code)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::pii::presidio::{
    AnonymizationOperator, PresidioAnonymizeResult, PresidioConfig, PresidioEntity,
    PresidioManager, PresidioStatus,
};

// Global state for Presidio manager
pub type PresidioState = Arc<Mutex<PresidioManager>>;

/// Presidio status response
#[derive(Debug, Serialize, Deserialize)]
pub struct PresidioStatusResponse {
    pub status: String,
    pub is_enabled: bool,
    pub docker_available: bool,
    pub message: String,
}

/// Analyze request
#[derive(Debug, Serialize, Deserialize)]
pub struct PresidioAnalyzeRequest {
    pub text: String,
    pub language: Option<String>,
    pub entity_types: Option<Vec<String>>,
    pub score_threshold: Option<f64>,
}

/// Anonymize request
#[derive(Debug, Serialize, Deserialize)]
pub struct PresidioAnonymizeRequest {
    pub text: String,
    pub language: Option<String>,
}

/// Installation progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallProgress {
    pub progress: f32,
    pub message: String,
}

/// Get Presidio status
#[tauri::command]
pub async fn get_presidio_status(
    presidio: State<'_, PresidioState>,
) -> Result<PresidioStatusResponse, String> {
    let manager = presidio.lock().await;

    let docker_available = manager.is_docker_available().await;
    let is_enabled = manager.is_enabled().await;

    let status = match manager.check_status().await {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to check status: {}", e)),
    };

    let (status_str, message) = match status {
        PresidioStatus::NotInstalled => (
            "not_installed".to_string(),
            "Presidio is not installed. Docker images need to be downloaded.".to_string(),
        ),
        PresidioStatus::Stopped => (
            "stopped".to_string(),
            "Presidio is installed but not running.".to_string(),
        ),
        PresidioStatus::Starting => (
            "starting".to_string(),
            "Presidio is starting up...".to_string(),
        ),
        PresidioStatus::Running => (
            "running".to_string(),
            "Presidio is running and ready.".to_string(),
        ),
        PresidioStatus::Error(e) => (
            "error".to_string(),
            format!("Presidio error: {}", e),
        ),
    };

    Ok(PresidioStatusResponse {
        status: status_str,
        is_enabled,
        docker_available,
        message,
    })
}

/// Check if Docker is available
#[tauri::command]
pub async fn is_docker_available(
    presidio: State<'_, PresidioState>,
) -> Result<bool, String> {
    let manager = presidio.lock().await;
    Ok(manager.is_docker_available().await)
}

/// Install Presidio (pull Docker images)
#[tauri::command]
pub async fn install_presidio(
    presidio: State<'_, PresidioState>,
) -> Result<String, String> {
    let manager = presidio.lock().await;

    // Check Docker first
    if !manager.is_docker_available().await {
        return Err("Docker is not available. Please install Docker Desktop first.".to_string());
    }

    // Install without progress callback for now (can be enhanced later)
    match manager.install(None).await {
        Ok(_) => Ok("Presidio installed successfully".to_string()),
        Err(e) => Err(format!("Failed to install Presidio: {}", e)),
    }
}

/// Start Presidio containers
#[tauri::command]
pub async fn start_presidio(
    presidio: State<'_, PresidioState>,
) -> Result<String, String> {
    let manager = presidio.lock().await;

    match manager.start().await {
        Ok(_) => Ok("Presidio started successfully".to_string()),
        Err(e) => Err(format!("Failed to start Presidio: {}", e)),
    }
}

/// Stop Presidio containers
#[tauri::command]
pub async fn stop_presidio(
    presidio: State<'_, PresidioState>,
) -> Result<String, String> {
    let manager = presidio.lock().await;

    match manager.stop().await {
        Ok(_) => Ok("Presidio stopped successfully".to_string()),
        Err(e) => Err(format!("Failed to stop Presidio: {}", e)),
    }
}

/// Enable Presidio integration
#[tauri::command]
pub async fn enable_presidio(
    presidio: State<'_, PresidioState>,
) -> Result<String, String> {
    let manager = presidio.lock().await;

    match manager.enable().await {
        Ok(_) => Ok("Presidio enabled successfully".to_string()),
        Err(e) => Err(format!("Failed to enable Presidio: {}", e)),
    }
}

/// Disable Presidio integration
#[tauri::command]
pub async fn disable_presidio(
    presidio: State<'_, PresidioState>,
) -> Result<(), String> {
    let manager = presidio.lock().await;
    manager.disable().await;
    Ok(())
}

/// Analyze text for PII using Presidio
#[tauri::command]
pub async fn presidio_analyze(
    request: PresidioAnalyzeRequest,
    presidio: State<'_, PresidioState>,
) -> Result<Vec<PresidioEntity>, String> {
    let manager = presidio.lock().await;

    if !manager.is_enabled().await {
        return Err("Presidio is not enabled. Enable it first.".to_string());
    }

    let language = request.language.unwrap_or_else(|| "en".to_string());

    match manager.analyze(&request.text, &language).await {
        Ok(entities) => Ok(entities),
        Err(e) => Err(format!("Analysis failed: {}", e)),
    }
}

/// Anonymize text using Presidio
#[tauri::command]
pub async fn presidio_anonymize(
    request: PresidioAnonymizeRequest,
    presidio: State<'_, PresidioState>,
) -> Result<PresidioAnonymizeResult, String> {
    let manager = presidio.lock().await;

    if !manager.is_enabled().await {
        return Err("Presidio is not enabled. Enable it first.".to_string());
    }

    let language = request.language.unwrap_or_else(|| "en".to_string());

    match manager.anonymize(&request.text, &language, None).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Anonymization failed: {}", e)),
    }
}

/// Get supported entity types from Presidio
#[tauri::command]
pub async fn get_presidio_entity_types(
    presidio: State<'_, PresidioState>,
) -> Result<Vec<String>, String> {
    let manager = presidio.lock().await;

    if !manager.is_enabled().await {
        // Return default list if not enabled
        return Ok(vec![
            "PERSON".to_string(),
            "LOCATION".to_string(),
            "ORGANIZATION".to_string(),
            "EMAIL_ADDRESS".to_string(),
            "PHONE_NUMBER".to_string(),
            "CREDIT_CARD".to_string(),
            "IBAN_CODE".to_string(),
            "US_SSN".to_string(),
            "DATE_TIME".to_string(),
            "IP_ADDRESS".to_string(),
            "URL".to_string(),
        ]);
    }

    match manager.get_supported_entities().await {
        Ok(entities) => Ok(entities),
        Err(e) => Err(format!("Failed to get entity types: {}", e)),
    }
}

/// Get supported languages
#[tauri::command]
pub async fn get_presidio_languages(
    presidio: State<'_, PresidioState>,
) -> Result<Vec<String>, String> {
    let manager = presidio.lock().await;

    match manager.get_supported_languages().await {
        Ok(languages) => Ok(languages),
        Err(e) => Err(format!("Failed to get languages: {}", e)),
    }
}

/// Get default Presidio configuration
#[tauri::command]
pub fn get_presidio_config() -> PresidioConfig {
    PresidioConfig::default()
}

/// Check if Presidio is enabled
#[tauri::command]
pub async fn is_presidio_enabled(
    presidio: State<'_, PresidioState>,
) -> Result<bool, String> {
    let manager = presidio.lock().await;
    Ok(manager.is_enabled().await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_response_serialization() {
        let response = PresidioStatusResponse {
            status: "running".to_string(),
            is_enabled: true,
            docker_available: true,
            message: "Presidio is running".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("running"));
    }

    #[test]
    fn test_analyze_request() {
        let request = PresidioAnalyzeRequest {
            text: "John Doe".to_string(),
            language: Some("en".to_string()),
            entity_types: None,
            score_threshold: Some(0.5),
        };

        assert_eq!(request.language, Some("en".to_string()));
    }
}
