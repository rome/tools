use std::{borrow::Cow, fmt, iter, marker::PhantomData, ops::Range};

use crate::{
	cursor, green::GreenTokenData, Direction, GreenNode, GreenNodeData, GreenToken, NodeOrToken,
	SyntaxKind, SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent,
};

pub trait Language: Sized + Clone + Copy + fmt::Debug + Eq + Ord + std::hash::Hash {
	type Kind: fmt::Debug;

	fn kind_from_raw(raw: SyntaxKind) -> Self::Kind;
	fn kind_to_raw(kind: Self::Kind) -> SyntaxKind;
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode<L: Language> {
	raw: cursor::SyntaxNode,
	_p: PhantomData<L>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken<L: Language> {
	raw: cursor::SyntaxToken,
	_p: PhantomData<L>,
}

pub type SyntaxElement<L> = NodeOrToken<SyntaxNode<L>, SyntaxToken<L>>;

impl<L: Language> fmt::Debug for SyntaxNode<L> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			let mut level = 0;
			for event in self.preorder_with_tokens() {
				match event {
					WalkEvent::Enter(element) => {
						for _ in 0..level {
							write!(f, "  ")?;
						}
						match element {
							NodeOrToken::Node(node) => writeln!(f, "{:?}", node)?,
							NodeOrToken::Token(token) => writeln!(f, "{:?}", token)?,
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

impl<L: Language> fmt::Debug for SyntaxToken<L> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}@{:?}", self.kind(), self.text_range())?;
		if self.text().len() < 25 {
			return write!(f, " {:?}", self.text());
		}
		let text = self.text();
		for idx in 21..25 {
			if text.is_char_boundary(idx) {
				let text = format!("{} ...", &text[..idx]);
				return write!(f, " {:?}", text);
			}
		}
		unreachable!()
	}
}

impl<L: Language> fmt::Display for SyntaxToken<L> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.raw, f)
	}
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxElement<L> {
	fn from(node: SyntaxNode<L>) -> SyntaxElement<L> {
		NodeOrToken::Node(node)
	}
}

impl<L: Language> From<SyntaxToken<L>> for SyntaxElement<L> {
	fn from(token: SyntaxToken<L>) -> SyntaxElement<L> {
		NodeOrToken::Token(token)
	}
}

impl<L: Language> SyntaxNode<L> {
	pub fn new_root(green: GreenNode) -> SyntaxNode<L> {
		SyntaxNode::from(cursor::SyntaxNode::new_root(green))
	}
	/// Returns a green tree, equal to the green tree this node
	/// belongs two, except with this node substitute. The complexity
	/// of operation is proportional to the depth of the tree
	pub fn replace_with(&self, replacement: GreenNode) -> GreenNode {
		self.raw.replace_with(replacement)
	}

	pub fn kind(&self) -> L::Kind {
		L::kind_from_raw(self.raw.kind())
	}

	pub fn text_range(&self) -> TextRange {
		self.raw.text_range()
	}

	pub fn index(&self) -> usize {
		self.raw.index()
	}

	pub fn text(&self) -> SyntaxText {
		self.raw.text()
	}

	pub fn green(&self) -> Cow<'_, GreenNodeData> {
		self.raw.green()
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
}

impl<L: Language> SyntaxToken<L> {
	/// Returns a green tree, equal to the green tree this token
	/// belongs two, except with this token substitute. The complexity
	/// of operation is proportional to the depth of the tree
	pub fn replace_with(&self, new_token: GreenToken) -> GreenNode {
		self.raw.replace_with(new_token)
	}

	pub fn kind(&self) -> L::Kind {
		L::kind_from_raw(self.raw.kind())
	}

	pub fn text_range(&self) -> TextRange {
		self.raw.text_range()
	}

	pub fn index(&self) -> usize {
		self.raw.index()
	}

	pub fn text(&self) -> &str {
		self.raw.text()
	}

	pub fn green(&self) -> &GreenTokenData {
		self.raw.green()
	}

	pub fn parent(&self) -> Option<SyntaxNode<L>> {
		self.raw.parent().map(SyntaxNode::from)
	}

	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
		self.raw.ancestors().map(SyntaxNode::from)
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.next_sibling_or_token().map(NodeOrToken::from)
	}
	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.prev_sibling_or_token().map(NodeOrToken::from)
	}

	pub fn siblings_with_tokens(
		&self,
		direction: Direction,
	) -> impl Iterator<Item = SyntaxElement<L>> {
		self.raw
			.siblings_with_tokens(direction)
			.map(SyntaxElement::from)
	}

	/// Next token in the tree (i.e, not necessary a sibling).
	pub fn next_token(&self) -> Option<SyntaxToken<L>> {
		self.raw.next_token().map(SyntaxToken::from)
	}
	/// Previous token in the tree (i.e, not necessary a sibling).
	pub fn prev_token(&self) -> Option<SyntaxToken<L>> {
		self.raw.prev_token().map(SyntaxToken::from)
	}

	pub fn detach(&self) {
		self.raw.detach()
	}
}

impl<L: Language> SyntaxElement<L> {
	pub fn text_range(&self) -> TextRange {
		match self {
			NodeOrToken::Node(it) => it.text_range(),
			NodeOrToken::Token(it) => it.text_range(),
		}
	}

	pub fn index(&self) -> usize {
		match self {
			NodeOrToken::Node(it) => it.index(),
			NodeOrToken::Token(it) => it.index(),
		}
	}

	pub fn kind(&self) -> L::Kind {
		match self {
			NodeOrToken::Node(it) => it.kind(),
			NodeOrToken::Token(it) => it.kind(),
		}
	}

	pub fn parent(&self) -> Option<SyntaxNode<L>> {
		match self {
			NodeOrToken::Node(it) => it.parent(),
			NodeOrToken::Token(it) => it.parent(),
		}
	}

	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
		let first = match self {
			NodeOrToken::Node(it) => Some(it.clone()),
			NodeOrToken::Token(it) => it.parent(),
		};
		iter::successors(first, SyntaxNode::parent)
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		match self {
			NodeOrToken::Node(it) => it.next_sibling_or_token(),
			NodeOrToken::Token(it) => it.next_sibling_or_token(),
		}
	}
	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		match self {
			NodeOrToken::Node(it) => it.prev_sibling_or_token(),
			NodeOrToken::Token(it) => it.prev_sibling_or_token(),
		}
	}
	pub fn detach(&self) {
		match self {
			NodeOrToken::Node(it) => it.detach(),
			NodeOrToken::Token(it) => it.detach(),
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

#[derive(Debug, Clone)]
pub struct SyntaxElementChildren<L: Language> {
	raw: cursor::SyntaxElementChildren,
	_p: PhantomData<L>,
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

impl<L: Language> From<cursor::SyntaxNode> for SyntaxNode<L> {
	fn from(raw: cursor::SyntaxNode) -> SyntaxNode<L> {
		SyntaxNode {
			raw,
			_p: PhantomData,
		}
	}
}

impl<L: Language> From<SyntaxNode<L>> for cursor::SyntaxNode {
	fn from(node: SyntaxNode<L>) -> cursor::SyntaxNode {
		node.raw
	}
}

impl<L: Language> From<cursor::SyntaxToken> for SyntaxToken<L> {
	fn from(raw: cursor::SyntaxToken) -> SyntaxToken<L> {
		SyntaxToken {
			raw,
			_p: PhantomData,
		}
	}
}

impl<L: Language> From<SyntaxToken<L>> for cursor::SyntaxToken {
	fn from(token: SyntaxToken<L>) -> cursor::SyntaxToken {
		token.raw
	}
}

impl<L: Language> From<cursor::SyntaxElement> for SyntaxElement<L> {
	fn from(raw: cursor::SyntaxElement) -> SyntaxElement<L> {
		match raw {
			NodeOrToken::Node(it) => NodeOrToken::Node(it.into()),
			NodeOrToken::Token(it) => NodeOrToken::Token(it.into()),
		}
	}
}

impl<L: Language> From<SyntaxElement<L>> for cursor::SyntaxElement {
	fn from(element: SyntaxElement<L>) -> cursor::SyntaxElement {
		match element {
			NodeOrToken::Node(it) => NodeOrToken::Node(it.into()),
			NodeOrToken::Token(it) => NodeOrToken::Token(it.into()),
		}
	}
}
