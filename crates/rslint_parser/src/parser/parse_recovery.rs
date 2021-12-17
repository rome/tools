use crate::{CompletedMarker, Parser, TokenSet};
use rslint_syntax::JsSyntaxKind;
use rslint_syntax::JsSyntaxKind::EOF;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum RecoveryError {
	/// Recovery failed because the parser reached the end of file
	Eof,

	/// Recovery failed because it didn't eat any tokens. Meaning, the parser is already in a recovered state.
	/// This is an error because:
	/// a) It shouldn't create a completed marker wrapping no tokens
	/// b) This results in an infinite-loop if the recovery is used inside of a while loop. For example,
	///    it's common that list parsing also recovers at the end of a statement or block. However, list elements
	///    don't start with a `;` or `}` which is why parsing, for example, an array element fails again and
	///    the array expression triggers another recovery. Handling this as an error ensures that list parsing
	///    rules break out of the loop the same way as they would at the EOF.
	AlreadyRecovered,
}

impl Error for RecoveryError {}

impl Display for RecoveryError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RecoveryError::Eof => write!(f, "EOF"),
			RecoveryError::AlreadyRecovered => write!(f, "already recovered"),
		}
	}
}

pub type RecoveryResult = Result<CompletedMarker, RecoveryError>;

/// Recovers the parser by finding a token/point (depending on the configuration) from where
/// the caller knows how to proceed parsing. The recovery wraps all the skipped tokens inside of an `Unknown` node.
/// A safe recovery point for an array element could by finding the next `,` or `]`.
pub struct ParseRecovery {
	node_kind: JsSyntaxKind,
	recovery_set: TokenSet,
	line_break: bool,
}

impl ParseRecovery {
	/// Creates a new parse recovery that eats all tokens until it finds any token in the passed recovery set.
	pub fn new(node_kind: JsSyntaxKind, recovery_set: TokenSet) -> Self {
		Self {
			node_kind,
			recovery_set,
			line_break: false,
		}
	}

	/// Enable recovery on line breaks
	pub fn enable_recovery_on_line_break(mut self) -> Self {
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

		if self.recovered(p) {
			return Err(RecoveryError::AlreadyRecovered);
		}

		let m = p.start();

		while !self.recovered(p) {
			p.bump_any();
		}

		Ok(m.complete(p, self.node_kind))
	}

	#[inline]
	fn recovered(&self, p: &Parser) -> bool {
		p.at_ts(self.recovery_set) || p.at(EOF) || (self.line_break && p.has_linebreak_before_n(0))
	}
}
