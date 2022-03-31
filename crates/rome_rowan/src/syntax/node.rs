use crate::syntax::element::SyntaxElement;
use crate::syntax::SyntaxTrivia;
use crate::{
    cursor, Direction, GreenNode, Language, NodeOrToken, SyntaxKind, SyntaxList, SyntaxText,
    SyntaxToken, TokenAtOffset, WalkEvent,
};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::Range;
use text_size::{TextRange, TextSize};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode<L: Language> {
    raw: cursor::SyntaxNode,
    _p: PhantomData<L>,
}

impl<L: Language> SyntaxNode<L> {
    pub(crate) fn new_root(green: GreenNode) -> SyntaxNode<L> {
        SyntaxNode::from(cursor::SyntaxNode::new_root(green))
    }

    /// Returns the element stored in the slot with the given index. Returns [None] if the slot is empty.
    ///
    /// ## Panics
    /// If the slot index is out of bounds
    pub fn element_in_slot(&self, slot: u32) -> Option<SyntaxElement<L>> {
        self.raw.element_in_slot(slot).map(SyntaxElement::from)
    }

    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.raw.kind())
    }

    /// Returns the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("\n\t let \t\ta; \t\t", node.text());
    /// ```
    pub fn text(&self) -> SyntaxText {
        self.raw.text()
    }

    /// Returns the text of all descendants tokens combined,
    /// excluding the first token leading trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("let \t\ta;", node.text_trimmed());
    /// ```
    pub fn text_trimmed(&self) -> SyntaxText {
        self.raw.text_trimmed()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_range();
    /// assert_eq!(0u32, range.start().into());
    /// assert_eq!(14u32, range.end().into());
    /// ```
    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined,
    /// excluding the first token leading  trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_trimmed_range();
    /// assert_eq!(3u32, range.start().into());
    /// assert_eq!(11u32, range.end().into());
    /// ```
    pub fn text_trimmed_range(&self) -> TextRange {
        self.raw.text_trimmed_range()
    }

    /// Returns the leading trivia of the [first_token](SyntaxNode::first_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!("\n\t ", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {});
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn first_leading_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.first_leading_trivia().map(SyntaxTrivia::new)
    }

    /// Returns the trailing trivia of the  [last_token](SyntaxNode::last_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
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
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!(" \t\t", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {});
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn last_trailing_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.last_trailing_trivia().map(SyntaxTrivia::new)
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        self.raw.parent().map(Self::from)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.ancestors().map(SyntaxNode::from)
    }

    pub fn children(&self) -> SyntaxNodeChildren<L> {
        SyntaxNodeChildren {
            raw: self.raw.children(),
            _p: PhantomData,
        }
    }

    /// Returns an iterator over all the slots of this syntax node.
    pub fn slots(&self) -> SyntaxSlots<L> {
        SyntaxSlots {
            raw: self.raw.slots(),
            _p: PhantomData,
        }
    }

    pub fn children_with_tokens(&self) -> SyntaxElementChildren<L> {
        SyntaxElementChildren {
            raw: self.raw.children_with_tokens(),
            _p: PhantomData,
        }
    }

    pub fn first_child(&self) -> Option<SyntaxNode<L>> {
        self.raw.first_child().map(Self::from)
    }
    pub fn last_child(&self) -> Option<SyntaxNode<L>> {
        self.raw.last_child().map(Self::from)
    }

    pub fn first_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.first_child_or_token().map(NodeOrToken::from)
    }
    pub fn last_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.last_child_or_token().map(NodeOrToken::from)
    }

    pub fn next_sibling(&self) -> Option<SyntaxNode<L>> {
        self.raw.next_sibling().map(Self::from)
    }
    pub fn prev_sibling(&self) -> Option<SyntaxNode<L>> {
        self.raw.prev_sibling().map(Self::from)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.next_sibling_or_token().map(NodeOrToken::from)
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.prev_sibling_or_token().map(NodeOrToken::from)
    }

    /// Return the leftmost token in the subtree of this node.
    pub fn first_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.first_token().map(SyntaxToken::from)
    }
    /// Return the rightmost token in the subtree of this node.
    pub fn last_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.last_token().map(SyntaxToken::from)
    }

    pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.siblings(direction).map(SyntaxNode::from)
    }

    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw
            .siblings_with_tokens(direction)
            .map(SyntaxElement::from)
    }

    pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.descendants().map(SyntaxNode::from)
    }

    pub fn descendants_tokens(&self) -> impl Iterator<Item = SyntaxToken<L>> {
        self.descendants_with_tokens()
            .filter_map(|x| x.as_token().cloned())
    }

    pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw.descendants_with_tokens().map(NodeOrToken::from)
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, excluding tokens.
    pub fn preorder(&self) -> Preorder<L> {
        Preorder {
            raw: self.raw.preorder(),
            _p: PhantomData,
        }
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, including tokens.
    pub fn preorder_with_tokens(&self) -> PreorderWithTokens<L> {
        PreorderWithTokens {
            raw: self.raw.preorder_with_tokens(),
            _p: PhantomData,
        }
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    /// Precondition: offset must be withing node's range.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken<L>> {
        self.raw.token_at_offset(offset).map(SyntaxToken::from)
    }

    /// Return the deepest node or token in the current subtree that fully
    /// contains the range. If the range is empty and is contained in two leaf
    /// nodes, either one can be returned. Precondition: range must be contained
    /// withing the current node
    pub fn covering_element(&self, range: TextRange) -> SyntaxElement<L> {
        NodeOrToken::from(self.raw.covering_element(range))
    }

    /// Finds a [`SyntaxElement`] which intersects with a given `range`. If
    /// there are several intersecting elements, any one can be returned.
    ///
    /// The method uses binary search internally, so it's complexity is
    /// `O(log(N))` where `N = self.children_with_tokens().count()`.
    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement<L>> {
        self.raw
            .child_or_token_at_range(range)
            .map(SyntaxElement::from)
    }

    /// Returns an independent copy of the subtree rooted at this node.
    ///
    /// The parent of the returned node will be `None`, the start offset will be
    /// zero, but, otherwise, it'll be equivalent to the source node.
    pub fn clone_subtree(&self) -> SyntaxNode<L> {
        SyntaxNode::from(self.raw.clone_subtree())
    }

    pub fn clone_for_update(&self) -> SyntaxNode<L> {
        SyntaxNode::from(self.raw.clone_for_update())
    }

    pub fn detach(&self) {
        self.raw.detach()
    }

    pub fn splice_children(&self, to_delete: Range<usize>, to_insert: Vec<SyntaxElement<L>>) {
        let to_insert = to_insert
            .into_iter()
            .map(cursor::SyntaxElement::from)
            .collect::<Vec<_>>();
        self.raw.splice_children(to_delete, to_insert)
    }

    pub fn into_list(self) -> SyntaxList<L> {
        SyntaxList::new(self)
    }
}

impl<L: Language> fmt::Debug for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut level = 0;
            for event in self.raw.preorder_slots() {
                match event {
                    WalkEvent::Enter(element) => {
                        for _ in 0..level {
                            write!(f, "  ")?;
                        }
                        match element {
                            cursor::SyntaxSlot::Node(node) => {
                                writeln!(f, "{}: {:?}", node.index(), SyntaxNode::<L>::from(node))?
                            }
                            cursor::SyntaxSlot::Token(token) => writeln!(
                                f,
                                "{}: {:?}",
                                token.index(),
                                SyntaxToken::<L>::from(token)
                            )?,
                            cursor::SyntaxSlot::Empty { index, .. } => {
                                writeln!(f, "{}: (empty)", index)?
                            }
                        }
                        level += 1;
                    }
                    WalkEvent::Leave(_) => level -= 1,
                }
            }
            assert_eq!(level, 0);
            Ok(())
        } else {
            write!(f, "{:?}@{:?}", self.kind(), self.text_range())
        }
    }
}

impl<L: Language> fmt::Display for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<L: Language> From<SyntaxNode<L>> for cursor::SyntaxNode {
    fn from(node: SyntaxNode<L>) -> cursor::SyntaxNode {
        node.raw
    }
}

impl<L: Language> From<cursor::SyntaxNode> for SyntaxNode<L> {
    fn from(raw: cursor::SyntaxNode) -> SyntaxNode<L> {
        SyntaxNode {
            raw,
            _p: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNodeChildren<L: Language> {
    raw: cursor::SyntaxNodeChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxNodeChildren<L> {
    type Item = SyntaxNode<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxNode::from)
    }
}

#[derive(Clone)]
pub struct SyntaxElementChildren<L: Language> {
    raw: cursor::SyntaxElementChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Debug for SyntaxElementChildren<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<L: Language> Default for SyntaxElementChildren<L> {
    fn default() -> Self {
        SyntaxElementChildren {
            raw: cursor::SyntaxElementChildren::default(),
            _p: PhantomData,
        }
    }
}

impl<L: Language> Iterator for SyntaxElementChildren<L> {
    type Item = SyntaxElement<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(NodeOrToken::from)
    }
}

pub struct Preorder<L: Language> {
    raw: cursor::Preorder,
    _p: PhantomData<L>,
}

impl<L: Language> Preorder<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for Preorder<L> {
    type Item = WalkEvent<SyntaxNode<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxNode::from))
    }
}

pub struct PreorderWithTokens<L: Language> {
    raw: cursor::PreorderWithTokens,
    _p: PhantomData<L>,
}

impl<L: Language> PreorderWithTokens<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for PreorderWithTokens<L> {
    type Item = WalkEvent<SyntaxElement<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxElement::from))
    }
}

/// Each node has a slot for each of its children regardless if the child is present or not.
/// A child that isn't present either because it's optional or because of a syntax error
/// is stored in an [SyntaxSlot::Empty] to preserve the index of each child.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SyntaxSlot<L: Language> {
    /// Slot that stores a node child
    Node(SyntaxNode<L>),
    /// Slot that stores a token child
    Token(SyntaxToken<L>),
    /// Slot that marks that the child in this position isn't present in the source code.
    Empty,
}

impl<L: Language> SyntaxSlot<L> {
    pub fn into_node(self) -> Option<SyntaxNode<L>> {
        match self {
            SyntaxSlot::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn into_token(self) -> Option<SyntaxToken<L>> {
        match self {
            SyntaxSlot::Token(token) => Some(token),
            _ => None,
        }
    }

    pub fn kind(&self) -> Option<L::Kind> {
        match self {
            SyntaxSlot::Node(node) => Some(node.kind()),
            SyntaxSlot::Token(token) => Some(token.kind()),
            SyntaxSlot::Empty => None,
        }
    }
}

impl<L: Language> From<cursor::SyntaxSlot> for SyntaxSlot<L> {
    fn from(raw: cursor::SyntaxSlot) -> Self {
        match raw {
            cursor::SyntaxSlot::Node(node) => SyntaxSlot::Node(node.into()),
            cursor::SyntaxSlot::Token(token) => SyntaxSlot::Token(token.into()),
            cursor::SyntaxSlot::Empty { .. } => SyntaxSlot::Empty,
        }
    }
}

/// Iterator over the slots of a node.
#[derive(Debug, Clone)]
pub struct SyntaxSlots<L> {
    raw: cursor::SyntaxSlots,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxSlots<L> {
    type Item = SyntaxSlot<L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxSlot::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.raw.last().map(SyntaxSlot::from)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n).map(SyntaxSlot::from)
    }
}

impl<'a, L: Language> FusedIterator for SyntaxSlots<L> {}

impl<'a, L: Language> ExactSizeIterator for SyntaxSlots<L> {
    fn len(&self) -> usize {
        self.raw.len()
    }
}
