//! HTTP client for communicating with Presidio REST API
//!
//! Handles all HTTP communication with the Presidio analyzer and anonymizer
//! services running in Docker containers on localhost.

use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

use super::docker::{ANALYZER_PORT, ANONYMIZER_PORT};
use super::types::{
    AnonymizationOperator, PresidioAnalyzeRequest, PresidioAnonymizeRequest,
    PresidioAnonymizeResult, PresidioEntity,
};

/// HTTP client for Presidio API communication
pub struct PresidioClient {
    client: Client,
    analyzer_url: String,
    anonymizer_url: String,
}

impl PresidioClient {
    /// Create a new Presidio client with default localhost endpoints
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            analyzer_url: format!("http://127.0.0.1:{}", ANALYZER_PORT),
            anonymizer_url: format!("http://127.0.0.1:{}", ANONYMIZER_PORT),
        }
    }

    /// Create a client with custom endpoints
    pub fn with_endpoints(analyzer_url: String, anonymizer_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            analyzer_url,
            anonymizer_url,
        }
    }

    /// Health check for the analyzer service
    pub async fn health_check(&self) -> Result<()> {
        let url = format!("{}/health", self.analyzer_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Presidio analyzer")?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "Presidio analyzer health check failed with status: {}",
                response.status()
            )
        }
    }

    /// Health check for the anonymizer service
    pub async fn anonymizer_health_check(&self) -> Result<()> {
        let url = format!("{}/health", self.anonymizer_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to connect to Presidio anonymizer")?;

        if response.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!(
                "Presidio anonymizer health check failed with status: {}",
                response.status()
            )
        }
    }

    /// Analyze text for PII entities
    pub async fn analyze(&self, text: &str, language: &str) -> Result<Vec<PresidioEntity>> {
        let url = format!("{}/analyze", self.analyzer_url);

        let request = PresidioAnalyzeRequest {
            text: text.to_string(),
            language: language.to_string(),
            entities: None,
            score_threshold: Some(0.5),
            return_decision_process: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send analyze request to Presidio")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Presidio analyze failed with status {}: {}",
                status,
                error_text
            );
        }

        let entities: Vec<PresidioEntity> = response
            .json()
            .await
            .context("Failed to parse Presidio analyze response")?;

        Ok(entities)
    }

    /// Analyze text with specific entity types
    pub async fn analyze_with_entities(
        &self,
        text: &str,
        language: &str,
        entity_types: Vec<String>,
        score_threshold: Option<f64>,
    ) -> Result<Vec<PresidioEntity>> {
        let url = format!("{}/analyze", self.analyzer_url);

        let request = PresidioAnalyzeRequest {
            text: text.to_string(),
            language: language.to_string(),
            entities: Some(entity_types),
            score_threshold: score_threshold.or(Some(0.5)),
            return_decision_process: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send analyze request to Presidio")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Presidio analyze failed with status {}: {}",
                status,
                error_text
            );
        }

        let entities: Vec<PresidioEntity> = response
            .json()
            .await
            .context("Failed to parse Presidio analyze response")?;

        Ok(entities)
    }

    /// Anonymize text based on detected PII
    pub async fn anonymize(
        &self,
        text: &str,
        language: &str,
        operators: Option<Vec<AnonymizationOperator>>,
    ) -> Result<PresidioAnonymizeResult> {
        // First, analyze the text to find PII
        let entities = self.analyze(text, language).await?;

        if entities.is_empty() {
            // No PII found, return original text
            return Ok(PresidioAnonymizeResult {
                text: text.to_string(),
                items: vec![],
            });
        }

        // Build operators map
        let operators_map = if let Some(ops) = operators {
            let mut map = std::collections::HashMap::new();
            // Apply same operator to all entity types for simplicity
            // Can be extended to map specific operators to specific types
            for entity in &entities {
                if !map.contains_key(&entity.entity_type) {
                    if let Some(op) = ops.first() {
                        map.insert(entity.entity_type.clone(), op.clone());
                    }
                }
            }
            Some(map)
        } else {
            None
        };

        let url = format!("{}/anonymize", self.anonymizer_url);

        let request = PresidioAnonymizeRequest {
            text: text.to_string(),
            analyzer_results: entities,
            operators: operators_map,
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send anonymize request to Presidio")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Presidio anonymize failed with status {}: {}",
                status,
                error_text
            );
        }

        let result: PresidioAnonymizeResult = response
            .json()
            .await
            .context("Failed to parse Presidio anonymize response")?;

        Ok(result)
    }

    /// Get supported entity types from the analyzer
    pub async fn get_supported_entities(&self) -> Result<Vec<String>> {
        let url = format!("{}/supportedentities", self.analyzer_url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get supported entities from Presidio")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to get supported entities with status {}: {}",
                status,
                error_text
            );
        }

        let entities: Vec<String> = response
            .json()
            .await
            .context("Failed to parse supported entities response")?;

        Ok(entities)
    }

    /// Get recognizers configured in the analyzer
    pub async fn get_recognizers(&self, language: Option<&str>) -> Result<Vec<RecognizerInfo>> {
        let mut url = format!("{}/recognizers", self.analyzer_url);

        if let Some(lang) = language {
            url = format!("{}?language={}", url, lang);
        }

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to get recognizers from Presidio")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to get recognizers with status {}: {}",
                status,
                error_text
            );
        }

        let recognizers: Vec<RecognizerInfo> = response
            .json()
            .await
            .context("Failed to parse recognizers response")?;

        Ok(recognizers)
    }

    /// Get analyzer URL
    pub fn analyzer_url(&self) -> &str {
        &self.analyzer_url
    }

    /// Get anonymizer URL
    pub fn anonymizer_url(&self) -> &str {
        &self.anonymizer_url
    }
}

impl Default for PresidioClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about a recognizer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecognizerInfo {
    /// Recognizer name
    pub name: String,
    /// Supported entities
    pub supported_entities: Vec<String>,
    /// Supported language
    #[serde(default)]
    pub supported_language: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = PresidioClient::new();
        assert!(client.analyzer_url.contains("5002"));
        assert!(client.anonymizer_url.contains("5001"));
    }

    #[test]
    fn test_custom_endpoints() {
        let client = PresidioClient::with_endpoints(
            "http://custom:8080".to_string(),
            "http://custom:8081".to_string(),
        );
        assert_eq!(client.analyzer_url, "http://custom:8080");
        assert_eq!(client.anonymizer_url, "http://custom:8081");
    }
}
