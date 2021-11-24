use crate::{Parser, ParserError, TokenSet};
use rslint_errors::Diagnostic;
use rslint_lexer::{SyntaxKind, T};

/// This struct contains the information needed to the parser to recover from a certain error
///
/// By default it doesn't check curly braces, use [with_braces_included] to turn opt-in the check
#[derive(Debug)]
pub struct SingleTokenParseRecovery {
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

impl SingleTokenParseRecovery {
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

	/// Enable check of curly braces as recovery tokens
	pub fn enabled_braces_check(mut self) -> Self {
		self.include_braces = true;
		self
	}

	/// The main function that tells to the parser how to recover itself.
	///
	/// Recover from an error with a [recovery set](TokenSet) or by using a `{` or `}`.
	///
	/// If [ParserRecoverer] has an error, it gets tracked in the events.
	pub fn recover(&self, p: &mut Parser) {
		let error = self.get_error();
		if let Some(error) = error {
			p.error(error);
		} else {
			// the check on state should be done only when there's no error
			if p.state.no_recovery {
				return;
			}
		}
		if !self.parsing_is_recoverable(p) {
			let m = p.start();
			p.bump_any();
			m.complete(p, self.get_unknown_node_kind());
		}
	}

	/// Checks if the parsing phase is recoverable by checking curly braces and [tokens set](TokenSet)
	fn parsing_is_recoverable(&self, parser: &Parser) -> bool {
		self.is_at_token_set(parser) || self.is_at_braces(parser) || self.is_at_eof(parser)
	}

	/// It returns the diagnostic
	fn get_error(&self) -> Option<Diagnostic> {
		self.error.to_owned()
	}

	/// It returns the unknown node kind that will be used to complete the parsing
	fn get_unknown_node_kind(&self) -> SyntaxKind {
		self.unknown_node_kind
	}

	fn is_at_braces(&self, parser: &Parser) -> bool {
		matches!(parser.cur(), T!['{'] | T!['}'] if self.include_braces)
	}

	fn is_at_token_set(&self, parser: &Parser) -> bool {
		parser.at_ts(self.recovery)
	}

	fn is_at_eof(&self, parser: &Parser) -> bool {
		parser.cur() == SyntaxKind::EOF
	}
}
