use serde::{Deserialize, Serialize};

/// AI model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub model_id: String,
    pub model_type: String,        // "mistral", "llama", "phi", etc.
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub intermediate_size: usize,
    pub max_position_embeddings: usize,
    pub rope_theta: f32,           // RoPE theta for position encoding
    pub use_flash_attn: bool,
}

impl Default for ModelConfig {
    fn default() -> Self {
        // Mistral 7B defaults
        Self {
            model_id: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
            model_type: "mistral".to_string(),
            vocab_size: 32000,
            hidden_size: 4096,
            num_hidden_layers: 32,
            num_attention_heads: 32,
            intermediate_size: 14336,
            max_position_embeddings: 32768,
            rope_theta: 10000.0,
            use_flash_attn: false,
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

/// Conversation context
#[derive(Debug, Clone)]
pub struct ConversationContext {
    pub messages: Vec<ChatMessage>,
    pub total_tokens: usize,
    pub max_context_length: usize,
}

impl ConversationContext {
    pub fn new(max_context_length: usize) -> Self {
        Self {
            messages: Vec::new(),
            total_tokens: 0,
            max_context_length,
        }
    }

    pub fn add_message(&mut self, message: ChatMessage, token_count: usize) {
        self.messages.push(message);
        self.total_tokens += token_count;
    }

    pub fn should_truncate(&self) -> bool {
        self.total_tokens > self.max_context_length * 8 / 10  // 80% threshold
    }

    pub fn truncate_to_fit(&mut self, target_tokens: usize) {
        // Keep system message if present
        let has_system = self.messages.first()
            .map(|m| m.role == "system")
            .unwrap_or(false);

        let start_idx = if has_system { 1 } else { 0 };

        // Remove oldest messages until we fit
        while self.total_tokens > target_tokens && self.messages.len() > start_idx {
            if start_idx < self.messages.len() {
                self.messages.remove(start_idx);
                // Recalculate total tokens (simplified)
                self.total_tokens = self.messages.iter()
                    .map(|m| m.content.split_whitespace().count())
                    .sum();
            } else {
                break;
            }
        }
    }
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

    #[test]
    fn test_conversation_context() {
        let mut ctx = ConversationContext::new(100);

        ctx.add_message(ChatMessage {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }, 10);

        assert_eq!(ctx.messages.len(), 1);
        assert_eq!(ctx.total_tokens, 10);
        assert!(!ctx.should_truncate());
    }

    #[test]
    fn test_context_truncation() {
        let mut ctx = ConversationContext::new(100);

        // Add messages exceeding threshold
        for i in 0..10 {
            ctx.add_message(ChatMessage {
                role: "user".to_string(),
                content: format!("Message {}", i),
            }, 15);
        }

        assert!(ctx.should_truncate());

        ctx.truncate_to_fit(50);
        assert!(ctx.total_tokens <= 50);
    }
}
