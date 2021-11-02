//! AST definitions for converting untyped syntax nodes into typed AST nodes.
//!
//! Every field of every AST node is optional, this is to allow the parser to recover
//! from any error and produce an ast from any source code. If you don't want to account for
//! optionals for everything, you can use ...

#[macro_use]
mod expr_ext;
mod generated;
mod stmt_ext;
mod ts_ext;

use crate::{syntax_node::*, util::SyntaxNodeExt, SyntaxKind, SyntaxList, TextRange};
use std::iter::FusedIterator;
use std::marker::PhantomData;

pub use self::{
	expr_ext::*,
	generated::{nodes::*, tokens::*},
	stmt_ext::*,
	ts_ext::*,
};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
	fn can_cast(kind: SyntaxKind) -> bool
	where
		Self: Sized;

	fn cast(syntax: SyntaxNode) -> Option<Self>
	where
		Self: Sized;

	fn syntax(&self) -> &SyntaxNode;

	fn text(&self) -> std::string::String {
		self.syntax().trimmed_text().to_string()
	}

	fn range(&self) -> TextRange {
		self.syntax().trimmed_range()
	}
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
	fn can_cast(token: SyntaxKind) -> bool
	where
		Self: Sized;

	fn cast(syntax: SyntaxToken) -> Option<Self>
	where
		Self: Sized;

	fn syntax(&self) -> &SyntaxToken;

	fn text(&self) -> &str {
		self.syntax().text()
	}
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
	inner: SyntaxNodeChildren,
	ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
	fn new(parent: &SyntaxNode) -> Self {
		AstChildren {
			inner: parent.children(),
			ph: PhantomData,
		}
	}
}

impl<N: AstNode> Iterator for AstChildren<N> {
	type Item = N;
	fn next(&mut self) -> Option<N> {
		self.inner.find_map(N::cast)
	}
}

/// List of homogenous nodes
#[derive(Debug, Clone)]
pub struct AstNodeList<N> {
	inner: SyntaxList,
	ph: PhantomData<N>,
}

impl<N> Default for AstNodeList<N> {
	fn default() -> Self {
		AstNodeList {
			inner: SyntaxList::default(),
			ph: PhantomData,
		}
	}
}

impl<N: AstNode> AstNodeList<N> {
	/// Creates a new node list wrapping the passed in syntax list
	fn new(list: SyntaxList) -> Self {
		AstNodeList {
			inner: list,
			ph: PhantomData,
		}
	}

	pub fn iter(&self) -> AstNodeListIterator<N> {
		AstNodeListIterator {
			inner: self.inner.iter(),
			ph: PhantomData,
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// Returns the first node from this list or None
	#[inline]
	pub fn first(&self) -> Option<N> {
		// TODO 1724: Use inner once trivia is attached to tokens (not safe yet)
		self.iter().next()
	}

	/// Returns the last node from this list or None
	pub fn last(&self) -> Option<N> {
		// TODO 1724: Use inner once trivia is attached to tokens (not safe yet)
		self.iter().last()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct AstNodeListIterator<N> {
	inner: SyntaxElementChildren,
	ph: PhantomData<N>,
}

impl<N: AstNode> Iterator for AstNodeListIterator<N> {
	type Item = N;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO 1724: Replace find map with force cast element into `N`.
		// The code gen guarantees us that all elements must be of type N
		self.inner.find_map(|e| {
			let syntax = e.into_node()?;

			if syntax.kind() == SyntaxKind::ERROR {
				None
			} else {
				Some(syntax.to::<N>())
			}
		})
	}
}

impl<N: AstNode> FusedIterator for AstNodeListIterator<N> {}

impl<N: AstNode> IntoIterator for &AstNodeList<N> {
	type Item = N;
	type IntoIter = AstNodeListIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<N: AstNode> IntoIterator for AstNodeList<N> {
	type Item = N;
	type IntoIter = AstNodeListIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

mod support {
	use super::{AstNode, AstNodeList, SyntaxKind, SyntaxNode, SyntaxToken};
	use crate::ast::AstChildren;
	use crate::SyntaxList;

	pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
		parent.children().find_map(N::cast)
	}

	pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
		AstChildren::new(parent)
	}

	fn nth_syntax_list(parent: &SyntaxNode, index: usize) -> SyntaxList {
		// TODO 1724 change parser to insert a missing for empty lists. Gracefully handle this here.
		parent
			.children()
			.filter_map(|node| node.into_list())
			.nth(index)
			.unwrap_or_default()
	}

	pub(super) fn node_list<N: AstNode>(parent: &SyntaxNode, index: usize) -> AstNodeList<N> {
		AstNodeList::new(nth_syntax_list(parent, index))
	}

	pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
	}

	pub(super) fn find_token(
		parent: &SyntaxNode,
		possible_kinds: &[SyntaxKind],
	) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| {
				possible_kinds
					.iter()
					.any(|possible_kind| *possible_kind == it.kind())
			})
	}
}
