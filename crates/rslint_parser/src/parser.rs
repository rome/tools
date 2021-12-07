//! The physical parser structure.
//! This may not hold your expectations of a traditional parser,
//! the parser yields events like `Start node`, `Error`, etc.
//! These events are then applied to a `TreeSink`.

pub(crate) mod parse_error;
mod parse_lists;
mod parse_recovery;
mod parsed_syntax;
pub(crate) mod single_token_parse_recovery;

use drop_bomb::DropBomb;
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind::EOF;
use std::borrow::BorrowMut;
use std::ops::Range;

pub use parse_error::*;
pub use parse_lists::{List, ParseNormalList, ParseSeparatedList};
pub use parsed_syntax::{ConditionalSyntax, InvalidSyntax, ParsedSyntax};
#[allow(deprecated)]
pub use single_token_parse_recovery::SingleTokenParseRecovery;

pub use crate::parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use crate::*;

/// Captures the progress of the parser and allows to test if the parsing is still making progress
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
pub struct ParserProgress(Option<usize>);

impl ParserProgress {
	/// Returns true if the current parser position is passed this position
	#[inline]
	pub fn has_progressed(&self, p: &Parser) -> bool {
		match self.0 {
			None => true,
			Some(pos) => pos < p.token_pos(),
		}
	}

	/// Asserts that the parsing is still making progress.
	/// ## Panics
	///
	/// Panics if the parser is still at this position
	#[inline]
	pub fn assert_progressing(&mut self, p: &Parser) {
		assert!(
			self.has_progressed(p),
			"The parser is no longer progressing. Stuck at {:?}",
			p.cur_tok()
		);

		self.0 = Some(p.token_pos());
	}
}

/// An extremely fast, error tolerant, completely lossless JavaScript parser
///
/// The Parser yields lower level events instead of nodes.
/// These events are then processed into a syntax tree through a [`TreeSink`] implementation.
///
/// ```
/// use rslint_parser::{
///     Parser,
///     syntax::expr,
///     tokenize,
///     TokenSource,
///     ast::JsParenthesizedExpression,
///     LosslessTreeSink,
///     SyntaxNode,
///     process,
///     AstNode,
///     Syntax
/// };
///
/// let source = "(delete b)";
///
/// // File id is used for the labels inside parser errors to report them, the file id
/// // is used to look up a file's source code and path inside of a codespan `Files` implementation.
/// let (tokens, lexer_errors) = tokenize(source, 0);
///
/// assert!(lexer_errors.is_empty());
///
/// // The parser uses a token source which manages yielding non-whitespace tokens,
/// // and giving raw tokens to allow the parser to turn completed markers into syntax nodes
/// let token_source = TokenSource::new(source, &tokens);
///
/// let mut parser = Parser::new(token_source, 0, Syntax::default());
///
/// // Use one of the syntax parsing functions to parse an expression.
/// // This adds node and token events to the parser which are then used to make a node.
/// // A completed marker marks the start and end indices in the events vec which signify
/// // the Start event, and the Finish event.
/// // Completed markers can be turned into an ast node with parse_marker on the parser
/// let completed_marker = expr::expr(&mut parser).unwrap();
///
/// parser.parse_marker::<JsParenthesizedExpression>(&completed_marker);
///
/// // Make a new text tree sink, its job is assembling events into a rowan GreenNode.
/// // At each point (Start, Token, Finish, Error) it also consumes whitespace.
/// // Other abstractions can also yield lossy syntax nodes if whitespace is not wanted.
/// // Swap this for a LossyTreeSink for a lossy AST result.
/// let mut sink = LosslessTreeSink::new(source, &tokens);
///
/// // Consume the parser and get its events, then apply them to a tree sink with `process`.
/// let (events, errors) = parser.finish();
/// process(&mut sink, events, errors);
///
/// let (untyped_node, errors) = sink.finish();
///
/// assert!(errors.is_empty());
///
/// assert!(JsParenthesizedExpression::can_cast(untyped_node.kind()));
///
/// // Convert the untyped SyntaxNode into a typed AST node
/// let typed_expr = JsParenthesizedExpression::cast(untyped_node).unwrap();
///
/// assert_eq!(typed_expr.expression().unwrap().syntax().text(), "delete b");
/// ```
pub struct Parser<'t> {
	pub file_id: usize,
	tokens: TokenSource<'t>,
	pub(crate) events: Vec<Event>,
	pub state: ParserState,
	pub syntax: Syntax,
	pub errors: Vec<ParserError>,
}

impl<'t> Parser<'t> {
	/// Make a new parser
	pub fn new(tokens: TokenSource<'t>, file_id: usize, syntax: Syntax) -> Parser<'t> {
		// TODO(RDambrosio016): Does TypeScript imply Module/Strict?
		let strict = if syntax.file_kind == FileKind::Module {
			Some(StrictMode::Module)
		} else {
			None
		};
		let state = ParserState {
			is_module: syntax.file_kind == FileKind::Module,
			strict,
			..ParserState::default()
		};

		Parser {
			file_id,
			tokens,
			events: vec![],
			state,
			syntax,
			errors: vec![],
		}
	}

	pub(crate) fn typescript(&self) -> bool {
		self.syntax.file_kind == FileKind::TypeScript
	}

	/// Get the source code of a token
	pub fn token_src(&self, token: &Token) -> &str {
		self.tokens
			.source()
			.get(token.range.to_owned())
			.expect("Token range and src mismatch")
	}

	/// Consume the parser and return the list of events it produced
	pub fn finish(self) -> (Vec<Event>, Vec<ParserError>) {
		(self.events, self.errors)
	}

	/// Get the current token kind of the parser
	pub fn cur(&self) -> SyntaxKind {
		self.nth(0)
	}

	/// Get the current token of the parser
	pub fn cur_tok(&self) -> Token {
		self.nth_tok(0)
	}

	/// Look ahead at a token and get its kind, **The max lookahead is 4**.
	pub fn nth(&self, n: usize) -> SyntaxKind {
		self.tokens.lookahead_nth(n).kind
	}

	/// Look ahead at a token, **The max lookahead is 4**.
	pub fn nth_tok(&self, n: usize) -> Token {
		self.tokens.lookahead_nth(n)
	}

	/// Check if the parser is currently at a specific token
	pub fn at(&self, kind: SyntaxKind) -> bool {
		self.nth_at(0, kind)
	}

	/// Check if a token lookahead is something, `n` must be smaller or equal to `4`
	pub fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
		self.nth_tok(n).kind == kind
	}

	/// Consume the next token if `kind` matches.
	pub fn eat(&mut self, kind: SyntaxKind) -> bool {
		if !self.at(kind) {
			return false;
		}
		self.do_bump(kind);
		true
	}

	/// Consumes the next optional token if `kind` matches or inserts a missing marker
	pub fn eat_optional(&mut self, kind: SyntaxKind) -> bool {
		if self.eat(kind) {
			true
		} else {
			self.missing();
			false
		}
	}

	/// Starts a new node in the syntax tree. All nodes and tokens
	/// consumed between the `start` and the corresponding `Marker::complete`
	/// belong to the same node.
	pub fn start(&mut self) -> Marker {
		let pos = self.events.len() as u32;
		self.push_event(Event::tombstone(self.tokens.cur_pos()));
		Marker::new(pos, self.tokens.cur_pos())
	}

	/// Check if there was a linebreak before a lookahead
	pub fn has_linebreak_before_n(&self, n: usize) -> bool {
		self.tokens.had_linebreak_before_nth(n)
	}

	/// Consume the next token if `kind` matches.
	pub fn bump(&mut self, kind: SyntaxKind) {
		assert!(
			self.eat(kind),
			"expected {:?} but at {:?}",
			kind,
			self.cur()
		);
	}

	/// Consume any token but cast it as a different kind
	pub fn bump_remap(&mut self, kind: SyntaxKind) {
		self.do_bump(kind)
	}

	/// Advances the parser by one token
	pub fn bump_any(&mut self) {
		let kind = self.nth(0);
		assert_ne!(kind, EOF);
		self.do_bump(kind)
	}

	/// Make a new error builder with `error` severity
	pub fn err_builder(&self, message: &str) -> Diagnostic {
		Diagnostic::error(self.file_id, "SyntaxError", message)
	}

	/// Add an error
	#[cold]
	pub fn error(&mut self, err: impl Into<ParserError>) {
		self.errors.push(err.into())
	}

	/// Check if the parser's current token is contained in a token set
	pub fn at_ts(&self, kinds: TokenSet) -> bool {
		kinds.contains(self.cur())
	}

	fn do_bump(&mut self, kind: SyntaxKind) {
		let range = self.cur_tok().range;
		self.tokens.bump();

		self.push_event(Event::Token { kind, range });
	}

	/// Inserts a marker for a missing child element because the child is optional and wasn't present in the source
	/// or it's a mandatory child that wasn't present in the source because of a syntax error.
	pub fn missing(&mut self) {
		self.push_event(Event::Missing);
	}

	fn push_event(&mut self, event: Event) {
		self.events.push(event)
	}

	/// Get the source code of the parser's current token.
	///
	/// # Panics
	/// This method panics if the token range and source code range mismatch
	pub fn cur_src(&self) -> &str {
		self.tokens
			.source()
			.get(self.nth_tok(0).range)
			.expect("Parser source and tokens mismatch")
	}

	pub fn nth_src(&self, n: usize) -> &str {
		self.tokens
			.source()
			.get(self.nth_tok(n).range)
			.expect("Parser source and tokens mismatch")
	}

	/// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
	pub fn expect(&mut self, kind: SyntaxKind) -> bool {
		if self.eat(kind) {
			true
		} else {
			let err = if self.cur() == SyntaxKind::EOF {
				self.err_builder(&format!(
					"expected `{}` but instead the file ends",
					kind.to_string()
						.map(|x| x.to_string())
						.unwrap_or_else(|| format!("{:?}", kind))
				))
				.primary(self.cur_tok().range, "the file ends here")
			} else {
				self.err_builder(&format!(
					"expected `{}` but instead found `{}`",
					kind.to_string()
						.map(|x| x.to_string())
						.unwrap_or_else(|| format!("{:?}", kind)),
					self.cur_src()
				))
				.primary(self.cur_tok().range, "unexpected")
			};

			self.error(err);
			false
		}
	}

	/// Tries to eat a specific token kind, adds a missing marker and an error to the events stack if it's not there.
	pub fn expect_required(&mut self, kind: SyntaxKind) -> bool {
		if !self.expect(kind) {
			self.missing();
			false
		} else {
			true
		}
	}

	/// Get the byte index range of a completed marker for error reporting.
	pub fn marker_range(&self, marker: &CompletedMarker) -> Range<usize> {
		match self.events[marker.start_pos as usize] {
			Event::Start { start, .. } => match self.events[marker.finish_pos as usize] {
				Event::Finish { end } => start..end,
				_ => unreachable!(),
			},
			_ => unreachable!(),
		}
	}

	/// Parse a completed marker into an ast node.
	///
	/// # Panics
	/// Panics if the AST node represented by the marker does not match the generic
	#[deprecated(note = "Unsafe and fairly expensive.")]
	pub fn parse_marker<T: AstNode>(&self, marker: &CompletedMarker) -> T {
		let events = self
			.events
			.get(marker.old_start as usize..(marker.finish_pos as usize + 1))
			.expect("Marker out of bounds")
			.to_vec();

		let start = match self.events[marker.old_start as usize] {
			Event::Start { start, .. } => start,
			_ => unreachable!(),
		};

		let mut sink =
			LosslessTreeSink::with_offset(self.tokens.source(), self.tokens.raw_tokens, start);
		process(&mut sink, events, vec![]);
		T::cast(sink.finish().0).expect("Marker was parsed to the wrong ast node")
	}

	/// Get the source code of a range
	pub fn source(&self, range: TextRange) -> &str {
		&self.tokens.source()[range]
	}

	/// Rewind the parser back to a previous position in time
	pub fn rewind(&mut self, checkpoint: Checkpoint) {
		let Checkpoint {
			token_pos,
			event_pos,
			errors_pos,
		} = checkpoint;
		self.tokens.rewind(token_pos);
		self.drain_events(self.cur_event_pos() - event_pos);
		self.errors.truncate(errors_pos);
	}

	/// Get a checkpoint representing the progress of the parser at this point in time
	#[must_use]
	pub fn checkpoint(&self) -> Checkpoint {
		Checkpoint {
			token_pos: self.token_pos(),
			event_pos: self.cur_event_pos(),
			errors_pos: self.errors.len(),
		}
	}

	/// Get the current index of the last event
	fn cur_event_pos(&self) -> usize {
		self.events.len().saturating_sub(1)
	}

	/// Remove `amount` events from the parser
	fn drain_events(&mut self, amount: usize) {
		self.events.truncate(self.events.len() - amount);
	}

	/// Get the current token position
	pub fn token_pos(&self) -> usize {
		self.tokens.cur_token_idx()
	}

	/// Make a new error builder with warning severity
	pub fn warning_builder(&self, message: &str) -> Diagnostic {
		Diagnostic::warning(self.file_id, "SyntaxError", message)
	}

	/// Bump and add an error event
	pub fn err_and_bump(&mut self, err: impl Into<ParserError>, unknown_syntax_kind: SyntaxKind) {
		let m = self.start();
		self.bump_any();
		m.complete(self, unknown_syntax_kind);
		self.error(err);
	}

	pub fn err_if_not_ts(
		&mut self,
		mut marker: impl BorrowMut<CompletedMarker>,
		err: &str,
		unknown_syntax_kind: SyntaxKind,
	) {
		if self.typescript() {
			return;
		}
		let borrow = marker.borrow_mut();
		borrow.change_kind(self, unknown_syntax_kind);
		let err = self.err_builder(err).primary(borrow.range(self), "");

		self.error(err);
	}

	/// Try running a parser function and backtrack if any errors occured
	pub fn try_parse<F>(&mut self, func: F) -> Option<CompletedMarker>
	where
		F: FnOnce(&mut Parser) -> Option<CompletedMarker>,
	{
		let checkpoint = self.checkpoint();
		let res = func(self);
		if checkpoint.errors_pos != self.errors.len() {
			self.rewind(checkpoint);
			return None;
		}
		if res.is_none() {
			self.rewind(checkpoint);
		}
		res
	}

	pub(crate) fn expect_no_recover(&mut self, kind: SyntaxKind) -> Option<bool> {
		if self.state.no_recovery {
			Some(true).filter(|_| self.eat(kind))
		} else {
			Some(self.expect_required(kind))
		}
	}

	pub fn span_text(&self, span: impl rslint_errors::Span) -> &str {
		&self.tokens.source()[span.as_range()]
	}

	pub(crate) fn bump_multiple(&mut self, amount: u8, kind: SyntaxKind) {
		self.push_event(Event::MultipleTokens { amount, kind });
		for _ in 0..amount {
			self.tokens.bump();
		}
	}

	pub fn marker_vec_range(&self, markers: &[CompletedMarker]) -> Range<usize> {
		let start = markers
			.first()
			.map(|x| usize::from(x.range(self).start()))
			.unwrap_or_default();
		let end = markers
			.last()
			.map(|x| usize::from(x.range(self).end()))
			.unwrap_or_default();
		start..end
	}

	#[deprecated(
		note = "Use ParseRecovery instead which signals with a Result if the recovery was successful or not"
	)]
	pub fn expr_with_semi_recovery(&mut self, assign: bool) -> Option<CompletedMarker> {
		let func = if assign {
			syntax::expr::expr_or_assignment
		} else {
			syntax::expr::expr
		};

		if self.at(T![;]) {
			let m = self.start();
			let err = self
				.err_builder("expected an expression, but found `;` instead")
				.primary(self.cur_tok().range, "");
			self.error(err);
			self.bump_any();
			m.complete(self, SyntaxKind::JS_UNKNOWN_EXPRESSION);

			return None;
		}

		func(self)
	}
}

/// A structure signifying the start of parsing of a syntax tree node
#[derive(Debug)]
#[must_use = "Marker must either be `completed` or `abandoned`"]
pub struct Marker {
	/// The index in the events list
	pub pos: u32,
	/// The byte index where the node starts
	pub start: usize,
	pub old_start: u32,
	pub(crate) child_idx: Option<usize>,
	bomb: DropBomb,
}

impl Marker {
	pub fn new(pos: u32, start: usize) -> Marker {
		Marker {
			pos,
			start,
			old_start: pos,
			child_idx: None,
			bomb: DropBomb::new("Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a markers parent."),
		}
	}

	pub(crate) fn old_start(mut self, old: u32) -> Self {
		if self.old_start >= old {
			self.old_start = old;
		};
		self
	}

	/// Finishes the syntax tree node and assigns `kind` to it,
	/// and mark the create a `CompletedMarker` for possible future
	/// operation like `.precede()` to deal with forward_parent.
	pub fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
		self.bomb.defuse();
		let idx = self.pos as usize;
		match p.events[idx] {
			Event::Start {
				kind: ref mut slot, ..
			} => {
				*slot = kind;
			}
			_ => unreachable!(),
		}
		let finish_pos = p.events.len() as u32;
		p.push_event(Event::Finish {
			end: p.tokens.last_tok().map(|t| t.range.end).unwrap_or(0),
		});
		let new = CompletedMarker::new(self.pos, finish_pos, kind);
		new.old_start(self.old_start)
	}

	/// Abandons the syntax tree node. All its children
	/// are attached to its parent instead.
	pub fn abandon(mut self, p: &mut Parser) {
		self.bomb.defuse();
		let idx = self.pos as usize;
		if idx == p.events.len() - 1 {
			match p.events.pop() {
				Some(Event::Start {
					kind: SyntaxKind::TOMBSTONE,
					forward_parent: None,
					..
				}) => (),
				_ => unreachable!(),
			}
		}
		if let Some(idx) = self.child_idx {
			match p.events[idx] {
				Event::Start {
					ref mut forward_parent,
					..
				} => {
					*forward_parent = None;
				}
				_ => unreachable!(),
			}
		}
	}
}

/// A structure signifying a completed node
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CompletedMarker {
	pub(crate) start_pos: u32,
	// Hack for parsing completed markers which have been preceded
	// This should be redone completely in the future
	pub(crate) old_start: u32,
	pub(crate) finish_pos: u32,
	kind: SyntaxKind,
}

impl CompletedMarker {
	pub fn new(start_pos: u32, finish_pos: u32, kind: SyntaxKind) -> Self {
		CompletedMarker {
			start_pos,
			old_start: start_pos,
			finish_pos,
			kind,
		}
	}

	pub(crate) fn old_start(mut self, old: u32) -> Self {
		// For multiple precedes we should not update the old start
		if self.old_start >= old {
			self.old_start = old;
		};
		self
	}

	/// Change the kind of node this marker represents
	pub fn change_kind(&mut self, p: &mut Parser, new_kind: SyntaxKind) {
		self.kind = new_kind;
		match p
			.events
			.get_mut(self.start_pos as usize)
			.expect("Finish position of marker is OOB")
		{
			Event::Start { kind, .. } => {
				*kind = new_kind;
			}
			_ => unreachable!(),
		}
	}

	// Get the correct offset range in source code of an item inside of a parsed marker
	pub fn offset_range(&self, p: &Parser, range: TextRange) -> TextRange {
		let offset = self.range(p).start();
		TextRange::new(range.start() + offset, range.end() + offset)
	}

	/// Get the range of the marker
	pub fn range(&self, p: &Parser) -> TextRange {
		let start = match p.events[self.old_start as usize] {
			Event::Start { start, .. } => start as u32,
			_ => unreachable!(),
		};
		let end = match p.events[self.finish_pos as usize] {
			Event::Finish { end } => end as u32,
			_ => unreachable!(),
		};
		TextRange::new(start.into(), end.into())
	}

	/// Get the underlying text of a marker
	pub fn text<'a>(&self, p: &'a Parser) -> &'a str {
		&p.tokens.source()[self.range(p)]
	}

	/// This method allows to create a new node which starts
	/// *before* the current one. That is, parser could start
	/// node `A`, then complete it, and then after parsing the
	/// whole `A`, decide that it should have started some node
	/// `B` before starting `A`. `precede` allows to do exactly
	/// that. See also docs about `forward_parent` in `Event::Start`.
	///
	/// Given completed events `[START, FINISH]` and its corresponding
	/// `CompletedMarker(pos: 0, _)`.
	/// Append a new `START` events as `[START, FINISH, NEWSTART]`,
	/// then mark `NEWSTART` as `START`'s parent with saving its relative
	/// distance to `NEWSTART` into forward_parent(=2 in this case);
	pub fn precede(self, p: &mut Parser) -> Marker {
		let mut new_pos = p.start();
		let idx = self.start_pos as usize;
		match p.events[idx] {
			Event::Start {
				ref mut forward_parent,
				..
			} => {
				*forward_parent = Some(new_pos.pos - self.start_pos);
			}
			_ => unreachable!(),
		}
		new_pos.child_idx = Some(self.start_pos as usize);
		new_pos.old_start(self.old_start as u32)
	}

	/// Undo this completion and turns into a `Marker`
	pub fn undo_completion(self, p: &mut Parser) -> Marker {
		let start_idx = self.start_pos as usize;
		let finish_idx = self.finish_pos as usize;
		let start_pos;

		match p.events[start_idx] {
			Event::Start {
				ref mut kind,
				forward_parent: None,
				start,
			} => {
				start_pos = start;
				*kind = SyntaxKind::TOMBSTONE
			}
			_ => unreachable!(),
		}
		match p.events[finish_idx] {
			ref mut slot @ Event::Finish { .. } => *slot = Event::tombstone(start_pos),
			_ => unreachable!(),
		}
		Marker::new(self.start_pos, start_pos)
	}

	pub fn kind(&self) -> SyntaxKind {
		self.kind
	}

	pub fn err_if_not_ts(&mut self, p: &mut Parser, err: &str) {
		p.err_if_not_ts(self, err, SyntaxKind::ERROR);
	}
}

/// A structure signifying the Parser progress at one point in time
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Checkpoint {
	pub token_pos: usize,
	pub event_pos: usize,
	pub errors_pos: usize,
}

#[cfg(test)]
mod tests {
	use crate::{Parser, Syntax, TokenSource};
	use rslint_lexer::Token;
	use rslint_syntax::SyntaxKind;

	#[test]
	#[should_panic(
		expected = "Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a markers parent."
	)]
	fn uncompleted_markers_panic() {
		let tokens = vec![Token::new(SyntaxKind::JS_STRING_LITERAL, 12)];
		let token_source = TokenSource::new("'use strict'", tokens.as_slice());

		let mut parser = Parser::new(token_source, 0, Syntax::default());

		let _ = parser.start();
		// drop the marker without calling complete or abandon
	}

	#[test]
	fn completed_marker_doesnt_panic() {
		let tokens = vec![Token::new(SyntaxKind::JS_STRING_LITERAL, 12)];
		let token_source = TokenSource::new("'use strict'", tokens.as_slice());

		let mut p = Parser::new(token_source, 0, Syntax::default());

		let m = p.start();
		p.expect_required(SyntaxKind::JS_STRING_LITERAL);
		m.complete(&mut p, SyntaxKind::JS_STRING_LITERAL_EXPRESSION);
	}

	#[test]
	fn abandoned_marker_doesnt_panic() {
		let tokens = vec![Token::new(SyntaxKind::JS_STRING_LITERAL, 12)];
		let token_source = TokenSource::new("'use strict'", tokens.as_slice());

		let mut p = Parser::new(token_source, 0, Syntax::default());

		let m = p.start();
		m.abandon(&mut p);
	}
}
