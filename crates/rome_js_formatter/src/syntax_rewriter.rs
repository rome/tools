use crate::parentheses::JsAnyParenthesized;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyExpression, JsLanguage, JsLogicalExpression, JsSyntaxKind, JsSyntaxNode,
};
use rome_rowan::syntax::SyntaxTrivia;
use rome_rowan::{
    AstNode, SyntaxKind, SyntaxRewriter, SyntaxTriviaPiece, SyntaxTriviaPieceComments,
    VisitNodeSignal,
};
use std::iter::FusedIterator;

#[derive(Default)]
pub(super) struct JsFormatSyntaxRewriter;

impl JsFormatSyntaxRewriter {
    /// Replaces parenthesized expression that:
    /// * have no syntax error: has no missing required child or no skipped token trivia attached to the left or right paren
    /// * inner expression isn't an unknown node
    /// * no closure or type cast type cast comment
    ///
    /// with the inner expression.
    fn visit_parenthesized(
        &mut self,
        parenthesized: JsAnyParenthesized,
    ) -> VisitNodeSignal<JsLanguage> {
        let (l_paren, inner, r_paren) = match (
            parenthesized.l_paren_token(),
            parenthesized.inner(),
            parenthesized.r_paren_token(),
        ) {
            (Ok(l_paren), Ok(inner), Ok(r_paren)) => {
                let prev_token = l_paren.prev_token();

                // Keep parentheses around unknown expressions. Rome can't know the precedence.
                if inner.kind().is_unknown()
                    // Don't remove parentheses if they have skipped trivia. We don't know for certain what the intended syntax is.
                    // Nor if there's a leading type cast comment
                    || has_type_cast_comment_or_skipped(&l_paren.leading_trivia())
                    || prev_token.map_or(false, |prev_token| has_type_cast_comment_or_skipped(&prev_token.trailing_trivia()))
                    || r_paren.leading_trivia().has_skipped()
                {
                    return VisitNodeSignal::Traverse(parenthesized.into_syntax());
                } else {
                    (l_paren, inner, r_paren)
                }
            }
            _ => {
                // At least one missing child, handle as a regular node
                return VisitNodeSignal::Traverse(parenthesized.into_syntax());
            }
        };

        let inner = self.transform(inner);

        match inner.first_token() {
            // This can only happen if we have `()` which is highly unlikely to ever be the case.
            // Return the parenthesized expression as is. This will be formatted as verbatim
            None => {
                let updated = match parenthesized {
                    JsAnyParenthesized::JsParenthesizedExpression(expression) => {
                        // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                        expression
                            .with_expression(JsAnyExpression::unwrap_cast(inner))
                            .into_syntax()
                    }
                    JsAnyParenthesized::JsParenthesizedAssignment(assignment) => {
                        // SAFETY: Safe because the rewriter never rewrites an assignment to a non assignment.
                        assignment
                            .with_assignment(JsAnyAssignment::unwrap_cast(inner))
                            .into_syntax()
                    }
                };

                VisitNodeSignal::Replace(updated)
            }

            Some(first_token) => {
                let l_paren_trivia = chain_pieces(
                    l_paren.leading_trivia().pieces(),
                    l_paren.trailing_trivia().pieces(),
                );

                let mut leading_trivia = first_token.leading_trivia().pieces().peekable();

                // The leading whitespace before the opening parens replaces the whitespace before the node.
                while let Some(trivia) = leading_trivia.peek() {
                    if trivia.is_whitespace() || trivia.is_newline() {
                        leading_trivia.next();
                    } else {
                        break;
                    }
                }

                let new_leading = chain_pieces(l_paren_trivia, leading_trivia);
                let new_first = first_token.with_leading_trivia_pieces(new_leading);

                // SAFETY: Calling `unwrap` is safe because we know that `inner_first` is part of the `inner` subtree.
                let updated = inner
                    .replace_child(first_token.into(), new_first.into())
                    .unwrap();

                let r_paren_trivia = chain_pieces(
                    r_paren.leading_trivia().pieces(),
                    r_paren.trailing_trivia().pieces(),
                );

                // SAFETY: Calling `unwrap` is safe because `last_token` only returns `None` if a node's subtree
                // doesn't contain ANY token, but we know that the subtree contains at least the first token.
                let last_token = updated.last_token().unwrap();
                let new_last = last_token.with_trailing_trivia_pieces(chain_pieces(
                    last_token.trailing_trivia().pieces(),
                    r_paren_trivia,
                ));

                // SAFETY: Calling `unwrap` is safe because we know that `last_token` is part of the `updated` subtree.
                VisitNodeSignal::Replace(
                    updated
                        .replace_child(last_token.into(), new_last.into())
                        .unwrap(),
                )
            }
        }
    }

    /// Re-balances right-recursive logical expressions with the same operator to be left recursive (relies on the parentheses removal)
    ///
    /// ```javascript
    /// a && (b && c)
    /// ```
    ///
    /// has the tree (parentheses omitted)
    ///
    /// ```text
    ///   &&
    /// a    &&
    ///    b    c
    /// ```
    ///
    /// This transform re-balances the tree so that it becomes left-recursive
    ///
    /// ```text
    ///     &&
    ///  &&    c
    /// a  b
    /// ```
    ///
    /// This is required so that the binary like expression formatting only has to resolve left recursive expressions.
    fn visit_logical_expression(
        &mut self,
        logical: JsLogicalExpression,
    ) -> VisitNodeSignal<JsLanguage> {
        match (logical.left(), logical.operator_token(), logical.right()) {
            (Ok(left), Ok(operator), Ok(right)) => {
                let left_key = left.syntax().key();
                let operator_key = operator.key();
                let right_key = right.syntax().key();

                // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                let left = JsAnyExpression::unwrap_cast(self.transform(left.into_syntax()));
                let operator = self.visit_token(operator);
                // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                let right = JsAnyExpression::unwrap_cast(self.transform(right.into_syntax()));

                let updated = match right {
                    JsAnyExpression::JsLogicalExpression(right_logical) => {
                        match (
                            right_logical.left(),
                            right_logical.operator_token(),
                            right_logical.right(),
                        ) {
                            (Ok(right_left), Ok(right_operator), Ok(right_right))
                                if right_operator.kind() == operator.kind() =>
                            {
                                logical
                                    .with_left(
                                        rome_js_factory::make::js_logical_expression(
                                            left, operator, right_left,
                                        )
                                        .into(),
                                    )
                                    .with_operator_token_token(right_operator)
                                    .with_right(right_right)
                            }

                            // Don't re-balance a logical expression that has syntax errors
                            _ => logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right_logical.into()),
                        }
                    }

                    // Don't re-balance logical expressions with different operators
                    // Avoid updating the node if none of the children have changed to avoid
                    // re-spinning all parents.
                    right => {
                        if left.syntax().key() != left_key
                            || operator.key() != operator_key
                            || right.syntax().key() != right_key
                        {
                            logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right)
                        } else {
                            logical
                        }
                    }
                };

                VisitNodeSignal::Replace(updated.into_syntax())
            }
            _ => VisitNodeSignal::Traverse(logical.into_syntax()),
        }
    }
}

impl SyntaxRewriter for JsFormatSyntaxRewriter {
    type Language = JsLanguage;

    fn visit_node(&mut self, node: JsSyntaxNode) -> VisitNodeSignal<Self::Language> {
        match node.kind() {
            kind if JsAnyParenthesized::can_cast(kind) => {
                let parenthesized = JsAnyParenthesized::unwrap_cast(node);

                self.visit_parenthesized(parenthesized)
            }
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                let logical = JsLogicalExpression::unwrap_cast(node);

                self.visit_logical_expression(logical)
            }
            _ => VisitNodeSignal::Traverse(node),
        }
    }
}

fn has_type_cast_comment_or_skipped(trivia: &SyntaxTrivia<JsLanguage>) -> bool {
    trivia.pieces().any(|piece| {
        if let Some(comment) = piece.as_comments() {
            is_type_comment(&comment)
        } else {
            piece.is_skipped()
        }
    })
}

/// Returns `true` if `comment` is a [Closure type comment](https://github.com/google/closure-compiler/wiki/Types-in-the-Closure-Type-System)
/// or [TypeScript type comment](https://www.typescriptlang.org/docs/handbook/jsdoc-supported-types.html#type)
fn is_type_comment(comment: &SyntaxTriviaPieceComments<JsLanguage>) -> bool {
    let text = comment.text();

    // Must be a `/**` comment
    if !text.starts_with("/**") {
        return false;
    }

    text.trim_start_matches("/**")
        .trim_end_matches("*/")
        .split_whitespace()
        .any(|word| match word.strip_prefix("@type") {
            Some(after) => after.is_empty() || after.starts_with('{'),
            None => false,
        })
}

fn chain_pieces<F, S>(first: F, second: S) -> ChainTriviaPiecesIterator<F, S>
where
    F: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    S: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    ChainTriviaPiecesIterator::new(first, second)
}

/// Chain iterator that chains two iterators over syntax trivia together.
///
/// This is the same as Rust's [Chain] iterator but implements [ExactSizeIterator].
/// Rust doesn't implement [ExactSizeIterator] because adding the sizes of both pieces may overflow.
///
/// Implementing [ExactSizeIterator] in our case is safe because our ranges are limited to [u32], which in turn
/// guarantees that a tree can never have more than 2^32 tokens or pieces and adding a `u32 + u32` is safely in the range
/// of a `usize`.
struct ChainTriviaPiecesIterator<F, S> {
    first: Option<F>,
    second: S,
}

impl<F, S> ChainTriviaPiecesIterator<F, S> {
    fn new(first: F, second: S) -> Self {
        Self {
            first: Some(first),
            second,
        }
    }
}

impl<F, S> Iterator for ChainTriviaPiecesIterator<F, S>
where
    F: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    S: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    type Item = SyntaxTriviaPiece<JsLanguage>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.first {
            Some(first) => match first.next() {
                Some(next) => Some(next),
                None => {
                    self.first.take();
                    self.second.next()
                }
            },
            None => self.second.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.first {
            Some(first) => {
                let (first_lower, first_upper) = first.size_hint();
                let (second_lower, second_upper) = self.second.size_hint();

                let lower = first_lower.saturating_add(second_lower);

                let upper = match (first_upper, second_upper) {
                    (Some(first), Some(second)) => first.checked_add(second),
                    _ => None,
                };

                (lower, upper)
            }
            None => self.second.size_hint(),
        }
    }
}

impl<F, S> FusedIterator for ChainTriviaPiecesIterator<F, S>
where
    F: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    S: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
}

impl<F, S> ExactSizeIterator for ChainTriviaPiecesIterator<F, S>
where
    F: ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>>,
    S: ExactSizeIterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    fn len(&self) -> usize {
        match &self.first {
            Some(first) => {
                let first_len = first.len();
                let second_len = self.second.len();

                // SAFETY: Should be safe because a program can never contain more than u32 pieces
                // because the text ranges are represented as u32 (and each piece must at least contain a single character).
                first_len + second_len
            }
            None => self.second.len(),
        }
    }
}
