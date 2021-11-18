//! The Js syntax itself and parser functions.
//!
//! The actual parsing is done in these modules.
//! Every single function is public, this is to allow people to
//! use the parser for their specific needs, for example, parsing
//! only an expression.
//!
//! Functions emit markers, see `CompletedMarker` and `Marker` docs for more info.

mod assignment_target;
mod class;
mod decl;
pub mod expr;
mod function;
mod js_parse_error;
mod object;
mod pat;
pub(crate) mod program;
mod stmt;
mod typescript;
mod util;
