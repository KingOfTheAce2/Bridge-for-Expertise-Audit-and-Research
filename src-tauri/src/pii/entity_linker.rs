use std::collections::HashMap;

/// Entity linker for matching variations of the same entity
pub struct EntityLinker {
    // Map canonical form to all variations
    entity_map: HashMap<String, Vec<String>>,
}

impl EntityLinker {
    pub fn new() -> Self {
        Self {
            entity_map: HashMap::new(),
        }
    }

    /// Get canonical form of an entity (for consistent replacement)
    pub fn get_canonical(&self, text: &str) -> String {
        // Normalize the text
        let normalized = self.normalize_text(text);

        // Check if we have a canonical form for this
        for (canonical, variations) in &self.entity_map {
            if variations.iter().any(|v| v == &normalized) {
                return canonical.clone();
            }
        }

        // No match found, this is a new canonical form
        normalized
    }

    /// Link a variation to a canonical form
    pub fn link_variation(&mut self, canonical: &str, variation: &str) {
        let canonical_normalized = self.normalize_text(canonical);
        let variation_normalized = self.normalize_text(variation);

        self.entity_map
            .entry(canonical_normalized.clone())
            .or_insert_with(|| vec![canonical_normalized.clone()])
            .push(variation_normalized);
    }

    /// Check if two entities might be the same person
    pub fn might_be_same_person(&self, text1: &str, text2: &str) -> bool {
        let norm1 = self.normalize_text(text1);
        let norm2 = self.normalize_text(text2);

        // Exact match
        if norm1 == norm2 {
            return true;
        }

        // Check if one is a substring of the other (e.g., "John Doe" and "Mr. John Doe")
        if norm1.contains(&norm2) || norm2.contains(&norm1) {
            return true;
        }

        // Check if they share the same last name
        if let (Some(last1), Some(last2)) = (self.extract_last_name(&norm1), self.extract_last_name(&norm2)) {
            if last1 == last2 && !last1.is_empty() {
                // Same last name - might be same person
                // Additional check: do they share initials?
                if self.share_initials(&norm1, &norm2) {
                    return true;
                }
            }
        }

        false
    }

    fn normalize_text(&self, text: &str) -> String {
        // Remove titles
        let without_titles = self.remove_titles(text);

        // Lowercase and trim
        without_titles.to_lowercase().trim().to_string()
    }

    fn remove_titles(&self, text: &str) -> String {
        let titles = ["mr.", "mrs.", "ms.", "dr.", "prof.", "mr", "mrs", "ms", "dr", "prof"];

        let mut result = text.to_string();
        for title in &titles {
            result = result.replace(&format!("{} ", title), "");
            result = result.replace(&format!("{}. ", title), "");
        }

        result.trim().to_string()
    }

    fn extract_last_name(&self, text: &str) -> Option<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.len() >= 2 {
            Some(words.last().unwrap().to_string())
        } else {
            None
        }
    }

    fn share_initials(&self, text1: &str, text2: &str) -> bool {
        let words1: Vec<&str> = text1.split_whitespace().collect();
        let words2: Vec<&str> = text2.split_whitespace().collect();

        if words1.is_empty() || words2.is_empty() {
            return false;
        }

        // Get first letter of each name
        let initials1: Vec<char> = words1.iter().filter_map(|w| w.chars().next()).collect();
        let initials2: Vec<char> = words2.iter().filter_map(|w| w.chars().next()).collect();

        // Check if initials overlap
        initials1.iter().any(|i1| initials2.contains(i1))
    }

    /// Auto-link entities based on similarity
    pub fn auto_link_entities(&mut self, entities: &[String]) {
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                if self.might_be_same_person(&entities[i], &entities[j]) {
                    // Link them
                    let canonical = entities[i].clone();
                    let variation = entities[j].clone();
                    self.link_variation(&canonical, &variation);
                }
            }
        }
    }
}

impl Default for EntityLinker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        let linker = EntityLinker::new();

        assert_eq!(linker.normalize_text("Mr. John Doe"), "john doe");
        assert_eq!(linker.normalize_text("Dr. Jane Smith"), "jane smith");
        assert_eq!(linker.normalize_text("John DOE"), "john doe");
    }

    #[test]
    fn test_might_be_same_person() {
        let linker = EntityLinker::new();

        // Same person different titles
        assert!(linker.might_be_same_person("John Doe", "Mr. John Doe"));
        assert!(linker.might_be_same_person("Dr. Smith", "Mr. Smith"));

        // Different people
        assert!(!linker.might_be_same_person("John Doe", "Jane Smith"));
    }

    #[test]
    fn test_extract_last_name() {
        let linker = EntityLinker::new();

        assert_eq!(linker.extract_last_name("john doe"), Some("doe".to_string()));
        assert_eq!(linker.extract_last_name("jane"), None);
    }

    #[test]
    fn test_auto_link_entities() {
        let mut linker = EntityLinker::new();

        let entities = vec![
            "John Doe".to_string(),
            "Mr. John Doe".to_string(),
            "J. Doe".to_string(),
            "Jane Smith".to_string(),
        ];

        linker.auto_link_entities(&entities);

        // John Doe variations should be linked
        assert!(linker.might_be_same_person("John Doe", "Mr. John Doe"));
        assert!(linker.might_be_same_person("John Doe", "J. Doe"));

        // Jane Smith should be separate
        assert!(!linker.might_be_same_person("John Doe", "Jane Smith"));
    }
}
