use std::marker::PhantomData;

use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion, Diagnostic, SubDiagnostic, SuggestionChange, SuggestionStyle,
};
use rome_js_syntax::{JsAnyRoot, TextRange};
use rome_rowan::AstNode;

use crate::{categories::ActionCategory, registry::Rule};

/// Event raised by the analyzer when a [Rule](crate::registry::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal {
    fn diagnostic(&self) -> Option<Diagnostic>;
    fn action(&self) -> Option<AnalyzerAction>;
}

/// Code Action object returned by the analyzer, generated from a [RuleAction](crate::registry::RuleAction)
/// with additional informations about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyzerAction {
    pub rule_name: &'static str,
    pub file_id: FileId,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    /// Range of the original document being modified by this action
    ///
    /// By default this is conservatively set to cover the entire document
    pub range: TextRange,
    pub root: JsAnyRoot,
}

impl From<AnalyzerAction> for CodeSuggestion {
    fn from(action: AnalyzerAction) -> Self {
        CodeSuggestion {
            substitution: SuggestionChange::String(action.root.to_string()),
            span: FileSpan {
                file: action.file_id,
                range: action.range,
            },
            applicability: action.applicability,
            msg: action.message,
            style: SuggestionStyle::Full,
            labels: Vec::new(),
        }
    }
}

/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::registry::Rule)
pub(crate) struct RuleSignal<'a, R: Rule> {
    file_id: FileId,
    root: &'a JsAnyRoot,
    node: R::Query,
    state: R::State,
    _rule: PhantomData<R>,
}

impl<'a, R: Rule + 'static> RuleSignal<'a, R> {
    pub(crate) fn new_boxed(
        file_id: FileId,
        root: &'a JsAnyRoot,
        node: R::Query,
        state: R::State,
    ) -> Box<dyn AnalyzerSignal + 'a> {
        Box::new(Self {
            file_id,
            root,
            node,
            state,
            _rule: PhantomData,
        })
    }
}

impl<'a, R: Rule> AnalyzerSignal for RuleSignal<'a, R> {
    fn diagnostic(&self) -> Option<Diagnostic> {
        R::diagnostic(&self.node, &self.state).map(|diag| Diagnostic {
            file_id: self.file_id,
            severity: diag.severity,
            code: Some(R::NAME.into()),
            title: diag.message.clone(),
            tag: None,
            primary: Some(SubDiagnostic {
                severity: diag.severity,
                msg: diag.message,
                span: FileSpan {
                    file: self.file_id,
                    range: diag.range,
                },
            }),
            children: Vec::new(),
            suggestions: Vec::new(),
            footers: Vec::new(),
        })
    }

    fn action(&self) -> Option<AnalyzerAction> {
        R::action(self.root.clone(), &self.node, &self.state).map(|action| AnalyzerAction {
            rule_name: R::NAME,
            file_id: self.file_id,
            category: action.category,
            applicability: action.applicability,
            message: action.message,
            range: self.root.syntax().text_range(),
            root: action.root,
        })
    }
}
