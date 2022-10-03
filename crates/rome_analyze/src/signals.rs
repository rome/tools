use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{RuleLanguage, RuleRoot},
    rule::Rule,
    AnalyzerDiagnostic, Queryable, RuleGroup, ServiceBag,
};
use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion,
};
use rome_rowan::{BatchMutation, Language};
use std::marker::PhantomData;

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
    fn action(&self) -> Option<AnalyzerAction<L>>;
}

/// Simple implementation of [AnalyzerSignal] generating a [AnalyzerDiagnostic] from a
/// provided factory function
pub(crate) struct DiagnosticSignal<F> {
    factory: F,
}

impl<F> DiagnosticSignal<F>
where
    F: Fn() -> AnalyzerDiagnostic,
{
    pub(crate) fn new(factory: F) -> Self {
        Self { factory }
    }
}

impl<L: Language, F> AnalyzerSignal<L> for DiagnosticSignal<F>
where
    F: Fn() -> AnalyzerDiagnostic,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
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
    pub mutation: BatchMutation<L>,
}

impl<L> From<AnalyzerAction<L>> for CodeSuggestion
where
    L: Language,
{
    fn from(action: AnalyzerAction<L>) -> Self {
        let (range, suggestion) = action.mutation.as_text_edits().unwrap_or_default();

        CodeSuggestion {
            span: FileSpan {
                file: action.file_id,
                range,
            },
            applicability: action.applicability,
            msg: action.message,
            suggestion,
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
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let ctx = RuleContext::new(&self.query_result, self.root, self.services).ok()?;
        R::diagnostic(&ctx, &self.state).map(|diag| diag.into_analyzer_diagnostic(self.file_id))
    }

    fn action(&self) -> Option<AnalyzerAction<RuleLanguage<R>>> {
        let ctx = RuleContext::new(&self.query_result, self.root, self.services).ok()?;

        R::action(&ctx, &self.state).map(|action| AnalyzerAction {
            group_name: G::NAME,
            rule_name: R::METADATA.name,
            file_id: self.file_id,
            category: action.category,
            applicability: action.applicability,
            message: action.message,
            mutation: action.mutation,
        })
    }
}
