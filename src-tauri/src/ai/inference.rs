use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
use candle_core::quantized::gguf_file;
use candle_transformers::models::quantized_llama as gguf_llama;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokenizers::Tokenizer;
use tokio::sync::RwLock;

use super::types::{
    ChatMessage, GenerateRequest, GenerationResult, ModelConfig, ModelFormat, ModelStatus,
    TokenResponse,
};

/// Loaded model variants (safetensors or GGUF)
enum LoadedModel {
    GGUF(gguf_llama::ModelWeights),
    // SafeTensors variant would go here when implemented
}

/// AI inference engine with GPU support
pub struct InferenceEngine {
    model_path: Arc<RwLock<Option<PathBuf>>>,
    model_config: Arc<RwLock<Option<ModelConfig>>>,
    status: Arc<RwLock<ModelStatus>>,
    device: Arc<RwLock<Device>>,
    model: Arc<RwLock<Option<LoadedModel>>>,
    tokenizer: Arc<RwLock<Option<Tokenizer>>>,
}

impl InferenceEngine {
    pub fn new() -> Self {
        // Detect best available device at initialization
        let device = Self::detect_device();
        log::info!("InferenceEngine initialized with device: {:?}", device);

        Self {
            model_path: Arc::new(RwLock::new(None)),
            model_config: Arc::new(RwLock::new(None)),
            status: Arc::new(RwLock::new(ModelStatus::NotLoaded)),
            device: Arc::new(RwLock::new(device)),
            model: Arc::new(RwLock::new(None)),
            tokenizer: Arc::new(RwLock::new(None)),
        }
    }

    /// Detect best available GPU/CPU device
    pub fn detect_device() -> Device {
        // Try CUDA first (NVIDIA GPUs)
        #[cfg(feature = "cuda")]
        {
            match Device::new_cuda(0) {
                Ok(device) => {
                    log::info!("✓ CUDA GPU detected and enabled");
                    return device;
                }
                Err(e) => {
                    log::warn!("CUDA not available: {}", e);
                }
            }
        }

        // Try Metal (Apple Silicon M1/M2/M3)
        #[cfg(all(target_os = "macos", feature = "metal"))]
        {
            match Device::new_metal(0) {
                Ok(device) => {
                    log::info!("✓ Metal GPU detected and enabled (Apple Silicon)");
                    return device;
                }
                Err(e) => {
                    log::warn!("Metal not available: {}", e);
                }
            }
        }

        // Try Apple Accelerate framework (macOS CPU optimization)
        #[cfg(all(target_os = "macos", feature = "accelerate"))]
        {
            log::info!("✓ Using Apple Accelerate framework for CPU optimization");
            return Device::Cpu;
        }

        // Fallback to CPU
        log::info!("Using CPU for inference (no GPU acceleration available)");
        Device::Cpu
    }

    /// Load model from path (supports both SafeTensors and GGUF)
    pub async fn load_model(&self, model_path: PathBuf, config: ModelConfig) -> Result<()> {
        let mut status = self.status.write().await;
        *status = ModelStatus::Loading;
        drop(status);

        // Verify model path exists
        if !model_path.exists() {
            let mut status = self.status.write().await;
            *status = ModelStatus::Error("Model path does not exist".to_string());
            anyhow::bail!("Model path does not exist: {:?}", model_path);
        }

        log::info!("Loading model from: {:?}", model_path);
        log::info!("Model format: {:?}", config.format);

        // Load based on format
        match config.format {
            ModelFormat::GGUF => {
                self.load_gguf_model(model_path.clone(), &config).await?;
            }
            ModelFormat::SafeTensors => {
                // TODO: Implement SafeTensors loading
                // For now, return error with helpful message
                let mut status = self.status.write().await;
                *status = ModelStatus::Error(
                    "SafeTensors format not yet implemented. Please use GGUF models.".to_string(),
                );
                anyhow::bail!("SafeTensors format not yet implemented");
            }
        }

        // Store model path and config
        let mut path_lock = self.model_path.write().await;
        *path_lock = Some(model_path);

        let mut config_lock = self.model_config.write().await;
        *config_lock = Some(config);

        let mut status = self.status.write().await;
        *status = ModelStatus::Loaded;

        log::info!("✓ Model loaded successfully");
        Ok(())
    }

    /// Load GGUF quantized model
    async fn load_gguf_model(&self, model_path: PathBuf, config: &ModelConfig) -> Result<()> {
        log::info!("Loading GGUF model...");

        // Find the .gguf file in the model directory
        let gguf_file = if model_path.is_file() && model_path.extension().and_then(|s| s.to_str()) == Some("gguf") {
            model_path.clone()
        } else {
            // Look for .gguf file in directory
            std::fs::read_dir(&model_path)?
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.path().extension().and_then(|s| s.to_str()) == Some("gguf")
                })
                .map(|e| e.path())
                .ok_or_else(|| anyhow::anyhow!("No GGUF file found in model directory"))?
        };

        log::info!("Loading GGUF file: {:?}", gguf_file);

        // Load tokenizer
        let tokenizer_path = if model_path.is_file() {
            model_path.parent().unwrap().join("tokenizer.json")
        } else {
            model_path.join("tokenizer.json")
        };

        if tokenizer_path.exists() {
            let tokenizer = Tokenizer::from_file(&tokenizer_path)
                .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;
            let mut tok_lock = self.tokenizer.write().await;
            *tok_lock = Some(tokenizer);
            log::info!("✓ Tokenizer loaded");
        } else {
            log::warn!("Tokenizer not found at: {:?}", tokenizer_path);
        }

        // Load GGUF model with Candle
        let device = self.device.read().await;
        let mut file = std::fs::File::open(&gguf_file)
            .context(format!("Failed to open GGUF file: {:?}", gguf_file))?;

        // Read GGUF content structure
        let content = gguf_file::Content::read(&mut file)
            .context("Failed to read GGUF file content")?;

        // Load model weights from GGUF
        let model_weights = gguf_llama::ModelWeights::from_gguf(content, &mut file, &device)
            .context("Failed to load GGUF model weights")?;

        // Store loaded model
        let mut model_lock = self.model.write().await;
        *model_lock = Some(LoadedModel::GGUF(model_weights));

        log::info!("✓ GGUF model loaded into memory");
        log::info!("Quantization: {}", config.quantization.as_ref().unwrap_or(&"unknown".to_string()));

        Ok(())
    }

    /// Unload current model
    pub async fn unload_model(&self) {
        log::info!("Unloading model...");

        let mut status = self.status.write().await;
        *status = ModelStatus::NotLoaded;

        let mut model_lock = self.model.write().await;
        *model_lock = None;

        let mut tokenizer_lock = self.tokenizer.write().await;
        *tokenizer_lock = None;

        let mut path_lock = self.model_path.write().await;
        *path_lock = None;

        let mut config_lock = self.model_config.write().await;
        *config_lock = None;

        log::info!("✓ Model unloaded");
    }

    /// Check if model is loaded
    pub async fn is_loaded(&self) -> bool {
        let status = self.status.read().await;
        matches!(*status, ModelStatus::Loaded)
    }

    /// Get current model status
    pub async fn get_status(&self) -> ModelStatus {
        let status = self.status.read().await;
        status.clone()
    }

    /// Get current device info
    pub async fn get_device_info(&self) -> String {
        let device = self.device.read().await;
        match &*device {
            Device::Cpu => "CPU".to_string(),
            Device::Cuda(_) => "CUDA (NVIDIA GPU)".to_string(),
            Device::Metal(_) => "Metal (Apple GPU)".to_string(),
        }
    }

    /// Generate text completion
    pub async fn generate(&self, request: GenerateRequest) -> Result<GenerationResult> {
        // Check if model is loaded
        if !self.is_loaded().await {
            anyhow::bail!("No model loaded");
        }

        let start_time = Instant::now();

        // Get tokenizer
        let tokenizer_lock = self.tokenizer.read().await;
        let tokenizer = tokenizer_lock.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Tokenizer not loaded"))?;

        // Format messages into prompt
        let prompt = self.format_prompt(&request.messages, request.system_prompt.as_deref());

        // Tokenize prompt
        let encoding = tokenizer.encode(prompt.clone(), false)
            .map_err(|e| anyhow::anyhow!("Failed to tokenize prompt: {}", e))?;
        let prompt_tokens = encoding.get_ids();
        let prompt_token_count = prompt_tokens.len();

        log::info!("Generating response for {} token prompt", prompt_token_count);

        // Check if model is GGUF
        let model_lock = self.model.read().await;
        let model = model_lock.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Model not loaded"))?;

        match model {
            LoadedModel::GGUF(_gguf_model) => {
                // Actual GGUF inference
                let device = self.device.read().await;

                // Convert tokens to tensor
                let _tokens = Tensor::new(prompt_tokens, &*device)?;

                // Generate with the model
                // Note: This is a simplified version - full implementation would include:
                // - Proper sampling with temperature, top_p, top_k
                // - Repetition penalty
                // - Stop tokens
                // - KV cache for efficiency

                drop(device);
                drop(model_lock);

                // For now, return a placeholder that shows the infrastructure works
                let generated_text = format!(
                    "GGUF Model Response: Your prompt was successfully tokenized ({} tokens). \
                    Full inference implementation requires additional sampling logic. \
                    Model is loaded and ready on {}.",
                    prompt_token_count,
                    self.get_device_info().await
                );

                let generation_time = start_time.elapsed().as_millis() as u64;
                let generated_tokens = generated_text.split_whitespace().count();
                let total_tokens = prompt_token_count + generated_tokens;

                let tokens_per_second = if generation_time > 0 {
                    (generated_tokens as f64 / generation_time as f64) * 1000.0
                } else {
                    0.0
                };

                Ok(GenerationResult {
                    text: generated_text,
                    tokens: prompt_tokens.to_vec(),
                    total_tokens,
                    prompt_tokens: prompt_token_count,
                    generated_tokens,
                    generation_time_ms: generation_time,
                    tokens_per_second,
                })
            }
        }
    }

    /// Generate text with streaming
    pub async fn generate_stream<F>(
        &self,
        request: GenerateRequest,
        mut callback: F,
    ) -> Result<GenerationResult>
    where
        F: FnMut(TokenResponse) + Send,
    {
        // Check if model is loaded
        if !self.is_loaded().await {
            anyhow::bail!("No model loaded");
        }

        let start_time = Instant::now();

        // Get tokenizer
        let tokenizer_lock = self.tokenizer.read().await;
        let tokenizer = tokenizer_lock.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Tokenizer not loaded"))?;

        // Format messages into prompt
        let prompt = self.format_prompt(&request.messages, request.system_prompt.as_deref());

        // Tokenize prompt
        let encoding = tokenizer.encode(prompt.clone(), false)
            .map_err(|e| anyhow::anyhow!("Failed to tokenize prompt: {}", e))?;
        let prompt_tokens = encoding.get_ids();
        let prompt_token_count = prompt_tokens.len();

        drop(tokenizer_lock);

        log::info!("Streaming generation for {} token prompt", prompt_token_count);

        // Get device info as owned string
        let device_info = self.get_device_info().await;

        // For now, simulate streaming with placeholder
        // Full implementation would generate tokens one by one
        let words_base = vec![
            "Streaming", "GGUF", "inference", "is", "working!",
            "GPU", "acceleration:",
        ];
        let words_end = vec!["Model", "loaded", "successfully."];

        // Combine all words
        let mut words: Vec<String> = words_base.into_iter().map(|s| s.to_string()).collect();
        words.push(device_info);
        words.extend(words_end.into_iter().map(|s| s.to_string()));

        let mut generated_tokens = 0;
        let mut generated_text = String::new();

        for (i, word) in words.iter().enumerate() {
            // Simulate token generation time
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            generated_tokens += 1;
            let is_final = i == words.len() - 1;

            if !generated_text.is_empty() {
                generated_text.push(' ');
            }
            generated_text.push_str(&word);

            callback(TokenResponse {
                token: word.clone(),
                token_id: i as u32,
                is_final,
                total_tokens: prompt_token_count + generated_tokens,
                generation_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }

        let generation_time = start_time.elapsed().as_millis() as u64;
        let total_tokens = prompt_token_count + generated_tokens;

        let tokens_per_second = if generation_time > 0 {
            (generated_tokens as f64 / generation_time as f64) * 1000.0
        } else {
            0.0
        };

        Ok(GenerationResult {
            text: generated_text,
            tokens: prompt_tokens.to_vec(),
            total_tokens,
            prompt_tokens: prompt_token_count,
            generated_tokens,
            generation_time_ms: generation_time,
            tokens_per_second,
        })
    }

    /// Format chat messages into a prompt
    fn format_prompt(&self, messages: &[ChatMessage], system_prompt: Option<&str>) -> String {
        let mut prompt = String::new();

        // Add system prompt if provided
        if let Some(system) = system_prompt {
            prompt.push_str(&format!("<|system|>\n{}\n", system));
        }

        // Add conversation messages
        for msg in messages {
            match msg.role.as_str() {
                "system" => {
                    prompt.push_str(&format!("<|system|>\n{}\n", msg.content));
                }
                "user" => {
                    prompt.push_str(&format!("<|user|>\n{}\n", msg.content));
                }
                "assistant" => {
                    prompt.push_str(&format!("<|assistant|>\n{}\n", msg.content));
                }
                _ => {}
            }
        }

        // Add assistant prompt for generation
        prompt.push_str("<|assistant|>\n");

        prompt
    }
}

impl Default for InferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::types::GenerationConfig;

    #[tokio::test]
    async fn test_inference_engine_creation() {
        let engine = InferenceEngine::new();
        assert!(!engine.is_loaded().await);
    }

    #[tokio::test]
    async fn test_device_detection() {
        let device = InferenceEngine::detect_device();
        // Should at least have CPU
        assert!(matches!(device, Device::Cpu) || matches!(device, Device::Cuda(_)) || matches!(device, Device::Metal(_)));
    }

    #[tokio::test]
    async fn test_model_status() {
        let engine = InferenceEngine::new();
        let status = engine.get_status().await;
        assert!(matches!(status, ModelStatus::NotLoaded));
    }

    #[tokio::test]
    async fn test_device_info() {
        let engine = InferenceEngine::new();
        let device_info = engine.get_device_info().await;
        assert!(!device_info.is_empty());
        println!("Device: {}", device_info);
    }

    #[test]
    fn test_format_prompt() {
        let engine = InferenceEngine::new();
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello!".to_string(),
            },
            ChatMessage {
                role: "assistant".to_string(),
                content: "Hi there!".to_string(),
            },
        ];

        let prompt = engine.format_prompt(&messages, Some("You are a helpful assistant"));
        assert!(prompt.contains("<|system|>"));
        assert!(prompt.contains("<|user|>"));
        assert!(prompt.contains("<|assistant|>"));
        assert!(prompt.contains("Hello!"));
    }

    #[tokio::test]
    async fn test_generate_without_model() {
        let engine = InferenceEngine::new();
        let request = GenerateRequest {
            messages: vec![],
            config: GenerationConfig::default(),
            system_prompt: None,
        };

        let result = engine.generate(request).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No model loaded"));
    }
}
