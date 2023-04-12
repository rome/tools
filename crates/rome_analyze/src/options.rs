use crate::RuleKey;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug, Default, Deserialize)]
pub struct RuleOptions(String);

impl RuleOptions {
    /// It returns the deserialized rule option
    pub fn value(&self) -> &String {
        &self.0
    }

    /// Creates a new [RuleOptions]
    pub fn new(options: String) -> Self {
        Self(options)
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Default)]
pub struct AnalyzerRules(HashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: String) {
        self.0.insert(rule_key, RuleOptions::new(options));
    }

    /// It retrieves the options of a stored rule, given its name
    pub fn get_rule(&self, rule_key: &RuleKey) -> Option<&RuleOptions> {
        self.0.get(rule_key)
    }
}

/// A data structured derived from the `rome.json` file
#[derive(Debug, Default)]
pub struct AnalyzerConfiguration {
    /// A list of rules and their options
    pub rules: AnalyzerRules,

    /// A collections of bindings that the analyzers should consider as "external".
    ///
    /// For example, lint rules should ignore them.
    pub globals: Vec<String>,
}

/// A set of information useful to the analyzer infrastructure
#[derive(Debug, Default)]
pub struct AnalyzerOptions {
    /// A data structured derived from the [`rome.json`] file
    pub configuration: AnalyzerConfiguration,

    /// The file that is being analyzed
    pub file_path: PathBuf,
}
