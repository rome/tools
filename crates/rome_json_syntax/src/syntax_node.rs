//! This module defines the Concrete Syntax Tree used by RSLint.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{JsonSyntaxKind, JsonValue};
use rome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct JsonLanguage;

impl Language for JsonLanguage {
    type Kind = JsonSyntaxKind;
    type Root = JsonValue;
}

pub type JsonSyntaxNode = rome_rowan::SyntaxNode<JsonLanguage>;
pub type JsonSyntaxToken = rome_rowan::SyntaxToken<JsonLanguage>;
pub type JsonSyntaxElement = rome_rowan::SyntaxElement<JsonLanguage>;
pub type JsonSyntaxNodeChildren = rome_rowan::SyntaxNodeChildren<JsonLanguage>;
pub type JsonSyntaxElementChildren = rome_rowan::SyntaxElementChildren<JsonLanguage>;
pub type JsonSyntaxList = rome_rowan::SyntaxList<JsonLanguage>;
