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

use crate::{syntax_node::*, util::SyntaxNodeExt, SyntaxKind, TextRange};
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

mod support {
	use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

	pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
		parent.children().find_map(N::cast)
	}

	pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
		AstChildren::new(parent)
	}

	pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
	}
}
