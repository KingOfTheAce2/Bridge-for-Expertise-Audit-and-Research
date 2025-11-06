use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::Client;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

/// Download progress information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DownloadProgress {
    pub model_id: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub percentage: f64,
    pub speed_mbps: f64,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DownloadStatus {
    Starting,
    Downloading,
    Completed,
    Failed,
    Cancelled,
}

/// Model downloader with progress tracking
pub struct ModelDownloader {
    client: Client,
    models_dir: PathBuf,
}

impl ModelDownloader {
    pub fn new(models_dir: PathBuf) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout
            .build()?;

        Ok(Self { client, models_dir })
    }

    /// Get the default models directory
    pub fn default_models_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .context("Could not determine home directory")?;

        let models_dir = home_dir.join(".bear-llm-ai").join("models");
        Ok(models_dir)
    }

    /// Download a model from a URL
    pub async fn download_model(
        &self,
        model_id: &str,
        download_url: &str,
        progress_callback: impl Fn(DownloadProgress) + Send + 'static,
    ) -> Result<PathBuf> {
        // Ensure models directory exists
        fs::create_dir_all(&self.models_dir).await?;

        // Generate filename from model_id
        let filename = self.generate_filename(model_id);
        let file_path = self.models_dir.join(&filename);

        // Send starting status
        progress_callback(DownloadProgress {
            model_id: model_id.to_string(),
            downloaded_bytes: 0,
            total_bytes: 0,
            percentage: 0.0,
            speed_mbps: 0.0,
            status: DownloadStatus::Starting,
        });

        // Start download
        let response = self
            .client
            .get(download_url)
            .send()
            .await
            .context("Failed to start download")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        let total_bytes = response.content_length().unwrap_or(0);
        let mut downloaded_bytes = 0u64;
        let mut file = File::create(&file_path)
            .await
            .context("Failed to create file")?;

        let mut stream = response.bytes_stream();
        let start_time = std::time::Instant::now();

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.context("Error while downloading")?;

            file.write_all(&chunk)
                .await
                .context("Failed to write to file")?;

            downloaded_bytes += chunk.len() as u64;

            // Calculate progress
            let percentage = if total_bytes > 0 {
                (downloaded_bytes as f64 / total_bytes as f64) * 100.0
            } else {
                0.0
            };

            // Calculate speed
            let elapsed_secs = start_time.elapsed().as_secs_f64();
            let speed_mbps = if elapsed_secs > 0.0 {
                (downloaded_bytes as f64 / 1_000_000.0) / elapsed_secs
            } else {
                0.0
            };

            // Send progress update
            progress_callback(DownloadProgress {
                model_id: model_id.to_string(),
                downloaded_bytes,
                total_bytes,
                percentage,
                speed_mbps,
                status: DownloadStatus::Downloading,
            });
        }

        file.flush().await?;

        // Send completion status
        progress_callback(DownloadProgress {
            model_id: model_id.to_string(),
            downloaded_bytes,
            total_bytes,
            percentage: 100.0,
            speed_mbps: 0.0,
            status: DownloadStatus::Completed,
        });

        Ok(file_path)
    }

    /// Generate a safe filename from model_id
    fn generate_filename(&self, model_id: &str) -> String {
        // Replace slashes and special characters with underscores
        let safe_name = model_id
            .replace('/', "_")
            .replace('\\', "_")
            .replace(' ', "_");

        // Add .gguf extension if not present
        if safe_name.ends_with(".gguf") {
            safe_name
        } else {
            format!("{}.gguf", safe_name)
        }
    }

    /// Delete a downloaded model
    pub async fn delete_model(&self, file_path: &Path) -> Result<()> {
        if file_path.exists() {
            fs::remove_file(file_path)
                .await
                .context("Failed to delete model file")?;
        }
        Ok(())
    }

    /// Get the size of a downloaded model
    pub async fn get_model_size(&self, file_path: &Path) -> Result<u64> {
        let metadata = fs::metadata(file_path)
            .await
            .context("Failed to get file metadata")?;
        Ok(metadata.len())
    }

    /// Check if a model file exists
    pub async fn model_exists(&self, model_id: &str) -> bool {
        let filename = self.generate_filename(model_id);
        let file_path = self.models_dir.join(&filename);
        file_path.exists()
    }

    /// List all downloaded model files
    pub async fn list_downloaded_models(&self) -> Result<Vec<PathBuf>> {
        let mut models = Vec::new();

        if !self.models_dir.exists() {
            return Ok(models);
        }

        let mut entries = fs::read_dir(&self.models_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("gguf") {
                models.push(path);
            }
        }

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename() {
        let temp_dir = PathBuf::from("/tmp/test_models");
        let downloader = ModelDownloader::new(temp_dir).unwrap();

        let filename = downloader.generate_filename("mistralai/Mistral-7B-Instruct-v0.2");
        assert_eq!(filename, "mistralai_Mistral-7B-Instruct-v0.2.gguf");

        let filename = downloader.generate_filename("model.gguf");
        assert_eq!(filename, "model.gguf");
    }
}
