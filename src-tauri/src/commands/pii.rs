use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

use crate::pii::{AnonymizationResult, AnonymizationSettings, Anonymizer, EntityType};

// Global state for anonymizer (to maintain consistent replacements across calls)
type AnonymizerState = Arc<Mutex<Anonymizer>>;

/// Request for anonymizing text
#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymizeRequest {
    pub text: String,
    pub settings: Option<AnonymizationSettings>,
}

/// Request for batch anonymization
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchAnonymizeRequest {
    pub texts: Vec<String>,
    pub settings: Option<AnonymizationSettings>,
}

/// Statistics about detected entities
#[derive(Debug, Serialize, Deserialize)]
pub struct EntityStatistics {
    pub entity_counts: Vec<(String, usize)>,
    pub total_entities: usize,
}

/// Anonymize text
#[tauri::command]
pub async fn anonymize_text(
    request: AnonymizeRequest,
    anonymizer: State<'_, AnonymizerState>,
) -> Result<AnonymizationResult, String> {
    let mut anon = anonymizer.lock().await;
    let settings = request.settings.unwrap_or_default();

    let result = anon.anonymize(&request.text, &settings);

    Ok(result)
}

/// Anonymize multiple texts while maintaining consistency
#[tauri::command]
pub async fn anonymize_batch(
    request: BatchAnonymizeRequest,
    anonymizer: State<'_, AnonymizerState>,
) -> Result<Vec<AnonymizationResult>, String> {
    let mut anon = anonymizer.lock().await;
    let settings = request.settings.unwrap_or_default();

    let results = anon.anonymize_batch(request.texts, &settings);

    Ok(results)
}

/// Clear replacement mappings (start fresh)
#[tauri::command]
pub async fn clear_pii_replacements(
    anonymizer: State<'_, AnonymizerState>,
) -> Result<String, String> {
    let mut anon = anonymizer.lock().await;
    anon.clear_replacements();

    Ok("Replacement mappings cleared".to_string())
}

/// Get statistics about detected entities
#[tauri::command]
pub async fn get_pii_statistics(
    anonymizer: State<'_, AnonymizerState>,
) -> Result<EntityStatistics, String> {
    let anon = anonymizer.lock().await;
    let stats = anon.get_statistics();

    let total_entities: usize = stats.values().sum();
    let entity_counts: Vec<(String, usize)> = stats
        .into_iter()
        .map(|(et, count)| (et.as_str().to_string(), count))
        .collect();

    Ok(EntityStatistics {
        entity_counts,
        total_entities,
    })
}

/// Get default anonymization settings
#[tauri::command]
pub fn get_default_pii_settings() -> AnonymizationSettings {
    AnonymizationSettings::default()
}

/// Get available entity types
#[tauri::command]
pub fn get_entity_types() -> Vec<String> {
    vec![
        EntityType::Person.as_str().to_string(),
        EntityType::Organization.as_str().to_string(),
        EntityType::Location.as_str().to_string(),
        EntityType::Date.as_str().to_string(),
        EntityType::Money.as_str().to_string(),
        EntityType::Email.as_str().to_string(),
        EntityType::Phone.as_str().to_string(),
        EntityType::Case.as_str().to_string(),
        EntityType::Identification.as_str().to_string(),
        EntityType::TechnicalIdentifier.as_str().to_string(),
        EntityType::Law.as_str().to_string(),
    ]
}

/// Detect entities without anonymizing
#[tauri::command]
pub async fn detect_pii_entities(
    text: String,
    anonymizer: State<'_, AnonymizerState>,
) -> Result<Vec<crate::pii::Entity>, String> {
    let anon = anonymizer.lock().await;
    let settings = AnonymizationSettings::default();

    // Just detect, don't anonymize
    let result = anon.detector.detect(&text);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_anonymize_text_command() {
        let anonymizer_state: AnonymizerState = Arc::new(Mutex::new(Anonymizer::new()));

        let request = AnonymizeRequest {
            text: "John Doe emailed jane@example.com.".to_string(),
            settings: None,
        };

        let result = anonymize_text(request, State::from(&anonymizer_state))
            .await
            .unwrap();

        assert!(!result.anonymized_text.contains("John Doe"));
        assert!(!result.anonymized_text.contains("jane@example.com"));
        assert!(!result.entities.is_empty());
    }
}
