use crate::parser::parse_recovery::RecoveryResult;
use crate::parser::ConditionalParsedSyntax::{Invalid, Valid};
use crate::parser::ParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Marker, Parser, SyntaxFeature};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
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

	/// It returns the syntax if present or adds a missing marker and a diagnostic at the current parser position.
	pub fn or_missing_with_error<E>(
		self,
		p: &mut Parser,
		error_builder: E,
	) -> Option<CompletedMarker>
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

	/// It returns the syntax if present or adds a missing marker.
	pub fn or_missing(self, p: &mut Parser) -> Option<CompletedMarker> {
		match self {
			Present(syntax) => Some(syntax),
			Absent => {
				p.missing();
				None
			}
		}
	}

	/// It returns a [CompletedMarker] from the current syntax
	///
	/// # Panics
	///
	/// The current syntax is [ParsedSyntax::Absent]
	pub fn unwrap(self) -> CompletedMarker {
		match self {
			Absent => {
				panic!("Can't call unwrap on a missing marker");
			}
			Present(marker) => marker,
		}
	}

	/// It creates and returns a marker preceding this parsed syntax if it is present or starts
	/// a new marker, marks the first slot as missing and adds an error to the current parser position.
	/// See [CompletedMarker.precede]
	#[must_use]
	pub fn precede_or_missing_with_error<E>(self, p: &mut Parser, error_builder: E) -> Marker
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

	/// It creates and returns a marker preceding this parsed syntax if it is present or starts a new marker
	/// and marks its first slot as missing.
	///
	/// See [CompletedMarker.precede]
	#[must_use]
	pub fn precede_or_missing(self, p: &mut Parser) -> Marker {
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

	/// Restricts this parsed syntax to only be valid if the current parsing context supports the passed in language feature
	/// and adds a diagnostic if not.
	///
	/// Returns [Valid] if the parsing context supports the passed syntax feature.
	///
	/// Creates a diagnostic using the passed error builder, adds it to the parsing diagnostics, and returns
	/// [Invalid] if the parsing context doesn't support the passed syntax feature.
	pub fn exclusive_for<F, E>(
		self,
		feature: &F,
		p: &mut Parser,
		error_builder: E,
	) -> ConditionalParsedSyntax
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		if feature.is_supported(p) {
			Valid(self)
		} else {
			if let Present(marker) = &self {
				let diagnostic = error_builder(p, marker);
				p.error(diagnostic);
			}

			Invalid(InvalidParsedSyntax(self))
		}
	}

	/// Restricts this parsed syntax to only be valid if the current parsing context supports the passed in language feature.
	///
	/// Returns [Valid] if the parsing context supports the passed syntax feature.
	///
	/// Returns [Invalid] if the parsing context doesn't support the passed syntax feature.
	pub fn exclusive_for_no_error<F>(self, feature: &F, p: &Parser) -> ConditionalParsedSyntax
	where
		F: SyntaxFeature,
	{
		if feature.is_supported(p) {
			Valid(self)
		} else {
			Invalid(InvalidParsedSyntax(self))
		}
	}

	/// Restricts this parsed syntax to only be valid if the current parsing context doesn't support the passed in language feature
	/// and adds a diagnostic if it does.
	///
	/// Returns [Valid] if the parsing context doesn't support the passed syntax feature.
	///
	/// Creates a diagnostic using the passed error builder, adds it to the parsing diagnostics, and returns
	/// [Invalid] if the parsing context does support the passed syntax feature.
	pub fn excluding<F, E>(
		self,
		feature: &F,
		p: &mut Parser,
		error_builder: E,
	) -> ConditionalParsedSyntax
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		if feature.is_unsupported(p) {
			Valid(self)
		} else {
			if let Present(marker) = &self {
				let diagnostic = error_builder(p, marker);
				p.error(diagnostic);
			}

			Invalid(InvalidParsedSyntax(self))
		}
	}

	/// Restricts this parsed syntax to only be valid if the current parsing context doesn't support the passed in language feature.
	///
	/// Returns [Valid] if the parsing context doesn't support the passed syntax feature.
	///
	/// Returns [Invalid] if the parsing context does support the passed syntax feature.
	pub fn excluding_no_error<F>(self, feature: &F, p: &Parser) -> ConditionalParsedSyntax
	where
		F: SyntaxFeature,
	{
		if feature.is_unsupported(p) {
			Valid(self)
		} else {
			Invalid(InvalidParsedSyntax(self))
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

/// A parsed syntax that is only valid in some parsing contexts but not in others.
/// One use case for this is for syntax that is only valid if the parsing context supports
/// a certain language feature, for example:
///
/// * Syntax that is only supported in strict or sloppy mode: for example, `with` statements
/// * Syntax that is only supported in certain file types: Typescript, JSX, Import / Export statements
/// * Syntax that is only available in certain language versions: experimental features, private field existence test
///
/// A parse rule must explicitly handle conditional syntax in the case it is invalid because it
/// represents content that shouldn't be there. This normally involves to wrap this syntax in an
/// `Unknown*` node or one of its parent.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "this `ConditionalParsedSyntax` may be an `Invalid` variant, which should be handled"]
pub enum ConditionalParsedSyntax {
	/// Syntax that is valid in the current parsing context
	Valid(ParsedSyntax),

	/// Syntax that is invalid in the current parsing context because it doesn't support a specific
	/// language feature.
	Invalid(InvalidParsedSyntax),
}

impl ConditionalParsedSyntax {
	/// Returns `true` if this syntax is valid in this parsing context.
	#[allow(unused)]
	#[must_use]
	pub fn is_valid(&self) -> bool {
		matches!(self, Invalid(_))
	}

	/// Returns `true` if this syntax is invalid in this parsing context.
	#[allow(unused)]
	pub fn is_invalid(&self) -> bool {
		matches!(self, Valid(_))
	}

	/// Returns `true` if this syntax is present in the source text.
	#[must_use]
	pub fn is_present(&self) -> bool {
		matches!(
			self,
			Valid(Present(_)) | Invalid(InvalidParsedSyntax(Present(_)))
		)
	}

	/// Returns `true` if this syntax is absent from the source text.
	pub fn is_absent(&self) -> bool {
		matches!(self, Valid(Absent) | Invalid(InvalidParsedSyntax(Absent)))
	}

	/// Converts this into a parsed syntax by wrapping any present invalid syntax in an unknown node.
	pub fn or_invalid_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParsedSyntax {
		match self {
			Valid(parsed) => parsed,
			Invalid(unsupported) => unsupported.or_to_unknown(p, unknown_kind),
		}
	}
}

/// Parsed syntax that is invalid in this parsing context.
#[must_use = "this 'UnsupportedParsedSyntax' contains syntax not supported in this parsing context, which must be handled."]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidParsedSyntax(ParsedSyntax);

impl InvalidParsedSyntax {
	pub fn new(syntax: ParsedSyntax) -> Self {
		Self(syntax)
	}

	/// Converts this into a parsed syntax by wrapping any present invalid syntax in an unknown node.
	/// Is a no-op if the syntax is absent in the source text.
	pub fn or_to_unknown(self, p: &mut Parser, unknown_kind: SyntaxKind) -> ParsedSyntax {
		match self.0 {
			Absent => Absent,
			Present(mut unsupported) => {
				unsupported.change_kind(p, unknown_kind);
				Present(unsupported)
			}
		}
	}
}

impl From<ParsedSyntax> for InvalidParsedSyntax {
	fn from(syntax: ParsedSyntax) -> Self {
		InvalidParsedSyntax::new(syntax)
	}
}

impl From<CompletedMarker> for InvalidParsedSyntax {
	fn from(marker: CompletedMarker) -> Self {
		InvalidParsedSyntax::new(Present(marker))
	}
}
