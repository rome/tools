use crate::{Language, SyntaxNode, SyntaxSlot, SyntaxToken, SyntaxTriviaPiece};
use std::iter::{once, FusedIterator};

/// A visitor that re-writes a syntax tree while visiting the nodes.
///
/// The rewriter visits the nodes in pre-order from top-down.
/// Meaning, it first visits the `root`, and then visits the children of the root from left to right,
/// recursively traversing into child nodes and calling [`visit_node`](SyntaxRewriter) for every node.
///
/// Inspired by Roslyn's [`CSharpSyntaxRewriter`](https://docs.microsoft.com/en-us/dotnet/api/microsoft.codeanalysis.csharp.csharpsyntaxrewriter?view=roslyn-dotnet-4.2.0)
///
/// # Unsupported
///
/// The current implementation does not yet support node removal.
///
/// # Examples
///
/// Implementation of a rewritter that replaces all literal expression nodes that contain a number token
/// with a bogus node.
///
/// ```
/// # use std::iter::once;
/// # use rome_rowan::{AstNode, SyntaxNode, SyntaxRewriter, VisitNodeSignal};
/// # use rome_rowan::raw_language::{LiteralExpression, RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
///
/// struct ReplaceNumberLiteralRewriter;
///
/// impl SyntaxRewriter for ReplaceNumberLiteralRewriter {
///     type Language = RawLanguage;
///
///     fn visit_node(
///         &mut self,
///         node: SyntaxNode<Self::Language>,
///     ) -> VisitNodeSignal<Self::Language> {
///         match node.kind() {
///             RawLanguageKind::LITERAL_EXPRESSION => {
///                 let expression = LiteralExpression::unwrap_cast(node);
///
///                 let mut token = expression
///                     .syntax()
///                     .slots()
///                     .nth(0)
///                     .unwrap()
///                     .into_token()
///                     .unwrap();
///
///                 match token.kind() {
///                     RawLanguageKind::NUMBER_TOKEN => {
///                         // Use your language's syntax factory instead
///                         let bogus_node = SyntaxNode::new_detached(
///                             RawLanguageKind::BOGUS,
///                             once(Some(token.into())),
///                         );
///
///                         VisitNodeSignal::Replace(bogus_node)
///                     }
///                     // Not interested in string literal expressions, continue traversal
///                     _ => VisitNodeSignal::Traverse(expression.into_syntax()),
///                 }
///             }
///             _ => {
///                 // Traverse into the childrens of node
///                 VisitNodeSignal::Traverse(node)
///             }
///         }
///     }
/// }
///
/// let mut builder = RawSyntaxTreeBuilder::new();
///
/// builder.start_node(RawLanguageKind::ROOT);
/// builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);
///
/// builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
/// builder.token(RawLanguageKind::NUMBER_TOKEN, "5");
/// builder.finish_node();
///
/// builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
/// builder.token(RawLanguageKind::STRING_TOKEN, "'abcd'");
/// builder.finish_node();
///
/// builder.finish_node();
/// builder.finish_node();
///
/// let root = builder.finish();
///
/// let transformed = ReplaceNumberLiteralRewriter.transform(root.clone());
///
/// let original_literal_expressions: Vec<_> = root
///     .descendants()
///     .filter(|p| p.kind() == RawLanguageKind::LITERAL_EXPRESSION)
///     .collect();
///
/// assert_ne!(
///     &root, &transformed,
///     "It returns a new root with the updated children"
/// );
///
/// let literal_expressions: Vec<_> = transformed
///     .descendants()
///     .filter(|p| p.kind() == RawLanguageKind::LITERAL_EXPRESSION)
///     .collect();
///
///  // The literal expression containing a string token should be unchanged
///  assert_eq!(&literal_expressions, &original_literal_expressions[1..]);
///
///  let mut bogus: Vec<_> = transformed
///     .descendants()
///     .filter(|p| p.kind() == RawLanguageKind::BOGUS)
///     .collect();
///
/// // It replaced the number literal expression with a bogus node.
/// assert_eq!(bogus.len(), 1);
/// assert_eq!(bogus.pop().unwrap().text(), "5");
/// ```
pub trait SyntaxRewriter {
    type Language: Language;

    /// Recursively transforms the subtree of `node` by calling [`visit_node`](SyntaxRewriter::visit_node)
    /// for every token and [`visit_token`](SyntaxRewriter::visit_token) for every token in the subtree.
    ///
    /// Returns a new syntax tree reflecting the changes by the rewriter if it replaced any node and
    /// returns `node` if no changes were made.
    fn transform(&mut self, node: SyntaxNode<Self::Language>) -> SyntaxNode<Self::Language>
    where
        Self: Sized,
    {
        match self.visit_node(node) {
            VisitNodeSignal::Replace(updated) => updated,
            VisitNodeSignal::Traverse(node) => traverse(node, self),
        }
    }

    /// Called for every node in the tree. The method should return a signal specifying what should be done with the node
    ///
    /// * [VisitNodeSignal::Traverse]: Recourse into `node` so that [`visit_node`](SyntaxRewriter::visit_node)
    /// gets called for all children of `node`. The `node` will only be replaced if any node in its subtree changes.
    /// * [VisitNodeSignal::Replace]: Replaces `node` with the node specified in the [`Replace`](VisitNodeSignal::Replace) variant.
    ///  It's your responsibility to call [`traverse`](SyntaxRewriter::transform) for any child of `node` for which you want the rewritter
    ///  to recurse into its content.
    fn visit_node(&mut self, node: SyntaxNode<Self::Language>) -> VisitNodeSignal<Self::Language> {
        VisitNodeSignal::Traverse(node)
    }

    /// Called for every token in the tree. Returning a new token changes the token in the parent node.
    fn visit_token(&mut self, token: SyntaxToken<Self::Language>) -> SyntaxToken<Self::Language> {
        token
    }
}

#[derive(Debug, Clone)]
pub enum VisitNodeSignal<L: Language> {
    /// Signals the [SyntaxRewriter] to replace the current node with the specified node.
    Replace(SyntaxNode<L>),

    /// Signals the [SyntaxRewriter] to traverse into the children of the specified node.
    Traverse(SyntaxNode<L>),
}

fn traverse<R>(mut parent: SyntaxNode<R::Language>, rewriter: &mut R) -> SyntaxNode<R::Language>
where
    R: SyntaxRewriter,
{
    for slot in parent.slots() {
        match slot {
            SyntaxSlot::Node(node) => {
                let original_key = node.key();
                let index = node.index();

                let updated = rewriter.transform(node);

                if updated.key() != original_key {
                    parent = parent.splice_slots(index..=index, once(Some(updated.into())));
                }
            }
            SyntaxSlot::Token(token) => {
                let original_key = token.key();
                let index = token.index();

                let updated = rewriter.visit_token(token);

                if updated.key() != original_key {
                    parent = parent.splice_slots(index..=index, once(Some(updated.into())));
                }
            }
            SyntaxSlot::Empty => {
                // Nothing to visit
            }
        }
    }

    parent
}

#[cfg(test)]
mod tests {
    use crate::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::{SyntaxNode, SyntaxRewriter, SyntaxToken, VisitNodeSignal};

    #[test]
    pub fn test_visits_each_node() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "5");
        builder.finish_node();
        builder.finish_node();

        let root = builder.finish();

        let mut recorder = RecordRewritter::default();
        let transformed = recorder.transform(root.clone());

        assert_eq!(
            &root, &transformed,
            "It should return the same node if the rewritter doesn't replace a node."
        );

        let literal_expression = root
            .descendants()
            .find(|node| node.kind() == RawLanguageKind::LITERAL_EXPRESSION)
            .unwrap();

        assert_eq!(&recorder.nodes, &[root.clone(), literal_expression]);

        let number_literal = root.first_token().unwrap();
        assert_eq!(&recorder.tokens, &[number_literal]);
    }

    /// Visitor that records every `visit_node` and `visit_token` call.
    #[derive(Default)]
    struct RecordRewritter {
        nodes: Vec<SyntaxNode<RawLanguage>>,
        tokens: Vec<SyntaxToken<RawLanguage>>,
    }

    impl SyntaxRewriter for RecordRewritter {
        type Language = RawLanguage;

        fn visit_node(
            &mut self,
            node: SyntaxNode<Self::Language>,
        ) -> VisitNodeSignal<Self::Language> {
            self.nodes.push(node.clone());
            VisitNodeSignal::Traverse(node)
        }

        fn visit_token(
            &mut self,
            token: SyntaxToken<Self::Language>,
        ) -> SyntaxToken<Self::Language> {
            self.tokens.push(token.clone());
            token
        }
    }
}

/// It creates an iterator by chaining two trivia pieces. This iterator
/// of trivia can be attached to a token using `*_pieces` APIs.
///
/// ## Examples
///
/// ```
/// use rome_rowan::raw_language::{RawLanguageKind, RawSyntaxTreeBuilder};
/// use rome_rowan::{chain_pieces, TriviaPiece, TriviaPieceKind};
/// let mut builder = RawSyntaxTreeBuilder::new();
///
/// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
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
/// });
/// let mut tokens = node.tokens();
/// let first_token = tokens.next().unwrap();
/// let second_token = tokens.next().unwrap();
/// let third_token = tokens.next().unwrap();
///
/// let leading_trivia = chain_pieces(
///     first_token.leading_trivia().pieces(),
///     third_token.leading_trivia().pieces()
/// );
///
/// let new_first_token = first_token.with_leading_trivia_pieces(leading_trivia);
///
/// let new_token = format!("{:?}", new_first_token);
/// assert_eq!(new_token, "LET_TOKEN@0..10 \"let\" [Whitespace(\"\\n\\t \"), Whitespace(\";\")] [Whitespace(\" \\t\\t\")]")
/// ```
///
pub fn chain_pieces<L, F, S>(first: F, second: S) -> ChainTriviaPiecesIterator<F, S>
where
    L: Language,
    F: Iterator<Item = SyntaxTriviaPiece<L>>,
    S: Iterator<Item = SyntaxTriviaPiece<L>>,
{
    ChainTriviaPiecesIterator::new(first, second)
}

/// Chain iterator that chains two iterators over syntax trivia together.
///
/// This is the same as Rust's [Chain] iterator but implements [ExactSizeIterator].
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
