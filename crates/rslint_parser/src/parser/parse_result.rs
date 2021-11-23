use crate::parser::parse_recovery::{RecoveryError, RecoveryResult};
use crate::parser::parse_result::ConditionalSyntax::{Supported, Unsupported};
use crate::parser::ParseRecovery;
use crate::{CompletedMarker, Marker, Parser};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait SyntaxFeature: Sized {
	/// Returns [true] if the current parsing context supports this syntax feature
	fn is_available(&self, p: &mut Parser) -> bool;
}

/// Syntax feature that tests if the current scope runs in sloppy / loose mode.
#[doc(alias = "LooseMode")]
pub struct SloppyMode {}

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
/// `parse_for_statement`. However, there are situations where rules can recover even if the
/// first expected token isn't present. For example, the `parse_assignment_target_with_optional_default`
/// can still parse `[ = "b"] = test` even tough the assignment target before the `=` is missing but
/// it can recover once it sees the `=` token and create a `JsAssignmentTargetWithDefault` that has a
/// missing `assignment_target`.
pub type ParseResult<T> = Result<T, ExpectedNodeError>;

impl From<CompletedMarker> for ParseResult<CompletedMarker> {
	fn from(marker: CompletedMarker) -> Self {
		Ok(marker)
	}
}

/// Expected a node with the given name but it wasn't present at the current parser position.
///
/// For example, `ExpectedNodeError::new("statement")` express that a statement node was expected but
/// none was found.
///
/// Returned from `parse_` functions if none of its children are present
#[derive(Debug, Clone)]
pub struct ExpectedNodeError {
	name: String,
}

impl ExpectedNodeError {
	pub fn new<S: Into<String>>(name: S) -> Self {
		ExpectedNodeError { name: name.into() }
	}

	fn subject(&self) -> String {
		let article = match self.name.chars().next() {
			Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
			_ => "a",
		};

		format!("{} {}", article, self.name)
	}

	// TODO: Probably better to extract this into an ErrorBuilder that provides different helpers
	// to build common errors like expected_token, expected_node, and so on. Could also provide a more
	// ergonomic API like ErrorBuilder.expected(name).with_span(span).build() or ErrorBuilder.expected(name).but_eof().build();
	pub(crate) fn into_diagnostic(self, p: &Parser, span: impl Span) -> Diagnostic {
		let range = &span.as_range();
		let msg = if range.is_empty() && p.tokens.source().get(range.to_owned()) == None {
			format!("expected {} but instead found end of file", self.subject(),)
		} else {
			format!(
				"expected {} but instead found '{}'",
				self.subject(),
				p.source(span.as_text_range())
			)
		};

		p.err_builder(&msg).primary(span, "")
	}
}

impl Error for ExpectedNodeError {}

impl Display for ExpectedNodeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Expected {}", self.subject())
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
	fn required(self, p: &mut Parser) -> Option<CompletedMarker>;

	/// Makes the node returned by the parse rule an optional child and returns the completed marker as an [Option].
	///
	/// It returns `Some(completed)` if the parse rule successfully parsed a node (this is `Ok(completed)`)
	///
	/// It adds a missing marker and returns `None` if the parse rule failed to parse the node (this is `Err`).
	/// However, it doesn't add an error in that case.
	fn optional(self, p: &mut Parser) -> Option<CompletedMarker>;

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
	) -> ParseResult<ConditionalSyntax>
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic;

	/// Converts this result into [Unsupported] conditional syntax without adding an error.
	/// Useful if this is a list and any of its elements is an unsupported node because the list
	/// then needs be wrapped as unsupported too but no error should be added (the error on the element is sufficient).
	///
	/// This doesn't apply if the element can be converted into an `Unknown*` node.
	fn into_unsupported(self) -> ParseResult<ConditionalSyntax>;
}

impl ParsedSyntax for ParseResult<CompletedMarker> {
	fn required(self, p: &mut Parser) -> Option<CompletedMarker> {
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

	fn optional(self, p: &mut Parser) -> Option<CompletedMarker> {
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

				Err(RecoveryError::Eof) => {
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
	) -> ParseResult<ConditionalSyntax>
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		self.map(|marker| {
			ConditionalSyntax::requires_syntax_feature(marker, p, feature, error_builder)
		})
	}

	fn into_unsupported(self) -> ParseResult<ConditionalSyntax> {
		self.map(Unsupported)
	}
}

/// Syntax that can be parsed but whatever its supported depends if certain language features are
/// supported in the current parsing context.
///
/// Some examples:
/// * Only TypeScript files support TypeScript syntax
/// * Only loose mode supports `with` statements
/// * Only modules support `import`/`export` statements
/// * `super()` is only supported inside constructors
#[must_use = "this `ConditionalSyntax` may be an `Unsupported` variant, which should be handled"]
pub enum ConditionalSyntax {
	/// Parsed node that isn't supported in the current parsing context
	Unsupported(CompletedMarker),

	/// Parsed node that is supported in the current parsing context
	Supported(CompletedMarker),
}

impl ConditionalSyntax {
	/// Returns `Supported` if the [feature] is available in the current parsing context.
	/// Adds a diagnostic and returns `Unsupported` otherwise.
	pub fn requires_syntax_feature<F, E>(
		completed: CompletedMarker,
		p: &mut Parser,
		feature: F,
		error_builder: E,
	) -> ConditionalSyntax
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		if feature.is_available(p) {
			Supported(completed)
		} else {
			let diagnostic = error_builder(p, &completed);
			p.error(diagnostic);
			Unsupported(completed)
		}
	}

	#[allow(unused)]
	/// Returns [true] if the syntax is [Supported]
	pub fn is_supported(&self) -> bool {
		matches!(self, Supported(_))
	}

	#[allow(unused)]
	/// Returns [true] if the syntax is [Unsupported]
	pub fn is_unsupported(&self) -> bool {
		matches!(self, Unsupported(_))
	}

	/// Returns the contained [Supported] completed marker or changes the kind of the [Unsupported] marker
	/// to the passed unknown kind before returning it.
	pub fn unwrap_or_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> CompletedMarker {
		match self {
			Supported(m) => m,
			Unsupported(mut marker) => {
				marker.change_kind(p, unknown_kind);
				// TODO throw away all nodes and only keep tokens? Roslyn keeps what it was able to parse
				marker
			}
		}
	}
}

pub trait ConditionalParsedSyntax {
	/// Converts this to a [ParsedSyntax] by converting any contained `Unsupported` node to the passed
	/// in `unknown_kind`.
	fn map_unsupported_to_unknown(
		self,
		p: &mut Parser,
		unknown_kind: SyntaxKind,
	) -> ParseResult<CompletedMarker>;

	/// Returns true if this result contains no parsed node or if the node is supported in this parsing context.
	fn is_supported(&self) -> bool;

	/// Returns true if this result contains a parsed node that isn't supported in this parsing context.
	fn is_unsupported(&self) -> bool;
}

impl ConditionalParsedSyntax for ParseResult<ConditionalSyntax> {
	fn map_unsupported_to_unknown(
		self,
		p: &mut Parser,
		unknown_kind: SyntaxKind,
	) -> ParseResult<CompletedMarker> {
		self.map(|syntax| syntax.unwrap_or_unknown(p, unknown_kind))
	}

	fn is_supported(&self) -> bool {
		matches!(self, Err(_) | Ok(Supported(_)))
	}

	fn is_unsupported(&self) -> bool {
		matches!(self, Ok(Unsupported(_)))
	}
}
