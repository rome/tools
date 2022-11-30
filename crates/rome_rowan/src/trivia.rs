//! Module that exports APIs to help working with trivia

use crate::{Language, SyntaxTriviaPiece};
use std::iter::FusedIterator;

/// It creates an iterator by chaining two trivia pieces. This iterator
/// of trivia can be attached to a token using `*_pieces` APIs.
///
/// ## Examples
///
/// ```
/// use rome_rowan::raw_language::{RawLanguageKind, RawSyntaxTreeBuilder};
/// use rome_rowan::{chain_trivia_pieces, TriviaPiece, TriviaPieceKind};
/// # use rome_rowan::SyntaxResult;
///
/// # // fn main() -> SyntaxResult<()> {
///  use rome_rowan::SyntaxError;
/// let mut builder = RawSyntaxTreeBuilder::new();
///  let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
///     builder.token_with_trivia(
///         RawLanguageKind::LET_TOKEN,
///         "\n\t let \t\t",
///         &[TriviaPiece::whitespace(3)],
///         &[TriviaPiece::whitespace(3)],
///     );
///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
///     builder.token_with_trivia(
///         RawLanguageKind::SEMICOLON_TOKEN,
///         "; \t\t",
///         &[TriviaPiece::whitespace(1)],
///         &[TriviaPiece::whitespace(1)],
///     );
///  });
///  let mut tokens = node.tokens();
///  let first_token = tokens.next().unwrap();
///  let second_token = tokens.next().unwrap();
///  let third_token = tokens.next().unwrap();
///
///  let leading_trivia = chain_trivia_pieces(
///     first_token.leading_trivia().pieces(),
///     third_token.leading_trivia().pieces()
///  );
///
///  let new_first_token = first_token.with_leading_trivia_pieces(leading_trivia);
///
///  let new_token = format!("{:?}", new_first_token);
///  assert_eq!(new_token, "LET_TOKEN@0..10 \"let\" [Whitespace(\"\\n\\t \"), Whitespace(\";\")] [Whitespace(\" \\t\\t\")]");
///
/// #    // Ok(())
/// # // }
///
/// ```
///
pub fn chain_trivia_pieces<L, F, S>(first: F, second: S) -> ChainTriviaPiecesIterator<F, S>
where
    L: Language,
    F: Iterator<Item = SyntaxTriviaPiece<L>>,
    S: Iterator<Item = SyntaxTriviaPiece<L>>,
{
    ChainTriviaPiecesIterator::new(first, second)
}

/// Chain iterator that chains two iterators over syntax trivia together.
///
/// This is the same as Rust's [std::iter::Chain] iterator but implements [ExactSizeIterator].
/// Rust doesn't implement [ExactSizeIterator] because adding the sizes of both pieces may overflow.
///
/// Implementing [ExactSizeIterator] in our case is safe because this may only overflow if
/// a source document has more than 2^32 trivia which isn't possible because our source documents are limited to 2^32
/// length.
pub struct ChainTriviaPiecesIterator<F, S> {
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

impl<L, F, S> Iterator for ChainTriviaPiecesIterator<F, S>
where
    L: Language,
    F: Iterator<Item = SyntaxTriviaPiece<L>>,
    S: Iterator<Item = SyntaxTriviaPiece<L>>,
{
    type Item = SyntaxTriviaPiece<L>;

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

impl<L, F, S> FusedIterator for ChainTriviaPiecesIterator<F, S>
where
    L: Language,
    F: Iterator<Item = SyntaxTriviaPiece<L>>,
    S: Iterator<Item = SyntaxTriviaPiece<L>>,
{
}

impl<L, F, S> ExactSizeIterator for ChainTriviaPiecesIterator<F, S>
where
    L: Language,
    F: ExactSizeIterator<Item = SyntaxTriviaPiece<L>>,
    S: ExactSizeIterator<Item = SyntaxTriviaPiece<L>>,
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
