use std::fmt::{Debug, Display, Formatter};

use rome_console::{markup, MarkupBuf};
use rome_diagnostics::{
    advice::CodeSuggestionAdvice, category, Advices, Category, Diagnostic, DiagnosticExt,
    DiagnosticTags, Error, FileId, Location, Severity, Visit,
};
use rome_rowan::TextRange;

use crate::rule::RuleDiagnostic;

/// Small wrapper for diagnostics during the analysis phase.
///
/// During these phases, analyzers can create various type diagnostics and some of them
/// don't have all the info to actually create a real [Diagnostic].
///
/// This wrapper serves as glue, which eventually is able to spit out full fledged diagnostics.
///
#[derive(Debug)]
pub struct AnalyzerDiagnostic {
    kind: DiagnosticKind,
    /// Series of code suggestions offered by rule code actions
    code_suggestion_list: Vec<CodeSuggestionAdvice<MarkupBuf>>,
}

#[derive(Debug)]
enum DiagnosticKind {
    /// It holds various info related to diagnostics emitted by the rules
    Rule {
        /// Reference to the file
        file_id: FileId,
        /// The severity of the rule
        severity: Option<Severity>,
        /// The diagnostic emitted by a rule
        rule_diagnostic: RuleDiagnostic,
    },
    /// We have raw information to create a basic [Diagnostic]
    Raw(Error),
}

impl Diagnostic for AnalyzerDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic, ..
            } => Some(rule_diagnostic.category),
            DiagnosticKind::Raw(error) => error.category(),
        }
    }
    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic, ..
            } => Debug::fmt(&rule_diagnostic.message, fmt),
            DiagnosticKind::Raw(error) => error.description(fmt),
        }
    }

    fn message(&self, fmt: &mut rome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic, ..
            } => rome_console::fmt::Display::fmt(&rule_diagnostic.message, fmt),
            DiagnosticKind::Raw(error) => error.message(fmt),
        }
    }

    fn severity(&self) -> Severity {
        match &self.kind {
            DiagnosticKind::Rule { severity, .. } => severity.unwrap_or(Severity::Error),
            DiagnosticKind::Raw(error) => error.severity(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic, ..
            } => rule_diagnostic.tags,
            DiagnosticKind::Raw(error) => error.tags(),
        }
    }

    fn location(&self) -> Option<Location<'_>> {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic,
                file_id,
                ..
            } => {
                let builder = Location::builder()
                    .span(&rule_diagnostic.span)
                    .resource(file_id);
                builder.build()
            }
            DiagnosticKind::Raw(error) => error.location(),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic,
                file_id,
                ..
            } => {
                let rule_advices = rule_diagnostic.advices();
                // we first print the details emitted by the rules
                for detail in &rule_advices.details {
                    visitor.record_log(
                        detail.log_category,
                        &markup! { {detail.message} }.to_owned(),
                    )?;
                    if let Some(location) = Location::builder()
                        .span(&detail.range)
                        .resource(file_id)
                        .build()
                    {
                        visitor.record_frame(location)?;
                    }
                }
                // we then print notes
                for (log_category, note) in &rule_advices.notes {
                    visitor.record_log(*log_category, &markup! { {note} }.to_owned())?;
                }
            }
            DiagnosticKind::Raw(error) => error.advices(visitor)?,
        }

        // finally, we print possible code suggestions on how to fix the issue
        for suggestion in &self.code_suggestion_list {
            suggestion.record(visitor)?;
        }

        Ok(())
    }
}

impl AnalyzerDiagnostic {
    /// Creates a diagnostic from a [RuleDiagnostic]
    pub fn from_rule_diagnostic(file_id: FileId, rule_diagnostic: RuleDiagnostic) -> Self {
        Self {
            kind: DiagnosticKind::Rule {
                file_id,
                rule_diagnostic,
                severity: None,
            },
            code_suggestion_list: vec![],
        }
    }

    /// Creates a diagnostic from a generic [Error]
    pub fn from_error(error: Error) -> Self {
        Self {
            kind: DiagnosticKind::Raw(error),
            code_suggestion_list: vec![],
        }
    }

    /// Sets the severity of the current diagnostic
    pub fn set_severity(&mut self, new_severity: Severity) {
        if let DiagnosticKind::Rule { severity, .. } = &mut self.kind {
            *severity = Some(new_severity);
        }
    }

    pub fn get_span(&self) -> Option<TextRange> {
        match &self.kind {
            DiagnosticKind::Rule {
                rule_diagnostic, ..
            } => rule_diagnostic.span,
            DiagnosticKind::Raw(error) => error.location().and_then(|location| location.span),
        }
    }

    /// It adds a code suggestion, use this API to tell the user that a rule can benefit from
    /// a automatic code fix.
    pub fn add_code_suggestion(mut self, suggestion: CodeSuggestionAdvice<MarkupBuf>) -> Self {
        self.kind = match self.kind {
            DiagnosticKind::Rule {
                file_id,
                severity,
                mut rule_diagnostic,
            } => {
                rule_diagnostic.tags = DiagnosticTags::FIXABLE;
                DiagnosticKind::Rule {
                    file_id,
                    severity,
                    rule_diagnostic,
                }
            }
            DiagnosticKind::Raw(error) => {
                DiagnosticKind::Raw(error.with_tags(DiagnosticTags::FIXABLE))
            }
        };

        self.code_suggestion_list.push(suggestion);
        self
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(severity = Warning)]
pub(crate) struct SuppressionDiagnostic {
    #[category]
    category: &'static Category,
    #[location(span)]
    range: TextRange,
    #[location(resource)]
    file_id: FileId,
    #[message]
    #[description]
    message: String,
    #[tags]
    tags: DiagnosticTags,
}

impl SuppressionDiagnostic {
    pub(crate) fn new(
        file_id: FileId,
        category: &'static Category,
        range: TextRange,
        message: impl Display,
    ) -> Self {
        Self {
            file_id,
            category,
            range,
            message: message.to_string(),
            tags: DiagnosticTags::empty(),
        }
    }

    pub(crate) fn with_tags(mut self, tags: DiagnosticTags) -> Self {
        self.tags |= tags;
        self
    }
}
