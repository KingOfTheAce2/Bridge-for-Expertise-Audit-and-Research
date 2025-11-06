use anyhow::{Context, Result};
use futures::StreamExt;
use reqwest::Client;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::RwLock;

use super::types::NerModelInfo;

/// Download progress information
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub file_name: String,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub progress_percent: f64,
    pub speed_mbps: f64,
}

/// NER model downloader
pub struct NerModelDownloader {
    client: Client,
    models_dir: PathBuf,
    cancel_flag: Arc<RwLock<bool>>,
}

impl NerModelDownloader {
    /// Create new downloader
    pub fn new(models_dir: PathBuf) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minutes timeout
            .build()?;

        Ok(Self {
            client,
            models_dir,
            cancel_flag: Arc::new(RwLock::new(false)),
        })
    }

    /// Set cancel flag to stop download
    pub async fn cancel(&self) {
        let mut flag = self.cancel_flag.write().await;
        *flag = true;
    }

    /// Reset cancel flag
    pub async fn reset_cancel(&self) {
        let mut flag = self.cancel_flag.write().await;
        *flag = false;
    }

    /// Check if download was cancelled
    async fn is_cancelled(&self) -> bool {
        let flag = self.cancel_flag.read().await;
        *flag
    }

    /// Download a complete NER model (model weights + config + tokenizer)
    pub async fn download_model<F>(
        &self,
        model_info: &NerModelInfo,
        progress_callback: F,
    ) -> Result<PathBuf>
    where
        F: Fn(DownloadProgress) + Send + Sync + 'static,
    {
        // Reset cancel flag
        self.reset_cancel().await;

        // Create model directory
        let model_dir = self.models_dir.join(&model_info.model_id.replace('/', "_"));
        fs::create_dir_all(&model_dir).await?;

        // Download model weights
        let model_path = model_dir.join("model.safetensors");
        self.download_file(
            &model_info.model_url,
            &model_path,
            "model.safetensors",
            &progress_callback,
        )
        .await?;

        if self.is_cancelled().await {
            self.cleanup_partial_download(&model_dir).await?;
            anyhow::bail!("Download cancelled");
        }

        // Download config
        let config_path = model_dir.join("config.json");
        self.download_file(
            &model_info.config_url,
            &config_path,
            "config.json",
            &progress_callback,
        )
        .await?;

        if self.is_cancelled().await {
            self.cleanup_partial_download(&model_dir).await?;
            anyhow::bail!("Download cancelled");
        }

        // Download tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        self.download_file(
            &model_info.tokenizer_url,
            &tokenizer_path,
            "tokenizer.json",
            &progress_callback,
        )
        .await?;

        if self.is_cancelled().await {
            self.cleanup_partial_download(&model_dir).await?;
            anyhow::bail!("Download cancelled");
        }

        Ok(model_dir)
    }

    /// Download a single file with progress tracking
    async fn download_file<F>(
        &self,
        url: &str,
        dest_path: &Path,
        file_name: &str,
        progress_callback: &F,
    ) -> Result<()>
    where
        F: Fn(DownloadProgress) + Send + Sync,
    {
        // Use temporary file during download
        let temp_path = dest_path.with_extension("tmp");

        // Send request
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Download failed with status: {}", response.status());
        }

        // Get total size
        let total_bytes = response.content_length().unwrap_or(0);

        // Open file
        let mut file = fs::File::create(&temp_path)
            .await
            .context("Failed to create file")?;

        // Download with progress tracking
        let mut stream = response.bytes_stream();
        let mut downloaded_bytes = 0u64;
        let start_time = std::time::Instant::now();

        while let Some(chunk) = stream.next().await {
            // Check for cancellation
            if self.is_cancelled().await {
                drop(file);
                let _ = fs::remove_file(&temp_path).await;
                anyhow::bail!("Download cancelled");
            }

            let chunk = chunk.context("Error reading chunk")?;
            file.write_all(&chunk)
                .await
                .context("Error writing to file")?;

            downloaded_bytes += chunk.len() as u64;

            // Calculate progress
            let progress_percent = if total_bytes > 0 {
                (downloaded_bytes as f64 / total_bytes as f64) * 100.0
            } else {
                0.0
            };

            // Calculate speed (MB/s)
            let elapsed_secs = start_time.elapsed().as_secs_f64();
            let speed_mbps = if elapsed_secs > 0.0 {
                (downloaded_bytes as f64 / 1_000_000.0) / elapsed_secs
            } else {
                0.0
            };

            // Report progress
            progress_callback(DownloadProgress {
                file_name: file_name.to_string(),
                downloaded_bytes,
                total_bytes,
                progress_percent,
                speed_mbps,
            });
        }

        // Flush and close file
        file.flush().await?;
        drop(file);

        // Rename temp file to final destination
        fs::rename(&temp_path, dest_path)
            .await
            .context("Failed to rename file")?;

        Ok(())
    }

    /// Delete model directory
    pub async fn delete_model(&self, model_id: &str) -> Result<()> {
        let model_dir = self.models_dir.join(model_id.replace('/', "_"));

        if model_dir.exists() {
            fs::remove_dir_all(&model_dir)
                .await
                .context("Failed to delete model directory")?;
        }

        Ok(())
    }

    /// Clean up partial download
    async fn cleanup_partial_download(&self, model_dir: &Path) -> Result<()> {
        if model_dir.exists() {
            fs::remove_dir_all(model_dir).await?;
        }
        Ok(())
    }

    /// Check if model is already downloaded
    pub async fn is_downloaded(&self, model_id: &str) -> bool {
        let model_dir = self.models_dir.join(model_id.replace('/', "_"));

        // Check if all required files exist
        let model_file = model_dir.join("model.safetensors");
        let config_file = model_dir.join("config.json");
        let tokenizer_file = model_dir.join("tokenizer.json");

        model_file.exists() && config_file.exists() && tokenizer_file.exists()
    }

    /// Get model directory path
    pub fn get_model_path(&self, model_id: &str) -> PathBuf {
        self.models_dir.join(model_id.replace('/', "_"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_model_path() {
        let temp_dir = PathBuf::from("/tmp/ner_models");
        let downloader = NerModelDownloader::new(temp_dir.clone()).unwrap();

        let path = downloader.get_model_path("dslim/bert-base-NER");
        assert_eq!(path, temp_dir.join("dslim_bert-base-NER"));
    }

    #[tokio::test]
    async fn test_cancel_flag() {
        let temp_dir = PathBuf::from("/tmp/ner_models");
        let downloader = NerModelDownloader::new(temp_dir).unwrap();

        assert!(!downloader.is_cancelled().await);

        downloader.cancel().await;
        assert!(downloader.is_cancelled().await);

        downloader.reset_cancel().await;
        assert!(!downloader.is_cancelled().await);
    }
}
