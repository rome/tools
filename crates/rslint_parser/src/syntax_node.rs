//! This module defines the Concrete Syntax Tree used by RSLint.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::SyntaxKind;
use rome_rowan::{Language, TreeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
pub type SyntaxList = rome_rowan::SyntaxList<JsLanguage>;
pub type SyntaxSlots = rome_rowan::SyntaxSlots<JsLanguage>;
pub type SyntaxSlot = rome_rowan::SyntaxSlot<JsLanguage>;

pub use rome_rowan::{Direction, NodeOrToken};

pub type SyntaxTreeBuilder = TreeBuilder<'static, JsLanguage>;
