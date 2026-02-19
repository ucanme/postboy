//! Environment and variable management models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Id, Timestamp, new_id, now, Temporal, Identifiable};

/// Environment containing variables for substitution in requests
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Environment {
    pub id: Id,
    pub name: String,

    /// Environment variables
    #[serde(default)]
    pub values: Vec<Variable>,

    /// Whether this is the currently active environment
    #[serde(default)]
    pub is_active: bool,

    /// Creation timestamp
    pub created_at: Timestamp,

    /// Last update timestamp
    pub updated_at: Timestamp,
}

impl Environment {
    pub fn new(name: String) -> Self {
        let now = now();
        Self {
            id: new_id(),
            name,
            values: Vec::new(),
            is_active: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_values(mut self, values: Vec<Variable>) -> Self {
        self.values = values;
        self
    }

    pub fn with_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    /// Add a variable to the environment
    pub fn add_variable(&mut self, key: String, value: String) {
        self.values.push(Variable::new(key, value));
        self.updated_at = now();
    }

    /// Get a variable value by key
    pub fn get(&self, key: &str) -> Option<String> {
        self.values
            .iter()
            .find(|v| v.enabled && v.key == key)
            .map(|v| v.value.clone())
    }

    /// Set a variable value (update if exists, add if not)
    pub fn set(&mut self, key: String, value: String) {
        if let Some(var) = self.values.iter_mut().find(|v| v.key == key) {
            var.value = value;
        } else {
            self.add_variable(key, value);
        }
        self.updated_at = now();
    }

    /// Remove a variable by key
    pub fn unset(&mut self, key: &str) -> bool {
        let original_len = self.values.len();
        self.values.retain(|v| v.key != key);
        let removed = self.values.len() < original_len;
        if removed {
            self.updated_at = now();
        }
        removed
    }

    /// Get all enabled variables as a map
    pub fn to_map(&self) -> HashMap<String, String> {
        self.values
            .iter()
            .filter(|v| v.enabled)
            .map(|v| (v.key.clone(), v.value.clone()))
            .collect()
    }

    /// Create a duplicate of this environment
    pub fn duplicate(&self) -> Self {
        let mut dup = self.clone();
        dup.id = new_id();
        dup.name = format!("{} (Copy)", dup.name);
        dup.is_active = false;
        dup.created_at = now();
        dup.updated_at = now();
        dup
    }
}

impl Temporal for Environment {
    fn created_at(&self) -> Timestamp {
        self.created_at
    }

    fn updated_at(&self) -> Timestamp {
        self.updated_at
    }
}

impl Identifiable for Environment {
    fn id(&self) -> Id {
        self.id
    }
}

/// Environment variable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Variable {
    pub key: String,
    pub value: String,

    /// Initial value (for secrets that get masked)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_value: Option<String>,

    /// Whether the variable is enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Variable type
    #[serde(default)]
    pub variable_type: VariableType,

    /// Description of what this variable is for
    pub description: Option<String>,
}

fn default_enabled() -> bool {
    true
}

impl Variable {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value,
            initial_value: None,
            enabled: true,
            variable_type: VariableType::Normal,
            description: None,
        }
    }

    pub fn secret(key: String, value: String) -> Self {
        Self {
            key,
            value,
            initial_value: Some(value.clone()),
            enabled: true,
            variable_type: VariableType::Secret,
            description: None,
        }
    }

    pub fn disabled(key: String, value: String) -> Self {
        Self {
            key,
            value,
            initial_value: None,
            enabled: false,
            variable_type: VariableType::Normal,
            description: None,
        }
    }

    pub fn with_type(mut self, variable_type: VariableType) -> Self {
        self.variable_type = variable_type;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if this is a secret variable
    pub fn is_secret(&self) -> bool {
        matches!(self.variable_type, VariableType::Secret)
    }

    /// Get the masked value for display (for secrets)
    pub fn display_value(&self) -> String {
        if self.is_secret() && !self.value.is_empty() {
            "••••••••".to_string()
        } else {
            self.value.clone()
        }
    }
}

/// Variable type for categorization and UI handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VariableType {
    /// Normal variable
    Normal,
    /// Secret variable (masked in UI)
    Secret,
    /// System variable (read-only in some contexts)
    System,
    /// Environment-specific variable
    Env,
}

impl Default for VariableType {
    fn default() -> Self {
        VariableType::Normal
    }
}

/// Global state for environments (like Postman globals)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Globals {
    /// Global variables
    #[serde(default)]
    pub values: Vec<Variable>,

    /// Last update timestamp
    pub updated_at: Timestamp,
}

impl Globals {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            updated_at: now(),
        }
    }

    pub fn with_values(mut self, values: Vec<Variable>) -> Self {
        self.values = values;
        self.updated_at = now();
        self
    }

    /// Get a global variable value by key
    pub fn get(&self, key: &str) -> Option<String> {
        self.values
            .iter()
            .find(|v| v.enabled && v.key == key)
            .map(|v| v.value.clone())
    }

    /// Set a global variable
    pub fn set(&mut self, key: String, value: String) {
        if let Some(var) = self.values.iter_mut().find(|v| v.key == key) {
            var.value = value;
        } else {
            self.values.push(Variable::new(key, value));
        }
        self.updated_at = now();
    }

    /// Remove a global variable
    pub fn unset(&mut self, key: &str) -> bool {
        let original_len = self.values.len();
        self.values.retain(|v| v.key != key);
        self.values.len() < original_len
    }

    /// Get all enabled globals as a map
    pub fn to_map(&self) -> HashMap<String, String> {
        self.values
            .iter()
            .filter(|v| v.enabled)
            .map(|v| (v.key.clone(), v.value.clone()))
            .collect()
    }
}

impl Default for Globals {
    fn default() -> Self {
        Self::new()
    }
}

/// Variable resolver for substituting {{variable}} patterns
pub struct VariableResolver {
    environment: HashMap<String, String>,
    globals: HashMap<String, String>,
    /// Additional system variables
    system: HashMap<String, String>,
}

impl VariableResolver {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
            globals: HashMap::new(),
            system: Self::init_system_vars(),
        }
    }

    pub fn with_environment(mut self, vars: HashMap<String, String>) -> Self {
        self.environment = vars;
        self
    }

    pub fn with_globals(mut self, vars: HashMap<String, String>) -> Self {
        self.globals = vars;
        self
    }

    /// Initialize system variables
    fn init_system_vars() -> HashMap<String, String> {
        let mut vars = HashMap::new();

        // Timestamp
        use chrono::Utc;
        vars.insert("$timestamp".to_string(), Utc::now().timestamp().to_string());
        vars.insert("$timestamp_iso".to_string(), Utc::now().to_rfc3339());

        // Random values
        vars.insert("$randomInt".to_string(),
            (rand::random::<u32>() % 10000).to_string());

        // GUID
        vars.insert("$guid".to_string(), Uuid::new_v4().to_string());

        vars
    }

    /// Resolve variables in a string (handles {{variable}} syntax)
    pub fn resolve(&self, input: &str) -> String {
        // Regex to match {{variable_name}} patterns
        let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();

        re.replace_all(input, |caps: &regex::Captures| {
            let key = &caps[1];

            // Priority: environment > globals > system
            self.environment
                .get(key)
                .or_else(|| self.globals.get(key))
                .or_else(|| self.system.get(key))
                .cloned()
                .unwrap_or_else(|| caps[0].to_string())
        }).to_string()
    }

    /// Resolve variables recursively (handles nested variables)
    pub fn resolve_recursive(&self, input: &str, max_depth: usize) -> String {
        let mut result = input.to_string();

        for _ in 0..max_depth {
            let resolved = self.resolve(&result);
            if resolved == result {
                break; // No more changes
            }
            result = resolved;
        }

        result
    }

    /// Update system variables (for dynamic values like timestamp)
    pub fn refresh_system_vars(&mut self) {
        self.system = Self::init_system_vars();
    }
}

impl Default for VariableResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let env = Environment::new("Production".to_string());

        assert_eq!(env.name, "Production");
        assert_eq!(env.values.len(), 0);
        assert!(!env.is_active);
    }

    #[test]
    fn test_environment_variables() {
        let mut env = Environment::new("Dev".to_string());

        env.add_variable("base_url".to_string(), "https://api.dev.com".to_string());
        env.add_variable("api_key".to_string(), "secret123".to_string());

        assert_eq!(env.get("base_url"), Some("https://api.dev.com".to_string()));
        assert_eq!(env.get("api_key"), Some("secret123".to_string()));
        assert_eq!(env.get("nonexistent"), None);
    }

    #[test]
    fn test_environment_set() {
        let mut env = Environment::new("Test".to_string());

        env.set("key1".to_string(), "value1".to_string());
        assert_eq!(env.get("key1"), Some("value1".to_string()));

        env.set("key1".to_string(), "value2".to_string());
        assert_eq!(env.get("key1"), Some("value2".to_string()));
        assert_eq!(env.values.len(), 1);
    }

    #[test]
    fn test_environment_unset() {
        let mut env = Environment::new("Test".to_string());

        env.add_variable("key1".to_string(), "value1".to_string());
        assert!(env.unset("key1"));
        assert!(!env.unset("key1")); // Already removed
        assert_eq!(env.get("key1"), None);
    }

    #[test]
    fn test_variable_types() {
        let normal = Variable::new("key".to_string(), "value".to_string());
        assert!(!normal.is_secret());

        let secret = Variable::secret("api_key".to_string(), "secret".to_string());
        assert!(secret.is_secret());
        assert_eq!(secret.display_value(), "••••••••");
    }

    #[test]
    fn test_variable_resolver() {
        let mut env_vars = HashMap::new();
        env_vars.insert("base_url".to_string(), "https://api.example.com".to_string());

        let mut global_vars = HashMap::new();
        global_vars.insert("version".to_string(), "v1".to_string());

        let resolver = VariableResolver::new()
            .with_environment(env_vars)
            .with_globals(global_vars);

        let url = resolver.resolve("{{base_url}}/users");
        assert_eq!(url, "https://api.example.com/users");

        let versioned = resolver.resolve("{{base_url}}/{{version}}");
        assert_eq!(versioned, "https://api.example.com/v1");
    }

    #[test]
    fn test_variable_resolution_priority() {
        let mut env_vars = HashMap::new();
        env_vars.insert("key".to_string(), "env_value".to_string());

        let mut global_vars = HashMap::new();
        global_vars.insert("key".to_string(), "global_value".to_string());

        let resolver = VariableResolver::new()
            .with_environment(env_vars)
            .with_globals(global_vars);

        // Environment should have priority over globals
        let result = resolver.resolve("{{key}}");
        assert_eq!(result, "env_value");
    }

    #[test]
    fn test_globals() {
        let mut globals = Globals::new();

        globals.set("api_key".to_string(), "global_key".to_string());
        assert_eq!(globals.get("api_key"), Some("global_key".to_string()));

        globals.set("api_key".to_string(), "new_key".to_string());
        assert_eq!(globals.get("api_key"), Some("new_key".to_string()));

        assert!(globals.unset("api_key"));
        assert_eq!(globals.get("api_key"), None);
    }

    #[test]
    fn test_environment_duplicate() {
        let original = Environment::new("Production".to_string())
            .with_values(vec![
                Variable::new("key1".to_string(), "value1".to_string()),
            ])
            .with_active(true);

        let copy = original.duplicate();

        assert_ne!(original.id, copy.id);
        assert_eq!(copy.name, "Production (Copy)");
        assert!(!copy.is_active);
        assert_eq!(copy.values.len(), 1);
    }
}
