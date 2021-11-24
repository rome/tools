use crate::parser::parse_recovery::{RecoveryError, RecoveryResult};
use crate::parser::ParseRecovery;
use crate::{CompletedMarker, Marker, Parser};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub trait SyntaxFeature: Sized {
	/// Returns [true] if the current parsing context supports this syntax feature
	fn is_available(&self, p: &mut Parser) -> bool;
}

/// Syntax feature that tests if the current scope runs in sloppy / loose mode.
#[doc(alias = "LooseMode")]
pub struct SloppyMode;

impl SyntaxFeature for SloppyMode {
	fn is_available(&self, p: &mut Parser) -> bool {
		p.state.strict.is_none()
	}
}

/// Result returned from a parse function.
/// A parse function should return [Ok] if it is able to parse at least a partial node. For example,
/// the `parse_for_statement` should return ok for `for (` even tough many of the required children are missing
/// because it is still able to parse parts of the for statement.
///
/// Parse rules return [Err] if the complete node is missing. In most cases, this means if the
/// first expected token isn't present, for example if the `for` keyword isn't present when calling
/// `parse_for_statement`. There are other rules that can recover even if the
/// first expected token isn't present. For example, the `parse_assignment_target_with_optional_default`
/// can still parse `[ = "b"] = test` even tough the assignment target before the `=` is missing but
/// it can recover once it sees the `=` token and create a `JsAssignmentTargetWithDefault` that has a
/// missing `assignment_target`.
///
/// The parse rule must rewind the parser if it parsed an incomplete node but can't correctly determine its type
/// to ensure that the caller and do the proper error recovery.
pub type ParseResult = Result<CompletedMarker, ExpectedError>;

impl From<CompletedMarker> for ParseResult {
	fn from(marker: CompletedMarker) -> Self {
		Ok(marker)
	}
}

/// Error that an expected node or a selection of nodes aren't present in the source code
///
/// For example, `ExpectedNodeError::expected_node("statement")` expresses that a statement node was expected but
/// none was found.
///
/// Returned from `parse_` functions if none of its children are present
#[derive(Debug, Clone)]
pub struct ExpectedError {
	message: &'static str,
	primary: Option<&'static str>,
}

impl ExpectedError {
	/// Creates a new error that the parser expected some content that wasn't present.
	/// The passed message is displayed in between: `expected {message} but instead found...`
	pub fn new(message: &'static str) -> Self {
		Self {
			message,
			primary: None,
		}
	}

	/// Overrides the default primary message with a custom one
	pub fn with_primary(mut self, primary: &'static str) -> Self {
		self.primary = Some(primary);
		self
	}

	pub(crate) fn into_diagnostic(self, p: &Parser, span: impl Span) -> Diagnostic {
		let range = &span.as_range();

		let msg = if range.is_empty() && p.tokens.source().get(range.to_owned()) == None {
			format!("expected {} but instead found end of file", self.message)
		} else {
			format!(
				"expected {} but instead found '{}'",
				self.message,
				p.source(span.as_text_range())
			)
		};

		let diag = p.err_builder(&msg);

		if let Some(primary) = self.primary {
			diag.primary(span, primary)
		} else {
			diag.primary(span, format!("Expected {} here", self.message))
		}
	}
}

impl Error for ExpectedError {}

impl Display for ExpectedError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "expected {}", self.message)
	}
}

/// Extension trait for `ParseResult<CompletedMarker>`
pub trait ParsedSyntax {
	/// Makes the node returned by the parse rule a required child and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the parse rule successfully parsed a node (this is `Ok(completed)`)
	///
	/// It adds a missing marker, adds a diagnostic that it expected a node of [ExpectedNodeError], and
	/// returns `None` if the parse rule failed to parse the node (this is `Err`).
	fn make_required(self, p: &mut Parser) -> Option<CompletedMarker>;

	/// Makes the node returned by the parse rule an optional child and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the parse rule successfully parsed a node (this is `Ok(completed)`)
	///
	/// It adds a missing marker and returns `None` if the parse rule failed to parse the node (this is `Err`).
	/// However, it doesn't add an error in that case.
	fn make_optional(self, p: &mut Parser) -> Option<CompletedMarker>;

	/// It creates a new marker that precedes the node returned by the parse rule.
	///
	/// It returns [CompletedMarker.precede] if this [Result] holds a parsed node.
	/// It returns a new marker by calling [p.start()] if this [Result] doesn't contains a parsed node, adds
	/// a missing marker, and a diagnostic that a node was expected but not present.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	fn precede_required(self, p: &mut Parser) -> Marker;

	/// It creates a new marker that precedes the node returned by the parse rule.
	///
	/// It returns [CompletedMarker.precede] if this [Result] holds a parsed node.
	/// It returns a new marker by calling [p.start()] if this [Results] doesn't contain a parsed node
	/// and adds a missing marker. No diagnostic will be added if the node isn't present (since it's optional)
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	fn precede_optional(self, p: &mut Parser) -> Marker;

	/// Returns the node contained by this [Result] or tries to recover by wrapping any
	/// unexpected tokens in an `Unknown*` node until the parser reaches one of the "safe tokens"
	/// configured in the [ParseRecovery], signaling that it successfully recovered.
	///
	/// Returns `RecoveryError::EOF` if the parser is currently positioned at the `EOF` token
	fn or_recover(self, p: &mut Parser, recovery: ParseRecovery) -> RecoveryResult;

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
	fn make_required(self, p: &mut Parser) -> Option<CompletedMarker> {
		match self {
			Ok(syntax) => Some(syntax),
			Err(error) => {
				p.missing();
				let diagnostic = error.into_diagnostic(p, p.cur_tok().range);
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
	fn precede_required(self, p: &mut Parser) -> Marker {
		self.map(|marker| marker.precede(p))
			.unwrap_or_else(|error| {
				let diagnostic = error.into_diagnostic(p, p.cur_tok().range);
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

	fn or_recover(self, p: &mut Parser, recovery: ParseRecovery) -> RecoveryResult {
		match self {
			Ok(syntax) => Ok(syntax),
			Err(error) => match recovery.recover(p) {
				Ok(recovered) => {
					let diagnostic = error.into_diagnostic(p, recovered.range(p));
					p.error(diagnostic);
					Ok(recovered)
				}

				Err(_) => {
					let diagnostic = error.into_diagnostic(p, p.cur_tok().range);
					p.error(diagnostic);
					Err(RecoveryError::Eof)
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
			Err(e) => Err(ConditionalSyntaxError::Expected(e)),
		}
	}

	fn into_unsupported(self) -> ConditionalSyntaxParseResult {
		match self {
			Ok(c) => Err(ConditionalSyntaxError::Unsupported(c)),
			Err(expected) => Err(ConditionalSyntaxError::Expected(expected)),
		}
	}
}

#[derive(Debug)]
pub enum ConditionalSyntaxError {
	/// Expected a node of a specific type that wasn't present
	Expected(ExpectedError),

	/// Parsed node that isn't supported in the current parsing context
	Unsupported(CompletedMarker),
}

impl Error for ConditionalSyntaxError {}

impl From<ExpectedError> for ConditionalSyntaxError {
	fn from(expected: ExpectedError) -> Self {
		ConditionalSyntaxError::Expected(expected)
	}
}

impl Display for ConditionalSyntaxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			ConditionalSyntaxError::Expected(node) => write!(f, "{}", node),
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
			ConditionalSyntaxError::Expected(c) => Err(c),
		})
	}

	fn is_unsupported(&self) -> bool {
		matches!(self, Err(ConditionalSyntaxError::Unsupported(_)))
	}
}
