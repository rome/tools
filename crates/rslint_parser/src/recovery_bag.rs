use crate::{Parser, ParserError, TokenSet};
use rslint_errors::Diagnostic;
use rslint_lexer::{SyntaxKind, T};

/// This struct contains the information needed to the parser to recover from a certain error
///
/// By default it doesn't check curly braces, use [with_braces_included] to turn opt-in the check
#[derive(Debug)]
pub struct RecoveryBag {
	/// The [Diagnostic] to emit
	error: Option<ParserError>,
	/// It tells the parser to recover if the position is inside a set of [tokens](TokenSet)
	recovery: TokenSet,
	/// It tells the parser to recover if the current token is a curly brace
	include_braces: bool,
	/// The kind of the unknown node the parser inserts if it isn't able to recover because
	/// the current token is neither in the recovery set nor any of `{` or `}`.
	unknown_node_kind: SyntaxKind,
}

impl RecoveryBag {
	pub fn new(recovery: TokenSet, unknown_node_kind: SyntaxKind) -> Self {
		Self {
			error: None,
			recovery,
			include_braces: false,
			unknown_node_kind,
		}
	}

	pub fn with_error<Err: Into<ParserError>>(
		recovery: TokenSet,
		unknown_node_kind: SyntaxKind,
		error: Err,
	) -> Self {
		Self {
			error: Some(error.into()),
			recovery,
			include_braces: false,
			unknown_node_kind,
		}
	}

	/// Disable check of curly braces as recovery tokens
	pub fn without_braces_included(mut self) -> Self {
		self.include_braces = false;
		self
	}

	/// Enable check of curly braces as recovery tokens
	pub fn with_braces_included(mut self) -> Self {
		self.include_braces = true;
		self
	}

	/// Checks if the parsing phase is recoverable by checking curly braces and toke set
	pub fn parsing_is_recoverable(&self, parser: &Parser) -> bool {
		self.is_at_token_set(parser) || self.is_at_braces(parser)
	}

	/// It returns the diagnostic
	pub fn get_error(&self) -> Option<Diagnostic> {
		self.error.to_owned()
	}

	/// It return the unknown node kind that will be used to complete the parsing
	pub fn get_unknown_node_kind(&self) -> SyntaxKind {
		self.unknown_node_kind
	}

	fn is_at_braces(&self, parser: &Parser) -> bool {
		matches!(parser.cur(), T!['{'] | T!['}'] if self.include_braces)
	}

	fn is_at_token_set(&self, parser: &Parser) -> bool {
		parser.at_ts(self.recovery)
	}
}
