use super::Prompt;

/// Search prompts by query string
///
/// Searches across:
/// - Name
/// - Description
/// - Content
/// - Tags
/// - Category
///
/// Returns prompts sorted by relevance score (highest first)
pub fn search_prompts(prompts: &[Prompt], query: &str) -> Vec<Prompt> {
    if query.is_empty() {
        return prompts.to_vec();
    }

    let query_lower = query.to_lowercase();
    let terms: Vec<&str> = query_lower.split_whitespace().collect();

    let mut scored_prompts: Vec<(Prompt, usize)> = prompts
        .iter()
        .filter_map(|prompt| {
            let score = calculate_relevance_score(prompt, &terms);
            if score > 0 {
                Some((prompt.clone(), score))
            } else {
                None
            }
        })
        .collect();

    // Sort by score (highest first)
    scored_prompts.sort_by(|a, b| b.1.cmp(&a.1));

    scored_prompts.into_iter().map(|(prompt, _)| prompt).collect()
}

/// Calculate relevance score for a prompt based on search terms
fn calculate_relevance_score(prompt: &Prompt, terms: &[&str]) -> usize {
    let mut score = 0;

    let name_lower = prompt.name.to_lowercase();
    let desc_lower = prompt.description.to_lowercase();
    let content_lower = prompt.content.to_lowercase();
    let category_lower = prompt.category.to_lowercase();

    for term in terms {
        // Name matches (highest weight)
        if name_lower.contains(term) {
            score += 50;
            if name_lower.starts_with(term) {
                score += 25; // Bonus for prefix match
            }
        }

        // Category matches (high weight)
        if category_lower.contains(term) {
            score += 30;
        }

        // Tag matches (high weight)
        for tag in &prompt.tags {
            if tag.to_lowercase().contains(term) {
                score += 25;
            }
        }

        // Description matches (medium weight)
        if desc_lower.contains(term) {
            score += 15;
        }

        // Content matches (lower weight)
        if content_lower.contains(term) {
            score += 5;
        }
    }

    score
}

/// Filter prompts by multiple criteria
#[allow(dead_code)]
pub struct PromptFilter {
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub tier: Option<super::LicenseTier>,
    pub builtin_only: bool,
    pub user_only: bool,
}

impl Default for PromptFilter {
    fn default() -> Self {
        Self {
            category: None,
            tags: Vec::new(),
            language: None,
            tier: None,
            builtin_only: false,
            user_only: false,
        }
    }
}

impl PromptFilter {
    /// Apply filter to a list of prompts
    #[allow(dead_code)]
    pub fn apply(&self, prompts: &[Prompt]) -> Vec<Prompt> {
        prompts
            .iter()
            .filter(|p| self.matches(p))
            .cloned()
            .collect()
    }

    /// Check if a prompt matches the filter criteria
    fn matches(&self, prompt: &Prompt) -> bool {
        // Category filter
        if let Some(ref category) = self.category {
            if !prompt.category.eq_ignore_ascii_case(category) {
                return false;
            }
        }

        // Tags filter (prompt must have ALL specified tags)
        if !self.tags.is_empty() {
            for tag in &self.tags {
                if !prompt
                    .tags
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case(tag))
                {
                    return false;
                }
            }
        }

        // Language filter
        if let Some(ref language) = self.language {
            if !prompt.language.eq_ignore_ascii_case(language) {
                return false;
            }
        }

        // Tier filter
        if let Some(ref tier) = self.tier {
            if !prompt.check_access(*tier) {
                return false;
            }
        }

        // Builtin filter
        if self.builtin_only && !prompt.is_builtin {
            return false;
        }

        // User filter
        if self.user_only && prompt.is_builtin {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompts::{LicenseTier, Prompt};

    fn create_test_prompts() -> Vec<Prompt> {
        vec![
            Prompt {
                id: "1".to_string(),
                name: "Contract Review".to_string(),
                description: "Review legal contracts".to_string(),
                category: "legal".to_string(),
                content: "Analyze this contract...".to_string(),
                variables: vec![],
                tags: vec!["contract".to_string(), "legal".to_string()],
                language: "en".to_string(),
                tier: LicenseTier::Basic,
                version: "1.0".to_string(),
                author: None,
                created: None,
                is_builtin: true,
                file_path: None,
            },
            Prompt {
                id: "2".to_string(),
                name: "GDPR Advisor".to_string(),
                description: "GDPR compliance guidance".to_string(),
                category: "compliance".to_string(),
                content: "Review for GDPR compliance...".to_string(),
                variables: vec![],
                tags: vec!["gdpr".to_string(), "privacy".to_string()],
                language: "en".to_string(),
                tier: LicenseTier::Pro,
                version: "1.0".to_string(),
                author: None,
                created: None,
                is_builtin: true,
                file_path: None,
            },
            Prompt {
                id: "3".to_string(),
                name: "Meeting Notes".to_string(),
                description: "Summarize meeting notes".to_string(),
                category: "general".to_string(),
                content: "Summarize these notes...".to_string(),
                variables: vec![],
                tags: vec!["summary".to_string()],
                language: "en".to_string(),
                tier: LicenseTier::Basic,
                version: "1.0".to_string(),
                author: None,
                created: None,
                is_builtin: false,
                file_path: None,
            },
        ]
    }

    #[test]
    fn test_search_by_name() {
        let prompts = create_test_prompts();
        let results = search_prompts(&prompts, "contract");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Contract Review");
    }

    #[test]
    fn test_search_by_tag() {
        let prompts = create_test_prompts();
        let results = search_prompts(&prompts, "gdpr");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "GDPR Advisor");
    }

    #[test]
    fn test_search_by_category() {
        let prompts = create_test_prompts();
        let results = search_prompts(&prompts, "legal");

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].category, "legal");
    }

    #[test]
    fn test_filter_by_category() {
        let prompts = create_test_prompts();
        let filter = PromptFilter {
            category: Some("legal".to_string()),
            ..Default::default()
        };

        let results = filter.apply(&prompts);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Contract Review");
    }

    #[test]
    fn test_filter_by_tier() {
        let prompts = create_test_prompts();
        let filter = PromptFilter {
            tier: Some(LicenseTier::Pro),
            ..Default::default()
        };

        let results = filter.apply(&prompts);
        // Pro tier can access Pro and Basic tiers, which is all 3 test prompts
        // (2 Basic + 1 Pro = 3 accessible prompts)
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_filter_builtin_only() {
        let prompts = create_test_prompts();
        let filter = PromptFilter {
            builtin_only: true,
            ..Default::default()
        };

        let results = filter.apply(&prompts);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.is_builtin));
    }
}
