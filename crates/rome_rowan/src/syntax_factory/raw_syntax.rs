use crate::green::GreenElement;
use crate::{GreenNode, GreenToken, NodeOrToken, SyntaxKind};
use std::marker::PhantomData;

#[derive(Debug)]
pub struct RawSyntaxNode<K: SyntaxKind> {
	raw: GreenNode,
	ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxNode<K> {
	#[inline]
	pub fn new<I>(kind: K, slots: I) -> Self
	where
		I: IntoIterator<Item = Option<RawSyntaxElement<K>>>,
		I::IntoIter: ExactSizeIterator,
	{
		Self {
			raw: GreenNode::new(
				kind.to_raw(),
				slots
					.into_iter()
					.map(|slot| slot.map(|element| element.green())),
			),
			ph: PhantomData,
		}
	}

	#[inline]
	pub fn kind(&self) -> K {
		K::from_raw(self.raw.kind())
	}

	#[inline]
	pub(crate) fn green(self) -> GreenNode {
		self.raw
	}
}

impl<K: SyntaxKind> From<GreenNode> for RawSyntaxNode<K> {
	#[inline]
	fn from(node: GreenNode) -> Self {
		Self {
			raw: node,
			ph: PhantomData,
		}
	}
}

#[derive(Debug)]
pub struct RawSyntaxToken<K: SyntaxKind> {
	raw: GreenToken,
	ph: PhantomData<K>,
}

impl<K: SyntaxKind> RawSyntaxToken<K> {
	#[inline]
	pub fn kind(&self) -> K {
		K::from_raw(self.raw.kind())
	}
}

impl<K: SyntaxKind> From<GreenToken> for RawSyntaxToken<K> {
	fn from(token: GreenToken) -> Self {
		Self {
			raw: token,
			ph: PhantomData,
		}
	}
}

pub type RawSyntaxElement<K> = NodeOrToken<RawSyntaxNode<K>, RawSyntaxToken<K>>;

impl<K: SyntaxKind> RawSyntaxElement<K> {
	#[inline]
	pub fn kind(&self) -> K {
		match self {
			NodeOrToken::Node(node) => node.kind(),
			NodeOrToken::Token(token) => token.kind(),
		}
	}

	#[inline]
	fn green(self) -> GreenElement {
		match self {
			NodeOrToken::Node(node) => NodeOrToken::Node(node.raw),
			NodeOrToken::Token(token) => NodeOrToken::Token(token.raw),
		}
	}
}

impl<K: SyntaxKind> From<GreenElement> for RawSyntaxElement<K> {
	#[inline]
	fn from(element: GreenElement) -> Self {
		match element {
			NodeOrToken::Node(node) => NodeOrToken::Node(RawSyntaxNode::from(node)),
			NodeOrToken::Token(token) => NodeOrToken::Token(RawSyntaxToken::from(token)),
		}
	}
}

#[derive(Debug)]
pub struct RawSyntaxNodeRef<'a, K: SyntaxKind> {
	raw: &'a GreenNode,
	ph: PhantomData<K>,
}

impl<'a, K: SyntaxKind> RawSyntaxNodeRef<'a, K> {
	#[inline]
	pub fn kind(&self) -> K {
		K::from_raw(self.raw.kind())
	}
}

impl<'a, K: SyntaxKind> From<&'a GreenNode> for RawSyntaxNodeRef<'a, K> {
	#[inline]
	fn from(node: &'a GreenNode) -> Self {
		Self {
			raw: node,
			ph: PhantomData,
		}
	}
}

#[derive(Debug)]
pub struct RawSyntaxTokenRef<'a, K: SyntaxKind> {
	raw: &'a GreenToken,
	ph: PhantomData<K>,
}

impl<'a, K: SyntaxKind> RawSyntaxTokenRef<'a, K> {
	#[inline]
	pub fn kind(&self) -> K {
		K::from_raw(self.raw.kind())
	}
}

impl<'a, K: SyntaxKind> From<&'a GreenToken> for RawSyntaxTokenRef<'a, K> {
	#[inline]
	fn from(token: &'a GreenToken) -> Self {
		Self {
			raw: token,
			ph: PhantomData,
		}
	}
}

pub type RawSyntaxElementRef<'a, K> =
	NodeOrToken<RawSyntaxNodeRef<'a, K>, RawSyntaxTokenRef<'a, K>>;

impl<'a, K: SyntaxKind> RawSyntaxElementRef<'a, K> {
	#[inline]
	pub fn kind(&self) -> K {
		match self {
			NodeOrToken::Node(node) => node.kind(),
			NodeOrToken::Token(token) => token.kind(),
		}
	}
}

impl<'a, K: SyntaxKind> From<NodeOrToken<&'a GreenNode, &'a GreenToken>>
	for RawSyntaxElementRef<'a, K>
{
	#[inline]
	fn from(element: NodeOrToken<&'a GreenNode, &'a GreenToken>) -> Self {
		match element {
			NodeOrToken::Node(node) => NodeOrToken::Node(RawSyntaxNodeRef::from(node)),
			NodeOrToken::Token(token) => NodeOrToken::Token(RawSyntaxTokenRef::from(token)),
		}
	}
}

impl<'a, K: SyntaxKind> From<&'a GreenElement> for RawSyntaxElementRef<'a, K> {
	#[inline]
	fn from(element: &'a GreenElement) -> Self {
		match element {
			NodeOrToken::Node(node) => NodeOrToken::Node(RawSyntaxNodeRef::from(node)),
			NodeOrToken::Token(token) => NodeOrToken::Token(RawSyntaxTokenRef::from(token)),
		}
	}
}
