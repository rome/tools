use std::{collections::VecDeque, marker::PhantomData};

use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion, Diagnostic, SuggestionChange, SuggestionStyle,
};
use rome_rowan::{
    AstNode, Direction, Language, SyntaxElement, SyntaxNode, SyntaxSlot, TextRange, TextSize,
    WalkEvent,
};

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
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyzerAction<L: Language> {
    pub group_name: &'static str,
    pub rule_name: &'static str,
    pub file_id: FileId,
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    /// Range of the original document being modified by this action
    pub original_range: TextRange,
    /// Range of the new document that differs from the original document
    pub new_range: TextRange,
    pub root: LanguageRoot<L>,
}

impl<L> From<AnalyzerAction<L>> for CodeSuggestion
where
    L: Language,
{
    fn from(action: AnalyzerAction<L>) -> Self {
        // Only print the relevant subset of tokens
        let mut code = String::new();

        let mut iter = action.root.syntax().preorder_with_tokens(Direction::Next);
        while let Some(event) = iter.next() {
            let elem = match event {
                WalkEvent::Enter(elem) => elem,
                WalkEvent::Leave(_) => continue,
            };

            let range = elem.text_range();
            let has_intersection = range
                .intersect(action.new_range)
                .filter(|range| !range.is_empty())
                .is_some();

            match elem {
                SyntaxElement::Node(_) => {
                    if !has_intersection {
                        iter.skip_subtree();
                    }
                }
                SyntaxElement::Token(token) => {
                    if has_intersection {
                        code.push_str(token.text());
                    }
                }
            }
        }

        CodeSuggestion {
            substitution: SuggestionChange::String(code),
            span: FileSpan {
                file: action.file_id,
                range: action.original_range,
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

        R::action(&ctx, &self.state).and_then(|action| {
            let (original_range, new_range) =
                find_diff_range(self.root.syntax(), action.root.syntax())?;
            Some(AnalyzerAction {
                group_name: G::NAME,
                rule_name: R::NAME,
                file_id: self.file_id,
                category: action.category,
                applicability: action.applicability,
                message: action.message,
                original_range,
                new_range,
                root: action.root,
            })
        })
    }
}

/// Compares two revisions of the same syntax tree and find the narrowest text
/// range that differs between the two
fn find_diff_range<L>(prev: &SyntaxNode<L>, next: &SyntaxNode<L>) -> Option<(TextRange, TextRange)>
where
    L: Language,
{
    let mut result: Option<(TextRange, TextRange)> = None;
    let mut queue = VecDeque::new();

    if prev.key().0 != next.key().0 {
        queue.push_back((prev.clone(), next.clone()));
    }

    // These buffers are kept between loops to amortize their allocation over the whole algorithm
    let mut prev_children = Vec::new();
    let mut next_children = Vec::new();

    while let Some((prev, next)) = queue.pop_front() {
        // Collect all `SyntaxElement` slots into two vectors
        // (makes it easier to implement backwards iteration + peeking)
        prev_children.clear();
        prev_children.extend(prev.slots().filter_map(SyntaxSlot::into_syntax_element));

        next_children.clear();
        next_children.extend(next.slots().filter_map(SyntaxSlot::into_syntax_element));

        // Remove identical children from the end of the vectors, keeping track
        // of the truncated range end for the sub-slice of children that differ
        let mut prev_end = prev.text_range().end();
        let mut next_end = next.text_range().end();

        while let (Some(prev), Some(next)) = (prev_children.last(), next_children.last()) {
            if prev.key().0 == next.key().0 {
                prev_end = prev_end.min(prev.text_range().start());
                next_end = next_end.min(next.text_range().start());
                prev_children.pop().unwrap();
                next_children.pop().unwrap();
            } else {
                break;
            }
        }

        // Zip the two vectors from the start and compare the previous and next version of each child
        let mut prev_children = prev_children.drain(..);
        let mut next_children = next_children.drain(..);

        loop {
            let (prev_range, next_range) = match (prev_children.next(), next_children.next()) {
                // The previous and next child are both nodes, push them to the
                // comparison queue if they differ
                (Some(SyntaxElement::Node(prev)), Some(SyntaxElement::Node(next))) => {
                    if prev.key().0 != next.key().0 {
                        queue.push_back((prev, next));
                    }

                    continue;
                }

                // The previous and next child are both tokens, extend the diff
                // range to their position if they differ
                (Some(SyntaxElement::Token(prev)), Some(SyntaxElement::Token(next))) => {
                    if prev.key().0 == next.key().0 {
                        continue;
                    }

                    (prev.text_range(), next.text_range())
                }

                // `(Some(Token), Some(Node))` or `(Some(Node), Some(Token))`
                (Some(prev), Some(next)) => (prev.text_range(), next.text_range()),

                // One children list is longer than the other
                (Some(prev), None) => (
                    prev.text_range(),
                    TextRange::at(next_end, TextSize::from(0)),
                ),
                (None, Some(next)) => (
                    TextRange::at(prev_end, TextSize::from(0)),
                    next.text_range(),
                ),

                (None, None) => break,
            };

            // Either extend the existing range or initialize it with the new values
            let new_result = match result {
                Some((prev, next)) => (prev.cover(prev_range), next.cover(next_range)),
                None => (prev_range, next_range),
            };

            result = Some(new_result);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use rome_js_factory::make;
    use rome_js_syntax::{JsAnyExpression, JsAnyStatement, TextRange, TextSize, T};
    use rome_rowan::{AstNode, AstNodeListExt};

    use super::find_diff_range;

    #[test]
    /// Checks the [find_diff_range] function returns the correct result when
    /// tokens are reused from the input in the middle of the range
    fn diff_range_split() {
        let before = make::js_if_statement(
            make::token(T![if]),
            make::token(T!['(']),
            JsAnyExpression::JsIdentifierExpression(make::js_identifier_expression(
                make::js_reference_identifier(make::ident("test")),
            )),
            make::token(T![')']),
            JsAnyStatement::JsExpressionStatement(
                make::js_expression_statement(JsAnyExpression::JsIdentifierExpression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "consequent",
                    ))),
                ))
                .with_semicolon_token(make::token(T![;]))
                .build(),
            ),
        )
        .with_else_clause(make::js_else_clause(
            make::token(T![else]),
            JsAnyStatement::JsExpressionStatement(
                make::js_expression_statement(JsAnyExpression::JsIdentifierExpression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "alternate",
                    ))),
                ))
                .with_semicolon_token(make::token(T![;]))
                .build(),
            ),
        ))
        .build();

        let consequent = before.consequent().unwrap();
        let else_clause = before.else_clause().unwrap();
        let alternate = else_clause.alternate().unwrap();

        let after = before
            .clone()
            .with_consequent(alternate)
            .with_else_clause(Some(else_clause.with_alternate(consequent)));

        let diff = find_diff_range(before.syntax(), after.syntax())
            .expect("failed to calculate diff range");

        let start = TextSize::of("if(test)");
        let end = TextSize::of("if(test)consequent;elsealternate;");

        assert_eq!(
            diff,
            (TextRange::new(start, end), TextRange::new(start, end))
        );
    }

    #[test]
    /// Checks the [find_diff_range] function returns the correct result when
    /// tokens are removed from the input
    fn diff_range_remove() {
        let before = make::js_statement_list(vec![
            JsAnyStatement::JsExpressionStatement(
                make::js_expression_statement(JsAnyExpression::JsIdentifierExpression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "statement1",
                    ))),
                ))
                .with_semicolon_token(make::token(T![;]))
                .build(),
            ),
            JsAnyStatement::JsExpressionStatement(
                make::js_expression_statement(JsAnyExpression::JsIdentifierExpression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "statement2",
                    ))),
                ))
                .with_semicolon_token(make::token(T![;]))
                .build(),
            ),
            JsAnyStatement::JsExpressionStatement(
                make::js_expression_statement(JsAnyExpression::JsIdentifierExpression(
                    make::js_identifier_expression(make::js_reference_identifier(make::ident(
                        "statement3",
                    ))),
                ))
                .with_semicolon_token(make::token(T![;]))
                .build(),
            ),
        ]);

        let after = before.clone().splice(1..=1, None);

        let diff = find_diff_range(before.syntax(), after.syntax())
            .expect("failed to calculate diff range");

        let start = TextSize::of("statement1;");
        let end = TextSize::of("statement1;statement2;");

        assert_eq!(
            diff,
            (TextRange::new(start, end), TextRange::new(start, start))
        );
    }
}
