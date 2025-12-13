use serde::{Deserialize, Serialize};

/// Model format types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelFormat {
    SafeTensors,  // Full precision safetensors
    GGUF,         // Quantized GGUF format
}

/// AI model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_id: String,
    pub model_type: String,        // "mistral", "llama", "phi", etc.
    pub format: ModelFormat,       // SafeTensors or GGUF
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub intermediate_size: usize,
    pub max_position_embeddings: usize,
    pub rope_theta: f32,           // RoPE theta for position encoding
    pub use_flash_attn: bool,
    pub quantization: Option<String>, // "Q4_0", "Q5_0", "Q8_0", etc. for GGUF
}

impl Default for ModelConfig {
    fn default() -> Self {
        // Mistral 7B defaults
        Self {
            model_id: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
            model_type: "mistral".to_string(),
            format: ModelFormat::SafeTensors,
            vocab_size: 32000,
            hidden_size: 4096,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            intermediate_size: 14336,
            max_position_embeddings: 32768,
            rope_theta: 10000.0,
            use_flash_attn: false,
            quantization: None,
        }
    }
}

/// Generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationConfig {
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: usize,
    pub max_new_tokens: usize,
    pub repetition_penalty: f64,
    pub do_sample: bool,
    pub seed: Option<u64>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            max_new_tokens: 2048,
            repetition_penalty: 1.1,
            do_sample: true,
            seed: None,
        }
    }
}

/// Chat message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,      // "system" | "user" | "assistant"
    pub content: String,
}

/// Generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub messages: Vec<ChatMessage>,
    pub config: GenerationConfig,
    pub system_prompt: Option<String>,
}

/// Streaming token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_id: u32,
    pub is_final: bool,
    pub total_tokens: usize,
    pub generation_time_ms: u64,
}

/// Complete generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub text: String,
    pub tokens: Vec<u32>,
    pub total_tokens: usize,
    pub prompt_tokens: usize,
    pub generated_tokens: usize,
    pub generation_time_ms: u64,
    pub tokens_per_second: f64,
}

/// Model loading status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    NotLoaded,
    Loading,
    Loaded,
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model_config() {
        let config = ModelConfig::default();
        assert_eq!(config.model_type, "mistral");
        assert_eq!(config.hidden_size, 4096);
    }

    #[test]
    fn test_default_generation_config() {
        let config = GenerationConfig::default();
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_new_tokens, 2048);
    }
}
