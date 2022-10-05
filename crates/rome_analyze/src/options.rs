use crate::RuleKey;
use serde_json::value::RawValue;
use std::collections::HashMap;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug, Clone)]
pub struct RuleOptions(Box<RawValue>);

impl RuleOptions {
    /// It returns the [RawValue] for the relative rule
    pub fn value(&self) -> &RawValue {
        &self.0
    }

    /// Creates a new [RuleOptions]
    pub fn new(options: Box<RawValue>) -> Self {
        Self(options)
    }
}

/// A convenient new type data structure to insert and get rules
#[derive(Debug, Clone, Default)]
pub struct AnalyzerRules(HashMap<RuleKey, RuleOptions>);

impl AnalyzerRules {
    /// It tracks the options of a specific rule
    pub fn push_rule(&mut self, rule_key: RuleKey, options: Box<RawValue>) {
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
    configuration: AnalyzerConfiguration,
}

impl AnalyzerOptions {
    /// It retrieves the options that belong to a rule, it they exists.
    ///
    /// In order to retrieve a typed data structure, the function has to accept a `FromType`, a
    /// `ToType` (this one, inferrable by the compiler) and a closure that does the mapping.
    ///
    /// Usually, options are a `serde::RawValue` and need to be mapped to a sized type.
    ///
    /// ## Examples
    ///
    /// ```rust,ignore
    /// use rome_analyze::{declare_rule, Rule, RuleCategory, RuleMeta, RuleMetadata};
    /// use rome_analyze::context::RuleContext;
    /// declare_rule! {    
    ///     /// Some doc
    ///     pub(crate) Name {
    ///         version: "0.0.0",
    ///         name: "name",
    ///         recommended: true,
    ///     }
    /// }
    ///
    /// impl Rule for Name {
    ///     const CATEGORY: RuleCategory = RuleCategory::Lint;
    ///     type Query = ();
    ///     type State = ();
    ///     type Signals = ();
    ///
    ///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    ///         let options = ctx.options();
    ///     }
    /// }
    /// ```
    pub fn rule_options<F: FnOnce(&RuleOptions) -> ToType, ToType>(
        &self,
        rule_key: &RuleKey,
        mapper: F,
    ) -> Option<ToType> {
        self.configuration.rules.get_rule(rule_key).map(mapper)
    }
}
