use crate::parser::parse_recovery::RecoveryResult;
use crate::parser::ParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Marker, Parser, SyntaxFeature};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

/// Result of a parse function.
///
/// A parse rule should return [Present] if it is able to parse a node or at least parts of it. For example,
/// the `parse_for_statement` should return [Present] for `for (` even tough many of the required children are missing
/// because it is still able to parse parts of the for statement.
///
/// A parse rule must return [Absent] if the expected node isn't present in the source code.
/// In most cases, this means if the first expected token isn't present, for example,
/// if the `for` keyword isn't present when parsing a for statement.
/// However, it can be possible for rules to recover even if the first token doesn't match. One example
/// is when parsing an assignment target that has an optional default. The rule can recover even
/// if the assignment target is missing as long as the cursor is then positioned at an `=` token.
/// The rule must then return [Present] with the partial parsed node.
///
/// A parse rule must rewind the parser and return [Absent] if it started parsing an incomplete node but
/// in the end can't determine its type to ensure that the caller can do a proper error recovery.
///
/// This is a custom enum over using `Option` because [Absent] values must be handled by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "this `ParsedSyntax` may be an `Absent` variant, which should be handled"]
pub enum ParsedSyntax {
	/// A syntax that isn't present in the source code. Used when a parse rule can't match the current
	/// token of the parser.
	Absent,

	/// A completed syntax node with all or some of its children.
	Present(CompletedMarker),
}

impl ParsedSyntax {
	/// Converts from `ParsedSyntax` to `Option<CompletedMarker>`.
	///
	/// Converts `self` into an `Option<CompletedMarker>`, consuming `self`
	pub fn ok(self) -> Option<CompletedMarker> {
		match self {
			Absent => None,
			Present(marker) => Some(marker),
		}
	}

	/// Returns `true` if the parsed syntax is [Present]
	#[must_use]
	pub fn is_present(&self) -> bool {
		matches!(self, Present(_))
	}

	/// Returns `true` if the parsed syntax is [Absent]
	#[must_use]
	pub fn is_absent(&self) -> bool {
		matches!(self, Absent)
	}

	/// Makes this a required syntax and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the syntax is present in the source text, and otherwise
	///
	/// * adds a `missing` marker to the parent that this child isn't present.
	/// * creates a diagnostic with the passed in error builder and adds it to the parse errors
	/// * returns [None]
	pub fn make_required<E>(self, p: &mut Parser, error_builder: E) -> Option<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Present(syntax) => Some(syntax),
			Absent => {
				p.missing();
				let diagnostic = error_builder(p, p.cur_tok().range);
				p.error(diagnostic);
				None
			}
		}
	}

	/// Makes this an optional syntax and returns the completed marker asn an [Option].
	///
	/// It returns `Some(completed)` if the syntax is present in the source text.
	///
	/// It adds a `missing` marker to the parent to indicate that this child isn't present and returns
	/// [None] if the node is absent.
	pub fn make_optional(self, p: &mut Parser) -> Option<CompletedMarker> {
		match self {
			Present(syntax) => Some(syntax),
			Absent => {
				p.missing();
				None
			}
		}
	}

	/// It makes this syntax a required child of its parent and creates a new marker that precedes this syntax.
	///
	/// It returns [CompletedMarker.precede] if this syntax is present in the source text.
	///
	/// It...
	/// * creates a new marker
	/// * marks this syntax as missing
	/// * creates a diagnostic with the passed error builder and adds it to the parser diagnostics
	/// * and returns the new marker
	///
	/// if the syntax is absent in the source text.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	pub fn precede_required<E>(self, p: &mut Parser, error_builder: E) -> Marker
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Present(completed) => completed.precede(p),
			Absent => {
				let diagnostic = error_builder(p, p.cur_tok().range);
				p.error(diagnostic);

				let m = p.start();
				p.missing();
				m
			}
		}
	}

	/// It makes this syntax an optional child of its parent and creates a new marker that precedes this syntax.
	///
	/// It returns [CompletedMarker.precede] if this syntax is present in the source text.
	///
	/// It creates and returns a new marker and marks this syntax as missing if the syntax is
	/// absent in the source text.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	pub fn precede_optional(self, p: &mut Parser) -> Marker {
		match self {
			Present(completed) => completed.precede(p),
			Absent => {
				let m = p.start();
				p.missing();
				m
			}
		}
	}

	/// Returns this Syntax if it is present in the source text or tries to recover the
	/// parser if the syntax is absent. The recovery...
	///
	/// * eats all unexpected tokens into an `Unknown*` node until the parser reaches one
	///   of the "safe tokens" configured in the [ParseRecovery].
	/// * creates an error using the passed in error builder and adds it to the parsing diagnostics.
	///
	/// The error recovery can fail if the parser is located at the EOF token or if the parser
	/// is already at a valid position according to the [ParseRecovery].
	pub fn or_recover<E>(
		self,
		p: &mut Parser,
		recovery: ParseRecovery,
		error_builder: E,
	) -> RecoveryResult
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Present(syntax) => Ok(syntax),
			Absent => match recovery.recover(p) {
				Ok(recovered) => {
					let diagnostic = error_builder(p, recovered.range(p).as_range());
					p.error(diagnostic);
					Ok(recovered)
				}

				Err(recovery_error) => {
					let diagnostic = error_builder(p, p.cur_tok().range);
					p.error(diagnostic);
					Err(recovery_error)
				}
			},
		}
	}

	/// Makes this a [ConditionalSyntax] that is only supported if the passed [feature] is available
	/// in the current parsing context.
	///
	/// It adds a diagnostic using the passed in error builder and returns `Ok(unsupported)`
	/// if the [feature] isn't available and the result contains a completed node (`Ok(completed)`).
	///
	/// It returns `Ok(Supported)` if the feature is available and this is `Ok(completed)`, and `Err` otherwise.
	pub fn requires_syntax_feature<F, E>(
		self,
		p: &mut Parser,
		feature: F,
		error_builder: E,
	) -> ConditionalSyntaxParseResult
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		match self {
			Present(c) => c.to_conditional_parse_result(p, feature, error_builder),
			Absent => Err(ConditionalSyntaxError::Absent),
		}
	}

	/// Converts this result into [Unsupported] conditional syntax without adding an error.
	/// Useful if this is a list and any of its elements is an unsupported node because the list
	/// then needs be wrapped as unsupported too but no error should be added (the error on the element is sufficient).
	///
	/// This doesn't apply if the element can be converted into an `Unknown*` node.
	pub fn into_unsupported(self) -> ConditionalSyntaxParseResult {
		match self {
			Present(c) => Err(ConditionalSyntaxError::Unsupported(c)),
			Absent => Err(ConditionalSyntaxError::Absent),
		}
	}
}

impl From<CompletedMarker> for ParsedSyntax {
	fn from(marker: CompletedMarker) -> Self {
		Present(marker)
	}
}

impl From<Option<CompletedMarker>> for ParsedSyntax {
	fn from(option: Option<CompletedMarker>) -> Self {
		match option {
			Some(completed) => Present(completed),
			None => Absent,
		}
	}
}

#[derive(Debug)]
pub enum ConditionalSyntaxError {
	/// The node isn't present in the source text. Same as [AbsentError]
	Absent,

	/// Parsed node that isn't supported in the current parsing context
	Unsupported(CompletedMarker),
}

impl Error for ConditionalSyntaxError {}

impl Display for ConditionalSyntaxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ConditionalSyntaxError::Absent => write!(f, "absent"),
			ConditionalSyntaxError::Unsupported(c) => {
				write!(f, "unsupported syntax {:?}", c.kind())
			}
		}
	}
}

/// Parse result for a parsing rule where the parsed syntax depends if a language feature is available
/// or not. This may be because
/// * Syntax is only available in strict or sloppy mode: For example, with statements
/// * Syntax support depends on the file type: Typescript, JSX, Import / Export statements
/// * Syntax depends on a newer language version: experimental features, private field existence test
///
/// See [CompletedMarker.requires_syntax_feature] that creates an `Err(Unsupported)` if the feature
/// isn't available in the current parsing context.
pub type ConditionalSyntaxParseResult = Result<CompletedMarker, ConditionalSyntaxError>;

pub trait ConditionalParsedSyntax {
	/// Converts this to a [ParsedSyntax] by converting any contained `Unsupported` node to the passed
	/// in `unknown_kind`.
	fn or_unsupported_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParsedSyntax;

	/// Returns true if this result contains a parsed node that isn't supported in this parsing context.
	fn is_unsupported(&self) -> bool;
}

impl ConditionalParsedSyntax for ConditionalSyntaxParseResult {
	fn or_unsupported_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParsedSyntax {
		match self {
			Ok(completed) => Present(completed),
			Err(ConditionalSyntaxError::Unsupported(mut completed)) => {
				completed.change_kind(p, unknown_kind);
				Present(completed)
			}
			Err(ConditionalSyntaxError::Absent) => Absent,
		}
	}

	fn is_unsupported(&self) -> bool {
		matches!(self, Err(ConditionalSyntaxError::Unsupported(_)))
	}
}
