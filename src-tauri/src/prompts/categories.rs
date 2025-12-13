use serde::{Deserialize, Serialize};

/// Standard prompt categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PromptCategory {
    /// General purpose prompts
    General,

    /// Contract analysis and review
    ContractAnalysis,

    /// GDPR and data privacy
    DataPrivacy,

    /// Legal research and analysis
    LegalResearch,

    /// Compliance checking
    Compliance,

    /// Document summarization
    Summarization,

    /// Due diligence
    DueDiligence,

    /// Timeline and chronology
    Timeline,

    /// Citation finding
    Citation,

    /// Formal writing
    FormalWriting,

    /// Custom user-defined category
    Custom(String),
}

impl PromptCategory {
    /// Get the category identifier string
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        match self {
            Self::General => "general",
            Self::ContractAnalysis => "contract_analysis",
            Self::DataPrivacy => "data_privacy",
            Self::LegalResearch => "legal_research",
            Self::Compliance => "compliance",
            Self::Summarization => "summarization",
            Self::DueDiligence => "due_diligence",
            Self::Timeline => "timeline",
            Self::Citation => "citation",
            Self::FormalWriting => "formal_writing",
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Get display name for the category
    pub fn display_name(&self) -> &str {
        match self {
            Self::General => "General",
            Self::ContractAnalysis => "Contract Analysis",
            Self::DataPrivacy => "Data Privacy & GDPR",
            Self::LegalResearch => "Legal Research",
            Self::Compliance => "Compliance",
            Self::Summarization => "Document Summarization",
            Self::DueDiligence => "Due Diligence",
            Self::Timeline => "Timeline & Chronology",
            Self::Citation => "Citation & References",
            Self::FormalWriting => "Formal Writing",
            Self::Custom(s) => s.as_str(),
        }
    }

    /// Get description for the category
    #[allow(dead_code)]
    pub fn description(&self) -> &str {
        match self {
            Self::General => "General purpose prompts for various tasks",
            Self::ContractAnalysis => "Review and analyze legal contracts",
            Self::DataPrivacy => "GDPR compliance and data privacy guidance",
            Self::LegalResearch => "Research legal questions and precedents",
            Self::Compliance => "Check documents against regulations",
            Self::Summarization => "Summarize documents and extract key points",
            Self::DueDiligence => "M&A and due diligence analysis",
            Self::Timeline => "Extract chronological events and timelines",
            Self::Citation => "Find and verify legal citations",
            Self::FormalWriting => "Professional and formal document writing",
            Self::Custom(_) => "Custom user-defined category",
        }
    }

    /// Parse category from string
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "general" => Self::General,
            "contract_analysis" | "contract" => Self::ContractAnalysis,
            "data_privacy" | "gdpr" | "privacy" => Self::DataPrivacy,
            "legal_research" | "research" => Self::LegalResearch,
            "compliance" => Self::Compliance,
            "summarization" | "summary" => Self::Summarization,
            "due_diligence" | "diligence" => Self::DueDiligence,
            "timeline" | "chronology" => Self::Timeline,
            "citation" | "citations" => Self::Citation,
            "formal_writing" | "formal" => Self::FormalWriting,
            _ => Self::Custom(s.to_string()),
        }
    }

    /// Get all standard categories
    #[allow(dead_code)]
    pub fn all_standard() -> Vec<Self> {
        vec![
            Self::General,
            Self::ContractAnalysis,
            Self::DataPrivacy,
            Self::LegalResearch,
            Self::Compliance,
            Self::Summarization,
            Self::DueDiligence,
            Self::Timeline,
            Self::Citation,
            Self::FormalWriting,
        ]
    }
}

impl Default for PromptCategory {
    fn default() -> Self {
        Self::General
    }
}

impl std::fmt::Display for PromptCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_as_str() {
        assert_eq!(PromptCategory::General.as_str(), "general");
        assert_eq!(PromptCategory::ContractAnalysis.as_str(), "contract_analysis");
        assert_eq!(PromptCategory::DataPrivacy.as_str(), "data_privacy");
    }

    #[test]
    fn test_category_from_str() {
        assert_eq!(
            PromptCategory::from_str("contract_analysis"),
            PromptCategory::ContractAnalysis
        );
        assert_eq!(
            PromptCategory::from_str("GDPR"),
            PromptCategory::DataPrivacy
        );
        assert_eq!(
            PromptCategory::from_str("custom_type"),
            PromptCategory::Custom("custom_type".to_string())
        );
    }

    #[test]
    fn test_all_standard_categories() {
        let categories = PromptCategory::all_standard();
        assert_eq!(categories.len(), 10);
    }

    #[test]
    fn test_category_display() {
        let category = PromptCategory::ContractAnalysis;
        assert_eq!(format!("{}", category), "Contract Analysis");
    }
}
