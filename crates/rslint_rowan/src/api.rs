use std::{fmt, marker::PhantomData};

use crate::{
	cursor, Direction, GreenNode, GreenToken, NodeOrToken, SmolStr, SyntaxKind, SyntaxText,
	TextRange, TextSize, TokenAtOffset, WalkEvent,
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken<L: Language> {
	raw: cursor::SyntaxToken,
	_p: PhantomData<L>,
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

impl<L: Language> fmt::Debug for SyntaxToken<L> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}@{:?}", self.kind(), self.text_range())?;
		if self.text().len() < 25 {
			return write!(f, " \"{}\"", self.text().escape_debug());
		}
		let text = self.text().as_str();
		for idx in 21..25 {
			if text.is_char_boundary(idx) {
				let text = format!("{} ...", &text[..idx]);
				return write!(f, " \"{}\"", text.escape_debug());
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

pub type SyntaxElement<L> = NodeOrToken<SyntaxNode<L>, SyntaxToken<L>>;

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

impl<L: Language> fmt::Display for SyntaxElement<L> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			NodeOrToken::Node(it) => fmt::Display::fmt(it, f),
			NodeOrToken::Token(it) => fmt::Display::fmt(it, f),
		}
	}
}

impl<L: Language> SyntaxNode<L> {
	pub fn new_root(green: GreenNode) -> SyntaxNode<L> {
		SyntaxNode::from(cursor::SyntaxNode::new_root(green))
	}
	pub fn replace_with(&self, replacement: GreenNode) -> GreenNode {
		self.raw.replace_with(replacement)
	}

	pub fn kind(&self) -> L::Kind {
		L::kind_from_raw(self.raw.kind())
	}

	pub fn text_range(&self) -> TextRange {
		self.raw.text_range()
	}

	pub fn text(&self) -> SyntaxText {
		self.raw.text()
	}

	pub fn green(&self) -> &GreenNode {
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

	pub fn first_child_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.first_child_or_token().map(NodeOrToken::from)
	}

	pub fn last_child(&self) -> Option<SyntaxNode<L>> {
		self.raw.last_child().map(Self::from)
	}

	pub fn last_child_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.last_child_or_token().map(NodeOrToken::from)
	}

	pub fn next_sibling(&self) -> Option<SyntaxNode<L>> {
		self.raw.next_sibling().map(Self::from)
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.next_sibling_or_token().map(NodeOrToken::from)
	}

	pub fn prev_sibling(&self) -> Option<SyntaxNode<L>> {
		self.raw.prev_sibling().map(Self::from)
	}

	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
		self.raw.prev_sibling_or_token().map(NodeOrToken::from)
	}

	pub fn first_token(&self) -> Option<SyntaxToken<L>> {
		self.raw.first_token().map(SyntaxToken::from)
	}

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

	pub fn preorder(&self) -> impl Iterator<Item = WalkEvent<SyntaxNode<L>>> {
		self.raw.preorder().map(|event| event.map(SyntaxNode::from))
	}

	pub fn preorder_with_tokens(&self) -> impl Iterator<Item = WalkEvent<SyntaxElement<L>>> {
		self.raw
			.preorder_with_tokens()
			.map(|event| event.map(NodeOrToken::from))
	}

	pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken<L>> {
		self.raw.token_at_offset(offset).map(SyntaxToken::from)
	}

	pub fn covering_element(&self, range: TextRange) -> SyntaxElement<L> {
		NodeOrToken::from(self.raw.covering_element(range))
	}
}

impl<L: Language> SyntaxToken<L> {
	pub fn replace_with(&self, new_token: GreenToken) -> GreenNode {
		self.raw.replace_with(new_token)
	}

	pub fn kind(&self) -> L::Kind {
		L::kind_from_raw(self.raw.kind())
	}

	pub fn text_range(&self) -> TextRange {
		self.raw.text_range()
	}

	pub fn text(&self) -> &SmolStr {
		self.raw.text()
	}

	pub fn green(&self) -> &GreenToken {
		self.raw.green()
	}

	pub fn parent(&self) -> SyntaxNode<L> {
		SyntaxNode::from(self.raw.parent())
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

	pub fn next_token(&self) -> Option<SyntaxToken<L>> {
		self.raw.next_token().map(SyntaxToken::from)
	}

	pub fn prev_token(&self) -> Option<SyntaxToken<L>> {
		self.raw.prev_token().map(SyntaxToken::from)
	}
}

impl<L: Language> SyntaxElement<L> {
	pub fn text_range(&self) -> TextRange {
		match self {
			NodeOrToken::Node(it) => it.text_range(),
			NodeOrToken::Token(it) => it.text_range(),
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
			NodeOrToken::Token(it) => Some(it.parent()),
		}
	}

	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
		match self {
			NodeOrToken::Node(it) => it.ancestors(),
			NodeOrToken::Token(it) => it.parent().ancestors(),
		}
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
