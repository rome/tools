//! This module defines the Concrete Syntax Tree used by RSLint.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{SmolStr, SyntaxKind};
use rslint_rowan::{GreenNodeBuilder, Language};

pub use rslint_rowan::GreenNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JsLanguage;

impl Language for JsLanguage {
	type Kind = SyntaxKind;

	fn kind_from_raw(raw: rslint_rowan::SyntaxKind) -> SyntaxKind {
		SyntaxKind::from(raw.0)
	}

	fn kind_to_raw(kind: SyntaxKind) -> rslint_rowan::SyntaxKind {
		rslint_rowan::SyntaxKind(kind.into())
	}
}

pub type SyntaxNode = rslint_rowan::SyntaxNode<JsLanguage>;
pub type SyntaxToken = rslint_rowan::SyntaxToken<JsLanguage>;
pub type SyntaxElement = rslint_rowan::SyntaxElement<JsLanguage>;
pub type SyntaxNodeChildren = rslint_rowan::SyntaxNodeChildren<JsLanguage>;
pub type SyntaxElementChildren = rslint_rowan::SyntaxElementChildren<JsLanguage>;

pub use rslint_rowan::{Direction, NodeOrToken};

/// Simple wrapper around a rslint_rowan [`GreenNodeBuilder`]
#[derive(Default, Debug)]
pub struct SyntaxTreeBuilder {
	inner: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
	pub fn finish(self) -> GreenNode {
		self.inner.finish()
	}

	pub fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
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
