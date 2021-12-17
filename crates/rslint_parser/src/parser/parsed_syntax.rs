use crate::parser::parse_recovery::RecoveryResult;
use crate::parser::ParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Marker, Parser};
use rslint_errors::{Diagnostic, Span};
use rslint_syntax::JsSyntaxKind;
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
		F: FnOnce(T) -> ParsedSyntax<T>,
	{
		match self {
			Absent => Absent,
			Present(marker) => op(marker),
		}
	}

	/// Calls `op` if the syntax is absent ond otherwise returns [ParsedSyntax::Present]
	pub fn or_else<F>(self, op: F) -> ParsedSyntax<T>
	where
		F: FnOnce() -> ParsedSyntax<T>,
	{
		match self {
			Absent => op(),
			t => t,
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
	pub fn kind(&self) -> Option<JsSyntaxKind> {
		match self {
			Absent => None,
			Present(marker) => Some(marker.kind()),
		}
	}

	/// It returns the syntax if present or adds a missing marker and a diagnostic at the current parser position.
	pub fn or_syntax_error<E>(self, p: &mut Parser, error_builder: E) -> Option<CompletedMarker>
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Present(syntax) => Some(syntax),
			Absent => {
				let diagnostic = error_builder(p, p.cur_tok().range);
				p.error(diagnostic);
				None
			}
		}
	}

	/// It creates and returns a marker preceding this parsed syntax if it is present or starts
	/// a new marker, marks the first slot as missing and adds an error to the current parser position.
	/// See [CompletedMarker.precede]
	pub fn precede_or_syntax_error<E>(self, p: &mut Parser, error_builder: E) -> Marker
	where
		E: FnOnce(&Parser, Range<usize>) -> Diagnostic,
	{
		match self {
			Present(completed) => completed.precede(p),
			Absent => {
				let diagnostic = error_builder(p, p.cur_tok().range);
				p.error(diagnostic);
				p.start()
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
