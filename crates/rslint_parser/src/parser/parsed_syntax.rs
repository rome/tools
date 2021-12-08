use crate::parser::parse_recovery::RecoveryResult;
use crate::parser::ConditionalSyntax::{Invalid, Valid};
use crate::parser::ParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Marker, Parser, SyntaxFeature};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::SyntaxKind;
use std::ops::Range;

/// Syntax that is either present in the source tree or absent.
///
/// This type is commonly used as the return type of a parse function with the following types for `T`
///
/// * [CompletedMarker]: Most commonly used type. Parse function that either returns [ParsedSyntax::Absent]
///   if the syntax isn't present or [ParsedSyntax::Present] with a valid syntax.
/// * [ConditionalSyntax]: Used for parse functions where the parsed syntax may be invalid (a syntax error)
///   depending on the parse context. Examples are: 1) Use of a `with` statement in strict mode,
///   2) use of `await` as an identifier inside of an `async` function, and so on. That's why these
///   parse functions must differentiate between syntax that is [ParsedSyntax::Absent] in the source text,
///   syntax that is [ParsedSyntax::Present] and [Valid], and syntax that is [ParsedSyntax::Present] but [Invalid].
///
///
/// ## Parse Rule conventions
/// * A parse rule must return [ParsedSyntax::Present] if it is able to parse a node or at least parts of it. For example,
/// the `parse_for_statement` should return [ParsedSyntax::Present] for `for (` even tough many of the required children are missing
/// because it is still able to parse parts of the for statement.
/// * A parse rule must return [ParsedSyntax::Absent] if the expected node isn't present in the source code.
/// In most cases, this means if the first expected token isn't present, for example,
/// if the `for` keyword isn't present when parsing a for statement.
/// However, it can be possible for rules to recover even if the first token doesn't match. One example
/// is when parsing an assignment target that has an optional default. The rule can recover even
/// if the assignment target is missing as long as the cursor is then positioned at an `=` token.
/// The rule must then return [ParsedSyntax::Present] with the partial parsed node.
/// * A parse rule must not eat any tokens when it returns [ParsedSyntax::Absent]
/// * A parse rule must not add any errors when it returns [ParsedSyntax::Absent]
///
/// This is a custom enum over using `Option` because [ParsedSyntax::Absent] values must be handled by the caller.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "this `ParsedSyntax` may be an `Absent` variant, which should be handled"]
pub enum ParsedSyntax<T> {
	/// A syntax that isn't present in the source code. Used when a parse rule can't match the current
	/// token of the parser.
	Absent,

	/// A completed syntax node with all or some of its children.
	Present(T),
}

impl<T> ParsedSyntax<T> {
	/// Converts from `ParsedSyntax<T>` to `Option<T>`.
	///
	/// Converts `self` into an `Option<T>`, consuming `self`
	pub fn ok(self) -> Option<T> {
		match self {
			Absent => None,
			Present(marker) => Some(marker),
		}
	}

	/// Calls `op` if the syntax is present and otherwise returns [ParsedSyntax::Absent]
	pub fn and_then<F>(self, op: F) -> ParsedSyntax<T>
	where
		F: FnOnce(T) -> T,
	{
		match self {
			Absent => Absent,
			Present(marker) => Present(op(marker)),
		}
	}

	/// Returns `true` if the parsed syntax is [ParsedSyntax::Present]
	#[must_use]
	pub fn is_present(&self) -> bool {
		matches!(self, Present(_))
	}

	/// Returns `true` if the parsed syntax is [ParsedSyntax::Absent]
	#[must_use]
	pub fn is_absent(&self) -> bool {
		matches!(self, Absent)
	}

	/// It returns the contained [ParsedSyntax::Present] value, consuming the `self` value
	///
	/// # Panics
	///
	///  Panics if the current syntax is [ParsedSyntax::Absent]
	pub fn unwrap(self) -> T {
		match self {
			Absent => {
				panic!("Called `unwrap` on an `Absent` syntax");
			}
			Present(marker) => marker,
		}
	}

	/// Maps a `ParsedSyntax<T>` to `ParsedSyntax<U>` by applying a function to a contained [ParsedSyntax::Present] value,
	/// leaving an [ParsedSyntax::Absent] value untouched.
	///
	/// This function can be used to compose the results of two functions.
	pub fn map<F, U>(self, mapper: F) -> ParsedSyntax<U>
	where
		F: FnOnce(T) -> U,
	{
		match self {
			Absent => Absent,
			Present(element) => Present(mapper(element)),
		}
	}
}

impl ParsedSyntax<CompletedMarker> {
	/// Returns the kind of the syntax if it is present or [None] otherwise
	pub fn kind(&self) -> Option<SyntaxKind> {
		match self {
			Absent => None,
			Present(marker) => Some(marker.kind()),
		}
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

	/// It creates and returns a marker preceding this parsed syntax if it is present or starts
	/// a new marker, marks the first slot as missing and adds an error to the current parser position.
	/// See [CompletedMarker.precede]
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

	/// Creates a new marker that precedes this syntax or starts a new marker
	pub fn precede(self, p: &mut Parser) -> Marker {
		match self {
			Present(marker) => marker.precede(p),
			Absent => p.start(),
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
		recovery: &ParseRecovery,
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

	/// Undoes the completion and abandons the marker if the syntax is present.
	pub fn abandon(self, p: &mut Parser) {
		if let Present(marker) = self {
			marker.undo_completion(p).abandon(p)
		}
	}

	/// Converts this syntax into a [Valid] syntax if it is present
	pub fn into_valid(self) -> ParsedSyntax<ConditionalSyntax> {
		self.map(Valid)
	}

	/// Converts this syntax into an [Invalid] if the syntax is present without adding an error
	pub fn into_invalid(self) -> ParsedSyntax<ConditionalSyntax> {
		self.map(|marker| Invalid(marker.into()))
	}

	/// Converts this syntax into an [ConditionalSyntax::Invalid] if `valid` is true and into [ConditionalSyntax::Invalid] otherwise.
	pub fn into_conditional(self, valid: bool) -> ParsedSyntax<ConditionalSyntax> {
		if valid {
			self.into_valid()
		} else {
			self.into_invalid()
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
	) -> ParsedSyntax<ConditionalSyntax>
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		match self {
			Present(marker) => {
				if feature.is_supported(p) {
					Present(marker).into_valid()
				} else {
					let diagnostic = error_builder(p, &marker);
					p.error(diagnostic);

					Present(marker).into_invalid()
				}
			}
			Absent => Absent,
		}
	}

	/// Restricts this parsed syntax to only be valid if the current parsing context supports the passed in language feature.
	///
	/// Returns [Valid] if the parsing context supports the passed syntax feature.
	///
	/// Returns [Invalid] if the parsing context doesn't support the passed syntax feature.
	pub fn exclusive_for_no_error<F>(
		self,
		feature: &F,
		p: &Parser,
	) -> ParsedSyntax<ConditionalSyntax>
	where
		F: SyntaxFeature,
	{
		match self {
			Present(marker) => {
				if feature.is_supported(p) {
					Present(marker).into_valid()
				} else {
					Present(marker).into_invalid()
				}
			}
			Absent => Absent,
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
	) -> ParsedSyntax<ConditionalSyntax>
	where
		F: SyntaxFeature,
		E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
	{
		match self {
			Present(marker) => {
				if feature.is_unsupported(p) {
					Present(marker).into_valid()
				} else {
					let diagnostic = error_builder(p, &marker);
					p.error(diagnostic);
					Present(marker).into_invalid()
				}
			}
			Absent => Absent,
		}
	}

	/// Restricts this parsed syntax to only be valid if the current parsing context doesn't support the passed in language feature.
	///
	/// Returns [Valid] if the parsing context doesn't support the passed syntax feature.
	///
	/// Returns [Invalid] if the parsing context does support the passed syntax feature.
	pub fn excluding_no_error<F>(self, feature: &F, p: &Parser) -> ParsedSyntax<ConditionalSyntax>
	where
		F: SyntaxFeature,
	{
		match self {
			Present(marker) => {
				if feature.is_unsupported(p) {
					Present(marker).into_valid()
				} else {
					Present(marker).into_invalid()
				}
			}
			Absent => Absent,
		}
	}
}

impl From<CompletedMarker> for ParsedSyntax<CompletedMarker> {
	fn from(marker: CompletedMarker) -> Self {
		Present(marker)
	}
}

impl From<Option<CompletedMarker>> for ParsedSyntax<CompletedMarker> {
	fn from(option: Option<CompletedMarker>) -> Self {
		match option {
			Some(completed) => Present(completed),
			None => Absent,
		}
	}
}

/// A parsed syntax that may be invalid because of a syntax error (the whole node, node one of its children).
/// Examples
/// * Parsing an identifier that turns out to not be valid. For example, `await` isn't a valid identifier in async functions or strict mode
/// * Syntax that is only supported in strict or sloppy mode: for example, `with` statements
/// * Syntax that is only supported in certain file types: Typescript, JSX, Import / Export statements
/// * Syntax that is only available in certain language versions: experimental features, private field existence test
///
/// A parse rule must explicitly handle conditional syntax in the case it is invalid because it
/// represents content that shouldn't be there. This normally involves to wrap this syntax (or one of its parents) in an
/// `Unknown*` node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "this `ConditionalParsedSyntax` may be an `Invalid` variant, which should be handled"]
pub enum ConditionalSyntax {
	/// Syntax that is valid in the current parsing context
	Valid(CompletedMarker),

	/// Syntax that is invalid in the current parsing context.
	Invalid(InvalidSyntax),
}

impl ConditionalSyntax {
	/// Returns `true` if this syntax is valid in this parsing context.
	#[must_use]
	pub fn is_valid(&self) -> bool {
		matches!(self, Valid(_))
	}

	/// Returns `true` if this syntax is invalid in this parsing context.
	pub fn is_invalid(&self) -> bool {
		matches!(self, Invalid(_))
	}

	/// Creates a marker that precedes this valid or invalid node
	pub fn precede(self, p: &mut Parser) -> Marker {
		match self {
			Valid(marker) => marker.precede(p),
			Invalid(marker) => marker.precede(p),
		}
	}

	/// Undoes the completion of the inner valid or invalid node
	pub fn undo_completion(self, p: &mut Parser) -> Marker {
		match self {
			Valid(marker) => marker.undo_completion(p),
			Invalid(marker) => marker.0.undo_completion(p),
		}
	}

	/// Wraps the parsed syntax in an unknown node if the syntax is [Invalid] or returns the valid syntax
	pub fn or_invalid_to_unknown(
		self,
		p: &mut Parser,
		unknown_kind: SyntaxKind,
	) -> CompletedMarker {
		match self {
			Valid(parsed) => parsed,
			Invalid(unsupported) => unsupported.or_to_unknown(p, unknown_kind),
		}
	}

	/// It returns a [CompletedMarker] from the current syntax
	///
	/// # Panics
	///
	///  Panics if the current syntax is [ConditionalSyntax::Invalid]
	pub fn unwrap(self) -> CompletedMarker {
		if let Valid(syntax) = self {
			syntax
		} else {
			panic!("Called `unwrap` on an `Invalid` syntax");
		}
	}
}

impl From<ConditionalSyntax> for ParsedSyntax<ConditionalSyntax> {
	fn from(syntax: ConditionalSyntax) -> Self {
		Present(syntax)
	}
}

impl ParsedSyntax<ConditionalSyntax> {
	/// Wraps the parsed syntax in an unknown node if it is [Present(Invalid)]. Otherwise returns
	/// the parsed syntax.
	pub fn or_invalid_to_unknown(
		self,
		p: &mut Parser,
		unknown_kind: SyntaxKind,
	) -> ParsedSyntax<CompletedMarker> {
		self.map(|syntax| syntax.or_invalid_to_unknown(p, unknown_kind))
	}

	/// It adds a `missing` marker if the syntax is absent and returns `Ok(None)`.
	/// It returns [Err(InvalidSyntax)] if this syntax is [Invalid] and [Ok] if this syntax is [Valid]
	pub fn or_missing(self, p: &mut Parser) -> Result<Option<CompletedMarker>, InvalidSyntax> {
		match self {
			Absent => {
				p.missing();
				Ok(None)
			}
			Present(Valid(marker)) => Ok(Some(marker)),
			Present(Invalid(invalid)) => Err(invalid),
		}
	}

	/// It adds a `missing` marker if the syntax is absent, creates a diagnostic with the `error_builder`,
	/// adds the diagnostic to the parsing context and returns `Ok(None)`.
	/// It returns [Err(InvalidSyntax)] if this syntax is [Invalid] and [Ok] if this syntax is [Valid]
	pub fn or_missing_with_error<E>(
		self,
		p: &mut Parser,
		error_builder: E,
	) -> Result<Option<CompletedMarker>, InvalidSyntax>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Absent => {
				p.missing();
				p.error(error_builder(p, p.cur_tok().range));

				Ok(None)
			}
			Present(Valid(marker)) => Ok(Some(marker)),
			Present(Invalid(invalid)) => Err(invalid),
		}
	}
}

/// Parsed syntax that is invalid in this parsing context.
#[must_use = "this 'InvalidSyntax' contains syntax not supported in this parsing context, which must be handled."]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidSyntax(CompletedMarker);

impl InvalidSyntax {
	pub fn new(syntax: CompletedMarker) -> Self {
		Self(syntax)
	}

	/// Converts this into a completed marker by changing the node kind to the passed in `unknown_kind`
	pub fn or_to_unknown(mut self, p: &mut Parser, unknown_kind: SyntaxKind) -> CompletedMarker {
		self.0.change_kind(p, unknown_kind);
		self.0
	}

	/// Undoes the completion and abandons the marker.
	pub fn abandon(self, p: &mut Parser) {
		self.0.undo_completion(p).abandon(p)
	}

	/// Creates a new marker that precedes this syntax.
	pub fn precede(self, p: &mut Parser) -> Marker {
		self.0.precede(p)
	}
}

impl From<CompletedMarker> for InvalidSyntax {
	fn from(marker: CompletedMarker) -> Self {
		InvalidSyntax::new(marker)
	}
}
