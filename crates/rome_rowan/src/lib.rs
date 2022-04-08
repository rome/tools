//! A generic library for lossless syntax trees.
//! See `examples/s_expressions.rs` for a tutorial.
#![forbid(
    // missing_debug_implementations,
    unconditional_recursion,
    future_incompatible,
    // missing_docs,
)]
#![deny(unsafe_code)]

#[allow(unsafe_code)]
pub mod cursor;
#[allow(unsafe_code)]
mod green;

pub mod syntax;
mod syntax_text;
mod utility_types;

#[allow(unsafe_code)]
mod arc;
mod ast;
mod cow_mut;
pub mod raw_language;
#[cfg(feature = "serde1")]
mod serde_impls;
#[allow(unsafe_code)]
mod sll;
mod syntax_factory;
mod tree_builder;

pub use text_size::{TextLen, TextRange, TextSize};

pub use crate::{
    ast::*,
    green::RawSyntaxKind,
    syntax::{
        Language, SyntaxElement, SyntaxElementChildren, SyntaxKind, SyntaxList, SyntaxNode,
        SyntaxNodeChildren, SyntaxToken, SyntaxTriviaPiece, SyntaxTriviaPieceComments, TriviaPiece,
        TriviaPieceKind,
    },
    syntax_factory::*,
    syntax_text::SyntaxText,
    tree_builder::{Checkpoint, TreeBuilder},
    utility_types::{Direction, NodeOrToken, TokenAtOffset, WalkEvent},
};

pub(crate) use crate::green::{GreenNode, GreenNodeData, GreenToken, GreenTokenData};
