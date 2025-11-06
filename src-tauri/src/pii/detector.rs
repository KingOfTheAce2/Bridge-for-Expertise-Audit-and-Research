use regex::Regex;
use std::collections::HashMap;

use super::types::{Entity, EntityType};

/// PII Detector using pattern-based recognition (Layer 1)
pub struct PIIDetector {
    patterns: HashMap<EntityType, Vec<Regex>>,
    legal_whitelist: Vec<Regex>,
}

impl PIIDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            patterns: HashMap::new(),
            legal_whitelist: Vec::new(),
        };

        detector.initialize_patterns();
        detector.initialize_legal_whitelist();
        detector
    }

    fn initialize_patterns(&mut self) {
        // Email patterns
        self.add_pattern(
            EntityType::Email,
            r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b",
        );

        // Phone patterns (various formats)
        self.add_pattern(EntityType::Phone, r"\b\+?[\d\s\-\(\)]{10,}\b");
        self.add_pattern(EntityType::Phone, r"\b\d{3}[-.\s]?\d{3}[-.\s]?\d{4}\b");
        self.add_pattern(EntityType::Phone, r"\b\(\d{3}\)\s?\d{3}[-.\s]?\d{4}\b");

        // US Social Security Numbers
        self.add_pattern(
            EntityType::Identification,
            r"\b\d{3}-\d{2}-\d{4}\b",
        );

        // European-style identification numbers
        self.add_pattern(
            EntityType::Identification,
            r"\b[A-Z]{2}\d{6,12}\b",
        );

        // Money patterns
        self.add_pattern(EntityType::Money, r"\$\s?\d{1,3}(?:,\d{3})*(?:\.\d{2})?");
        self.add_pattern(EntityType::Money, r"€\s?\d{1,3}(?:[.,]\d{3})*(?:[.,]\d{2})?");
        self.add_pattern(EntityType::Money, r"£\s?\d{1,3}(?:,\d{3})*(?:\.\d{2})?");
        self.add_pattern(
            EntityType::Money,
            r"\b\d{1,3}(?:,\d{3})*(?:\.\d{2})?\s?(?:USD|EUR|GBP)\b",
        );

        // Date patterns
        self.add_pattern(
            EntityType::Date,
            r"\b\d{1,2}[-/]\d{1,2}[-/]\d{2,4}\b",
        );
        self.add_pattern(
            EntityType::Date,
            r"\b\d{4}[-/]\d{1,2}[-/]\d{1,2}\b",
        );
        self.add_pattern(
            EntityType::Date,
            r"\b(?:Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)[a-z]*\s+\d{1,2},?\s+\d{4}\b",
        );

        // Case numbers
        self.add_pattern(
            EntityType::Case,
            r"\b(?:Case|Docket|File)\s+(?:No\.?|Number|#)\s*:?\s*\d+[-/]?\d*\b",
        );
        self.add_pattern(EntityType::Case, r"\b\d{2}-[A-Z]{2,4}-\d{4,}\b");

        // Legal references (to preserve, not anonymize)
        self.add_pattern(
            EntityType::Law,
            r"\b(?:Article|Section|§)\s+\d+(?:\(\d+\))?(?:\s+[A-Z][A-Za-z\s]+)?",
        );
        self.add_pattern(EntityType::Law, r"\b\d+\s+U\.S\.C\.?\s+§?\s*\d+\b");
        self.add_pattern(EntityType::Law, r"\bGDPR\b");
        self.add_pattern(EntityType::Law, r"\b(?:Act|Code|Regulation)\s+\d+\b");

        // IP addresses
        self.add_pattern(
            EntityType::TechnicalIdentifier,
            r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b",
        );

        // Person names (basic patterns - title + name)
        self.add_pattern(
            EntityType::Person,
            r"\b(?:Mr\.|Mrs\.|Ms\.|Dr\.|Prof\.)\s+[A-Z][a-z]+(?:\s+[A-Z][a-z]+)*\b",
        );

        // Organizations (common suffixes)
        self.add_pattern(
            EntityType::Organization,
            r"\b[A-Z][A-Za-z\s&]+(?:Inc\.|LLC|Ltd\.|Corp\.|Corporation|Company|Co\.)\b",
        );
        self.add_pattern(
            EntityType::Organization,
            r"\b(?:Court of|Supreme Court|District Court|Circuit Court)\s+[A-Za-z\s]+\b",
        );
    }

    fn initialize_legal_whitelist(&mut self) {
        // Legal terms and references that should NOT be anonymized
        let whitelist_patterns = vec![
            r"\b(?:Article|Section|Paragraph)\s+\d+",
            r"\bGDPR\b",
            r"\b[A-Z]{2,4}\s+(?:Act|Code|Regulation)\b",
            r"\b\d+\s+U\.S\.C\.?\s+§?\s*\d+",
            r"\b(?:First|Second|Third|Fourth|Fifth|Sixth|Seventh|Eighth|Ninth|Tenth|Eleventh)\s+Amendment\b",
            r"\b(?:Constitutional|Federal|State)\s+(?:Law|Statute|Regulation)\b",
        ];

        for pattern in whitelist_patterns {
            if let Ok(regex) = Regex::new(pattern) {
                self.legal_whitelist.push(regex);
            }
        }
    }

    fn add_pattern(&mut self, entity_type: EntityType, pattern: &str) {
        if let Ok(regex) = Regex::new(pattern) {
            self.patterns
                .entry(entity_type)
                .or_insert_with(Vec::new)
                .push(regex);
        }
    }

    /// Detect entities in text
    pub fn detect(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        for (entity_type, regexes) in &self.patterns {
            for regex in regexes {
                for cap in regex.find_iter(text) {
                    let matched_text = cap.as_str().to_string();
                    let start = cap.start();
                    let end = cap.end();

                    // Check if this match is in the legal whitelist
                    if *entity_type != EntityType::Law && self.is_whitelisted(&matched_text) {
                        continue;
                    }

                    entities.push(Entity::new(
                        *entity_type,
                        matched_text,
                        start,
                        end,
                        0.85, // Pattern-based detection confidence
                    ));
                }
            }
        }

        // Sort by position
        entities.sort_by_key(|e| e.start);

        // Remove overlapping entities (keep the longer/more specific one)
        self.remove_overlaps(entities)
    }

    fn is_whitelisted(&self, text: &str) -> bool {
        self.legal_whitelist.iter().any(|regex| regex.is_match(text))
    }

    fn remove_overlaps(&self, mut entities: Vec<Entity>) -> Vec<Entity> {
        if entities.is_empty() {
            return entities;
        }

        let mut result = Vec::new();
        let mut last_end = 0;

        for entity in entities.drain(..) {
            if entity.start >= last_end {
                last_end = entity.end;
                result.push(entity);
            } else if entity.end > last_end {
                // Overlapping - keep the longer one
                if let Some(last) = result.last_mut() {
                    if entity.text.len() > last.text.len() {
                        *last = entity.clone();
                        last_end = entity.end;
                    }
                }
            }
        }

        result
    }

    /// Detect person names using common patterns
    pub fn detect_person_names(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();

        // Pattern: Capitalized words (likely names)
        let name_pattern = Regex::new(r"\b[A-Z][a-z]+(?:\s+[A-Z][a-z]+)+\b").unwrap();

        for cap in name_pattern.find_iter(text) {
            let matched_text = cap.as_str();

            // Filter out common non-name phrases
            if self.is_likely_name(matched_text) {
                entities.push(Entity::new(
                    EntityType::Person,
                    matched_text.to_string(),
                    cap.start(),
                    cap.end(),
                    0.75, // Lower confidence for name detection
                ));
            }
        }

        entities
    }

    fn is_likely_name(&self, text: &str) -> bool {
        // Exclude common non-name phrases
        let exclusions = [
            "United States",
            "Supreme Court",
            "District Court",
            "Court Of",
            "State Of",
            "City Of",
            "County Of",
        ];

        !exclusions.iter().any(|&excl| text.eq_ignore_ascii_case(excl))
    }
}

impl Default for PIIDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_detection() {
        let detector = PIIDetector::new();
        let text = "Contact me at john.doe@example.com for more info.";
        let entities = detector.detect(text);

        assert!(entities.iter().any(|e| e.entity_type == EntityType::Email));
    }

    #[test]
    fn test_phone_detection() {
        let detector = PIIDetector::new();
        let text = "Call me at 555-123-4567 or (555) 987-6543.";
        let entities = detector.detect(text);

        let phone_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Phone)
            .collect();

        assert!(phone_entities.len() >= 1);
    }

    #[test]
    fn test_legal_reference_preservation() {
        let detector = PIIDetector::new();
        let text = "Under Article 6 GDPR and Section 101 of the Act...";
        let entities = detector.detect(text);

        // Legal references should be detected as LAW type
        assert!(entities.iter().any(|e| e.entity_type == EntityType::Law));
    }

    #[test]
    fn test_money_detection() {
        let detector = PIIDetector::new();
        let text = "The amount was $1,234.56 or €2,000.00.";
        let entities = detector.detect(text);

        let money_entities: Vec<_> = entities
            .iter()
            .filter(|e| e.entity_type == EntityType::Money)
            .collect();

        assert!(money_entities.len() >= 1);
    }
}
