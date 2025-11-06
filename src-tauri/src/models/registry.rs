use serde::{Deserialize, Serialize};

/// Model size categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelSize {
    Small,  // 1-3B parameters
    Medium, // 7-13B parameters
    Large,  // 30-70B parameters
}

impl ModelSize {
    pub fn as_str(&self) -> &str {
        match self {
            ModelSize::Small => "small",
            ModelSize::Medium => "medium",
            ModelSize::Large => "large",
        }
    }
}

/// Model metadata from registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub size: String,
    pub parameters: String,
    pub quantization: Option<String>,
    pub format: String,
    pub download_url: String,
    pub file_size: i64,
    pub checksum: String,
    pub license: String,
    pub tags: Vec<String>,
}

/// Model registry service
pub struct ModelRegistry {
    models: Vec<ModelInfo>,
}

impl ModelRegistry {
    pub fn new() -> Self {
        Self {
            models: Self::load_default_models(),
        }
    }

    /// Load default model catalog
    fn load_default_models() -> Vec<ModelInfo> {
        vec![
            // Mistral 7B Instruct - Recommended for general legal work
            ModelInfo {
                model_id: "mistralai/Mistral-7B-Instruct-v0.2".to_string(),
                name: "Mistral 7B Instruct v0.2".to_string(),
                description: "High-quality instruction-following model, excellent for legal analysis and document review. Balanced speed and quality.".to_string(),
                provider: "huggingface".to_string(),
                size: "medium".to_string(),
                parameters: "7B".to_string(),
                quantization: Some("Q4_K_M".to_string()),
                format: "gguf".to_string(),
                download_url: "https://huggingface.co/TheBloke/Mistral-7B-Instruct-v0.2-GGUF/resolve/main/mistral-7b-instruct-v0.2.Q4_K_M.gguf".to_string(),
                file_size: 4_370_000_000, // ~4.37 GB
                checksum: "placeholder_checksum_mistral_7b".to_string(),
                license: "Apache 2.0".to_string(),
                tags: vec!["instruction".to_string(), "chat".to_string(), "legal".to_string()],
            },
            // TinyLlama 1.1B - Fast inference for quick tasks
            ModelInfo {
                model_id: "TinyLlama/TinyLlama-1.1B-Chat-v1.0".to_string(),
                name: "TinyLlama 1.1B Chat".to_string(),
                description: "Compact model for fast inference. Good for quick summaries and simple queries. Lower quality but very fast.".to_string(),
                provider: "huggingface".to_string(),
                size: "small".to_string(),
                parameters: "1.1B".to_string(),
                quantization: Some("Q4_K_M".to_string()),
                format: "gguf".to_string(),
                download_url: "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf".to_string(),
                file_size: 669_000_000, // ~669 MB
                checksum: "placeholder_checksum_tinyllama".to_string(),
                license: "Apache 2.0".to_string(),
                tags: vec!["chat".to_string(), "fast".to_string(), "compact".to_string()],
            },
            // Phi-2 2.7B - Microsoft's efficient model
            ModelInfo {
                model_id: "microsoft/phi-2".to_string(),
                name: "Phi-2 2.7B".to_string(),
                description: "Microsoft's efficient 2.7B parameter model. Good balance of quality and speed for legal tasks.".to_string(),
                provider: "huggingface".to_string(),
                size: "small".to_string(),
                parameters: "2.7B".to_string(),
                quantization: Some("Q4_K_M".to_string()),
                format: "gguf".to_string(),
                download_url: "https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf".to_string(),
                file_size: 1_600_000_000, // ~1.6 GB
                checksum: "placeholder_checksum_phi2".to_string(),
                license: "MIT".to_string(),
                tags: vec!["instruction".to_string(), "efficient".to_string()],
            },
            // Llama 2 7B Chat - Meta's popular model
            ModelInfo {
                model_id: "meta-llama/Llama-2-7b-chat-hf".to_string(),
                name: "Llama 2 7B Chat".to_string(),
                description: "Meta's Llama 2 chat model. Excellent for conversational legal assistance and document analysis.".to_string(),
                provider: "huggingface".to_string(),
                size: "medium".to_string(),
                parameters: "7B".to_string(),
                quantization: Some("Q4_K_M".to_string()),
                format: "gguf".to_string(),
                download_url: "https://huggingface.co/TheBloke/Llama-2-7B-Chat-GGUF/resolve/main/llama-2-7b-chat.Q4_K_M.gguf".to_string(),
                file_size: 4_080_000_000, // ~4.08 GB
                checksum: "placeholder_checksum_llama2_7b".to_string(),
                license: "Llama 2 Community License".to_string(),
                tags: vec!["chat".to_string(), "instruction".to_string(), "meta".to_string()],
            },
            // Llama 2 13B Chat - Larger, higher quality
            ModelInfo {
                model_id: "meta-llama/Llama-2-13b-chat-hf".to_string(),
                name: "Llama 2 13B Chat".to_string(),
                description: "Larger Llama 2 model for higher quality legal analysis. Requires more RAM but provides better results.".to_string(),
                provider: "huggingface".to_string(),
                size: "medium".to_string(),
                parameters: "13B".to_string(),
                quantization: Some("Q4_K_M".to_string()),
                format: "gguf".to_string(),
                download_url: "https://huggingface.co/TheBloke/Llama-2-13B-Chat-GGUF/resolve/main/llama-2-13b-chat.Q4_K_M.gguf".to_string(),
                file_size: 7_370_000_000, // ~7.37 GB
                checksum: "placeholder_checksum_llama2_13b".to_string(),
                license: "Llama 2 Community License".to_string(),
                tags: vec!["chat".to_string(), "instruction".to_string(), "high-quality".to_string()],
            },
        ]
    }

    /// Get all available models
    pub fn list_models(&self) -> &[ModelInfo] {
        &self.models
    }

    /// Filter models by size
    pub fn filter_by_size(&self, size: ModelSize) -> Vec<&ModelInfo> {
        self.models
            .iter()
            .filter(|m| m.size == size.as_str())
            .collect()
    }

    /// Filter models by tag
    pub fn filter_by_tag(&self, tag: &str) -> Vec<&ModelInfo> {
        self.models
            .iter()
            .filter(|m| m.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&ModelInfo> {
        self.models.iter().find(|m| m.model_id == model_id)
    }

    /// Get recommended model for legal work
    pub fn get_recommended_model(&self) -> Option<&ModelInfo> {
        // Default recommendation: Mistral 7B Instruct
        self.get_model("mistralai/Mistral-7B-Instruct-v0.2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_registry() {
        let registry = ModelRegistry::new();
        assert!(registry.list_models().len() > 0);

        let recommended = registry.get_recommended_model();
        assert!(recommended.is_some());
    }

    #[test]
    fn test_filter_by_size() {
        let registry = ModelRegistry::new();
        let small_models = registry.filter_by_size(ModelSize::Small);
        assert!(small_models.len() > 0);
    }
}
