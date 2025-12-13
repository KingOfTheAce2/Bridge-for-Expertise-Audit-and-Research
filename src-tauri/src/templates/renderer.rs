use anyhow::Result;
use std::collections::HashMap;

use crate::prompts::substitute_variables;

/// Render a template with the given variables
///
/// This is a simple pass-through to the variable substitution engine
/// for now. In the future, this could handle more complex rendering
/// like markdown-to-HTML conversion, PDF generation, etc.
#[allow(dead_code)]
pub fn render_template(template: &str, values: &HashMap<String, String>) -> Result<String> {
    substitute_variables(template, values)
}

/// Render template to HTML (future enhancement)
#[allow(dead_code)]
fn render_to_html(_template: &str, _values: &HashMap<String, String>) -> Result<String> {
    // Future: Use a markdown-to-HTML library like pulldown-cmark
    // For now, just return an error
    anyhow::bail!("HTML rendering not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_template() {
        let template = "Hello {NAME}, today is {DATE}";
        let mut values = HashMap::new();
        values.insert("NAME".to_string(), "World".to_string());
        values.insert("DATE".to_string(), "2025-01-26".to_string());

        let result = render_template(template, &values).unwrap();
        assert_eq!(result, "Hello World, today is 2025-01-26");
    }
}
