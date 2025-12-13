use serde::{Deserialize, Serialize};
use std::fmt;

/// Entity types that can be detected in text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// Person names and titles
    Person,
    /// Organizations, companies, firms, courts
    Organization,
    /// Addresses, cities, countries, locations
    Location,
    /// Specific dates, birth dates, time references
    Date,
    /// Financial amounts, monetary values
    Money,
    /// Legal references, statutes, regulations
    Law,
    /// Case numbers, file references, docket numbers
    Case,
    /// Email addresses
    Email,
    /// Phone numbers
    Phone,
    /// Social security numbers, tax IDs, identification numbers
    Identification,
    /// IP addresses, URLs
    TechnicalIdentifier,
}

impl EntityType {
    pub fn as_str(&self) -> &str {
        match self {
            EntityType::Person => "PERSON",
            EntityType::Organization => "ORGANIZATION",
            EntityType::Location => "LOCATION",
            EntityType::Date => "DATE",
            EntityType::Money => "MONEY",
            EntityType::Law => "LAW",
            EntityType::Case => "CASE",
            EntityType::Email => "EMAIL",
            EntityType::Phone => "PHONE",
            EntityType::Identification => "IDENTIFICATION",
            EntityType::TechnicalIdentifier => "TECHNICAL_IDENTIFIER",
        }
    }

    pub fn should_anonymize(&self) -> bool {
        match self {
            // Legal references should be preserved
            EntityType::Law => false,
            // Everything else should be anonymized
            _ => true,
        }
    }
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A detected entity in text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    /// Type of entity
    pub entity_type: EntityType,
    /// Original text of the entity
    pub text: String,
    /// Start position in document (character index)
    pub start: usize,
    /// End position in document (character index)
    pub end: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Replacement text for anonymization
    pub replacement: Option<String>,
}

impl Entity {
    pub fn new(
        entity_type: EntityType,
        text: String,
        start: usize,
        end: usize,
        confidence: f64,
    ) -> Self {
        Self {
            entity_type,
            text,
            start,
            end,
            confidence,
            replacement: None,
        }
    }

    pub fn with_replacement(mut self, replacement: String) -> Self {
        self.replacement = Some(replacement);
        self
    }
}

/// Anonymization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationResult {
    /// Original text
    pub original_text: String,
    /// Anonymized text
    pub anonymized_text: String,
    /// Detected entities
    pub entities: Vec<Entity>,
    /// Mapping of original text to replacement
    pub replacements: Vec<(String, String)>,
}

/// Anonymization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizationSettings {
    /// Entity types to anonymize
    pub entity_types: Vec<EntityType>,
    /// Minimum confidence threshold (0.0 to 1.0)
    pub confidence_threshold: f64,
    /// Whether to preserve legal references
    pub preserve_legal_references: bool,
    /// Whether to use consistent replacement (same entity = same replacement)
    pub consistent_replacement: bool,
    /// Language code (e.g., "en", "nl", "de")
    pub language: String,
}

impl Default for AnonymizationSettings {
    fn default() -> Self {
        Self {
            entity_types: vec![
                EntityType::Person,
                EntityType::Organization,
                EntityType::Location,
                EntityType::Date,
                EntityType::Email,
                EntityType::Phone,
                EntityType::Identification,
            ],
            confidence_threshold: 0.7,
            preserve_legal_references: true,
            consistent_replacement: true,
            language: "en".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_should_anonymize() {
        assert!(EntityType::Person.should_anonymize());
        assert!(EntityType::Email.should_anonymize());
        assert!(!EntityType::Law.should_anonymize());
    }

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new(
            EntityType::Person,
            "John Doe".to_string(),
            0,
            8,
            0.95,
        );

        assert_eq!(entity.entity_type, EntityType::Person);
        assert_eq!(entity.text, "John Doe");
        assert_eq!(entity.confidence, 0.95);
    }
}
