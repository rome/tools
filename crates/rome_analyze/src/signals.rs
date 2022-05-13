use std::marker::PhantomData;

use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion, Diagnostic, SubDiagnostic, SuggestionChange, SuggestionStyle,
};
use rome_js_syntax::{JsAnyRoot, TextRange};
use rome_rowan::{AstNode, Language, SyntaxNode};

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
    pub prev_range: TextRange,
    /// Range of the new document that differs from the original document
    pub next_range: TextRange,
    pub root: JsAnyRoot,
}

impl From<AnalyzerAction> for CodeSuggestion {
    fn from(action: AnalyzerAction) -> Self {
        // Only print the relevant subset of tokens
        let mut code = String::new();

        for token in action.root.syntax().descendants_tokens() {
            let range = token.text_range();
            if range.end() <= action.next_range.start() || range.start() >= action.next_range.end()
            {
                continue;
            }

            code.push_str(token.text());
        }

        CodeSuggestion {
            substitution: SuggestionChange::String(code),
            span: FileSpan {
                file: action.file_id,
                range: action.prev_range,
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
        R::action(self.root.clone(), &self.node, &self.state).and_then(|action| {
            let (prev_range, next_range) =
                find_diff_range(self.root.syntax(), action.root.syntax())?;
            Some(AnalyzerAction {
                rule_name: R::NAME,
                file_id: self.file_id,
                category: action.category,
                applicability: action.applicability,
                message: action.message,
                prev_range,
                next_range,
                root: action.root,
            })
        })
    }
}

/// Compares the tokens that make up the two trees and find the narrowest text
/// range that differs between the two
fn find_diff_range<L>(prev: &SyntaxNode<L>, next: &SyntaxNode<L>) -> Option<(TextRange, TextRange)>
where
    L: Language,
{
    let prev_tokens = prev.descendants_tokens();
    let next_tokens = next.descendants_tokens();

    let mut range_start = None;
    let mut range_end = None;

    for (prev_token, next_token) in prev_tokens.zip(next_tokens) {
        if range_start.is_none() {
            debug_assert_eq!(
                prev_token.text_range().start(),
                next_token.text_range().start(),
            );

            if prev_token != next_token {
                range_start = Some(prev_token.text_range().start());
                continue;
            }
        } else if prev_token == next_token {
            range_end = Some((
                prev_token.text_range().start(),
                next_token.text_range().start(),
            ));
            break;
        }
    }

    match (range_start, range_end) {
        (Some(start), Some((prev_end, next_end))) => Some((
            TextRange::new(start, prev_end),
            TextRange::new(start, next_end),
        )),
        (Some(start), None) => Some((
            TextRange::new(start, prev.text_range().end()),
            TextRange::new(start, next.text_range().end()),
        )),

        (None, None) => None,
        (None, Some(_)) => unreachable!(),
    }
}
