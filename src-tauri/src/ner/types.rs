use serde::{Deserialize, Serialize};

/// NER entity labels following standard BIO tagging scheme
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NerLabel {
    // Outside any entity
    O,

    // Person entity
    #[serde(rename = "B-PER")]
    BeginPerson,
    #[serde(rename = "I-PER")]
    InsidePerson,

    // Organization entity
    #[serde(rename = "B-ORG")]
    BeginOrganization,
    #[serde(rename = "I-ORG")]
    InsideOrganization,

    // Location entity
    #[serde(rename = "B-LOC")]
    BeginLocation,
    #[serde(rename = "I-LOC")]
    InsideLocation,

    // Miscellaneous entity (dates, events, etc.)
    #[serde(rename = "B-MISC")]
    BeginMisc,
    #[serde(rename = "I-MISC")]
    InsideMisc,
}

impl NerLabel {
    /// Convert label to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            NerLabel::O => "O",
            NerLabel::BeginPerson => "B-PER",
            NerLabel::InsidePerson => "I-PER",
            NerLabel::BeginOrganization => "B-ORG",
            NerLabel::InsideOrganization => "I-ORG",
            NerLabel::BeginLocation => "B-LOC",
            NerLabel::InsideLocation => "I-LOC",
            NerLabel::BeginMisc => "B-MISC",
            NerLabel::InsideMisc => "I-MISC",
        }
    }

    /// Convert from label ID (0-8)
    pub fn from_id(id: usize) -> Option<Self> {
        match id {
            0 => Some(NerLabel::O),
            1 => Some(NerLabel::BeginPerson),
            2 => Some(NerLabel::InsidePerson),
            3 => Some(NerLabel::BeginOrganization),
            4 => Some(NerLabel::InsideOrganization),
            5 => Some(NerLabel::BeginLocation),
            6 => Some(NerLabel::InsideLocation),
            7 => Some(NerLabel::BeginMisc),
            8 => Some(NerLabel::InsideMisc),
            _ => None,
        }
    }

    /// Get label ID
    pub fn to_id(&self) -> usize {
        match self {
            NerLabel::O => 0,
            NerLabel::BeginPerson => 1,
            NerLabel::InsidePerson => 2,
            NerLabel::BeginOrganization => 3,
            NerLabel::InsideOrganization => 4,
            NerLabel::BeginLocation => 5,
            NerLabel::InsideLocation => 6,
            NerLabel::BeginMisc => 7,
            NerLabel::InsideMisc => 8,
        }
    }

    /// Check if this is a beginning tag
    pub fn is_begin(&self) -> bool {
        matches!(
            self,
            NerLabel::BeginPerson
                | NerLabel::BeginOrganization
                | NerLabel::BeginLocation
                | NerLabel::BeginMisc
        )
    }

    /// Check if this is an inside tag
    pub fn is_inside(&self) -> bool {
        matches!(
            self,
            NerLabel::InsidePerson
                | NerLabel::InsideOrganization
                | NerLabel::InsideLocation
                | NerLabel::InsideMisc
        )
    }

    /// Get entity type (without B-/I- prefix)
    pub fn entity_type(&self) -> Option<&'static str> {
        match self {
            NerLabel::O => None,
            NerLabel::BeginPerson | NerLabel::InsidePerson => Some("PER"),
            NerLabel::BeginOrganization | NerLabel::InsideOrganization => Some("ORG"),
            NerLabel::BeginLocation | NerLabel::InsideLocation => Some("LOC"),
            NerLabel::BeginMisc | NerLabel::InsideMisc => Some("MISC"),
        }
    }
}

/// NER prediction for a single token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrediction {
    pub token: String,
    pub label: NerLabel,
    pub confidence: f32,
    pub start: usize,  // Character start position in original text
    pub end: usize,    // Character end position in original text
}

/// Extracted named entity (multiple tokens combined)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerEntity {
    pub text: String,
    pub entity_type: String,  // "PER", "ORG", "LOC", "MISC"
    pub confidence: f32,      // Average confidence of constituent tokens
    pub start: usize,
    pub end: usize,
    pub tokens: Vec<TokenPrediction>,
}

/// NER inference result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerResult {
    pub text: String,
    pub entities: Vec<NerEntity>,
    pub token_predictions: Vec<TokenPrediction>,
    pub inference_time_ms: u64,
}

/// NER model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerModelConfig {
    pub model_id: String,
    pub model_type: String,       // "bert", "roberta", "distilbert"
    pub num_labels: usize,         // Usually 9 for BIO tagging
    pub max_sequence_length: usize, // Usually 512
    pub hidden_size: usize,        // e.g., 768 for BERT-base
    pub vocab_size: usize,
    pub label_map: Vec<String>,    // Maps label IDs to names
}

impl Default for NerModelConfig {
    fn default() -> Self {
        Self {
            model_id: "dslim/bert-base-NER".to_string(),
            model_type: "bert".to_string(),
            num_labels: 9,
            max_sequence_length: 512,
            hidden_size: 768,
            vocab_size: 28996,
            label_map: vec![
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
        }
    }
}

/// NER model information for registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NerModelInfo {
    pub model_id: String,
    pub name: String,
    pub description: String,
    pub provider: String,
    pub model_type: String,
    pub language: String,
    pub entity_labels: Vec<String>,
    pub size: String,
    pub parameters: String,
    pub format: String,
    pub model_url: String,
    pub config_url: String,
    pub tokenizer_url: String,
    pub file_size: i64,
    pub checksum: Option<String>,
    pub license: String,
    pub accuracy: Option<f64>,  // F1 score on CoNLL-2003 or similar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ner_label_conversions() {
        assert_eq!(NerLabel::from_id(0), Some(NerLabel::O));
        assert_eq!(NerLabel::from_id(1), Some(NerLabel::BeginPerson));
        assert_eq!(NerLabel::BeginPerson.to_id(), 1);
        assert_eq!(NerLabel::BeginPerson.as_str(), "B-PER");
    }

    #[test]
    fn test_ner_label_properties() {
        assert!(NerLabel::BeginPerson.is_begin());
        assert!(!NerLabel::InsidePerson.is_begin());
        assert!(NerLabel::InsidePerson.is_inside());
        assert_eq!(NerLabel::BeginPerson.entity_type(), Some("PER"));
        assert_eq!(NerLabel::O.entity_type(), None);
    }
}
