use crate::parser::parse_recovery::RecoveryResult;
use crate::parser::ParseRecovery;
use crate::{CompletedMarker, Marker, Parser, SyntaxFeature};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;

/// Result returned from a parse function.
/// A parse function should return [Ok] if it is able to parse at least a partial node. For example,
/// the `parse_for_statement` should return ok for `for (` even tough many of the required children are missing
/// because it is still able to parse parts of the for statement.
///
/// Parse rules return [Err] if the complete node is absent. In most cases, this means if the
/// first expected token isn't present, for example if the `for` keyword isn't present when calling
/// `parse_for_statement`. There are other rules that can recover even if the
/// first expected token isn't present. For example, the `parse_assignment_target_with_optional_default`
/// can still parse `[ = "b"] = test` even tough the assignment target before the `=` is missing but
/// it can recover once it sees the `=` token and create a `JsAssignmentTargetWithDefault` that has a
/// missing `assignment_target`.
///
/// The parse rule must rewind the parser if it parsed an incomplete node but can't correctly determine its type
/// to ensure that the caller can do a proper error recovery.
pub type ParseResult = Result<CompletedMarker, AbsentError>;

impl From<CompletedMarker> for ParseResult {
	fn from(marker: CompletedMarker) -> Self {
		Ok(marker)
	}
}

/// Error that a node is absent (not there). It doesn't mean that the node was expected on the call side.
///
/// Returned from `parse_` functions if none of its children are present
#[derive(Debug, Clone)]
pub struct AbsentError;

impl Error for AbsentError {}

impl Display for AbsentError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "missing node")
	}
}

/// Extension trait for `ParseResult<CompletedMarker>`
pub trait ParsedSyntax {
	/// Makes the node returned by the parse rule a required child and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the parse rule successfully parsed a node (this is `Ok(completed)`)
	///
	/// It adds a missing marker, creates a diagnostic using the passed in builder and adds it to the parse errors,
	/// and returns `None` if the node is absent (`Err(AbsentError)`)
	fn make_required<E>(self, p: &mut Parser, error_builder: E) -> Option<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic;

	/// Makes the node returned by the parse rule an optional child and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the parse rule successfully parsed a node (this is `Ok(completed)`)
	///
	/// It adds a missing marker and returns `None` if the node is absent.
	fn make_optional(self, p: &mut Parser) -> Option<CompletedMarker>;

	/// It creates a new marker that precedes the node returned by the parse rule.
	///
	/// It returns [CompletedMarker.precede] if this [Result] holds a parsed node.
	/// It returns a new marker by calling [p.start()] if the node is absent, adds
	/// a missing marker, and creates a diagnostic using the passed error builder and adds it to the parser diagnostics.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	fn precede_required<E>(self, p: &mut Parser, error_builder: E) -> Marker
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic;

	/// It creates a new marker that precedes the node returned by the parse rule.
	///
	/// It returns [CompletedMarker.precede] if this [Result] holds a parsed node.
	/// It returns a new marker by calling [p.start()] if the node is absent
	/// and adds a missing marker.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	fn precede_optional(self, p: &mut Parser) -> Marker;

	/// Returns the node contained by this [Result] or recovers the parsing by:
	///
	/// * eating all unexpected tokens into an `Unknown*` node until the parser reaches one
	///   of the "safe tokens" configured in the [ParseRecovery].
	/// * Creating an error using the passed in error builder and adds it to the parsing diagnostics
	///
	/// The error recovery can fail if the parser is located at the EOF token or if the parser
	/// is already at a valid position according to the [ParseRecovery].
	fn or_recover<E>(
		self,
		p: &mut Parser,
		recovery: ParseRecovery,
		error_builder: E,
	) -> RecoveryResult
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic;

	/// Makes this a [ConditionalSyntax] that is only supported if the passed [feature] is available
	/// in the current parsing context.
	///
	/// It adds a diagnostic using the passed in error builder and returns `Ok(unsupported)`
	/// if the [feature] isn't available and the result contains a completed node (`Ok(completed)`).
	///
	/// It returns `Ok(Supported)` if the feature is available and this is `Ok(completed)`, and `Err` otherwise.
	fn requires_syntax_feature<F, E>(
		self,
		p: &mut Parser,
		feature: F,
		error_builder: E,
	) -> ConditionalSyntaxParseResult
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic;

	/// Converts this result into [Unsupported] conditional syntax without adding an error.
	/// Useful if this is a list and any of its elements is an unsupported node because the list
	/// then needs be wrapped as unsupported too but no error should be added (the error on the element is sufficient).
	///
	/// This doesn't apply if the element can be converted into an `Unknown*` node.
	fn into_unsupported(self) -> ConditionalSyntaxParseResult;
}

impl ParsedSyntax for ParseResult {
	fn make_required<E>(self, p: &mut Parser, error_builder: E) -> Option<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Ok(syntax) => Some(syntax),
			Err(_) => {
				p.missing();
				let diagnostic = error_builder(p, p.cur_tok().range);
				p.error(diagnostic);
				None
			}
		}
	}

	fn make_optional(self, p: &mut Parser) -> Option<CompletedMarker> {
		match self {
			Ok(syntax) => Some(syntax),
			Err(_) => {
				p.missing();
				None
			}
		}
	}

	#[must_use]
	fn precede_required<E>(self, p: &mut Parser, error_builder: E) -> Marker
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		self.map(|marker| marker.precede(p)).unwrap_or_else(|_| {
			let diagnostic = error_builder(p, p.cur_tok().range);
			p.error(diagnostic);

			let m = p.start();
			p.missing();
			m
		})
	}

	#[must_use]
	fn precede_optional(self, p: &mut Parser) -> Marker {
		self.map(|marker| marker.precede(p)).unwrap_or_else(|_| {
			let m = p.start();
			p.missing();
			m
		})
	}

	fn or_recover<E>(
		self,
		p: &mut Parser,
		recovery: ParseRecovery,
		error_builder: E,
	) -> RecoveryResult
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Ok(syntax) => Ok(syntax),
			Err(_) => match recovery.recover(p) {
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

	fn requires_syntax_feature<F, E>(
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
			Ok(c) => c.to_conditional_parse_result(p, feature, error_builder),
			Err(_) => Err(ConditionalSyntaxError::Absent),
		}
	}

	fn into_unsupported(self) -> ConditionalSyntaxParseResult {
		match self {
			Ok(c) => Err(ConditionalSyntaxError::Unsupported(c)),
			Err(_) => Err(ConditionalSyntaxError::Absent),
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

impl From<AbsentError> for ConditionalSyntaxError {
	fn from(_: AbsentError) -> Self {
		ConditionalSyntaxError::Absent
	}
}

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
	fn or_unsupported_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParseResult;

	/// Returns true if this result contains a parsed node that isn't supported in this parsing context.
	fn is_unsupported(&self) -> bool;
}

impl ConditionalParsedSyntax for ConditionalSyntaxParseResult {
	fn or_unsupported_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParseResult {
		self.or_else(|err| match err {
			ConditionalSyntaxError::Unsupported(mut completed) => {
				completed.change_kind(p, unknown_kind);
				Ok(completed)
			}
			ConditionalSyntaxError::Absent => Err(AbsentError),
		})
	}

	fn is_unsupported(&self) -> bool {
		matches!(self, Err(ConditionalSyntaxError::Unsupported(_)))
	}
}
