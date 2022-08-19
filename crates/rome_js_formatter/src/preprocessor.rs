use rome_js_syntax::{
    JsAnyExpression, JsLanguage, JsParenthesizedExpression, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{AstNode, BatchMutation, SyntaxKind, SyntaxTriviaPiece};
use std::collections::HashMap;
use std::iter::FusedIterator;

pub(super) fn preprocess(root: &JsSyntaxNode) -> JsSyntaxNode {
    let mut mutation = BatchMutation::new(root.clone());
    // tracks all changed tokens where the key is the token in the original tree and the value is the
    // updated token. Necessary, because removing parentheses may require adding leading/trailing trivia
    // to the same node: `(a) /* leading */ + (/* trailing 2*/ b)` Both comments should be merged with the `+` tokens trivia.
    let mut tokens: HashMap<JsSyntaxToken, JsSyntaxToken> = HashMap::new();

    let mut prev_token_trailing = Vec::new();
    let mut first_inner_leading = Vec::new();
    let mut last_inner_trailing = Vec::new();
    let mut next_token_leading = Vec::new();

    let mut parentheses: Option<JsParenthesizedExpression> = None;
    let mut prev_token: Option<JsSyntaxToken> = None;
    let mut next_token: Option<JsSyntaxToken> = None;

    for node in root.descendants() {
        match get_left_paren_inner_and_right_paren(node) {
            Ok((l_paren, inner, r_paren, parenthesized)) => {
                let l_paren_leading = l_paren.leading_trivia();
                let l_paren_trailing = l_paren.trailing_trivia();

                if parentheses.is_none() {
                    debug_assert!(prev_token_trailing.is_empty());
                    debug_assert!(first_inner_leading.is_empty());
                    debug_assert!(last_inner_trailing.is_empty());
                    debug_assert!(next_token_leading.is_empty());
                    debug_assert!(prev_token.is_none());
                    debug_assert!(next_token.is_none());

                    prev_token = l_paren.prev_token();
                    next_token = r_paren.next_token();
                    parentheses = Some(parenthesized);
                }

                if first_inner_leading.is_empty() && l_paren_leading.is_empty() {
                    prev_token_trailing.extend(l_paren_trailing.pieces());
                } else {
                    first_inner_leading
                        .extend(l_paren_leading.pieces().chain(l_paren_trailing.pieces()));
                }

                let r_paren_leading = r_paren.leading_trivia();
                let r_paren_trailing = r_paren.trailing_trivia();

                if next_token_leading.is_empty() && r_paren_leading.is_empty() {
                    last_inner_trailing.extend(r_paren_trailing.pieces());
                } else {
                    next_token_leading
                        .extend(r_paren_leading.pieces().chain(r_paren_trailing.pieces()))
                }
            }
            Err(node) => {
                let parenthesized = parentheses.take();
                let prev_token = prev_token.take();
                let next_token = next_token.take();

                if !prev_token_trailing.is_empty() {
                    debug_assert!(parenthesized.is_some());

                    match prev_token {
                        Some(prev_token) => {
                            let new_prev_token = tokens.get(&prev_token).unwrap_or(&prev_token);

                            let new_prev_token =
                                new_prev_token.with_trailing_trivia_pieces(chain_pieces(
                                    new_prev_token.trailing_trivia().pieces(),
                                    prev_token_trailing.drain(..),
                                ));

                            tokens.insert(prev_token, new_prev_token);
                        }
                        None => {
                            // No previous token, everything becomes the leading of the inner token.
                            // May happen if the parenthesized expression is at the start of the program.
                            first_inner_leading.append(&mut prev_token_trailing);
                        }
                    }
                }

                let mut new_node = node;

                if !first_inner_leading.is_empty() {
                    debug_assert!(parenthesized.is_some());

                    match new_node.first_token() {
                        Some(first_token) => {
                            let new_first_token =
                                tokens.remove(&first_token).unwrap_or(first_token.clone());

                            let new_first_token =
                                new_first_token.with_leading_trivia_pieces(chain_pieces(
                                    new_first_token.trailing_trivia().pieces(),
                                    first_inner_leading.drain(..),
                                ));

                            new_node = new_node
                                .replace_child(first_token.into(), new_first_token.into())
                                .unwrap();
                        }
                        None => {
                            // The only case this can happen is if we have `()` which isn't valid code.
                            // Make the trivia the leading trivia of the next token (doesn't change order because `prev_token` will also be None).
                            next_token_leading.append(&mut first_inner_leading);
                        }
                    }
                }

                if !last_inner_trailing.is_empty() {
                    debug_assert!(parenthesized.is_some());

                    match new_node.last_token() {
                        Some(last_token) => {
                            let new_last_token =
                                tokens.remove(&last_token).unwrap_or(last_token.clone());

                            let new_last_token =
                                new_last_token.with_trailing_trivia_pieces(chain_pieces(
                                    new_last_token.trailing_trivia().pieces(),
                                    last_inner_trailing.drain(..),
                                ));

                            new_node = new_node
                                .replace_child(last_token.into(), new_last_token.into())
                                .unwrap();
                        }
                        None => {
                            // This happens if the expression is `()` which isn't valid code.
                            // Make the trivia the leading trivia of the next token (which hopefully will exist).
                            next_token_leading.append(&mut last_inner_trailing);
                        }
                    }
                }

                if !next_token_leading.is_empty() {
                    debug_assert!(parenthesized.is_some());

                    match next_token {
                        Some(next_token) => {
                            let new_next_token = tokens.get(&next_token).unwrap_or(&next_token);
                            let new_next_token =
                                new_next_token.with_leading_trivia_pieces(chain_pieces(
                                    next_token_leading.drain(..),
                                    new_next_token.leading_trivia().pieces(),
                                ));

                            tokens.insert(next_token, new_next_token);
                        }
                        None => {
                            panic!("Missing `EOF` token.");
                        }
                    }
                }

                if let Some(parenthesized) = parenthesized {
                    mutation.replace_element_discard_trivia(
                        parenthesized.into_syntax().into(),
                        new_node.into(),
                    );
                }
            }
        }
    }

    for (old, new) in tokens.into_iter() {
        mutation.replace_element_discard_trivia(old.into(), new.into())
    }

    mutation.commit()
}

fn get_left_paren_inner_and_right_paren(
    node: JsSyntaxNode,
) -> Result<
    (
        JsSyntaxToken,
        JsAnyExpression,
        JsSyntaxToken,
        JsParenthesizedExpression,
    ),
    JsSyntaxNode,
> {
    match JsParenthesizedExpression::try_cast(node) {
        Ok(parenthesized) => match (
            parenthesized.l_paren_token(),
            parenthesized.expression(),
            parenthesized.r_paren_token(),
        ) {
            (Ok(l_paren), Ok(expression), Ok(r_paren)) => {
                // Keep parentheses around unknown expressions. Rome can't know the precedence.
                if expression.syntax().kind().is_unknown()
                    // Don't remove parentheses if they have skipped trivia. We don't know for certain what the intended syntax is.
                    || l_paren.leading_trivia().has_skipped()
                    || r_paren.leading_trivia().has_skipped()
                {
                    Err(parenthesized.into_syntax())
                } else if expression.syntax().first_token().is_none() {
                    // This should never happen but we need to be sure.
                    Err(parenthesized.into_syntax())
                } else {
                    Ok((l_paren, expression, r_paren, parenthesized))
                }
            }
            _ => {
                // At least one missing child, handle as a regular node
                Err(parenthesized.into_syntax())
            }
        },
        Err(node) => Err(node),
    }
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
