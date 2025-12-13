//! Hybrid PII detector combining three detection layers:
//! - Layer 1: Pattern-based detection (regex)
//! - Layer 2: NER model detection (transformer-based)
//! - Layer 3: Presidio integration (optional, advanced)

// Allow dead code - these are API components that will be used from frontend
#![allow(dead_code)]

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::pii::detector::PIIDetector;
use crate::pii::presidio::{EntityTypeMapper, PresidioManager, PresidioStatus};
use crate::pii::types::{Entity, EntityType};

use super::inference::NerPipeline;
use super::types::NerResult;

/// Detection mode for hybrid detector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMode {
    /// Use only pattern-based detection (regex) - Layer 1
    PatternOnly,
    /// Use only NER model detection - Layer 2
    NerOnly,
    /// Use pattern + NER (default) - Layer 1 + 2
    Hybrid,
    /// Use all three layers including Presidio - Layer 1 + 2 + 3
    Full,
    /// Use only Presidio - Layer 3 only
    PresidioOnly,
}

impl Default for DetectionMode {
    fn default() -> Self {
        Self::Hybrid
    }
}

/// Hybrid PII detector combining pattern-based, NER, and Presidio approaches
pub struct HybridDetector {
    pattern_detector: PIIDetector,
    ner_pipeline: Arc<NerPipeline>,
    presidio_manager: Arc<PresidioManager>,
    entity_mapper: EntityTypeMapper,
    detection_mode: Arc<RwLock<DetectionMode>>,
    default_language: Arc<RwLock<String>>,
}

impl HybridDetector {
    /// Create a new hybrid detector with all three layers
    pub fn new(
        ner_pipeline: Arc<NerPipeline>,
        presidio_manager: Arc<PresidioManager>,
    ) -> Self {
        Self {
            pattern_detector: PIIDetector::new(),
            ner_pipeline,
            presidio_manager,
            entity_mapper: EntityTypeMapper::new(),
            detection_mode: Arc::new(RwLock::new(DetectionMode::default())),
            default_language: Arc::new(RwLock::new("en".to_string())),
        }
    }

    /// Create a detector without Presidio (Layer 1 + 2 only)
    pub fn without_presidio(ner_pipeline: Arc<NerPipeline>) -> Self {
        Self {
            pattern_detector: PIIDetector::new(),
            ner_pipeline,
            presidio_manager: Arc::new(PresidioManager::new()),
            entity_mapper: EntityTypeMapper::new(),
            detection_mode: Arc::new(RwLock::new(DetectionMode::Hybrid)),
            default_language: Arc::new(RwLock::new("en".to_string())),
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

    /// Set default language for detection
    pub async fn set_language(&self, language: &str) {
        let mut lang_lock = self.default_language.write().await;
        *lang_lock = language.to_string();
    }

    /// Get default language
    pub async fn get_language(&self) -> String {
        self.default_language.read().await.clone()
    }

    /// Check if Presidio is available
    pub async fn is_presidio_available(&self) -> bool {
        matches!(
            self.presidio_manager.check_status().await,
            Ok(PresidioStatus::Running)
        )
    }

    /// Detect PII entities in text using configured mode
    pub async fn detect(&self, text: &str) -> Result<Vec<Entity>> {
        let mode = self.get_mode().await;
        let language = self.get_language().await;

        match mode {
            DetectionMode::PatternOnly => Ok(self.detect_with_patterns(text)),
            DetectionMode::NerOnly => self.detect_with_ner(text).await,
            DetectionMode::Hybrid => self.detect_hybrid(text).await,
            DetectionMode::Full => self.detect_full(text, &language).await,
            DetectionMode::PresidioOnly => self.detect_with_presidio(text, &language).await,
        }
    }

    /// Detect with specific language override
    pub async fn detect_with_language(&self, text: &str, language: &str) -> Result<Vec<Entity>> {
        let mode = self.get_mode().await;

        match mode {
            DetectionMode::PatternOnly => Ok(self.detect_with_patterns(text)),
            DetectionMode::NerOnly => self.detect_with_ner(text).await,
            DetectionMode::Hybrid => self.detect_hybrid(text).await,
            DetectionMode::Full => self.detect_full(text, language).await,
            DetectionMode::PresidioOnly => self.detect_with_presidio(text, language).await,
        }
    }

    /// Layer 1: Detect using pattern-based approach only
    fn detect_with_patterns(&self, text: &str) -> Vec<Entity> {
        let mut entities = self.pattern_detector.detect(text);

        // Add person names detected by pattern detector
        let person_entities = self.pattern_detector.detect_person_names(text);
        entities.extend(person_entities);

        entities.sort_by_key(|e| e.start);
        entities
    }

    /// Layer 2: Detect using NER model only
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

    /// Layer 3: Detect using Presidio only
    async fn detect_with_presidio(&self, text: &str, language: &str) -> Result<Vec<Entity>> {
        // Check if Presidio is available
        if !self.presidio_manager.is_enabled().await {
            // Fall back to hybrid detection
            return self.detect_hybrid(text).await;
        }

        let presidio_entities = self.presidio_manager.analyze(text, language).await?;
        let entities = self.entity_mapper.convert_entities(&presidio_entities, text);

        Ok(entities)
    }

    /// Layer 1 + 2: Detect using patterns and NER, merge results
    async fn detect_hybrid(&self, text: &str) -> Result<Vec<Entity>> {
        // Get pattern-based detections
        let pattern_entities = self.detect_with_patterns(text);

        // Get NER detections (if available)
        let ner_entities = if self.ner_pipeline.is_ready().await {
            match self.ner_pipeline.predict(text).await {
                Ok(ner_result) => self.convert_ner_to_entities(&ner_result),
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        };

        // Merge and deduplicate entities
        let merged = self.merge_entities(pattern_entities, ner_entities);

        Ok(merged)
    }

    /// Full detection: Layer 1 + 2 + 3
    async fn detect_full(&self, text: &str, language: &str) -> Result<Vec<Entity>> {
        // Get Layer 1 + 2 results
        let hybrid_entities = self.detect_hybrid(text).await?;

        // Get Layer 3 (Presidio) results if available
        let presidio_entities = if self.presidio_manager.is_enabled().await {
            match self.presidio_manager.analyze(text, language).await {
                Ok(entities) => self.entity_mapper.convert_entities(&entities, text),
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        };

        // Merge all results, preferring higher confidence
        let merged = self.merge_all_layers(hybrid_entities, presidio_entities);

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
                    replacement: None,
                })
            })
            .collect()
    }

    /// Merge entities from Layer 1 + 2
    fn merge_entities(&self, pattern_entities: Vec<Entity>, ner_entities: Vec<Entity>) -> Vec<Entity> {
        let mut merged = pattern_entities.clone();

        for ner_entity in ner_entities {
            if !self.has_overlap(&pattern_entities, &ner_entity) {
                merged.push(ner_entity);
            } else {
                if let Some(idx) = self.find_overlapping_index(&pattern_entities, &ner_entity) {
                    if ner_entity.confidence > pattern_entities[idx].confidence {
                        if let Some(merge_idx) = self.find_overlapping_index(&merged, &pattern_entities[idx]) {
                            merged[merge_idx] = ner_entity.clone();
                        }
                    }
                }
            }
        }

        merged.sort_by_key(|e| e.start);
        merged.dedup_by(|a, b| {
            a.entity_type == b.entity_type && a.start == b.start && a.end == b.end
        });

        merged
    }

    /// Merge all three layers of detection
    fn merge_all_layers(&self, hybrid_entities: Vec<Entity>, presidio_entities: Vec<Entity>) -> Vec<Entity> {
        let mut merged = hybrid_entities.clone();

        for presidio_entity in presidio_entities {
            if !self.has_overlap(&hybrid_entities, &presidio_entity) {
                // No overlap - add Presidio entity
                merged.push(presidio_entity);
            } else {
                // Overlap exists - compare confidence and choose best
                if let Some(idx) = self.find_overlapping_index(&hybrid_entities, &presidio_entity) {
                    // Presidio often has better confidence for certain entity types
                    // Give slight preference to Presidio for identification types
                    let presidio_boost = match presidio_entity.entity_type {
                        EntityType::Identification | EntityType::Email | EntityType::Phone => 0.05,
                        _ => 0.0,
                    };

                    if presidio_entity.confidence + presidio_boost > hybrid_entities[idx].confidence {
                        if let Some(merge_idx) = self.find_overlapping_index(&merged, &hybrid_entities[idx]) {
                            merged[merge_idx] = presidio_entity.clone();
                        }
                    }
                }
            }
        }

        // Sort by position
        merged.sort_by_key(|e| e.start);

        // Final deduplication
        merged.dedup_by(|a, b| {
            a.entity_type == b.entity_type && a.start == b.start && a.end == b.end
        });

        merged
    }

    /// Check if entity overlaps with any in the list
    fn has_overlap(&self, entities: &[Entity], entity: &Entity) -> bool {
        entities.iter().any(|e| {
            !(e.end <= entity.start || e.start >= entity.end)
        })
    }

    /// Find index of overlapping entity
    fn find_overlapping_index(&self, entities: &[Entity], entity: &Entity) -> Option<usize> {
        entities.iter().position(|e| {
            !(e.end <= entity.start || e.start >= entity.end)
        })
    }

    /// Get statistics about available detection layers
    pub async fn get_layer_status(&self) -> LayerStatus {
        LayerStatus {
            layer1_pattern: true, // Always available
            layer2_ner: self.ner_pipeline.is_ready().await,
            layer3_presidio: self.is_presidio_available().await,
        }
    }
}

/// Status of detection layers
#[derive(Debug, Clone)]
pub struct LayerStatus {
    /// Layer 1: Pattern-based detection (always available)
    pub layer1_pattern: bool,
    /// Layer 2: NER model detection
    pub layer2_ner: bool,
    /// Layer 3: Presidio integration
    pub layer3_presidio: bool,
}

impl LayerStatus {
    /// Get recommended detection mode based on available layers
    pub fn recommended_mode(&self) -> DetectionMode {
        if self.layer3_presidio && self.layer2_ner {
            DetectionMode::Full
        } else if self.layer2_ner {
            DetectionMode::Hybrid
        } else {
            DetectionMode::PatternOnly
        }
    }

    /// Count available layers
    pub fn available_layers(&self) -> u8 {
        let mut count = 0;
        if self.layer1_pattern { count += 1; }
        if self.layer2_ner { count += 1; }
        if self.layer3_presidio { count += 1; }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_status_recommended_mode() {
        let status = LayerStatus {
            layer1_pattern: true,
            layer2_ner: true,
            layer3_presidio: true,
        };
        assert_eq!(status.recommended_mode(), DetectionMode::Full);

        let status = LayerStatus {
            layer1_pattern: true,
            layer2_ner: true,
            layer3_presidio: false,
        };
        assert_eq!(status.recommended_mode(), DetectionMode::Hybrid);

        let status = LayerStatus {
            layer1_pattern: true,
            layer2_ner: false,
            layer3_presidio: false,
        };
        assert_eq!(status.recommended_mode(), DetectionMode::PatternOnly);
    }

    #[test]
    fn test_available_layers_count() {
        let status = LayerStatus {
            layer1_pattern: true,
            layer2_ner: true,
            layer3_presidio: true,
        };
        assert_eq!(status.available_layers(), 3);
    }
}
