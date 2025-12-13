//! Presidio integration module for advanced PII detection (Layer 3)
//!
//! This module provides optional integration with Microsoft Presidio for
//! advanced anonymization capabilities. Presidio runs in a local Docker
//! container and communicates via REST API on localhost only.
//!
//! Features:
//! - 50+ PII entity types
//! - Custom recognizer support
//! - Multi-language support
//! - Advanced anonymization strategies
//! - Cross-document entity resolution
//!
//! Note: This is an optional enhancement. The application falls back to
//! Layer 1 (regex) + Layer 2 (NER) when Presidio is unavailable.

// Allow dead code for this module - it's an API that will be used from frontend
#![allow(dead_code)]

pub mod types;
pub mod docker;
pub mod client;
pub mod mapping;

pub use types::*;
pub use docker::PresidioDockerManager;
pub use client::PresidioClient;
pub use mapping::EntityTypeMapper;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Presidio integration status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresidioStatus {
    /// Not installed (Docker image not present)
    NotInstalled,
    /// Installed but container not running
    Stopped,
    /// Container is starting up
    Starting,
    /// Container is running and healthy
    Running,
    /// Container is unhealthy or errored
    Error(String),
}

/// Main Presidio integration manager
pub struct PresidioManager {
    docker_manager: Arc<PresidioDockerManager>,
    client: Arc<PresidioClient>,
    status: Arc<RwLock<PresidioStatus>>,
    enabled: Arc<RwLock<bool>>,
}

impl PresidioManager {
    /// Create a new Presidio manager
    pub fn new() -> Self {
        let docker_manager = Arc::new(PresidioDockerManager::new());
        let client = Arc::new(PresidioClient::new());

        Self {
            docker_manager,
            client,
            status: Arc::new(RwLock::new(PresidioStatus::NotInstalled)),
            enabled: Arc::new(RwLock::new(false)),
        }
    }

    /// Check current status of Presidio
    pub async fn check_status(&self) -> Result<PresidioStatus> {
        let docker_status = self.docker_manager.check_container_status().await?;

        let status = match docker_status {
            docker::ContainerStatus::NotFound => {
                // Check if image exists
                if self.docker_manager.image_exists().await? {
                    PresidioStatus::Stopped
                } else {
                    PresidioStatus::NotInstalled
                }
            }
            docker::ContainerStatus::Created | docker::ContainerStatus::Exited => {
                PresidioStatus::Stopped
            }
            docker::ContainerStatus::Running => {
                // Verify health via API
                if self.client.health_check().await.is_ok() {
                    PresidioStatus::Running
                } else {
                    PresidioStatus::Starting
                }
            }
            docker::ContainerStatus::Error(msg) => PresidioStatus::Error(msg),
        };

        // Update cached status
        let mut status_lock = self.status.write().await;
        *status_lock = status.clone();

        Ok(status)
    }

    /// Get cached status (does not query Docker)
    pub async fn get_cached_status(&self) -> PresidioStatus {
        self.status.read().await.clone()
    }

    /// Check if Presidio is enabled
    pub async fn is_enabled(&self) -> bool {
        *self.enabled.read().await
    }

    /// Enable Presidio integration
    pub async fn enable(&self) -> Result<()> {
        // First check if it's available
        let status = self.check_status().await?;

        match status {
            PresidioStatus::Running => {
                let mut enabled = self.enabled.write().await;
                *enabled = true;
                Ok(())
            }
            PresidioStatus::Stopped => {
                // Try to start the container
                self.start().await?;
                let mut enabled = self.enabled.write().await;
                *enabled = true;
                Ok(())
            }
            PresidioStatus::NotInstalled => {
                anyhow::bail!("Presidio is not installed. Please install Docker and run setup.")
            }
            PresidioStatus::Starting => {
                // Wait for it to be ready
                self.wait_for_ready().await?;
                let mut enabled = self.enabled.write().await;
                *enabled = true;
                Ok(())
            }
            PresidioStatus::Error(msg) => {
                anyhow::bail!("Presidio error: {}", msg)
            }
        }
    }

    /// Disable Presidio integration
    pub async fn disable(&self) {
        let mut enabled = self.enabled.write().await;
        *enabled = false;
    }

    /// Install Presidio (pull Docker images)
    pub async fn install(&self, progress_callback: Option<Box<dyn Fn(f32, &str) + Send>>) -> Result<()> {
        self.docker_manager.pull_images(progress_callback).await?;

        let mut status = self.status.write().await;
        *status = PresidioStatus::Stopped;

        Ok(())
    }

    /// Start Presidio containers
    pub async fn start(&self) -> Result<()> {
        {
            let mut status = self.status.write().await;
            *status = PresidioStatus::Starting;
        }

        self.docker_manager.start_containers().await?;
        self.wait_for_ready().await?;

        {
            let mut status = self.status.write().await;
            *status = PresidioStatus::Running;
        }

        Ok(())
    }

    /// Stop Presidio containers
    pub async fn stop(&self) -> Result<()> {
        self.docker_manager.stop_containers().await?;

        let mut status = self.status.write().await;
        *status = PresidioStatus::Stopped;

        Ok(())
    }

    /// Wait for Presidio to be ready
    async fn wait_for_ready(&self) -> Result<()> {
        let max_attempts = 30;
        let delay = tokio::time::Duration::from_secs(2);

        for _ in 0..max_attempts {
            if self.client.health_check().await.is_ok() {
                return Ok(());
            }
            tokio::time::sleep(delay).await;
        }

        anyhow::bail!("Presidio did not become ready within timeout")
    }

    /// Analyze text for PII using Presidio
    pub async fn analyze(&self, text: &str, language: &str) -> Result<Vec<PresidioEntity>> {
        if !self.is_enabled().await {
            anyhow::bail!("Presidio is not enabled")
        }

        self.client.analyze(text, language).await
    }

    /// Anonymize text using Presidio
    pub async fn anonymize(
        &self,
        text: &str,
        language: &str,
        operators: Option<Vec<AnonymizationOperator>>,
    ) -> Result<PresidioAnonymizeResult> {
        if !self.is_enabled().await {
            anyhow::bail!("Presidio is not enabled")
        }

        self.client.anonymize(text, language, operators).await
    }

    /// Get supported entity types
    pub async fn get_supported_entities(&self) -> Result<Vec<String>> {
        self.client.get_supported_entities().await
    }

    /// Get supported languages
    pub async fn get_supported_languages(&self) -> Result<Vec<String>> {
        // Presidio default supported languages
        Ok(vec![
            "en".to_string(),
            "de".to_string(),
            "es".to_string(),
            "fr".to_string(),
            "it".to_string(),
            "nl".to_string(),
            "pt".to_string(),
        ])
    }

    /// Check if Docker is available on the system
    pub async fn is_docker_available(&self) -> bool {
        self.docker_manager.is_docker_available().await
    }
}

impl Default for PresidioManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_presidio_manager_creation() {
        let manager = PresidioManager::new();
        assert!(!manager.is_enabled().await);
    }

    #[tokio::test]
    async fn test_presidio_status_default() {
        let manager = PresidioManager::new();
        let status = manager.get_cached_status().await;
        assert_eq!(status, PresidioStatus::NotInstalled);
    }
}
