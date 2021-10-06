#![forbid(
    // missing_debug_implementations,
    unconditional_recursion,
    future_incompatible,
    // missing_docs,
)]
#![deny(unsafe_code)]
// this is ~~stolen~~ borrowed from servo_arc so a lot of this is just legacy code
#![allow(clippy::all, warnings, unsafe_code)]
mod arc;
#[allow(unsafe_code)]
pub mod cursor;
#[allow(unsafe_code)]
mod green;

pub mod api;
#[cfg(feature = "serde1")]
mod serde_impls;
mod syntax_text;
mod utility_types;

// Reexport types for working with strings. We might be too opinionated about
// these, as a custom interner might work better, but `SmolStr` is a pretty good
// default.
pub use smol_str::SmolStr;
pub use text_size::{TextLen, TextRange, TextSize};

pub use crate::{
	api::{
		Language, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
	},
	green::{Checkpoint, Children, GreenNode, GreenNodeBuilder, GreenToken, SyntaxKind},
	syntax_text::SyntaxText,
	utility_types::{Direction, NodeOrToken, TokenAtOffset, WalkEvent},
};
