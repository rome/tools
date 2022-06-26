pub(crate) mod event;
pub mod lexer;
pub(crate) mod lossless_tree_sink;
pub(crate) mod parse;
pub(crate) mod parse_error;
pub(crate) mod parse_recovery;
pub(crate) mod parse_syntax;
pub(crate) mod parser;
pub(crate) mod token_set;
pub mod token_source;

pub use parse::parse;
pub(crate) use parser::Parser;

#[cfg(test)]
mod tests;