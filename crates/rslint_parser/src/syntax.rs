//! The Js syntax itself and parser functions.
//!
//! The actual parsing is done in these modules.
//! Every single function is public, this is to allow people to
//! use the parser for their specific needs, for example, parsing
//! only an expression.
//!
//! Functions emit markers, see `CompletedMarker` and `Marker` docs for more info.

pub mod decl;
pub mod expr;
pub mod pat;
pub mod program;
pub mod stmt;
pub mod typescript;
pub mod util;
