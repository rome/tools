use crate::{Parser, ParserError, TokenSet};
use rslint_errors::Diagnostic;
use rslint_lexer::{SyntaxKind, T};

/// This struct contains the information needed to the parser to recover from a certain error
pub struct RecoveryBag {
	/// The [Diagnostic] to emit
	error: Option<ParserError>,
	/// It tells the parser to recover if the position is inside a set [tokens](TokenSet)
	recovery: TokenSet,
	/// It tells the parser to recover if the current token is a curly brace
	include_braces: bool,
	/// The kind of the unknown node the parser inserts if it isn't able to recover because
	/// the current token is neither in the recovery set nor any of `{` or `}`.
	mysterious_node: SyntaxKind,
}

impl RecoveryBag {
	pub fn new(recovery: TokenSet, include_braces: bool, mysterious_node: SyntaxKind) -> Self {
		Self {
			error: None,
			recovery,
			include_braces,
			mysterious_node,
		}
	}

	pub fn with_error<Err: Into<ParserError>>(
		recovery: TokenSet,
		include_braces: bool,
		mysterious_node: SyntaxKind,
		error: Err,
	) -> Self {
		Self {
			error: Some(error.into()),
			recovery,
			include_braces,
			mysterious_node,
		}
	}

	pub fn has_braces(&self, parser: &Parser) -> bool {
		match parser.cur() {
			T!['{'] | T!['}'] if self.include_braces => {
				return true;
			}
			_ => return false,
		}
	}

	pub fn is_at_token_set(&self, parser: &Parser) -> bool {
		parser.at_ts(self.recovery)
	}

	pub fn get_error(&self) -> Option<Diagnostic> {
		self.error.to_owned()
	}

	pub fn get_mysterious_node(&self) -> SyntaxKind {
		self.mysterious_node
	}
}
