use crate::AnalyzerSignal;
use crate::{RuleKey, TextRange, TextSize};
use rome_diagnostics::{Diagnostic, LineIndexBuf, Resource, SourceCode};
use serde::Deserialize;
use serde_json::Error;
use serde_json::Value;
use std::collections::HashMap;

/// A convenient new type data structure to store the options that belong to a rule
#[derive(Debug, Clone, Deserialize)]
pub struct RuleOptions(Value);

impl RuleOptions {
    /// It returns the deserialized rule option
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

#[derive(Debug, Clone, Diagnostic)]
#[diagnostic(category = "lint/configuration")]
pub struct OptionsDeserializationDiagnostic {
    #[message]
    message: String,
    #[description]
    description: String,
    #[location(resource)]
    path: Resource<&'static str>,
    #[location(span)]
    span: Option<TextRange>,
    #[location(source_code)]
    source_code: Option<SourceCode<String, LineIndexBuf>>,
}

impl OptionsDeserializationDiagnostic {
    pub fn new(rule_name: &str, input: String, error: Error) -> Self {
        let line_starts = LineIndexBuf::from_source_text(&input);

        let line_index = error.line().checked_sub(1);
        let span = line_index.and_then(|line_index| {
            let line_start = line_starts.get(line_index)?;

            let column_index = error.column().checked_sub(1)?;
            let column_offset = TextSize::try_from(column_index).ok()?;

            let span_start = line_start + column_offset;
            Some(TextRange::at(span_start, TextSize::from(0)))
        });

        let message = format!(
            "Errors while reading options for rule {rule_name}: \n {}",
            error
        );

        Self {
            message: message.clone(),
            description: message,
            path: Resource::Memory,
            span,
            source_code: Some(SourceCode {
                text: input,
                line_starts: Some(line_starts),
            }),
        }
    }
}

impl<L: Language> AnalyzerSignal<L> for OptionsDeserializationDiagnostic {
    fn diagnostic(&self) -> Option<crate::AnalyzerDiagnostic> {
        let err = rome_diagnostics::v2::Error::from(self.clone());
        Some(crate::AnalyzerDiagnostic::Raw(err))
    }

    fn action(&self) -> Option<crate::AnalyzerAction<L>> {
        None
    }
}