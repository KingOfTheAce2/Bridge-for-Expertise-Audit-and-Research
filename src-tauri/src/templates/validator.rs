use anyhow::Result;
use regex::Regex;

/// Validate template syntax
///
/// Checks for:
/// - Properly formatted variable placeholders
/// - Balanced braces
/// - No invalid variable names
pub fn validate_template(template: &str) -> Result<()> {
    // Check for balanced braces
    let mut brace_count = 0;
    for ch in template.chars() {
        match ch {
            '{' => brace_count += 1,
            '}' => brace_count -= 1,
            _ => {}
        }

        if brace_count < 0 {
            anyhow::bail!("Unbalanced braces: closing brace without opening brace");
        }
    }

    if brace_count != 0 {
        anyhow::bail!("Unbalanced braces: {} unclosed opening braces", brace_count);
    }

    // Check variable names
    let var_regex = Regex::new(r"\{([^}]+)\}").unwrap();
    let valid_var_regex = Regex::new(r"^[A-Z_][A-Z0-9_]*$").unwrap();

    for cap in var_regex.captures_iter(template) {
        let var_name = &cap[1];

        if !valid_var_regex.is_match(var_name) {
            anyhow::bail!(
                "Invalid variable name '{}': must be uppercase with underscores (A-Z, 0-9, _)",
                var_name
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_template() {
        let template = "Hello {NAME}, your email is {EMAIL_ADDRESS}";
        assert!(validate_template(template).is_ok());
    }

    #[test]
    fn test_unbalanced_braces_opening() {
        let template = "Hello {NAME";
        let result = validate_template(template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unclosed opening braces"));
    }

    #[test]
    fn test_unbalanced_braces_closing() {
        let template = "Hello NAME}";
        let result = validate_template(template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("closing brace without opening"));
    }

    #[test]
    fn test_invalid_variable_name_lowercase() {
        let template = "Hello {name}";
        let result = validate_template(template);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid variable name"));
    }

    #[test]
    fn test_invalid_variable_name_spaces() {
        let template = "Hello {NAME WITH SPACES}";
        let result = validate_template(template);
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_valid_variables() {
        let template = "Contract between {PARTY_A} and {PARTY_B} dated {DATE}";
        assert!(validate_template(template).is_ok());
    }
}
