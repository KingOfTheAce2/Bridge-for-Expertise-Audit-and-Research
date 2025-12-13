// Allow dead code - these are API components that will be used from frontend
#![allow(dead_code)]

use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use super::model_loader::NerModelManager;
use super::tokenizer::{align_tokens_with_text, merge_subword_predictions, NerTokenizer};
use super::types::{NerEntity, NerLabel, NerResult, TokenPrediction};

/// NER inference pipeline
pub struct NerPipeline {
    model_manager: Arc<NerModelManager>,
    tokenizer: Arc<RwLock<Option<NerTokenizer>>>,
}

impl NerPipeline {
    /// Create a new NER pipeline
    pub fn new(model_manager: Arc<NerModelManager>) -> Self {
        Self {
            model_manager,
            tokenizer: Arc::new(RwLock::new(None)),
        }
    }

    /// Check if pipeline is ready (model and tokenizer loaded)
    pub async fn is_ready(&self) -> bool {
        let model_loaded = self.model_manager.is_loaded().await;
        let tok_lock = self.tokenizer.read().await;
        let tokenizer_loaded = tok_lock.is_some();

        model_loaded && tokenizer_loaded
    }

    /// Run NER inference on text
    pub async fn predict(&self, text: &str) -> Result<NerResult> {
        let start_time = Instant::now();

        // Check if pipeline is ready
        if !self.is_ready().await {
            anyhow::bail!("Pipeline not ready. Load model and tokenizer first.");
        }

        // Get device
        let _config = self
            .model_manager
            .get_config()
            .await
            .context("Model config not available")?;

        let device = candle_core::Device::Cpu;

        // Tokenize input
        let tok_lock = self.tokenizer.read().await;
        let tokenizer = tok_lock
            .as_ref()
            .context("Tokenizer not loaded")?;

        let encoding = tokenizer.encode(text, &device)?;
        let tokens = encoding.tokens.clone();
        let offsets = encoding.offsets.clone();

        drop(tok_lock); // Release tokenizer lock

        // Run model inference
        let logits = self
            .model_manager
            .predict(
                encoding.input_ids,
                Some(encoding.attention_mask),
                Some(encoding.token_type_ids),
            )
            .await?;

        // Get predictions (argmax over labels dimension)
        let predictions = logits.argmax(2)?; // Shape: [batch_size, sequence_length]

        // Convert to Vec<u32>
        let predictions_vec: Vec<u32> = predictions
            .to_vec2::<u32>()?
            .into_iter()
            .flatten()
            .collect();

        // Get confidence scores (softmax)
        let probs = candle_nn::ops::softmax(&logits, 2)?;

        // Extract max probabilities
        let max_probs = probs.max(2)?; // Shape: [batch_size, sequence_length]
        let confidence_vec: Vec<f32> = max_probs
            .to_vec2::<f32>()?
            .into_iter()
            .flatten()
            .collect();

        // Align tokens with original text
        let alignments = align_tokens_with_text(&tokens, &offsets, text);

        // Filter out special tokens and create predictions
        let mut valid_predictions = Vec::new();
        let mut valid_confidences = Vec::new();

        for (i, _) in alignments.iter().enumerate() {
            if i < predictions_vec.len() {
                valid_predictions.push(predictions_vec[i] as usize);
                valid_confidences.push(confidence_vec[i]);
            }
        }

        // Merge subword tokens
        let merged = merge_subword_predictions(
            alignments.clone(),
            valid_predictions
                .iter()
                .zip(valid_confidences.iter())
                .map(|(&label, &conf)| (label, conf))
                .collect(),
        );

        // Create token predictions
        let mut token_predictions = Vec::new();
        for (token_text, label_id, confidence, start, end) in &merged {
            if let Some(label) = NerLabel::from_id(*label_id) {
                token_predictions.push(TokenPrediction {
                    token: token_text.clone(),
                    label,
                    confidence: *confidence,
                    start: *start,
                    end: *end,
                });
            }
        }

        // Extract entities (combine B- and I- tags)
        let entities = self.extract_entities(&token_predictions);

        let inference_time = start_time.elapsed().as_millis() as u64;

        Ok(NerResult {
            text: text.to_string(),
            entities,
            token_predictions,
            inference_time_ms: inference_time,
        })
    }

    /// Extract named entities from token predictions using BIO tagging
    fn extract_entities(&self, predictions: &[TokenPrediction]) -> Vec<NerEntity> {
        let mut entities = Vec::new();
        let mut current_entity: Option<NerEntity> = None;

        for pred in predictions {
            match pred.label {
                NerLabel::O => {
                    // Outside any entity - finalize current entity if exists
                    if let Some(entity) = current_entity.take() {
                        entities.push(entity);
                    }
                }
                label if label.is_begin() => {
                    // Beginning of new entity - finalize current and start new
                    if let Some(entity) = current_entity.take() {
                        entities.push(entity);
                    }

                    if let Some(entity_type) = label.entity_type() {
                        current_entity = Some(NerEntity {
                            text: pred.token.clone(),
                            entity_type: entity_type.to_string(),
                            confidence: pred.confidence,
                            start: pred.start,
                            end: pred.end,
                            tokens: vec![pred.clone()],
                        });
                    }
                }
                label if label.is_inside() => {
                    // Inside entity - extend current entity
                    if let Some(ref mut entity) = current_entity {
                        // Check if label matches current entity type
                        if let Some(entity_type) = label.entity_type() {
                            if entity.entity_type == entity_type {
                                entity.text.push(' ');
                                entity.text.push_str(&pred.token);
                                entity.end = pred.end;
                                entity.tokens.push(pred.clone());
                                // Update average confidence
                                let total_conf: f32 = entity.tokens.iter().map(|t| t.confidence).sum();
                                entity.confidence = total_conf / entity.tokens.len() as f32;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Don't forget last entity
        if let Some(entity) = current_entity {
            entities.push(entity);
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_extraction() {
        let pipeline = NerPipeline::new(Arc::new(NerModelManager::new()));

        let predictions = vec![
            TokenPrediction {
                token: "John".to_string(),
                label: NerLabel::BeginPerson,
                confidence: 0.9,
                start: 0,
                end: 4,
            },
            TokenPrediction {
                token: "Doe".to_string(),
                label: NerLabel::InsidePerson,
                confidence: 0.85,
                start: 5,
                end: 8,
            },
            TokenPrediction {
                token: "works".to_string(),
                label: NerLabel::O,
                confidence: 0.95,
                start: 9,
                end: 14,
            },
            TokenPrediction {
                token: "at".to_string(),
                label: NerLabel::O,
                confidence: 0.99,
                start: 15,
                end: 17,
            },
            TokenPrediction {
                token: "Google".to_string(),
                label: NerLabel::BeginOrganization,
                confidence: 0.92,
                start: 18,
                end: 24,
            },
        ];

        let entities = pipeline.extract_entities(&predictions);

        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].text, "John Doe");
        assert_eq!(entities[0].entity_type, "PER");
        assert_eq!(entities[1].text, "Google");
        assert_eq!(entities[1].entity_type, "ORG");
    }

    #[test]
    fn test_entity_extraction_with_multi_token() {
        let pipeline = NerPipeline::new(Arc::new(NerModelManager::new()));

        let predictions = vec![
            TokenPrediction {
                token: "New".to_string(),
                label: NerLabel::BeginLocation,
                confidence: 0.9,
                start: 0,
                end: 3,
            },
            TokenPrediction {
                token: "York".to_string(),
                label: NerLabel::InsideLocation,
                confidence: 0.88,
                start: 4,
                end: 8,
            },
            TokenPrediction {
                token: "City".to_string(),
                label: NerLabel::InsideLocation,
                confidence: 0.85,
                start: 9,
                end: 13,
            },
        ];

        let entities = pipeline.extract_entities(&predictions);

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].text, "New York City");
        assert_eq!(entities[0].entity_type, "LOC");
        assert_eq!(entities[0].tokens.len(), 3);
    }
}
