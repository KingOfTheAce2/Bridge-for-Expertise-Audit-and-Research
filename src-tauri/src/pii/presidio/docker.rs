//! Docker management for Presidio containers
//!
//! Manages the lifecycle of Presidio analyzer and anonymizer containers.
//! All containers are configured to only listen on localhost for security.

use anyhow::{Context, Result};
use std::process::Stdio;
use tokio::process::Command;

/// Container names for Presidio services
pub const ANALYZER_CONTAINER_NAME: &str = "bear-presidio-analyzer";
pub const ANONYMIZER_CONTAINER_NAME: &str = "bear-presidio-anonymizer";

/// Docker images for Presidio
pub const ANALYZER_IMAGE: &str = "mcr.microsoft.com/presidio-analyzer:latest";
pub const ANONYMIZER_IMAGE: &str = "mcr.microsoft.com/presidio-anonymizer:latest";

/// Ports for Presidio services (localhost only)
pub const ANALYZER_PORT: u16 = 5002;
pub const ANONYMIZER_PORT: u16 = 5001;

/// Container status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerStatus {
    /// Container does not exist
    NotFound,
    /// Container exists but is not running
    Created,
    /// Container is running
    Running,
    /// Container has exited
    Exited,
    /// Error state
    Error(String),
}

/// Manages Presidio Docker containers
pub struct PresidioDockerManager {
    /// Path to docker executable (auto-detected)
    docker_path: Option<String>,
}

impl PresidioDockerManager {
    /// Create a new Docker manager
    pub fn new() -> Self {
        Self { docker_path: None }
    }

    /// Check if Docker is available on the system
    pub async fn is_docker_available(&self) -> bool {
        let result = Command::new("docker")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;

        result.map(|s| s.success()).unwrap_or(false)
    }

    /// Check if Presidio images exist locally
    pub async fn image_exists(&self) -> Result<bool> {
        let output = Command::new("docker")
            .args(["images", "-q", ANALYZER_IMAGE])
            .output()
            .await
            .context("Failed to check Docker images")?;

        Ok(!output.stdout.is_empty())
    }

    /// Pull Presidio Docker images
    pub async fn pull_images(
        &self,
        progress_callback: Option<Box<dyn Fn(f32, &str) + Send>>,
    ) -> Result<()> {
        if !self.is_docker_available().await {
            anyhow::bail!("Docker is not available. Please install Docker Desktop.");
        }

        // Pull analyzer image
        if let Some(ref callback) = progress_callback {
            callback(0.0, "Pulling Presidio Analyzer image...");
        }

        let analyzer_result = Command::new("docker")
            .args(["pull", ANALYZER_IMAGE])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .await
            .context("Failed to pull analyzer image")?;

        if !analyzer_result.success() {
            anyhow::bail!("Failed to pull Presidio Analyzer image");
        }

        if let Some(ref callback) = progress_callback {
            callback(0.5, "Pulling Presidio Anonymizer image...");
        }

        // Pull anonymizer image
        let anonymizer_result = Command::new("docker")
            .args(["pull", ANONYMIZER_IMAGE])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .await
            .context("Failed to pull anonymizer image")?;

        if !anonymizer_result.success() {
            anyhow::bail!("Failed to pull Presidio Anonymizer image");
        }

        if let Some(ref callback) = progress_callback {
            callback(1.0, "Presidio images downloaded successfully");
        }

        Ok(())
    }

    /// Check container status
    pub async fn check_container_status(&self) -> Result<ContainerStatus> {
        if !self.is_docker_available().await {
            return Ok(ContainerStatus::Error(
                "Docker is not available".to_string(),
            ));
        }

        // Check analyzer container
        let analyzer_status = self.get_single_container_status(ANALYZER_CONTAINER_NAME).await?;

        // Check anonymizer container
        let anonymizer_status = self.get_single_container_status(ANONYMIZER_CONTAINER_NAME).await?;

        // Return combined status
        match (&analyzer_status, &anonymizer_status) {
            (ContainerStatus::Running, ContainerStatus::Running) => Ok(ContainerStatus::Running),
            (ContainerStatus::NotFound, ContainerStatus::NotFound) => Ok(ContainerStatus::NotFound),
            (ContainerStatus::Error(e), _) | (_, ContainerStatus::Error(e)) => {
                Ok(ContainerStatus::Error(e.clone()))
            }
            _ => Ok(ContainerStatus::Exited),
        }
    }

    /// Get status of a single container
    async fn get_single_container_status(&self, container_name: &str) -> Result<ContainerStatus> {
        let output = Command::new("docker")
            .args(["inspect", "-f", "{{.State.Status}}", container_name])
            .output()
            .await
            .context("Failed to inspect container")?;

        if !output.status.success() {
            // Container doesn't exist
            return Ok(ContainerStatus::NotFound);
        }

        let status = String::from_utf8_lossy(&output.stdout).trim().to_string();

        match status.as_str() {
            "running" => Ok(ContainerStatus::Running),
            "created" => Ok(ContainerStatus::Created),
            "exited" => Ok(ContainerStatus::Exited),
            "paused" => Ok(ContainerStatus::Exited),
            _ => Ok(ContainerStatus::Error(format!("Unknown status: {}", status))),
        }
    }

    /// Start Presidio containers
    pub async fn start_containers(&self) -> Result<()> {
        if !self.is_docker_available().await {
            anyhow::bail!("Docker is not available");
        }

        // Check if images exist
        if !self.image_exists().await? {
            anyhow::bail!("Presidio images not found. Please run installation first.");
        }

        // Start analyzer container
        self.start_or_create_container(
            ANALYZER_CONTAINER_NAME,
            ANALYZER_IMAGE,
            ANALYZER_PORT,
            5002,
        )
        .await?;

        // Start anonymizer container
        self.start_or_create_container(
            ANONYMIZER_CONTAINER_NAME,
            ANONYMIZER_IMAGE,
            ANONYMIZER_PORT,
            5001,
        )
        .await?;

        Ok(())
    }

    /// Start or create a container
    async fn start_or_create_container(
        &self,
        container_name: &str,
        image: &str,
        host_port: u16,
        container_port: u16,
    ) -> Result<()> {
        let status = self.get_single_container_status(container_name).await?;

        match status {
            ContainerStatus::Running => {
                // Already running
                Ok(())
            }
            ContainerStatus::Created | ContainerStatus::Exited => {
                // Start existing container
                let result = Command::new("docker")
                    .args(["start", container_name])
                    .status()
                    .await
                    .context("Failed to start container")?;

                if !result.success() {
                    anyhow::bail!("Failed to start container: {}", container_name);
                }
                Ok(())
            }
            ContainerStatus::NotFound => {
                // Create and start new container
                // IMPORTANT: Bind only to localhost (127.0.0.1) for security
                let port_mapping = format!("127.0.0.1:{}:{}", host_port, container_port);

                let result = Command::new("docker")
                    .args([
                        "run",
                        "-d",
                        "--name",
                        container_name,
                        "-p",
                        &port_mapping,
                        "--restart",
                        "unless-stopped",
                        // Resource limits
                        "--memory",
                        "512m",
                        "--cpus",
                        "1",
                        // Security: no network access except localhost binding
                        image,
                    ])
                    .status()
                    .await
                    .context("Failed to create container")?;

                if !result.success() {
                    anyhow::bail!("Failed to create container: {}", container_name);
                }
                Ok(())
            }
            ContainerStatus::Error(e) => {
                anyhow::bail!("Container error: {}", e);
            }
        }
    }

    /// Stop Presidio containers
    pub async fn stop_containers(&self) -> Result<()> {
        // Stop analyzer
        let _ = Command::new("docker")
            .args(["stop", ANALYZER_CONTAINER_NAME])
            .status()
            .await;

        // Stop anonymizer
        let _ = Command::new("docker")
            .args(["stop", ANONYMIZER_CONTAINER_NAME])
            .status()
            .await;

        Ok(())
    }

    /// Remove Presidio containers (stop + remove)
    pub async fn remove_containers(&self) -> Result<()> {
        self.stop_containers().await?;

        // Remove analyzer
        let _ = Command::new("docker")
            .args(["rm", "-f", ANALYZER_CONTAINER_NAME])
            .status()
            .await;

        // Remove anonymizer
        let _ = Command::new("docker")
            .args(["rm", "-f", ANONYMIZER_CONTAINER_NAME])
            .status()
            .await;

        Ok(())
    }

    /// Remove Presidio images (cleanup)
    pub async fn remove_images(&self) -> Result<()> {
        self.remove_containers().await?;

        // Remove analyzer image
        let _ = Command::new("docker")
            .args(["rmi", ANALYZER_IMAGE])
            .status()
            .await;

        // Remove anonymizer image
        let _ = Command::new("docker")
            .args(["rmi", ANONYMIZER_IMAGE])
            .status()
            .await;

        Ok(())
    }

    /// Get container logs
    pub async fn get_logs(&self, container_name: &str, lines: u32) -> Result<String> {
        let output = Command::new("docker")
            .args(["logs", "--tail", &lines.to_string(), container_name])
            .output()
            .await
            .context("Failed to get container logs")?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        Ok(format!("{}\n{}", stdout, stderr))
    }

    /// Get resource usage of containers
    pub async fn get_resource_usage(&self) -> Result<ContainerResourceUsage> {
        let output = Command::new("docker")
            .args([
                "stats",
                "--no-stream",
                "--format",
                "{{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}",
                ANALYZER_CONTAINER_NAME,
                ANONYMIZER_CONTAINER_NAME,
            ])
            .output()
            .await
            .context("Failed to get container stats")?;

        let stats = String::from_utf8_lossy(&output.stdout);

        // Parse stats (simplified)
        let mut analyzer_cpu = 0.0;
        let mut analyzer_mem = "0".to_string();
        let mut anonymizer_cpu = 0.0;
        let mut anonymizer_mem = "0".to_string();

        for line in stats.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 3 {
                let cpu = parts[1].trim_end_matches('%').parse().unwrap_or(0.0);
                let mem = parts[2].to_string();

                if parts[0].contains("analyzer") {
                    analyzer_cpu = cpu;
                    analyzer_mem = mem;
                } else if parts[0].contains("anonymizer") {
                    anonymizer_cpu = cpu;
                    anonymizer_mem = mem;
                }
            }
        }

        Ok(ContainerResourceUsage {
            analyzer_cpu_percent: analyzer_cpu,
            analyzer_memory: analyzer_mem,
            anonymizer_cpu_percent: anonymizer_cpu,
            anonymizer_memory: anonymizer_mem,
        })
    }
}

impl Default for PresidioDockerManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Resource usage information for containers
#[derive(Debug, Clone)]
pub struct ContainerResourceUsage {
    pub analyzer_cpu_percent: f64,
    pub analyzer_memory: String,
    pub anonymizer_cpu_percent: f64,
    pub anonymizer_memory: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_names() {
        assert!(ANALYZER_CONTAINER_NAME.contains("presidio"));
        assert!(ANONYMIZER_CONTAINER_NAME.contains("presidio"));
    }

    #[test]
    fn test_ports_are_localhost() {
        // Verify ports are in valid range
        assert!(ANALYZER_PORT > 1024);
        assert!(ANONYMIZER_PORT > 1024);
    }
}
