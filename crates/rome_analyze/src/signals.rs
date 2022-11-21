use std::marker::PhantomData;

use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{RuleLanguage, RuleRoot},
    rule::Rule,
    AnalyzerDiagnostic, AnalyzerOptions, Queryable, RuleGroup, ServiceBag,
};
use rome_console::MarkupBuf;
use rome_diagnostics::{
    advice::CodeSuggestionAdvice, location::FileId, Applicability, CodeSuggestion, Diagnostic,
    Error, FileSpan,
};
use rome_rowan::{BatchMutation, Language};

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
    fn action(&self) -> Option<AnalyzerAction<L>>;
}

/// Simple implementation of [AnalyzerSignal] generating a [AnalyzerDiagnostic]
/// from a provided factory function. Optionally, this signal can be configured
/// to also emit a code action, by calling `.with_action` with a secondary
/// factory function for said action.
pub(crate) struct DiagnosticSignal<D, A, L, T> {
    diagnostic: D,
    action: A,
    _diag: PhantomData<(L, T)>,
}

impl<L: Language, D, T> DiagnosticSignal<D, fn() -> Option<AnalyzerAction<L>>, L, T>
where
    D: Fn() -> T,
    T: Diagnostic + Send + Sync + 'static,
{
    pub(crate) fn new(factory: D) -> Self {
        Self {
            diagnostic: factory,
            action: || None,
            _diag: PhantomData,
        }
    }
}

impl<L: Language, D, A, T> DiagnosticSignal<D, A, L, T> {
    pub(crate) fn with_action<B>(self, factory: B) -> DiagnosticSignal<D, B, L, T>
    where
        B: Fn() -> Option<AnalyzerAction<L>>,
    {
        DiagnosticSignal {
            diagnostic: self.diagnostic,
            action: factory,
            _diag: PhantomData,
        }
    }
}

impl<L: Language, D, A, T> AnalyzerSignal<L> for DiagnosticSignal<D, A, L, T>
where
    D: Fn() -> T,
    T: Diagnostic + Send + Sync + 'static,
    A: Fn() -> Option<AnalyzerAction<L>>,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let diag = (self.diagnostic)();
        let error = Error::from(diag);
        Some(AnalyzerDiagnostic::from_error(error))
    }

    fn action(&self) -> Option<AnalyzerAction<L>> {
        (self.action)()
    }
}

/// Code Action object returned by the analyzer, generated from a [crate::RuleAction]
/// with additional information about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug)]
pub struct AnalyzerAction<L: Language> {
    pub rule_name: Option<(&'static str, &'static str)>,
    pub file_id: FileId,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
}

impl<L> From<AnalyzerAction<L>> for CodeSuggestionAdvice<MarkupBuf>
where
    L: Language,
{
    fn from(action: AnalyzerAction<L>) -> Self {
        let (_, suggestion) = action.mutation.as_text_edits().unwrap_or_default();

        CodeSuggestionAdvice {
            applicability: action.applicability,
            msg: action.message,
            suggestion,
        }
    }
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
            labels: vec![],
        }
    }
}

/// Analyzer-internal implementation of [AnalyzerSignal] for a specific [Rule](crate::registry::Rule)
pub(crate) struct RuleSignal<'phase, R: Rule> {
    file_id: FileId,
    root: &'phase RuleRoot<R>,
    query_result: <<R as Rule>::Query as Queryable>::Output,
    state: R::State,
    services: &'phase ServiceBag,
    options: AnalyzerOptions,
}

impl<'phase, R> RuleSignal<'phase, R>
where
    R: Rule + 'static,
{
    pub(crate) fn new(
        file_id: FileId,
        root: &'phase RuleRoot<R>,
        query_result: <<R as Rule>::Query as Queryable>::Output,
        state: R::State,
        services: &'phase ServiceBag,
        options: AnalyzerOptions,
    ) -> Self {
        Self {
            file_id,
            root,
            query_result,
            state,
            services,
            options,
        }
    }
}

impl<'bag, R> AnalyzerSignal<RuleLanguage<R>> for RuleSignal<'bag, R>
where
    R: Rule,
{
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic> {
        let ctx =
            RuleContext::new(&self.query_result, self.root, self.services, &self.options).ok()?;

        R::diagnostic(&ctx, &self.state).map(|diag| diag.into_analyzer_diagnostic(self.file_id))
    }

    fn action(&self) -> Option<AnalyzerAction<RuleLanguage<R>>> {
        let ctx =
            RuleContext::new(&self.query_result, self.root, self.services, &self.options).ok()?;

        R::action(&ctx, &self.state).map(|action| AnalyzerAction {
            rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
            file_id: self.file_id,
            category: action.category,
            applicability: action.applicability,
            message: action.message,
            mutation: action.mutation,
        })
    }
}
