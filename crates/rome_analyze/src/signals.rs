use std::marker::PhantomData;

use rome_console::MarkupBuf;
use rome_diagnostics::{
    file::{FileId, FileSpan},
    Applicability, CodeSuggestion, Diagnostic, SuggestionChange, SuggestionStyle,
};
use rome_rowan::{AstNode, Direction, Language, SyntaxNode, TextRange};

use crate::{
    categories::ActionCategory,
    context::RuleContext,
    registry::{LanguageRoot, RuleLanguage, RuleRoot},
    rule::Rule,
    services::ServiceBag,
    Queryable,
};

/// Event raised by the analyzer when a [Rule](crate::Rule)
/// emits a diagnostic, a code action, or both
pub trait AnalyzerSignal<L: Language> {
    fn diagnostic(&self) -> Option<Diagnostic>;
    fn action(&self) -> Option<AnalyzerAction<L>>;
}

/// Code Action object returned by the analyzer, generated from a [crate::RuleAction]
/// with additional information about the rule injected by the analyzer
///
/// This struct can be converted into a [CodeSuggestion] and injected into
/// a diagnostic emitted by the same signal
#[derive(Debug, PartialEq, Eq)]
pub struct AnalyzerAction<L: Language> {
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

        for token in action.root.syntax().descendants_tokens(Direction::Next) {
            let range = token.text_range();
            if range
                .intersect(action.new_range)
                .filter(|range| !range.is_empty())
                .is_none()
            {
                continue;
            }

            code.push_str(token.text());
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
pub(crate) struct RuleSignal<R: Rule> {
    file_id: FileId,
    root: RuleRoot<R>,
    query_result: <<R as Rule>::Query as Queryable>::Output,
    state: R::State,
    services: ServiceBag,
    _rule: PhantomData<R>,
}

impl<R> RuleSignal<R>
where
    R: Rule + 'static,
{
    pub(crate) fn new(
        file_id: FileId,
        root: RuleRoot<R>,
        query_result: <<R as Rule>::Query as Queryable>::Output,
        state: R::State,
        services: ServiceBag,
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

impl<R> AnalyzerSignal<RuleLanguage<R>> for RuleSignal<R>
where
    R: Rule,
{
    fn diagnostic(&self) -> Option<Diagnostic> {
        let ctx = RuleContext::new(&self.query_result, &self.root, self.services.clone()).ok()?;

        R::diagnostic(&ctx, &self.state).map(|diag| diag.into_diagnostic(self.file_id, R::NAME))
    }

    fn action(&self) -> Option<AnalyzerAction<RuleLanguage<R>>> {
        let ctx = RuleContext::new(&self.query_result, &self.root, self.services.clone()).ok()?;

        R::action(&ctx, &self.state).and_then(|action| {
            let (original_range, new_range) =
                find_diff_range(self.root.syntax(), action.root.syntax())?;
            Some(AnalyzerAction {
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
    let prev_tokens = prev.descendants_tokens(Direction::Next);
    let next_tokens = next.descendants_tokens(Direction::Next);
    let mut tokens = prev_tokens.zip(next_tokens);

    let range_start = tokens.find_map(|(prev_token, next_token)| {
        debug_assert_eq!(
            prev_token.text_range().start(),
            next_token.text_range().start(),
        );

        if prev_token != next_token {
            Some(prev_token.text_range().start())
        } else {
            None
        }
    });

    let prev_tokens = prev.descendants_tokens(Direction::Prev);
    let next_tokens = next.descendants_tokens(Direction::Prev);
    let tokens = prev_tokens.zip(next_tokens);

    let range_end = tokens
        .take_while(|(prev_token, next_token)| {
            let prev_range = prev_token.text_range();
            let next_range = next_token.text_range();

            if let Some(range_start) = range_start {
                if prev_range.start() < range_start || next_range.start() < range_start {
                    return false;
                }
            }

            // This compares the texts instead of the tokens themselves, since the
            // implementation of PartialEq for SyntaxToken compares the text offset
            // of the tokens (which may be different since we're starting from the
            // end of the file, after the edited section)
            // It should still be rather efficient though as identical tokens will
            // reuse the same underlying green node after an edit, so the equality
            // check can stop at doing a pointer + length equality without having
            // to actually check the content of the string
            prev_token.text() == next_token.text()
        })
        .last()
        .map(|(prev_token, next_token)| {
            (
                prev_token.text_range().start(),
                next_token.text_range().start(),
            )
        });

    match (range_start, range_end) {
        (Some(start), Some((prev_end, next_end))) => Some((
            TextRange::new(start, prev_end),
            TextRange::new(start, next_end),
        )),
        (Some(start), None) => Some((
            TextRange::new(start, prev.text_range().end()),
            TextRange::new(start, next.text_range().end()),
        )),

        (None, _) => None,
    }
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
        let end = TextSize::of("if(test)consequent;elsealternate");

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
