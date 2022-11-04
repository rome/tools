use crate::categories::SUPPRESSION_ACTION_CATEGORY;
use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{RuleLanguage, RuleRoot},
    rule::Rule,
    AnalyzerDiagnostic, AnalyzerOptions, Queryable, RuleGroup, ServiceBag,
};
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::{
    advice::CodeSuggestionAdvice, location::FileId, Applicability, CodeSuggestion, Diagnostic,
    Error, FileSpan,
};
use rome_rowan::{AstNode, BatchMutation, BatchMutationExt, Language, TriviaPieceKind};
use std::borrow::Cow;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::vec::IntoIter;

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<AnalyzerDiagnostic>;
    fn actions(&self) -> Option<AnalyzerActionIter<L>>;
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

    fn actions(&self) -> Option<AnalyzerActionIter<L>> {
        (self.action)()
    }
}

/// Code Action object returned by the analyzer, generated from a [crate::RuleAction]
/// with additional information about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug, Clone)]
pub struct AnalyzerAction<L: Language> {
    pub rule_name: Option<(&'static str, &'static str)>,
    pub file_id: FileId,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
}

impl<L: Language> AnalyzerAction<L> {
    pub fn is_suppression(&self) -> bool {
        self.category.matches("quickfix.rome.suppressRule")
    }
}

pub struct AnalyzerActionIter<L: Language> {
    analyzer_actions: IntoIter<AnalyzerAction<L>>,
}

impl<L: Language> From<AnalyzerAction<L>> for CodeSuggestionAdvice<MarkupBuf> {
    fn from(action: AnalyzerAction<L>) -> Self {
        let (_, suggestion) = action.mutation.as_text_edits().unwrap_or_default();
        CodeSuggestionAdvice {
            applicability: action.applicability,
            msg: action.message,
            suggestion,
        }
    }
}

impl<'a, L: Language> From<AnalyzerAction<L>> for CodeSuggestionItem<'a> {
    fn from(action: AnalyzerAction<L>) -> Self {
        let (range, suggestion) = action.mutation.as_text_edits().unwrap_or_default();

        CodeSuggestionItem {
            rule_name: action.rule_name,
            category: action.category,
            group_name: action.group_name,
            suggestion: CodeSuggestion {
                span: FileSpan {
                    file: action.file_id,
                    range,
                },
                applicability: action.applicability,
                msg: action.message,
                suggestion,
                labels: vec![],
            },
        }
    }
}

impl<L: Language> AnalyzerActionIter<L> {
    pub fn new(actions: Vec<AnalyzerAction<L>>) -> Self {
        Self {
            analyzer_actions: actions.into_iter(),
        }
    }
}

impl<L: Language> Iterator for AnalyzerActionIter<L> {
    type Item = AnalyzerAction<L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.analyzer_actions.next()
    }
}

impl<L: Language> FusedIterator for AnalyzerActionIter<L> {}

impl<L: Language> ExactSizeIterator for AnalyzerActionIter<L> {
    fn len(&self) -> usize {
        self.analyzer_actions.len()
    }
}

#[derive(Debug)]
pub struct AnalyzerMutation<L: Language> {
    pub message: MarkupBuf,
    pub mutation: BatchMutation<L>,
    pub category: ActionCategory,
    pub rule_name: String,
}

pub struct CodeSuggestionAdviceIter<L: Language> {
    iter: IntoIter<AnalyzerAction<L>>,
}

impl<L: Language> Iterator for CodeSuggestionAdviceIter<L> {
    type Item = CodeSuggestionAdvice<MarkupBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.iter.next()?;
        Some(action.into())
    }
}

impl<L: Language> FusedIterator for CodeSuggestionAdviceIter<L> {}

impl<L: Language> ExactSizeIterator for CodeSuggestionAdviceIter<L> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

pub struct CodeActionIter<L: Language> {
    iter: IntoIter<AnalyzerAction<L>>,
}

pub struct CodeSuggestionItem<'a> {
    pub category: ActionCategory,
    pub suggestion: CodeSuggestion,
    pub rule_name: &'a str,
    pub group_name: &'a str,
}

impl<L: Language> Iterator for CodeActionIter<L> {
    type Item = CodeSuggestionItem<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let action = self.iter.next()?;
        Some(action.into())
    }
}

impl<L: Language> FusedIterator for CodeActionIter<L> {}

impl<L: Language> ExactSizeIterator for CodeActionIter<L> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<L: Language> AnalyzerActionIter<L> {
    /// Returns an iterator that yields [CodeSuggestionAdvice]
    pub fn into_code_suggestion_advices(self) -> CodeSuggestionAdviceIter<L> {
        CodeSuggestionAdviceIter {
            iter: self.analyzer_actions,
        }
    }

    /// Returns an iterator that yields [CodeAction]
    pub fn into_code_action_iter(self) -> CodeActionIter<L> {
        CodeActionIter {
            iter: self.analyzer_actions,
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

    fn actions(&self) -> Option<AnalyzerActionIter<RuleLanguage<R>>> {
        let ctx =
            RuleContext::new(&self.query_result, self.root, self.services, &self.options).ok()?;
        let mut actions = Vec::new();
        if let Some(action) = R::action(&ctx, &self.state) {
            actions.push(AnalyzerAction {
                rule_name: Some((<R::Group as RuleGroup>::NAME, R::METADATA.name)),
                file_id: self.file_id,
                category: action.category,
                applicability: action.applicability,
                mutation: action.mutation,
                message: action.message,
            });
        };
        let node_to_suppress = R::can_suppress(&ctx, &self.state);
        let suppression_node = node_to_suppress.and_then(|suppression_node| {
            let ancestor = suppression_node.node().ancestors().find_map(|node| {
                if node
                    .first_token()
                    .map(|token| {
                        token
                            .leading_trivia()
                            .pieces()
                            .any(|trivia| trivia.is_newline())
                    })
                    .unwrap_or(false)
                {
                    Some(node)
                } else {
                    None
                }
            });
            if ancestor.is_some() {
                ancestor
            } else {
                Some(ctx.root().syntax().clone())
            }
        });
        let suppression_action = suppression_node.and_then(|suppression_node| {
            let first_token = suppression_node.first_token();
            let rule = format!(
                "lint({}/{})",
                <R::Group as RuleGroup>::NAME,
                R::METADATA.name
            );
            let mes = format!("// rome-ignore {}: suppressed", rule);

            first_token.map(|first_token| {
                let trivia = vec![
                    (TriviaPieceKind::Newline, "\n"),
                    (TriviaPieceKind::SingleLineComment, mes.as_str()),
                    (TriviaPieceKind::Newline, "\n"),
                ];
                let mut mutation = ctx.root().begin();
                let new_token = first_token.with_leading_trivia(trivia.clone());

                mutation.replace_token_discard_trivia(first_token, new_token);
                AnalyzerAction {
                    group_name: <R::Group as RuleGroup>::NAME,
                    rule_name: R::METADATA.name,
                    file_id: self.file_id,
                    category: ActionCategory::Other(Cow::Borrowed(SUPPRESSION_ACTION_CATEGORY)),
                    applicability: Applicability::Always,
                    mutation,
                    message: markup! { "Suppress rule " {rule} }.to_owned(),
                }
            })
        });
        if let Some(suppression_action) = suppression_action {
            actions.push(suppression_action);
        }
        Some(AnalyzerActionIter::new(actions))
    }
}
