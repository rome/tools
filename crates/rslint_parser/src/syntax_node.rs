//! This module defines the Concrete Syntax Tree used by RSLint.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::SyntaxKind;
use rome_rowan::{Language, TreeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsLanguage;

impl Language for JsLanguage {
	type Kind = SyntaxKind;

	fn kind_from_raw(raw: rome_rowan::SyntaxKind) -> SyntaxKind {
		SyntaxKind::from(raw.0)
	}

	fn kind_to_raw(kind: SyntaxKind) -> rome_rowan::SyntaxKind {
		rome_rowan::SyntaxKind(kind.into())
	}
}

pub type SyntaxNode = rome_rowan::SyntaxNode<JsLanguage>;
pub type SyntaxToken = rome_rowan::SyntaxToken<JsLanguage>;
pub type SyntaxElement = rome_rowan::SyntaxElement<JsLanguage>;
pub type SyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<JsLanguage>;
pub type SyntaxElementChildren = rome_rowan::SyntaxElementChildren<JsLanguage>;

pub use rome_rowan::{Direction, NodeOrToken};

/// Simple wrapper around a rome_rowan [`GreenNodeBuilder`]
#[derive(Default, Debug)]
pub struct SyntaxTreeBuilder {
	inner: TreeBuilder<'static>,
}

impl SyntaxTreeBuilder {
	pub fn finish(self) -> SyntaxNode {
		self.inner.finish()
	}

	pub fn token(&mut self, kind: SyntaxKind, text: &str) {
		let kind = JsLanguage::kind_to_raw(kind);
		self.inner.token(kind, text)
	}

	pub fn start_node(&mut self, kind: SyntaxKind) {
		let kind = JsLanguage::kind_to_raw(kind);
		self.inner.start_node(kind)
	}

	pub fn finish_node(&mut self) {
		self.inner.finish_node()
	}
}
