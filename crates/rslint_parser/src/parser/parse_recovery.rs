use crate::{CompletedMarker, Parser, TokenSet};
use rslint_syntax::SyntaxKind;
use rslint_syntax::SyntaxKind::EOF;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RecoveryError {
	/// Recovery failed because the parser reached the end of file
	Eof,
}

impl Error for RecoveryError {}

impl Display for RecoveryError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RecoveryError::Eof => write!(f, "EOF"),
		}
	}
}

pub type RecoveryResult = Result<CompletedMarker, RecoveryError>;

/// Recovers the parser by finding a token/point (depending on the configuration) from where
/// the caller knows how to proceed parsing. The recovery wraps all the skipped tokens inside of an `Unknown` node.
/// A safe recovery point for an array element could by finding the next `,` or `]`.
pub struct ParseRecovery {
	node_kind: SyntaxKind,
	recovery_set: TokenSet,
	line_break: bool,
}

impl ParseRecovery {
	/// Creates a new parse recovery that eats all tokens until it finds any token in the passed recovery set.
	pub fn new(node_kind: SyntaxKind, recovery_set: TokenSet) -> Self {
		Self {
			node_kind,
			recovery_set,
			line_break: false,
		}
	}

	/// Enable recovery on line breaks
	pub fn with_recovery_on_line_break(mut self) -> Self {
		self.line_break = true;
		self
	}

	// TODO: Add a `recover_until` which recovers until the parser reached a token inside of the recovery set
	// or the passed in `parse_*` rule was able to successfully parse an element.

	/// Tries to recover by parsing all tokens into an `Unknown*` node until the parser finds any token
	/// specified in the recovery set, the EOF, or a line break (depending on configuration).
	/// Returns `Ok(unknown_node)` if recovery was successful, and `Err(RecoveryError::Eof)` if the parser
	/// is at the end of the file (before starting recovery).
	pub fn recover(&self, p: &mut Parser) -> RecoveryResult {
		if p.at(EOF) {
			return Err(RecoveryError::Eof);
		}

		let m = p.start();

		while !(p.at_ts(self.recovery_set)
			|| p.at(EOF) || (self.line_break && p.has_linebreak_before_n(0)))
		{
			p.bump_any();
		}

		Ok(m.complete(p, self.node_kind))
	}
}
