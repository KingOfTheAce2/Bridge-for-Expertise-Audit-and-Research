use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

/// Substitute variables in a template string
///
/// Variables are in the format {VARIABLE_NAME}
/// Variable names must be uppercase with underscores
///
/// Example:
/// ```
/// let template = "Hello {NAME}, your email is {EMAIL}";
/// let mut values = HashMap::new();
/// values.insert("NAME".to_string(), "John".to_string());
/// values.insert("EMAIL".to_string(), "john@example.com".to_string());
///
/// let result = substitute_variables(template, &values)?;
/// // result: "Hello John, your email is john@example.com"
/// ```
pub fn substitute_variables(template: &str, values: &HashMap<String, String>) -> Result<String> {
    let re = Regex::new(r"\{([A-Z_][A-Z0-9_]*)\}").unwrap();
    let mut result = template.to_string();
    let mut missing_vars = Vec::new();

    // Find all variables in template
    for cap in re.captures_iter(template) {
        let var_name = &cap[1];

        if let Some(value) = values.get(var_name) {
            // Replace all occurrences of this variable
            let placeholder = format!("{{{}}}", var_name);
            result = result.replace(&placeholder, value);
        } else {
            missing_vars.push(var_name.to_string());
        }
    }

    // Report missing variables
    if !missing_vars.is_empty() {
        anyhow::bail!(
            "Missing values for variables: {}",
            missing_vars.join(", ")
        );
    }

    Ok(result)
}

/// Extract variable names from a template string
///
/// Returns a list of unique variable names found in the template
#[allow(dead_code)]
pub fn extract_variables(template: &str) -> Vec<String> {
    let re = Regex::new(r"\{([A-Z_][A-Z0-9_]*)\}").unwrap();

    let mut vars: Vec<String> = re
        .captures_iter(template)
        .map(|cap| cap[1].to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    vars.sort();
    vars
}

/// Validate that all variables in template have values
#[allow(dead_code)]
pub fn validate_variables(template: &str, values: &HashMap<String, String>) -> Result<()> {
    let required_vars = extract_variables(template);

    let missing: Vec<String> = required_vars
        .iter()
        .filter(|var| !values.contains_key(*var))
        .cloned()
        .collect();

    if !missing.is_empty() {
        anyhow::bail!("Missing values for variables: {}", missing.join(", "));
    }

    Ok(())
}

/// Create a variable substitution context with default values
#[allow(dead_code)]
pub fn create_default_context() -> HashMap<String, String> {
    let mut context = HashMap::new();

    // Date/time variables
    let now = chrono::Utc::now();
    context.insert("DATE".to_string(), now.format("%Y-%m-%d").to_string());
    context.insert("TIME".to_string(), now.format("%H:%M:%S").to_string());
    context.insert(
        "DATETIME".to_string(),
        now.format("%Y-%m-%d %H:%M:%S").to_string(),
    );
    context.insert("YEAR".to_string(), now.format("%Y").to_string());
    context.insert("MONTH".to_string(), now.format("%m").to_string());
    context.insert("DAY".to_string(), now.format("%d").to_string());

    context
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_variables() {
        let template = "Hello {NAME}, your email is {EMAIL}!";
        let mut values = HashMap::new();
        values.insert("NAME".to_string(), "John".to_string());
        values.insert("EMAIL".to_string(), "john@example.com".to_string());

        let result = substitute_variables(template, &values).unwrap();
        assert_eq!(result, "Hello John, your email is john@example.com!");
    }

    #[test]
    fn test_substitute_missing_variable() {
        let template = "Hello {NAME}!";
        let values = HashMap::new();

        let result = substitute_variables(template, &values);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Missing values for variables: NAME"));
    }

    #[test]
    fn test_extract_variables() {
        let template = "Contract for {CLIENT_NAME} dated {DATE} with amount {AMOUNT}";
        let vars = extract_variables(template);

        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&"CLIENT_NAME".to_string()));
        assert!(vars.contains(&"DATE".to_string()));
        assert!(vars.contains(&"AMOUNT".to_string()));
    }

    #[test]
    fn test_extract_duplicate_variables() {
        let template = "Hello {NAME}! Welcome {NAME}!";
        let vars = extract_variables(template);

        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0], "NAME");
    }

    #[test]
    fn test_validate_variables() {
        let template = "Hello {NAME}!";
        let mut values = HashMap::new();
        values.insert("NAME".to_string(), "John".to_string());

        let result = validate_variables(template, &values);
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_context() {
        let context = create_default_context();

        assert!(context.contains_key("DATE"));
        assert!(context.contains_key("TIME"));
        assert!(context.contains_key("DATETIME"));
        assert!(context.contains_key("YEAR"));
    }

    #[test]
    fn test_substitute_with_default_context() {
        let template = "Today is {DATE}";
        let context = create_default_context();

        let result = substitute_variables(template, &context);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("Today is 20")); // Should start with year 20xx
    }
}
