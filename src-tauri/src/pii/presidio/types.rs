//! Presidio-specific types and structures

use serde::{Deserialize, Serialize};

/// Presidio entity types (comprehensive list)
/// See: https://microsoft.github.io/presidio/supported_entities/
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PresidioEntityType {
    // Person-related
    Person,

    // Location-related
    Location,
    GpgCoordinates,

    // Organization
    Organization,

    // Contact information
    Email,
    PhoneNumber,
    Url,
    IpAddress,

    // Financial
    CreditCard,
    Crypto,
    Iban,
    UsBankNumber,
    UkNhs,

    // National IDs
    UsDriverLicense,
    UsSocialSecurityNumber,
    UsItin,
    UsPassport,
    UkNino,
    AuAbn,
    AuAcn,
    AuTfn,
    AuMedicare,
    InAadhaar,
    InPan,
    InVoterId,
    SgNric,
    SgFin,
    ItFiscalCode,
    ItDriverLicense,
    ItVat,
    ItPassport,
    ItIdentityCard,
    EsNif,
    EsNie,
    PlPesel,
    PlNip,
    PlRegon,
    MedicalLicense,

    // Dates
    DateTime,
    DateOfBirth,

    // Generic
    NrpNumber,
    AgeNumber,

    // Custom/Other
    #[serde(other)]
    Custom,
}

impl PresidioEntityType {
    /// Get all standard entity types
    pub fn all_standard() -> Vec<Self> {
        vec![
            Self::Person,
            Self::Location,
            Self::Organization,
            Self::Email,
            Self::PhoneNumber,
            Self::Url,
            Self::IpAddress,
            Self::CreditCard,
            Self::Iban,
            Self::UsSocialSecurityNumber,
            Self::UsDriverLicense,
            Self::UsPassport,
            Self::UkNino,
            Self::UkNhs,
            Self::DateTime,
        ]
    }

    /// Get entity types relevant for European legal documents
    pub fn european_legal() -> Vec<Self> {
        vec![
            Self::Person,
            Self::Location,
            Self::Organization,
            Self::Email,
            Self::PhoneNumber,
            Self::Iban,
            Self::ItFiscalCode,
            Self::EsNif,
            Self::EsNie,
            Self::PlPesel,
            Self::DateTime,
        ]
    }

    /// Convert to string for API calls
    pub fn as_str(&self) -> &str {
        match self {
            Self::Person => "PERSON",
            Self::Location => "LOCATION",
            Self::GpgCoordinates => "GPG_COORDINATES",
            Self::Organization => "ORGANIZATION",
            Self::Email => "EMAIL_ADDRESS",
            Self::PhoneNumber => "PHONE_NUMBER",
            Self::Url => "URL",
            Self::IpAddress => "IP_ADDRESS",
            Self::CreditCard => "CREDIT_CARD",
            Self::Crypto => "CRYPTO",
            Self::Iban => "IBAN_CODE",
            Self::UsBankNumber => "US_BANK_NUMBER",
            Self::UkNhs => "UK_NHS",
            Self::UsDriverLicense => "US_DRIVER_LICENSE",
            Self::UsSocialSecurityNumber => "US_SSN",
            Self::UsItin => "US_ITIN",
            Self::UsPassport => "US_PASSPORT",
            Self::UkNino => "UK_NINO",
            Self::AuAbn => "AU_ABN",
            Self::AuAcn => "AU_ACN",
            Self::AuTfn => "AU_TFN",
            Self::AuMedicare => "AU_MEDICARE",
            Self::InAadhaar => "IN_AADHAAR",
            Self::InPan => "IN_PAN",
            Self::InVoterId => "IN_VOTER",
            Self::SgNric => "SG_NRIC_FIN",
            Self::SgFin => "SG_NRIC_FIN",
            Self::ItFiscalCode => "IT_FISCAL_CODE",
            Self::ItDriverLicense => "IT_DRIVER_LICENSE",
            Self::ItVat => "IT_VAT_CODE",
            Self::ItPassport => "IT_PASSPORT",
            Self::ItIdentityCard => "IT_IDENTITY_CARD",
            Self::EsNif => "ES_NIF",
            Self::EsNie => "ES_NIE",
            Self::PlPesel => "PL_PESEL",
            Self::PlNip => "PL_NIP",
            Self::PlRegon => "PL_REGON",
            Self::MedicalLicense => "MEDICAL_LICENSE",
            Self::DateTime => "DATE_TIME",
            Self::DateOfBirth => "DATE_OF_BIRTH",
            Self::NrpNumber => "NRP",
            Self::AgeNumber => "AGE",
            Self::Custom => "CUSTOM",
        }
    }
}

/// Entity detected by Presidio analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidioEntity {
    /// Entity type (e.g., "PERSON", "EMAIL_ADDRESS")
    pub entity_type: String,
    /// Start position in text
    pub start: usize,
    /// End position in text
    pub end: usize,
    /// Confidence score (0.0 to 1.0)
    pub score: f64,
    /// Analysis explanation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_explanation: Option<AnalysisExplanation>,
    /// Recognition metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_metadata: Option<RecognitionMetadata>,
}

/// Explanation for why an entity was detected
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisExplanation {
    /// Recognizer name that detected this entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<String>,
    /// Pattern name (if pattern-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_name: Option<String>,
    /// Pattern (if pattern-based)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Original score before adjustments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_score: Option<f64>,
    /// Score adjustments applied
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_context_improvement: Option<f64>,
    /// Supportive context words found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportive_context_word: Option<String>,
    /// Validation result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_result: Option<f64>,
}

/// Recognition metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionMetadata {
    /// Recognizer name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_name: Option<String>,
    /// Recognizer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_identifier: Option<String>,
}

/// Anonymization operator types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AnonymizationOperator {
    /// Replace with a fixed value
    Replace {
        new_value: String,
    },
    /// Redact (remove) the entity
    Redact,
    /// Hash the entity value
    Hash {
        #[serde(skip_serializing_if = "Option::is_none")]
        hash_type: Option<String>,
    },
    /// Mask with a character
    Mask {
        masking_char: char,
        chars_to_mask: usize,
        from_end: bool,
    },
    /// Encrypt the value
    Encrypt {
        key: String,
    },
    /// Keep the original value (no anonymization)
    Keep,
    /// Custom operator
    Custom {
        lambda: String,
    },
}

impl Default for AnonymizationOperator {
    fn default() -> Self {
        Self::Replace {
            new_value: "<REDACTED>".to_string(),
        }
    }
}

/// Configuration for a specific entity type's anonymization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityOperatorConfig {
    /// Entity type this config applies to
    pub entity_type: String,
    /// Operator to use for this entity type
    pub operator: AnonymizationOperator,
}

/// Anonymization request to Presidio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidioAnonymizeRequest {
    /// Text to anonymize
    pub text: String,
    /// Analyzer results (entities found)
    pub analyzer_results: Vec<PresidioEntity>,
    /// Operators to use for anonymization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operators: Option<std::collections::HashMap<String, AnonymizationOperator>>,
}

/// Result from Presidio anonymizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidioAnonymizeResult {
    /// Anonymized text
    pub text: String,
    /// Items that were anonymized
    #[serde(default)]
    pub items: Vec<AnonymizedItem>,
}

/// An item that was anonymized
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymizedItem {
    /// Start position in original text
    pub start: usize,
    /// End position in original text
    pub end: usize,
    /// Entity type
    pub entity_type: String,
    /// Original text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Operator used
    pub operator: String,
}

/// Analyze request to Presidio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidioAnalyzeRequest {
    /// Text to analyze
    pub text: String,
    /// Language of the text
    pub language: String,
    /// Specific entity types to look for (optional, all if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<String>>,
    /// Minimum confidence score threshold
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    /// Whether to return analysis explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_decision_process: Option<bool>,
}

/// Presidio analyzer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresidioConfig {
    /// Analyzer API endpoint
    pub analyzer_url: String,
    /// Anonymizer API endpoint
    pub anonymizer_url: String,
    /// Default language
    pub default_language: String,
    /// Minimum confidence score
    pub score_threshold: f64,
    /// Entity types to detect
    pub entity_types: Vec<String>,
    /// Custom operators per entity type
    pub operators: std::collections::HashMap<String, AnonymizationOperator>,
}

impl Default for PresidioConfig {
    fn default() -> Self {
        Self {
            analyzer_url: "http://localhost:5002".to_string(),
            anonymizer_url: "http://localhost:5001".to_string(),
            default_language: "en".to_string(),
            score_threshold: 0.5,
            entity_types: PresidioEntityType::all_standard()
                .iter()
                .map(|e| e.as_str().to_string())
                .collect(),
            operators: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_type_as_str() {
        assert_eq!(PresidioEntityType::Person.as_str(), "PERSON");
        assert_eq!(PresidioEntityType::Email.as_str(), "EMAIL_ADDRESS");
        assert_eq!(PresidioEntityType::UsSocialSecurityNumber.as_str(), "US_SSN");
    }

    #[test]
    fn test_default_config() {
        let config = PresidioConfig::default();
        assert_eq!(config.analyzer_url, "http://localhost:5002");
        assert_eq!(config.score_threshold, 0.5);
    }

    #[test]
    fn test_anonymization_operator_serialization() {
        let operator = AnonymizationOperator::Mask {
            masking_char: '*',
            chars_to_mask: 4,
            from_end: false,
        };

        let json = serde_json::to_string(&operator).unwrap();
        assert!(json.contains("mask"));
    }
}
