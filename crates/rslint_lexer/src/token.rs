//! Token definitions for the lexer

use crate::JsSyntaxKind;

/// A single raw token such as `>>` or `||` or `"abc"`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
	/// The kind of token this is.
	pub kind: JsSyntaxKind,
	/// How long the token is in bytes. For tokens with escape sequences
	/// like strings with `\uXXXX` escapes, the length is the raw length, not considering the char backed by the escape.
	pub len: usize,
}

impl Token {
	/// Create a new token which has an exact length of 1.
	pub fn single(kind: JsSyntaxKind) -> Self {
		Self { kind, len: 1 }
	}

	/// Create a new token which has a specific length.
	pub fn new(kind: JsSyntaxKind, len: usize) -> Self {
		Self { kind, len }
	}
}

macro_rules! tok {
	($tok:tt) => {
		(Token::new(T![$tok], stringify!($tok).len()), None)
	};
	($tok:ident, $len:expr) => {
		(Token::new(JsSyntaxKind::$tok, $len), None)
	};
}
