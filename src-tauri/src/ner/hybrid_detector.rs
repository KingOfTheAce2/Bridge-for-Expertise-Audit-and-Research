use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::pii::detector::PIIDetector;
use crate::pii::types::{Entity, EntityType};

use super::inference::NerPipeline;
use super::types::NerResult;

/// Detection mode for hybrid detector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMode {
    /// Use only pattern-based detection (regex)
    PatternOnly,
    /// Use only NER model detection
    NerOnly,
    /// Use both and merge results (default)
    Hybrid,
}

/// Hybrid PII detector combining pattern-based and NER approaches
pub struct HybridDetector {
    pattern_detector: PIIDetector,
    ner_pipeline: Arc<NerPipeline>,
    detection_mode: Arc<RwLock<DetectionMode>>,
}

impl HybridDetector {
    pub fn new(pattern_detector: PIIDetector, ner_pipeline: Arc<NerPipeline>) -> Self {
        Self {
            pattern_detector,
            ner_pipeline,
            detection_mode: Arc::new(RwLock::new(DetectionMode::Hybrid)),
        }
    }

    /// Set detection mode
    pub async fn set_mode(&self, mode: DetectionMode) {
        let mut mode_lock = self.detection_mode.write().await;
        *mode_lock = mode;
    }

    /// Get current detection mode
    pub async fn get_mode(&self) -> DetectionMode {
        let mode_lock = self.detection_mode.read().await;
        *mode_lock
    }

    /// Detect PII entities in text using hybrid approach
    pub async fn detect(&self, text: &str) -> Result<Vec<Entity>> {
        let mode = self.get_mode().await;

        match mode {
            DetectionMode::PatternOnly => Ok(self.detect_with_patterns(text)),
            DetectionMode::NerOnly => self.detect_with_ner(text).await,
            DetectionMode::Hybrid => self.detect_hybrid(text).await,
        }
    }

    /// Detect using pattern-based approach only
    fn detect_with_patterns(&self, text: &str) -> Vec<Entity> {
        let mut entities = self.pattern_detector.detect(text);

        // Add person names detected by pattern detector
        let person_entities = self.pattern_detector.detect_person_names(text);
        entities.extend(person_entities);

        entities.sort_by_key(|e| e.start);
        entities
    }

    /// Detect using NER model only
    async fn detect_with_ner(&self, text: &str) -> Result<Vec<Entity>> {
        // Check if NER pipeline is ready
        if !self.ner_pipeline.is_ready().await {
            // Fall back to pattern-based detection
            return Ok(self.detect_with_patterns(text));
        }

        let ner_result = self.ner_pipeline.predict(text).await?;
        let entities = self.convert_ner_to_entities(&ner_result);

        Ok(entities)
    }

    /// Detect using both approaches and merge results
    async fn detect_hybrid(&self, text: &str) -> Result<Vec<Entity>> {
        // Get pattern-based detections
        let pattern_entities = self.detect_with_patterns(text);

        // Get NER detections (if available)
        let ner_entities = if self.ner_pipeline.is_ready().await {
            match self.ner_pipeline.predict(text).await {
                Ok(ner_result) => self.convert_ner_to_entities(&ner_result),
                Err(_) => Vec::new(), // If NER fails, just use patterns
            }
        } else {
            Vec::new()
        };

        // Merge and deduplicate entities
        let merged = self.merge_entities(pattern_entities, ner_entities);

        Ok(merged)
    }

    /// Convert NER results to PII Entity format
    fn convert_ner_to_entities(&self, ner_result: &NerResult) -> Vec<Entity> {
        ner_result
            .entities
            .iter()
            .filter_map(|ner_entity| {
                // Map NER labels to PII entity types
                let entity_type = match ner_entity.entity_type.as_str() {
                    "PER" => EntityType::Person,
                    "ORG" => EntityType::Organization,
                    "LOC" => EntityType::Location,
                    "MISC" => {
                        // Try to classify MISC based on content
                        // For now, just skip MISC entities or treat as misc
                        return None;
                    }
                    _ => return None,
                };

                Some(Entity {
                    entity_type,
                    text: ner_entity.text.clone(),
                    start: ner_entity.start,
                    end: ner_entity.end,
                    confidence: ner_entity.confidence as f64,
                })
            })
            .collect()
    }

    /// Merge entities from different sources and remove duplicates
    fn merge_entities(&self, mut pattern_entities: Vec<Entity>, ner_entities: Vec<Entity>) -> Vec<Entity> {
        // Start with all pattern entities
        let mut merged = pattern_entities.clone();

        // Add NER entities that don't overlap with pattern entities
        for ner_entity in ner_entities {
            if !self.has_overlap(&pattern_entities, &ner_entity) {
                // Check if NER entity has higher confidence than similar pattern entity
                // For now, add if no overlap
                merged.push(ner_entity);
            } else {
                // There's an overlap - use the one with higher confidence
                // Find overlapping entity
                if let Some(idx) = self.find_overlapping_index(&pattern_entities, &ner_entity) {
                    // Replace if NER has higher confidence
                    if ner_entity.confidence > pattern_entities[idx].confidence {
                        // Replace in merged list
                        if let Some(merge_idx) = self.find_overlapping_index(&merged, &pattern_entities[idx]) {
                            merged[merge_idx] = ner_entity.clone();
                        }
                    }
                }
            }
        }

        // Sort by position
        merged.sort_by_key(|e| e.start);

        // Final deduplication pass - remove exact duplicates
        merged.dedup_by(|a, b| {
            a.entity_type == b.entity_type && a.start == b.start && a.end == b.end
        });

        merged
    }

    /// Check if entity overlaps with any in the list
    fn has_overlap(&self, entities: &[Entity], entity: &Entity) -> bool {
        entities.iter().any(|e| {
            // Check for any overlap in text spans
            !(e.end <= entity.start || e.start >= entity.end)
        })
    }

    /// Find index of overlapping entity
    fn find_overlapping_index(&self, entities: &[Entity], entity: &Entity) -> Option<usize> {
        entities.iter().position(|e| {
            // Check for any overlap in text spans
            !(e.end <= entity.start || e.start >= entity.end)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ner::model_loader::NerModelManager;

    #[test]
    fn test_has_overlap() {
        let detector = HybridDetector::new(
            PIIDetector::new(),
            Arc::new(NerPipeline::new(Arc::new(NerModelManager::new()))),
        );

        let entities = vec![Entity {
            entity_type: EntityType::Person,
            text: "John".to_string(),
            start: 0,
            end: 4,
            confidence: 0.9,
        }];

        let overlapping = Entity {
            entity_type: EntityType::Person,
            text: "John Doe".to_string(),
            start: 0,
            end: 8,
            confidence: 0.85,
        };

        let non_overlapping = Entity {
            entity_type: EntityType::Person,
            text: "Jane".to_string(),
            start: 10,
            end: 14,
            confidence: 0.9,
        };

        assert!(detector.has_overlap(&entities, &overlapping));
        assert!(!detector.has_overlap(&entities, &non_overlapping));
    }

    #[test]
    fn test_merge_entities() {
        let detector = HybridDetector::new(
            PIIDetector::new(),
            Arc::new(NerPipeline::new(Arc::new(NerModelManager::new()))),
        );

        let pattern_entities = vec![Entity {
            entity_type: EntityType::Email,
            text: "test@example.com".to_string(),
            start: 0,
            end: 16,
            confidence: 0.95,
        }];

        let ner_entities = vec![
            Entity {
                entity_type: EntityType::Person,
                text: "John Doe".to_string(),
                start: 20,
                end: 28,
                confidence: 0.9,
            },
        ];

        let merged = detector.merge_entities(pattern_entities, ner_entities);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].entity_type, EntityType::Email);
        assert_eq!(merged[1].entity_type, EntityType::Person);
    }

    #[test]
    fn test_merge_with_overlaps() {
        let detector = HybridDetector::new(
            PIIDetector::new(),
            Arc::new(NerPipeline::new(Arc::new(NerModelManager::new()))),
        );

        let pattern_entities = vec![Entity {
            entity_type: EntityType::Person,
            text: "John".to_string(),
            start: 0,
            end: 4,
            confidence: 0.7,
        }];

        let ner_entities = vec![Entity {
            entity_type: EntityType::Person,
            text: "John Doe".to_string(),
            start: 0,
            end: 8,
            confidence: 0.9, // Higher confidence
        }];

        let merged = detector.merge_entities(pattern_entities, ner_entities);

        // Should prefer NER entity due to higher confidence
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].text, "John Doe");
        assert_eq!(merged[0].confidence, 0.9);
    }
}
