//! The physical parser structure.
//! This may not hold your expectations of a traditional parser,
//! the parser yields events like `Start node`, `Error`, etc.
//! These events are then applied to a `TreeSink`.

pub(crate) mod parse_error;
mod parse_lists;
mod parse_recovery;
mod parsed_syntax;
pub(crate) mod rewrite_parser;
pub(crate) mod single_token_parse_recovery;

use drop_bomb::DebugDropBomb;
use rome_js_syntax::{
    JsSyntaxKind::{self},
    SourceType, TextRange,
};
use std::num::NonZeroU32;

pub(crate) use parse_error::*;
pub(crate) use parse_lists::{ParseNodeList, ParseSeparatedList};
pub(crate) use parsed_syntax::ParsedSyntax;
use rome_rowan::{SyntaxKind, TextSize};

use crate::lexer::{LexContext, ReLexContext};
pub(crate) use crate::parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use crate::*;
use crate::{
    state::ParserStateCheckpoint,
    token_source::{TokenSource, TokenSourceCheckpoint, Trivia},
};

/// Captures the progress of the parser and allows to test if the parsing is still making progress
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Hash, Default)]
pub(crate) struct ParserProgress(Option<TextSize>);

impl ParserProgress {
    /// Returns true if the current parser position is passed this position
    #[inline]
    pub fn has_progressed(&self, p: &Parser) -> bool {
        match self.0 {
            None => true,
            Some(pos) => pos < p.tokens.position(),
        }
    }

    /// Asserts that the parsing is still making progress.
    ///
    /// # Panics
    ///
    /// Panics if the parser is still at this position
    #[inline]
    pub fn assert_progressing(&mut self, p: &Parser) {
        assert!(
            self.has_progressed(p),
            "The parser is no longer progressing. Stuck at '{}' {:?}:{:?}",
            p.cur_src(),
            p.cur(),
            p.cur_range(),
        );

        self.0 = Some(p.tokens.position());
    }
}

/// An extremely fast, error tolerant, completely lossless JavaScript parser
///
/// The Parser yields lower level events instead of nodes.
/// These events are then processed into a syntax tree through a [`TreeSink`] implementation.
pub(crate) struct Parser<'s> {
    file_id: usize,
    pub(super) tokens: TokenSource<'s>,
    pub(super) events: Vec<Event>,
    pub(super) state: ParserState,
    pub source_type: SourceType,
    pub diagnostics: Vec<ParseDiagnostic>,
    // If the parser should skip tokens as trivia
    skipping: bool,
}

impl<'s> Parser<'s> {
    /// Creates a new parser that parses the `source`.
    pub fn new(source: &'s str, file_id: usize, source_type: SourceType) -> Parser<'s> {
        let token_source = TokenSource::from_str(source, file_id);

        Parser {
            file_id,
            events: vec![],
            state: ParserState::new(&source_type),
            tokens: token_source,
            source_type,
            diagnostics: vec![],
            skipping: false,
        }
    }

    /// Consume the parser and returns the list of events, the source text's trivia, and the diagnostics.
    pub fn finish(mut self) -> (Vec<Event>, Vec<Trivia>, Vec<ParseDiagnostic>) {
        let (trivia, source_diagnostics) = self.tokens.finish();
        self.diagnostics.extend(source_diagnostics);
        (self.events, trivia, self.diagnostics)
    }

    /// Gets the current token kind of the parser
    #[inline]
    pub fn cur(&self) -> JsSyntaxKind {
        self.tokens.current()
    }

    /// Gets the range of the current token
    #[inline]
    pub fn cur_range(&self) -> TextRange {
        self.tokens.current_range()
    }

    /// Checks if the parser is currently at a specific token
    #[inline]
    pub fn at(&self, kind: JsSyntaxKind) -> bool {
        self.cur() == kind
    }

    /// Look ahead at a token and get its kind.
    #[inline]
    pub fn nth(&mut self, n: usize) -> JsSyntaxKind {
        self.tokens.nth(n)
    }

    /// Checks if a token lookahead is something
    #[inline]
    pub fn nth_at(&mut self, n: usize, kind: JsSyntaxKind) -> bool {
        self.nth(n) == kind
    }

    /// Returns the kind of the last bumped token.
    pub fn last(&self) -> Option<JsSyntaxKind> {
        self.events.iter().rev().find_map(|event| match event {
            Event::Token { kind, .. } => Some(*kind),
            _ => None,
        })
    }

    /// Returns the end offset of the last bumped token.
    pub fn last_end(&self) -> Option<TextSize> {
        self.events.iter().rev().find_map(|event| match event {
            Event::Token { end, .. } => Some(*end),
            _ => None,
        })
    }

    /// Consume the next token if `kind` matches.
    #[inline]
    pub fn eat(&mut self, kind: JsSyntaxKind) -> bool {
        if !self.at(kind) {
            return false;
        }

        self.do_bump(kind, LexContext::default());

        true
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        let start = self.tokens.position();
        self.push_event(Event::tombstone());
        Marker::new(pos, start)
    }

    /// Tests if there's a line break before the nth token.
    #[inline]
    pub fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        self.tokens.has_nth_preceding_line_break(n)
    }

    /// Tests if there's a line break before the current token (between the last and current)
    #[inline]
    pub fn has_preceding_line_break(&self) -> bool {
        self.tokens.has_preceding_line_break()
    }

    /// Consume the current token if `kind` matches.
    #[inline]
    pub fn bump(&mut self, kind: JsSyntaxKind) {
        self.bump_with_context(kind, LexContext::default())
    }

    /// Consumes the current token if `kind` matches and lexes the next token using the
    /// specified `context.
    #[inline]
    pub fn bump_with_context(&mut self, kind: JsSyntaxKind, context: LexContext) {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        self.do_bump(kind, context);
    }

    /// Consume any token but cast it as a different kind
    pub fn bump_remap(&mut self, kind: JsSyntaxKind) {
        self.do_bump(kind, LexContext::default())
    }

    /// Bumps the current token regardless of its kind and advances to the next token.
    pub fn bump_any(&mut self) {
        let kind = self.nth(0);
        assert_ne!(kind, JsSyntaxKind::EOF);
        self.do_bump(kind, LexContext::default())
    }

    /// Make a new error builder with `error` severity
    #[must_use]
    pub fn err_builder(&self, message: &str) -> Diagnostic {
        Diagnostic::error(self.file_id, "SyntaxError", message)
    }

    /// Add an error
    pub fn error(&mut self, err: impl ToDiagnostic) {
        let err = err.to_diagnostic(self);

        // Don't report another error if it would just be at the same position as the last error.
        if let Some(previous) = self.diagnostics.last() {
            if err.code.as_deref() == Some("SyntaxError")
                && previous.code == err.code
                && previous.file_id == err.file_id
            {
                match (&err.primary, &previous.primary) {
                    (Some(err_primary), Some(previous_primary))
                        if err_primary.span.range.start()
                            == previous_primary.span.range.start() =>
                    {
                        return;
                    }
                    _ => {}
                }
            }
        }
        self.diagnostics.push(err)
    }

    /// Check if the parser's current token is contained in a token set
    pub fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.cur())
    }

    fn do_bump(&mut self, kind: JsSyntaxKind, context: LexContext) {
        let kind = if kind.is_keyword() && self.tokens.has_unicode_escape() {
            self.error(
                self.err_builder(&format!(
                    "'{}' keyword cannot contain escape character.",
                    kind.to_string().expect("to return a value for a keyword")
                ))
                .primary(self.cur_range(), ""),
            );
            JsSyntaxKind::ERROR_TOKEN
        } else {
            kind
        };

        let range = self.cur_range();
        self.push_token(kind, range.end());

        if self.skipping {
            self.tokens.skip_as_trivia(context);
        } else {
            self.tokens.bump(context);
        }
    }

    fn push_token(&mut self, kind: JsSyntaxKind, end: TextSize) {
        self.push_event(Event::Token { kind, end });
    }

    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }

    /// Get the source code of the parser's current token.
    pub fn cur_src(&self) -> &str {
        &self.tokens.source()[self.cur_range()]
    }

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
    pub fn expect(&mut self, kind: JsSyntaxKind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(expected_token(kind));
            false
        }
    }

    /// Re-lexes the current token in the specified context. Returns the kind
    /// of the re-lexed token (can be the same as before if the context doesn't make a difference for the current token)
    pub fn re_lex(&mut self, context: ReLexContext) -> JsSyntaxKind {
        self.tokens.re_lex(context)
    }

    /// Rewind the parser back to a previous position in time
    pub fn rewind(&mut self, checkpoint: Checkpoint) {
        let Checkpoint {
            token_source,
            event_pos,
            errors_pos,
            state,
        } = checkpoint;
        self.tokens.rewind(token_source);
        self.drain_events(self.cur_event_pos() - event_pos);
        self.diagnostics.truncate(errors_pos as usize);
        self.state.restore(state)
    }

    /// Get a checkpoint representing the progress of the parser at this point in time
    #[must_use]
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            token_source: self.tokens.checkpoint(),
            event_pos: self.cur_event_pos(),
            errors_pos: self.diagnostics.len() as u32,
            state: self.state.checkpoint(),
        }
    }

    /// Allows parsing an unsupported syntax as skipped trivia tokens.
    pub fn parse_as_skipped_trivia_tokens<P>(&mut self, parse: P)
    where
        P: FnOnce(&mut Parser),
    {
        let events_pos = self.events.len();
        self.skipping = true;
        parse(self);
        self.skipping = false;

        // Truncate any start/finish events
        self.events.truncate(events_pos);
    }

    /// Stores the parser state and position before calling the function and restores the state
    /// and position before returning.
    ///
    /// Useful in situation where the parser must advance a few tokens to determine whatever a syntax is
    /// of one or the other kind.
    #[inline]
    pub fn lookahead<F, R>(&mut self, op: F) -> R
    where
        F: FnOnce(&mut Parser) -> R,
    {
        let checkpoint = self.checkpoint();
        let result = op(self);
        self.rewind(checkpoint);

        result
    }

    /// Get the current index of the last event
    fn cur_event_pos(&self) -> usize {
        self.events.len().saturating_sub(1)
    }

    /// Remove `amount` events from the parser
    fn drain_events(&mut self, amount: usize) {
        self.events.truncate(self.events.len() - amount);
    }

    /// Bump and add an error event
    pub fn err_and_bump(&mut self, err: impl ToDiagnostic, unknown_syntax_kind: JsSyntaxKind) {
        let m = self.start();
        self.bump_any();
        m.complete(self, unknown_syntax_kind);
        self.error(err);
    }

    /// Gets the source of a range
    pub fn source(&self, span: TextRange) -> &'s str {
        &self.tokens.source()[span]
    }

    /// Whether the code we are parsing is a module
    pub fn is_module(&self) -> bool {
        self.source_type.module_kind().is_module()
    }
}

/// A structure signifying the start of parsing of a syntax tree node
#[derive(Debug)]
#[must_use = "Marker must either be `completed` or `abandoned`"]
pub(crate) struct Marker {
    /// The index in the events list
    pos: u32,
    /// The byte index where the node starts
    start: TextSize,
    pub(crate) old_start: u32,
    child_idx: Option<usize>,
    bomb: DebugDropBomb,
}

impl Marker {
    pub fn new(pos: u32, start: TextSize) -> Marker {
        Marker {
            pos,
            start,
            old_start: pos,
            child_idx: None,
            bomb: DebugDropBomb::new("Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent."),
        }
    }

    fn old_start(mut self, old: u32) -> Self {
        if self.old_start >= old {
            self.old_start = old;
        };
        self
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub fn complete(mut self, p: &mut Parser, kind: JsSyntaxKind) -> CompletedMarker {
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
        p.push_event(Event::Finish);

        let new = CompletedMarker::new(self.pos, finish_pos, self.start, kind);
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
                    kind: JsSyntaxKind::TOMBSTONE,
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

    pub fn start(&self) -> TextSize {
        self.start
    }
}

/// A structure signifying a completed node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct CompletedMarker {
    start_pos: u32,
    offset: TextSize,
    // Hack for parsing completed markers which have been preceded
    // This should be redone completely in the future
    old_start: u32,
    finish_pos: u32,
    kind: JsSyntaxKind,
}

impl CompletedMarker {
    pub fn new(start_pos: u32, finish_pos: u32, offset: TextSize, kind: JsSyntaxKind) -> Self {
        CompletedMarker {
            start_pos,
            offset,
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
    pub fn change_kind(&mut self, p: &mut Parser, new_kind: JsSyntaxKind) {
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

    pub fn change_to_unknown(&mut self, p: &mut Parser) {
        self.change_kind(p, self.kind().to_unknown());
    }

    /// Get the range of the marker
    pub fn range(&self, p: &Parser) -> TextRange {
        let end = p.events[self.old_start as usize..self.finish_pos as usize]
            .iter()
            .rev()
            .find_map(|event| match event {
                Event::Token { end, .. } => Some(*end),
                _ => None,
            })
            .unwrap_or(self.offset);

        TextRange::new(self.offset, end)
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
                // Safety: The new marker is always inserted after the start marker of this node, thus
                // subtracting the two positions can never be 0.
                *forward_parent = Some(NonZeroU32::try_from(new_pos.pos - self.start_pos).unwrap());
            }
            _ => unreachable!(),
        }
        new_pos.child_idx = Some(self.start_pos as usize);
        new_pos.start = self.offset;
        new_pos.old_start(self.old_start as u32)
    }

    /// Undo this completion and turns into a `Marker`
    pub fn undo_completion(self, p: &mut Parser) -> Marker {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;

        match p.events[start_idx] {
            Event::Start {
                ref mut kind,
                forward_parent: None,
            } => *kind = JsSyntaxKind::TOMBSTONE,
            _ => unreachable!(),
        }
        match p.events[finish_idx] {
            ref mut slot @ Event::Finish { .. } => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos, self.offset)
    }

    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }
}

/// A structure signifying the Parser progress at one point in time
#[derive(Debug)]
pub struct Checkpoint {
    pub(super) event_pos: usize,
    /// The length of the errors list at the time the checkpoint was created.
    /// Safety: The parser only supports files <= 4Gb. Storing a `u32` is sufficient to store one error
    /// for each single character in the file, which should be sufficient for any realistic file.
    errors_pos: u32,
    state: ParserStateCheckpoint,
    pub(super) token_source: TokenSourceCheckpoint,
}

#[cfg(test)]
mod tests {
    use crate::Parser;
    use rome_js_syntax::{JsSyntaxKind, SourceType};
    use rome_rowan::AstNode;

    #[test]
    fn example() {
        use crate::syntax::expr::parse_expression_snipped;
        use crate::{process, LosslessTreeSink, Parser};
        use rome_js_syntax::{JsAnyExpression, JsExpressionSnipped};

        let source = "(void b)";

        // File id is used for the labels inside parser errors to report them, the file id
        // is used to look up a file's source code and path inside of a codespan `Files` implementation.
        let mut parser = Parser::new(source, 0, SourceType::default());

        // Use one of the syntax parsing functions to parse an expression.
        // This adds node and token events to the parser which are then used to make a node.
        // A completed marker marks the start and end indices in the events vec which signify
        // the Start event, and the Finish event.
        // Completed markers can be turned into an ast node with parse_marker on the parser
        parse_expression_snipped(&mut parser).unwrap();

        // Consume the parser and get its events, then apply them to a tree sink with `process`.
        let (events, tokens, errors) = parser.finish();

        // Make a new text tree sink, its job is assembling events into a rowan GreenNode.
        // At each point (Start, Token, Finish, Error) it also consumes whitespace.
        // Other abstractions can also yield lossy syntax nodes if whitespace is not wanted.
        // Swap this for a LossyTreeSink for a lossy AST result.
        let mut sink = LosslessTreeSink::new(source, &tokens);

        process(&mut sink, events, errors);

        let (untyped_node, errors) = sink.finish();

        assert!(errors.is_empty());
        assert!(JsExpressionSnipped::can_cast(untyped_node.kind()));

        // Convert the untyped SyntaxNode into a typed AST node
        let expression_snipped = JsExpressionSnipped::cast(untyped_node).unwrap();
        let expression = expression_snipped.expression().unwrap();

        match expression {
            JsAnyExpression::JsParenthesizedExpression(parenthesized) => {
                assert_eq!(
                    parenthesized.expression().unwrap().syntax().text(),
                    "void b"
                );
            }
            _ => panic!("Expected parenthesized expression"),
        }
    }

    #[test]
    #[should_panic(
        expected = "Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent."
    )]
    fn uncompleted_markers_panic() {
        let mut parser = Parser::new("'use strict'", 0, SourceType::default());

        let _ = parser.start();
        // drop the marker without calling complete or abandon
    }

    #[test]
    fn completed_marker_doesnt_panic() {
        let mut p = Parser::new("'use strict'", 0, SourceType::default());

        let m = p.start();
        p.expect(JsSyntaxKind::JS_STRING_LITERAL);
        m.complete(&mut p, JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION);
    }

    #[test]
    fn abandoned_marker_doesnt_panic() {
        let mut p = Parser::new("'use strict'", 0, SourceType::default());

        let m = p.start();
        m.abandon(&mut p);
    }
}
