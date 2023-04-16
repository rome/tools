//! This module defines the Concrete Syntax Tree used by Rome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{AnyMdElement, MdSyntaxKind};
use rome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MdLanguage;

impl Language for MdLanguage {
    type Kind = MdSyntaxKind;
    type Root = AnyMdElement;
}

pub type MdSyntaxNode = rome_rowan::SyntaxNode<MdLanguage>;
pub type MdSyntaxToken = rome_rowan::SyntaxToken<MdLanguage>;
pub type MdSyntaxElement = rome_rowan::SyntaxElement<MdLanguage>;
pub type MdSyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<MdLanguage>;
pub type MdSyntaxElementChildren = rome_rowan::SyntaxElementChildren<MdLanguage>;
pub type MdSyntaxList = rome_rowan::SyntaxList<MdLanguage>;
