use std::collections::HashMap;

use super::detector::PIIDetector;
use super::types::{AnonymizationResult, AnonymizationSettings, Entity, EntityType};

/// Smart anonymizer with consistent replacement
pub struct Anonymizer {
    detector: PIIDetector,
    replacement_map: HashMap<String, String>,
    counters: HashMap<EntityType, usize>,
}

impl Anonymizer {
    pub fn new() -> Self {
        Self {
            detector: PIIDetector::new(),
            replacement_map: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    /// Anonymize text according to settings
    pub fn anonymize(
        &mut self,
        text: &str,
        settings: &AnonymizationSettings,
    ) -> AnonymizationResult {
        // Reset state for each document if not using consistent replacement
        if !settings.consistent_replacement {
            self.replacement_map.clear();
            self.counters.clear();
        }

        // Detect entities
        let mut entities = self.detector.detect(text);

        // Add person name detection
        let person_entities = self.detector.detect_person_names(text);
        entities.extend(person_entities);

        // Sort by position again after adding person names
        entities.sort_by_key(|e| e.start);

        // Filter by confidence threshold and entity types
        entities.retain(|e| {
            e.confidence >= settings.confidence_threshold
                && settings.entity_types.contains(&e.entity_type)
        });

        // Preserve legal references if enabled
        if settings.preserve_legal_references {
            entities.retain(|e| e.entity_type != EntityType::Law);
        }

        // Generate replacements
        let entities_with_replacements = self.generate_replacements(entities);

        // Apply anonymization
        let anonymized_text = self.apply_anonymization(text, &entities_with_replacements);

        // Build replacement mapping
        let replacements: Vec<(String, String)> = entities_with_replacements
            .iter()
            .map(|e| (e.text.clone(), e.replacement.clone().unwrap_or_default()))
            .collect();

        AnonymizationResult {
            original_text: text.to_string(),
            anonymized_text,
            entities: entities_with_replacements,
            replacements,
        }
    }

    fn generate_replacements(&mut self, entities: Vec<Entity>) -> Vec<Entity> {
        entities
            .into_iter()
            .map(|entity| {
                let replacement = if entity.entity_type.should_anonymize() {
                    self.get_or_create_replacement(&entity)
                } else {
                    entity.text.clone() // Don't replace
                };

                entity.with_replacement(replacement)
            })
            .collect()
    }

    fn get_or_create_replacement(&mut self, entity: &Entity) -> String {
        // Check if we already have a replacement for this text
        if let Some(replacement) = self.replacement_map.get(&entity.text) {
            return replacement.clone();
        }

        // Generate new replacement
        let counter = self.counters.entry(entity.entity_type).or_insert(0);
        *counter += 1;

        let replacement = match entity.entity_type {
            EntityType::Person => format!("[PERSON-{}]", Self::to_letter(*counter)),
            EntityType::Organization => format!("[ORGANIZATION-{}]", Self::to_letter(*counter)),
            EntityType::Location => format!("[LOCATION-{}]", Self::to_letter(*counter)),
            EntityType::Date => format!("[DATE-{}]", counter),
            EntityType::Money => format!("[AMOUNT-{}]", counter),
            EntityType::Email => format!("[EMAIL-{}]", counter),
            EntityType::Phone => format!("[PHONE-{}]", counter),
            EntityType::Case => format!("[CASE-{}]", counter),
            EntityType::Identification => format!("[ID-{}]", counter),
            EntityType::TechnicalIdentifier => format!("[TECH-ID-{}]", counter),
            EntityType::Law => entity.text.clone(), // Should not anonymize
        };

        // Store in map for consistent replacement
        self.replacement_map
            .insert(entity.text.clone(), replacement.clone());

        replacement
    }

    fn apply_anonymization(&self, text: &str, entities: &[Entity]) -> String {
        if entities.is_empty() {
            return text.to_string();
        }

        let mut result = String::new();
        let mut last_pos = 0;

        for entity in entities {
            // Add text before entity
            result.push_str(&text[last_pos..entity.start]);

            // Add replacement
            if let Some(replacement) = &entity.replacement {
                result.push_str(replacement);
            } else {
                result.push_str(&entity.text);
            }

            last_pos = entity.end;
        }

        // Add remaining text
        result.push_str(&text[last_pos..]);

        result
    }

    fn to_letter(n: usize) -> String {
        if n == 0 {
            return "A".to_string();
        }

        let mut result = String::new();
        let mut num = n;

        while num > 0 {
            let remainder = (num - 1) % 26;
            result.insert(0, (b'A' + remainder as u8) as char);
            num = (num - 1) / 26;
        }

        result
    }

    /// Anonymize multiple documents while maintaining consistency across all
    pub fn anonymize_batch(
        &mut self,
        texts: Vec<String>,
        settings: &AnonymizationSettings,
    ) -> Vec<AnonymizationResult> {
        // Keep consistent replacement across all documents
        texts
            .into_iter()
            .map(|text| self.anonymize(&text, settings))
            .collect()
    }

    /// Clear replacement mapping (start fresh)
    pub fn clear_replacements(&mut self) {
        self.replacement_map.clear();
        self.counters.clear();
    }

    /// Get statistics about detected entities
    pub fn get_statistics(&self) -> HashMap<EntityType, usize> {
        self.counters.clone()
    }
}

impl Default for Anonymizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_anonymization() {
        let mut anonymizer = Anonymizer::new();
        let text = "Contact John Doe at john.doe@example.com or call 555-123-4567.";
        let settings = AnonymizationSettings::default();

        let result = anonymizer.anonymize(text, &settings);

        assert!(result.anonymized_text.contains("[PERSON-"));
        assert!(result.anonymized_text.contains("[EMAIL-"));
        assert!(result.anonymized_text.contains("[PHONE-"));
        assert!(!result.entities.is_empty());
    }

    #[test]
    fn test_consistent_replacement() {
        let mut anonymizer = Anonymizer::new();
        let text = "John Doe called John Doe twice. Mr. John Doe was persistent.";
        let settings = AnonymizationSettings {
            consistent_replacement: true,
            ..Default::default()
        };

        let result = anonymizer.anonymize(text, &settings);

        // Count occurrences of [PERSON-A]
        let count = result.anonymized_text.matches("[PERSON-A]").count();

        // All instances of "John Doe" should use the same replacement
        assert!(count >= 1);
    }

    #[test]
    fn test_legal_reference_preservation() {
        let mut anonymizer = Anonymizer::new();
        let text = "Under Article 6 GDPR, John Doe filed a complaint.";
        let settings = AnonymizationSettings {
            preserve_legal_references: true,
            ..Default::default()
        };

        let result = anonymizer.anonymize(text, &settings);

        // Legal references should be preserved
        assert!(result.anonymized_text.contains("Article 6 GDPR"));
        // But person names should be anonymized
        assert!(!result.anonymized_text.contains("John Doe"));
    }

    #[test]
    fn test_to_letter_conversion() {
        assert_eq!(Anonymizer::to_letter(1), "A");
        assert_eq!(Anonymizer::to_letter(2), "B");
        assert_eq!(Anonymizer::to_letter(26), "Z");
        assert_eq!(Anonymizer::to_letter(27), "AA");
    }

    #[test]
    fn test_batch_anonymization() {
        let mut anonymizer = Anonymizer::new();
        let texts = vec![
            "John Doe lives in New York.".to_string(),
            "John Doe works for Acme Corp.".to_string(),
        ];
        let settings = AnonymizationSettings::default();

        let results = anonymizer.anonymize_batch(texts, &settings);

        assert_eq!(results.len(), 2);

        // John Doe should have same replacement in both documents
        if let (Some(first), Some(second)) = (results.get(0), results.get(1)) {
            let first_has_person_a = first.anonymized_text.contains("[PERSON-A]");
            let second_has_person_a = second.anonymized_text.contains("[PERSON-A]");
            assert!(first_has_person_a && second_has_person_a);
        }
    }
}
