use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};

/// Model validator for checksum verification
pub struct ModelValidator;

impl ModelValidator {
    /// Verify model file checksum
    pub async fn verify_checksum(file_path: &Path, expected_checksum: &str) -> Result<bool> {
        let calculated_checksum = Self::calculate_sha256(file_path).await?;
        Ok(calculated_checksum.to_lowercase() == expected_checksum.to_lowercase())
    }

    /// Calculate SHA256 checksum of a file
    pub async fn calculate_sha256(file_path: &Path) -> Result<String> {
        let file = File::open(file_path)
            .await
            .context("Failed to open file for checksum calculation")?;

        let mut reader = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192]; // 8KB buffer

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .await
                .context("Failed to read file")?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let result = hasher.finalize();
        Ok(hex::encode(result))
    }

    /// Validate model file structure (basic checks)
    pub async fn validate_model_file(file_path: &Path) -> Result<bool> {
        // Check if file exists
        if !file_path.exists() {
            anyhow::bail!("Model file does not exist");
        }

        // Check if file is not empty
        let metadata = tokio::fs::metadata(file_path)
            .await
            .context("Failed to read file metadata")?;

        if metadata.len() == 0 {
            anyhow::bail!("Model file is empty");
        }

        // Check file extension (should be .gguf or .bin)
        let extension = file_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if !["gguf", "bin", "safetensors"].contains(&extension) {
            anyhow::bail!("Invalid model file extension: {}", extension);
        }

        // For GGUF files, check magic number
        if extension == "gguf" {
            let is_valid_gguf = Self::validate_gguf_magic(file_path).await?;
            if !is_valid_gguf {
                anyhow::bail!("Invalid GGUF file format");
            }
        }

        Ok(true)
    }

    /// Validate GGUF file magic number
    async fn validate_gguf_magic(file_path: &Path) -> Result<bool> {
        let mut file = File::open(file_path).await?;
        let mut magic = [0u8; 4];

        file.read_exact(&mut magic).await?;

        // GGUF magic number is "GGUF" in ASCII
        Ok(&magic == b"GGUF" || &magic == b"GGML" || &magic == b"GGJT")
    }
}
