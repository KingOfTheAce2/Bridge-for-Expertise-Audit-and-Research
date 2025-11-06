use super::types::NerModelInfo;

/// Registry of pre-configured NER models
pub struct NerModelRegistry {
    models: Vec<NerModelInfo>,
}

impl NerModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: Vec::new(),
        };

        registry.register_default_models();
        registry
    }

    /// Register default NER models
    fn register_default_models(&mut self) {
        // 1. BERT-base NER (dslim/bert-base-NER)
        self.models.push(NerModelInfo {
            model_id: "dslim/bert-base-NER".to_string(),
            name: "BERT-base NER".to_string(),
            description: "BERT-base fine-tuned on CoNLL-2003 for Named Entity Recognition. Supports Person, Organization, Location, and Miscellaneous entities.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000, // ~440 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.956), // F1 score on CoNLL-2003 test set
        });

        // 2. DistilBERT NER (lightweight)
        self.models.push(NerModelInfo {
            model_id: "dslim/distilbert-NER".to_string(),
            name: "DistilBERT NER (Fast)".to_string(),
            description: "DistilBERT fine-tuned for NER. Smaller and faster than BERT-base with competitive accuracy. Good for resource-constrained environments.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "distilbert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "small".to_string(),
            parameters: "66M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 260_000_000, // ~260 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.941), // F1 score
        });

        // 3. RoBERTa-base NER (high accuracy)
        self.models.push(NerModelInfo {
            model_id: "dslim/roberta-base-NER".to_string(),
            name: "RoBERTa-base NER (Accurate)".to_string(),
            description: "RoBERTa-base fine-tuned for NER. Higher accuracy than BERT-base, especially for complex entities. Best for legal and professional documents.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "roberta".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "125M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 500_000_000, // ~500 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.964), // F1 score - highest accuracy
        });

        // 4. XLM-RoBERTa NER (multilingual)
        self.models.push(NerModelInfo {
            model_id: "Davlan/xlm-roberta-base-ner-hrl".to_string(),
            name: "XLM-RoBERTa NER (Multilingual)".to_string(),
            description: "Multilingual NER model supporting 100+ languages. Ideal for non-English legal documents and multilingual workflows.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "xlm-roberta".to_string(),
            language: "multilingual".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "125M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/tokenizer.json".to_string(),
            file_size: 550_000_000, // ~550 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.93), // Average F1 across languages
        });

        // 5. TinyBERT NER (ultra-fast, smallest)
        self.models.push(NerModelInfo {
            model_id: "mrm8488/bert-tiny-finetuned-ner".to_string(),
            name: "TinyBERT NER (Ultra-Fast)".to_string(),
            description: "Extremely small and fast BERT model for NER. Sacrifices some accuracy for speed. Perfect for real-time applications and low-resource devices.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "small".to_string(),
            parameters: "15M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 60_000_000, // ~60 MB
            checksum: None,
            license: "Apache 2.0".to_string(),
            accuracy: Some(0.87), // Lower accuracy, much faster
        });
    }

    /// Get all registered models
    pub fn list_models(&self) -> &[NerModelInfo] {
        &self.models
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&NerModelInfo> {
        self.models.iter().find(|m| m.model_id == model_id)
    }

    /// Get models by language
    pub fn get_models_by_language(&self, language: &str) -> Vec<&NerModelInfo> {
        self.models
            .iter()
            .filter(|m| m.language == language || m.language == "multilingual")
            .collect()
    }

    /// Get models by size
    pub fn get_models_by_size(&self, size: &str) -> Vec<&NerModelInfo> {
        self.models.iter().filter(|m| m.size == size).collect()
    }

    /// Add custom model
    pub fn add_model(&mut self, model: NerModelInfo) {
        // Check if model already exists
        if let Some(idx) = self.models.iter().position(|m| m.model_id == model.model_id) {
            // Replace existing model
            self.models[idx] = model;
        } else {
            // Add new model
            self.models.push(model);
        }
    }

    /// Remove model by ID
    pub fn remove_model(&mut self, model_id: &str) -> bool {
        if let Some(idx) = self.models.iter().position(|m| m.model_id == model_id) {
            self.models.remove(idx);
            true
        } else {
            false
        }
    }

    /// Get recommended model for general use
    pub fn get_recommended_model(&self) -> Option<&NerModelInfo> {
        // BERT-base NER is a good balanced choice
        self.get_model("dslim/bert-base-NER")
    }

    /// Get fastest model
    pub fn get_fastest_model(&self) -> Option<&NerModelInfo> {
        self.get_model("mrm8488/bert-tiny-finetuned-ner")
    }

    /// Get most accurate model
    pub fn get_most_accurate_model(&self) -> Option<&NerModelInfo> {
        self.get_model("dslim/roberta-base-NER")
    }

    /// Get multilingual model
    pub fn get_multilingual_model(&self) -> Option<&NerModelInfo> {
        self.get_model("Davlan/xlm-roberta-base-ner-hrl")
    }
}

impl Default for NerModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_models() {
        let registry = NerModelRegistry::new();
        assert!(registry.list_models().len() > 0);
    }

    #[test]
    fn test_get_model_by_id() {
        let registry = NerModelRegistry::new();
        let model = registry.get_model("dslim/bert-base-NER");
        assert!(model.is_some());
        assert_eq!(model.unwrap().name, "BERT-base NER");
    }

    #[test]
    fn test_get_models_by_language() {
        let registry = NerModelRegistry::new();
        let en_models = registry.get_models_by_language("en");
        assert!(en_models.len() > 0);
    }

    #[test]
    fn test_recommended_models() {
        let registry = NerModelRegistry::new();

        assert!(registry.get_recommended_model().is_some());
        assert!(registry.get_fastest_model().is_some());
        assert!(registry.get_most_accurate_model().is_some());
        assert!(registry.get_multilingual_model().is_some());
    }

    #[test]
    fn test_add_custom_model() {
        let mut registry = NerModelRegistry::new();
        let initial_count = registry.list_models().len();

        let custom_model = NerModelInfo {
            model_id: "custom/test-model".to_string(),
            name: "Test Model".to_string(),
            description: "A test model".to_string(),
            provider: "Custom".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec!["O".to_string()],
            size: "small".to_string(),
            parameters: "10M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://example.com/model".to_string(),
            config_url: "https://example.com/config".to_string(),
            tokenizer_url: "https://example.com/tokenizer".to_string(),
            file_size: 10_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: None,
        };

        registry.add_model(custom_model);
        assert_eq!(registry.list_models().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_model() {
        let mut registry = NerModelRegistry::new();
        let initial_count = registry.list_models().len();

        let removed = registry.remove_model("dslim/bert-base-NER");
        assert!(removed);
        assert_eq!(registry.list_models().len(), initial_count - 1);
    }
}
