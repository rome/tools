use std::marker::PhantomData;

use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion, Diagnostic, SuggestionChange, SuggestionStyle,
};
use rome_rowan::{BatchMutation, Language};
use rome_text_edit::Indel;

use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{LanguageRoot, RuleLanguage, RuleRoot},
    rule::Rule,
    Queryable, RuleGroup, ServiceBag,
};

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<Diagnostic>;
    fn action(&self) -> Option<AnalyzerAction<L>>;
}

/// Simple implementation of [AnalyzerSignal] generating a diagnostic from a
/// provided factory function
pub(crate) struct DiagnosticSignal<F> {
    factory: F,
}

impl<F> DiagnosticSignal<F>
where
    F: Fn() -> Diagnostic,
{
    pub(crate) fn new(factory: F) -> Self {
        Self { factory }
    }
}

impl<L: Language, F> AnalyzerSignal<L> for DiagnosticSignal<F>
where
    F: Fn() -> Diagnostic,
{
    fn diagnostic(&self) -> Option<Diagnostic> {
        Some((self.factory)())
    }

    fn action(&self) -> Option<AnalyzerAction<L>> {
        None
    }
}

/// Code Action object returned by the analyzer, generated from a [crate::RuleAction]
/// with additional information about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug)]
pub struct AnalyzerAction<L: Language> {
    pub group_name: &'static str,
    pub rule_name: &'static str,
    pub file_id: FileId,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L, LanguageRoot<L>>,
}

impl<L> AnalyzerAction<L>
where
    L: Language,
{
    /// Generates a list of [Indel] from the mutation applied by this action
    pub fn as_indels(&self) -> Vec<Indel> {
        let mut result: Vec<_> = self
            .mutation
            .as_text_edits()
            .map(|(delete, insert)| Indel { insert, delete })
            .collect();

        result.sort_unstable_by(|a, b| a.delete.ordering(b.delete));

        result
    }
}

impl<L> From<AnalyzerAction<L>> for CodeSuggestion
where
    L: Language,
{
    fn from(action: AnalyzerAction<L>) -> Self {
        let indels = action.as_indels();

        let range = indels.iter().fold(None, |state, indel| match state {
            None => Some(indel.delete),
            Some(state) => Some(state.cover(indel.delete)),
        });

        CodeSuggestion {
            substitution: SuggestionChange::Indels(indels),
            span: FileSpan {
                file: action.file_id,
                range: range.unwrap_or_default(),
            },
            applicability: action.applicability,
            msg: action.message,
            style: SuggestionStyle::Full,
            labels: Vec::new(),
        }
    }
}

/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::registry::Rule)
pub(crate) struct RuleSignal<'phase, G, R: Rule> {
    file_id: FileId,
    root: &'phase RuleRoot<R>,
    query_result: <<R as Rule>::Query as Queryable>::Output,
    state: R::State,
    services: &'phase ServiceBag,
    _rule: PhantomData<(G, R)>,
}

impl<'phase, G, R> RuleSignal<'phase, G, R>
where
    R: Rule + 'static,
{
    pub(crate) fn new(
        file_id: FileId,
        root: &'phase RuleRoot<R>,
        query_result: <<R as Rule>::Query as Queryable>::Output,
        state: R::State,
        services: &'phase ServiceBag,
    ) -> Self {
        Self {
            file_id,
            root,
            query_result,
            state,
            _rule: PhantomData,
            services,
        }
    }
}

impl<'bag, G, R> AnalyzerSignal<RuleLanguage<R>> for RuleSignal<'bag, G, R>
where
    G: RuleGroup,
    R: Rule,
{
    fn diagnostic(&self) -> Option<Diagnostic> {
        let ctx = RuleContext::new(&self.query_result, self.root, self.services).ok()?;

        R::diagnostic(&ctx, &self.state).map(|diag| {
            diag.into_diagnostic(
                self.file_id,
                format!("{}/{}", G::NAME, R::NAME),
                format!("https://rome.tools/docs/lint/rules/{}/", R::NAME),
            )
        })
    }

    fn action(&self) -> Option<AnalyzerAction<RuleLanguage<R>>> {
        let ctx = RuleContext::new(&self.query_result, self.root, self.services).ok()?;

        R::action(&ctx, &self.state).map(|action| AnalyzerAction {
            group_name: G::NAME,
            rule_name: R::NAME,
            file_id: self.file_id,
            category: action.category,
            applicability: action.applicability,
            message: action.message,
            mutation: action.mutation,
        })
    }
}
