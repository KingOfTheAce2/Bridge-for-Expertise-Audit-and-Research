use anyhow::{Context, Result};
use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::NerModelConfig;

/// Token classification head for NER
pub struct TokenClassificationHead {
    dropout: candle_nn::Dropout,
    classifier: candle_nn::Linear,
}

impl TokenClassificationHead {
    pub fn new(
        hidden_size: usize,
        num_labels: usize,
        vb: VarBuilder,
    ) -> Result<Self> {
        let dropout = candle_nn::Dropout::new(0.1);
        let classifier = candle_nn::linear(
            hidden_size,
            num_labels,
            vb.pp("classifier"),
        )?;

        Ok(Self {
            dropout,
            classifier,
        })
    }

    pub fn forward(&self, sequence_output: &Tensor, train: bool) -> Result<Tensor> {
        let output = self.dropout.forward(sequence_output, train)?;
        self.classifier.forward(&output)
    }
}

/// Complete NER model (BERT + classification head)
pub struct NerModel {
    bert: BertModel,
    classifier_head: TokenClassificationHead,
    config: NerModelConfig,
    device: Device,
}

impl NerModel {
    /// Load NER model from local directory
    pub fn load(model_path: &Path, config: NerModelConfig) -> Result<Self> {
        let device = Device::Cpu; // Use CPU for now, can add GPU support later

        // Load model weights
        let weights_path = model_path.join("model.safetensors");
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[weights_path],
                DType::F32,
                &device,
            )?
        };

        // Load BERT config
        let bert_config = BertConfig {
            vocab_size: config.vocab_size,
            hidden_size: config.hidden_size,
            num_hidden_layers: 12, // BERT-base has 12 layers
            num_attention_heads: 12,
            intermediate_size: 3072,
            hidden_act: candle_nn::Activation::Gelu,
            hidden_dropout_prob: 0.1,
            max_position_embeddings: config.max_sequence_length,
            type_vocab_size: 2,
            ..Default::default()
        };

        // Create BERT model
        let bert = BertModel::load(vb.pp("bert"), &bert_config)?;

        // Create classification head
        let classifier_head = TokenClassificationHead::new(
            config.hidden_size,
            config.num_labels,
            vb,
        )?;

        Ok(Self {
            bert,
            classifier_head,
            config,
            device,
        })
    }

    /// Run inference on tokenized input
    /// Returns logits of shape [batch_size, sequence_length, num_labels]
    pub fn forward(
        &self,
        input_ids: &Tensor,
        attention_mask: Option<&Tensor>,
        token_type_ids: Option<&Tensor>,
    ) -> Result<Tensor> {
        // Get BERT output
        let bert_output = self.bert.forward(
            input_ids,
            token_type_ids.unwrap_or(&Tensor::zeros_like(input_ids)?),
        )?;

        // Get sequence output (last hidden state)
        let sequence_output = &bert_output;

        // Apply classification head
        let logits = self.classifier_head.forward(sequence_output, false)?;

        Ok(logits)
    }

    /// Get model configuration
    pub fn config(&self) -> &NerModelConfig {
        &self.config
    }

    /// Get device
    pub fn device(&self) -> &Device {
        &self.device
    }
}

/// Thread-safe NER model manager
pub struct NerModelManager {
    model: Arc<RwLock<Option<NerModel>>>,
    model_path: Arc<RwLock<Option<PathBuf>>>,
    config: Arc<RwLock<Option<NerModelConfig>>>,
}

impl NerModelManager {
    pub fn new() -> Self {
        Self {
            model: Arc::new(RwLock::new(None)),
            model_path: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(None)),
        }
    }

    /// Load a model from disk
    pub async fn load_model(&self, model_path: PathBuf, config: NerModelConfig) -> Result<()> {
        let model = NerModel::load(&model_path, config.clone())
            .context("Failed to load NER model")?;

        let mut model_lock = self.model.write().await;
        *model_lock = Some(model);

        let mut path_lock = self.model_path.write().await;
        *path_lock = Some(model_path);

        let mut config_lock = self.config.write().await;
        *config_lock = Some(config);

        Ok(())
    }

    /// Unload current model
    pub async fn unload_model(&self) {
        let mut model_lock = self.model.write().await;
        *model_lock = None;

        let mut path_lock = self.model_path.write().await;
        *path_lock = None;

        let mut config_lock = self.config.write().await;
        *config_lock = None;
    }

    /// Check if a model is loaded
    pub async fn is_loaded(&self) -> bool {
        let model_lock = self.model.read().await;
        model_lock.is_some()
    }

    /// Get current model path
    pub async fn get_model_path(&self) -> Option<PathBuf> {
        let path_lock = self.model_path.read().await;
        path_lock.clone()
    }

    /// Get current model config
    pub async fn get_config(&self) -> Option<NerModelConfig> {
        let config_lock = self.config.read().await;
        config_lock.clone()
    }

    /// Run inference with loaded model
    pub async fn predict(
        &self,
        input_ids: Tensor,
        attention_mask: Option<Tensor>,
        token_type_ids: Option<Tensor>,
    ) -> Result<Tensor> {
        let model_lock = self.model.read().await;
        let model = model_lock
            .as_ref()
            .context("No model loaded")?;

        model.forward(
            &input_ids,
            attention_mask.as_ref(),
            token_type_ids.as_ref(),
        )
    }
}

impl Default for NerModelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_model_manager_lifecycle() {
        let manager = NerModelManager::new();

        // Initially no model loaded
        assert!(!manager.is_loaded().await);
        assert!(manager.get_model_path().await.is_none());

        // Unload should work even with no model
        manager.unload_model().await;
        assert!(!manager.is_loaded().await);
    }

    #[test]
    fn test_ner_model_config_default() {
        let config = NerModelConfig::default();
        assert_eq!(config.num_labels, 9);
        assert_eq!(config.model_type, "bert");
        assert_eq!(config.label_map.len(), 9);
    }
}
