use crate::RuleKey;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug, Clone, Deserialize)]
pub struct RuleOptions(Value);

impl RuleOptions {
    /// It returns the string contained in [RawValue], for the relative rule
    pub fn value(&self) -> &Value {
        &self.0
    }

    /// Creates a new [RuleOptions]
    pub fn new(options: Value) -> Self {
        Self(options)
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Clone, Default)]
pub struct AnalyzerRules(HashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: Value) {
        self.0.insert(rule_key, RuleOptions::new(options));
    }

    /// It retrieves the options of a stored rule, given its name
    pub fn get_rule(&self, rule_key: &RuleKey) -> Option<&RuleOptions> {
        self.0.get(rule_key)
    }
}

/// A data structured derived from the `rome.json` file
#[derive(Debug, Clone, Default)]
pub struct AnalyzerConfiguration {
    /// A list of rules and their options
    pub rules: AnalyzerRules,

    /// A collections of bindings that the analyzers should consider as "external".
    ///
    /// For example, lint rules should ignore them.
    pub globals: Vec<String>,
}

/// A set of information useful to the analyzer infrastructure
#[derive(Debug, Clone, Default)]
pub struct AnalyzerOptions {
    /// A data structured derived from the [`rome.json`] file
    pub configuration: AnalyzerConfiguration,
}
