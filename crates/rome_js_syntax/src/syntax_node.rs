//! This module defines the Concrete Syntax Tree used by Rome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{JsSyntaxFactory, JsSyntaxKind};
use rome_rowan::{Language, TreeBuilder};
#[cfg(feature = "serde")]
use serde_crate::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(crate = "serde_crate"))]
pub struct JsLanguage;

impl Language for JsLanguage {
    type Kind = JsSyntaxKind;
}

pub type JsSyntaxNode = rome_rowan::SyntaxNode<JsLanguage>;
pub type JsSyntaxToken = rome_rowan::SyntaxToken<JsLanguage>;
pub type JsSyntaxElement = rome_rowan::SyntaxElement<JsLanguage>;
pub type JsSyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<JsLanguage>;
pub type JsSyntaxElementChildren = rome_rowan::SyntaxElementChildren<JsLanguage>;
pub type JsSyntaxList = rome_rowan::SyntaxList<JsLanguage>;

pub type JsSyntaxTreeBuilder = TreeBuilder<'static, JsLanguage, JsSyntaxFactory>;
