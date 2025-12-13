//! Entity type mapping between Presidio and internal PII types
//!
//! Provides bidirectional mapping between Presidio's 50+ entity types
//! and our internal EntityType enum used throughout the application.

use std::collections::HashMap;

use crate::pii::types::{Entity, EntityType};
use super::types::PresidioEntity;

/// Maps between Presidio entity types and internal entity types
pub struct EntityTypeMapper {
    /// Presidio type -> Internal type
    presidio_to_internal: HashMap<String, EntityType>,
    /// Internal type -> Presidio type
    internal_to_presidio: HashMap<EntityType, String>,
}

impl EntityTypeMapper {
    /// Create a new mapper with default mappings
    pub fn new() -> Self {
        let mut presidio_to_internal = HashMap::new();
        let mut internal_to_presidio = HashMap::new();

        // Person-related mappings
        presidio_to_internal.insert("PERSON".to_string(), EntityType::Person);
        internal_to_presidio.insert(EntityType::Person, "PERSON".to_string());

        // Location mappings
        presidio_to_internal.insert("LOCATION".to_string(), EntityType::Location);
        presidio_to_internal.insert("GPE".to_string(), EntityType::Location); // Geo-political entity
        presidio_to_internal.insert("LOC".to_string(), EntityType::Location);
        internal_to_presidio.insert(EntityType::Location, "LOCATION".to_string());

        // Organization mappings
        presidio_to_internal.insert("ORGANIZATION".to_string(), EntityType::Organization);
        presidio_to_internal.insert("ORG".to_string(), EntityType::Organization);
        internal_to_presidio.insert(EntityType::Organization, "ORGANIZATION".to_string());

        // Email
        presidio_to_internal.insert("EMAIL_ADDRESS".to_string(), EntityType::Email);
        presidio_to_internal.insert("EMAIL".to_string(), EntityType::Email);
        internal_to_presidio.insert(EntityType::Email, "EMAIL_ADDRESS".to_string());

        // Phone
        presidio_to_internal.insert("PHONE_NUMBER".to_string(), EntityType::Phone);
        presidio_to_internal.insert("PHONE".to_string(), EntityType::Phone);
        internal_to_presidio.insert(EntityType::Phone, "PHONE_NUMBER".to_string());

        // Date/Time
        presidio_to_internal.insert("DATE_TIME".to_string(), EntityType::Date);
        presidio_to_internal.insert("DATE".to_string(), EntityType::Date);
        presidio_to_internal.insert("TIME".to_string(), EntityType::Date);
        presidio_to_internal.insert("DATE_OF_BIRTH".to_string(), EntityType::Date);
        internal_to_presidio.insert(EntityType::Date, "DATE_TIME".to_string());

        // Money/Financial
        presidio_to_internal.insert("MONEY".to_string(), EntityType::Money);
        presidio_to_internal.insert("CREDIT_CARD".to_string(), EntityType::Identification);
        presidio_to_internal.insert("IBAN_CODE".to_string(), EntityType::Identification);
        presidio_to_internal.insert("US_BANK_NUMBER".to_string(), EntityType::Identification);
        presidio_to_internal.insert("CRYPTO".to_string(), EntityType::Identification);
        internal_to_presidio.insert(EntityType::Money, "MONEY".to_string());

        // Identification numbers (national IDs, SSN, etc.)
        let id_types = [
            "US_SSN",
            "US_ITIN",
            "US_PASSPORT",
            "US_DRIVER_LICENSE",
            "UK_NHS",
            "UK_NINO",
            "AU_ABN",
            "AU_ACN",
            "AU_TFN",
            "AU_MEDICARE",
            "IN_AADHAAR",
            "IN_PAN",
            "IN_VOTER",
            "SG_NRIC_FIN",
            "IT_FISCAL_CODE",
            "IT_DRIVER_LICENSE",
            "IT_VAT_CODE",
            "IT_PASSPORT",
            "IT_IDENTITY_CARD",
            "ES_NIF",
            "ES_NIE",
            "PL_PESEL",
            "PL_NIP",
            "PL_REGON",
            "MEDICAL_LICENSE",
            "NRP",
        ];

        for id_type in id_types {
            presidio_to_internal.insert(id_type.to_string(), EntityType::Identification);
        }
        internal_to_presidio.insert(EntityType::Identification, "US_SSN".to_string());

        // Technical identifiers
        presidio_to_internal.insert("IP_ADDRESS".to_string(), EntityType::TechnicalIdentifier);
        presidio_to_internal.insert("URL".to_string(), EntityType::TechnicalIdentifier);
        presidio_to_internal.insert("MAC_ADDRESS".to_string(), EntityType::TechnicalIdentifier);
        internal_to_presidio.insert(EntityType::TechnicalIdentifier, "IP_ADDRESS".to_string());

        // Case numbers (not directly in Presidio, but we map custom types)
        internal_to_presidio.insert(EntityType::Case, "CASE_NUMBER".to_string());

        // Law references (not in Presidio - preserve these)
        internal_to_presidio.insert(EntityType::Law, "LAW_REFERENCE".to_string());

        Self {
            presidio_to_internal,
            internal_to_presidio,
        }
    }

    /// Convert Presidio entity type string to internal EntityType
    pub fn to_internal(&self, presidio_type: &str) -> Option<EntityType> {
        self.presidio_to_internal.get(presidio_type).copied()
    }

    /// Convert internal EntityType to Presidio type string
    pub fn to_presidio(&self, internal_type: EntityType) -> Option<String> {
        self.internal_to_presidio.get(&internal_type).cloned()
    }

    /// Convert a Presidio entity to internal Entity format
    pub fn convert_entity(&self, presidio_entity: &PresidioEntity, text: &str) -> Option<Entity> {
        let entity_type = self.to_internal(&presidio_entity.entity_type)?;

        // Extract the actual text from the original
        let entity_text = if presidio_entity.end <= text.len() {
            text[presidio_entity.start..presidio_entity.end].to_string()
        } else {
            // Fallback if indices are out of bounds
            return None;
        };

        Some(Entity::new(
            entity_type,
            entity_text,
            presidio_entity.start,
            presidio_entity.end,
            presidio_entity.score,
        ))
    }

    /// Convert multiple Presidio entities to internal format
    pub fn convert_entities(&self, presidio_entities: &[PresidioEntity], text: &str) -> Vec<Entity> {
        presidio_entities
            .iter()
            .filter_map(|e| self.convert_entity(e, text))
            .collect()
    }

    /// Get all Presidio types that map to a specific internal type
    pub fn get_presidio_types_for(&self, internal_type: EntityType) -> Vec<String> {
        self.presidio_to_internal
            .iter()
            .filter(|(_, v)| **v == internal_type)
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Check if a Presidio type is recognized
    pub fn is_recognized(&self, presidio_type: &str) -> bool {
        self.presidio_to_internal.contains_key(presidio_type)
    }

    /// Get all recognized Presidio types
    pub fn get_all_presidio_types(&self) -> Vec<String> {
        self.presidio_to_internal.keys().cloned().collect()
    }

    /// Add a custom mapping
    pub fn add_mapping(&mut self, presidio_type: String, internal_type: EntityType) {
        self.presidio_to_internal.insert(presidio_type.clone(), internal_type);
        // Only update internal_to_presidio if not already set
        self.internal_to_presidio.entry(internal_type).or_insert(presidio_type);
    }
}

impl Default for EntityTypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Confidence level adjustment for Presidio results
/// Presidio scores can be adjusted based on context
pub struct ConfidenceAdjuster {
    /// Boost factor for entities found by multiple recognizers
    multi_recognizer_boost: f64,
    /// Minimum confidence to keep an entity
    min_confidence: f64,
    /// Context keywords that boost confidence per entity type
    context_keywords: HashMap<EntityType, Vec<String>>,
}

impl ConfidenceAdjuster {
    pub fn new() -> Self {
        let mut context_keywords = HashMap::new();

        // Keywords that increase confidence for Person entities
        context_keywords.insert(
            EntityType::Person,
            vec![
                "mr.".to_string(),
                "mrs.".to_string(),
                "ms.".to_string(),
                "dr.".to_string(),
                "prof.".to_string(),
                "attorney".to_string(),
                "counsel".to_string(),
                "plaintiff".to_string(),
                "defendant".to_string(),
                "witness".to_string(),
                "client".to_string(),
            ],
        );

        // Keywords for Organization
        context_keywords.insert(
            EntityType::Organization,
            vec![
                "inc.".to_string(),
                "llc".to_string(),
                "ltd.".to_string(),
                "corp.".to_string(),
                "company".to_string(),
                "firm".to_string(),
                "court".to_string(),
                "tribunal".to_string(),
            ],
        );

        // Keywords for Legal references (to potentially filter out)
        context_keywords.insert(
            EntityType::Law,
            vec![
                "article".to_string(),
                "section".to_string(),
                "§".to_string(),
                "gdpr".to_string(),
                "regulation".to_string(),
                "directive".to_string(),
                "statute".to_string(),
            ],
        );

        Self {
            multi_recognizer_boost: 0.1,
            min_confidence: 0.5,
            context_keywords,
        }
    }

    /// Adjust confidence based on surrounding context
    pub fn adjust_confidence(&self, entity: &Entity, surrounding_text: &str) -> f64 {
        let mut confidence = entity.confidence;

        // Check for context keywords
        if let Some(keywords) = self.context_keywords.get(&entity.entity_type) {
            let lower_context = surrounding_text.to_lowercase();
            for keyword in keywords {
                if lower_context.contains(keyword) {
                    confidence += 0.05;
                }
            }
        }

        // Cap at 1.0
        confidence.min(1.0)
    }

    /// Filter entities by minimum confidence
    pub fn filter_by_confidence(&self, entities: Vec<Entity>) -> Vec<Entity> {
        entities
            .into_iter()
            .filter(|e| e.confidence >= self.min_confidence)
            .collect()
    }

    /// Set minimum confidence threshold
    pub fn set_min_confidence(&mut self, min: f64) {
        self.min_confidence = min.max(0.0).min(1.0);
    }
}

impl Default for ConfidenceAdjuster {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presidio_to_internal_mapping() {
        let mapper = EntityTypeMapper::new();

        assert_eq!(mapper.to_internal("PERSON"), Some(EntityType::Person));
        assert_eq!(mapper.to_internal("EMAIL_ADDRESS"), Some(EntityType::Email));
        assert_eq!(mapper.to_internal("US_SSN"), Some(EntityType::Identification));
        assert_eq!(mapper.to_internal("LOCATION"), Some(EntityType::Location));
    }

    #[test]
    fn test_internal_to_presidio_mapping() {
        let mapper = EntityTypeMapper::new();

        assert_eq!(mapper.to_presidio(EntityType::Person), Some("PERSON".to_string()));
        assert_eq!(mapper.to_presidio(EntityType::Email), Some("EMAIL_ADDRESS".to_string()));
    }

    #[test]
    fn test_unknown_type() {
        let mapper = EntityTypeMapper::new();

        assert_eq!(mapper.to_internal("UNKNOWN_TYPE"), None);
    }

    #[test]
    fn test_convert_entity() {
        let mapper = EntityTypeMapper::new();
        let text = "John Doe is here";

        let presidio_entity = PresidioEntity {
            entity_type: "PERSON".to_string(),
            start: 0,
            end: 8,
            score: 0.95,
            analysis_explanation: None,
            recognition_metadata: None,
        };

        let entity = mapper.convert_entity(&presidio_entity, text).unwrap();

        assert_eq!(entity.entity_type, EntityType::Person);
        assert_eq!(entity.text, "John Doe");
        assert_eq!(entity.confidence, 0.95);
    }

    #[test]
    fn test_get_presidio_types_for() {
        let mapper = EntityTypeMapper::new();

        let id_types = mapper.get_presidio_types_for(EntityType::Identification);
        assert!(id_types.contains(&"US_SSN".to_string()));
        assert!(id_types.contains(&"CREDIT_CARD".to_string()));
    }

    #[test]
    fn test_confidence_adjuster() {
        let adjuster = ConfidenceAdjuster::new();
        let entity = Entity::new(EntityType::Person, "John".to_string(), 0, 4, 0.6);

        // Context with "Mr." should boost confidence
        let adjusted = adjuster.adjust_confidence(&entity, "Mr. John testified");
        assert!(adjusted > 0.6);
    }
}
