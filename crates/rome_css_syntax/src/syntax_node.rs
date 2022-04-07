//! This module defines the Concrete Syntax Tree used by RSLint.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{CssSyntaxFactory, CssSyntaxKind};
use rome_rowan::{Language, TreeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CssLanguage;

impl Language for CssLanguage {
    type Kind = CssSyntaxKind;
}

pub type SyntaxNode = rome_rowan::SyntaxNode<CssLanguage>;
pub type SyntaxToken = rome_rowan::SyntaxToken<CssLanguage>;
pub type SyntaxElement = rome_rowan::SyntaxElement<CssLanguage>;
pub type SyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<CssLanguage>;
pub type SyntaxElementChildren = rome_rowan::SyntaxElementChildren<CssLanguage>;
pub type SyntaxList = rome_rowan::SyntaxList<CssLanguage>;
pub type SyntaxSlots = rome_rowan::SyntaxSlots<CssLanguage>;
pub type SyntaxSlot = rome_rowan::SyntaxSlot<CssLanguage>;

pub use rome_rowan::{Direction, NodeOrToken};

pub type SyntaxTreeBuilder = TreeBuilder<'static, CssLanguage, CssSyntaxFactory>;
