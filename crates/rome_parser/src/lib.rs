use crate::diagnostic::{expected_token, ParseDiagnostic, ToDiagnostic};
use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::{BumpWithContext, NthToken, TokenSource};
use rome_console::fmt::Display;
use rome_diagnostics::location::AsSpan;
use rome_diagnostics::FileId;
use rome_rowan::{SyntaxKind, TextRange, TextSize};

pub mod diagnostic;
pub mod event;
mod marker;
pub mod prelude;
mod token_set;
pub mod token_source;

pub use marker::{CompletedMarker, Marker};
pub use token_set::TokenSet;

pub struct ParserContext<K: SyntaxKind> {
    file_id: FileId,
    events: Vec<Event<K>>,
    skipping: bool,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<K: SyntaxKind> ParserContext<K> {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            skipping: false,
            events: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn events(&self) -> &[Event<K>] {
        &self.events
    }

    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    pub fn truncate_diagnostics(&mut self, at: usize) {
        self.diagnostics.truncate(at);
    }

    pub fn push_token(&mut self, kind: K, end: TextSize) {
        self.push_event(Token { kind, end });
    }

    pub fn push_event(&mut self, event: Event<K>) {
        self.events.push(event)
    }

    pub fn is_skipping(&self) -> bool {
        self.skipping
    }

    /// Splits the events into two at the given `position`. Returns a newly allocated vector containing the
    /// elements from `[position;len]`.
    ///
    /// ## Safety
    /// The method is marked as `unsafe` to discourage its usage. Removing events can lead to
    /// corrupted events if not done carefully.
    pub unsafe fn split_off_events(&mut self, position: usize) -> Vec<Event<K>> {
        self.events.split_off(position)
    }

    /// Get the current index of the last event
    fn cur_event_pos(&self) -> usize {
        self.events.len().saturating_sub(1)
    }

    /// Remove `amount` events from the parser
    fn drain_events(&mut self, amount: usize) {
        self.events.truncate(self.events.len() - amount);
    }

    /// Rewind the parser back to a previous position in time
    pub fn rewind(&mut self, checkpoint: ParserContextCheckpoint) {
        let ParserContextCheckpoint {
            event_pos,
            errors_pos,
        } = checkpoint;
        self.drain_events(self.cur_event_pos() - event_pos);
        self.diagnostics.truncate(errors_pos as usize);
    }

    /// Get a checkpoint representing the progress of the parser at this point of time
    #[must_use]
    pub fn checkpoint(&self) -> ParserContextCheckpoint {
        ParserContextCheckpoint {
            event_pos: self.cur_event_pos(),
            errors_pos: self.diagnostics.len() as u32,
        }
    }

    pub fn finish(self) -> (Vec<Event<K>>, Vec<ParseDiagnostic>) {
        (self.events, self.diagnostics)
    }
}

/// A structure signifying the Parser progress at one point in time
#[derive(Debug)]
pub struct ParserContextCheckpoint {
    event_pos: usize,
    /// The length of the errors list at the time the checkpoint was created.
    /// Safety: The parser only supports files <= 4Gb. Storing a `u32` is sufficient to store one error
    /// for each single character in the file, which should be sufficient for any realistic file.
    errors_pos: u32,
}

impl ParserContextCheckpoint {
    pub fn event_position(&self) -> usize {
        self.event_pos
    }
}

pub trait Parser<'source>: Sized {
    type Kind: SyntaxKind;
    type Source: TokenSource<'source, Kind = Self::Kind>;

    const EOF: Self::Kind;

    fn context(&self) -> &ParserContext<Self::Kind>;

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind>;

    fn source(&self) -> &Self::Source;

    fn source_mut(&mut self) -> &mut Self::Source;

    /// Gets the source text of a range
    ///
    /// # Panics
    ///
    /// If the range is out of bounds
    fn text(&self, span: TextRange) -> &'source str {
        &self.source().text()[span]
    }

    /// Gets the current token kind of the parser
    fn cur(&self) -> Self::Kind {
        self.source().current()
    }

    /// Gets the range of the current token
    fn cur_range(&self) -> TextRange {
        self.source().current_range()
    }

    /// Get the source code of the parser's current token.
    fn cur_text(&self) -> &'source str {
        &self.source().text()[self.cur_range()]
    }

    /// Checks if the parser is currently at a specific token
    fn at(&self, kind: Self::Kind) -> bool {
        self.cur() == kind
    }

    /// Check if the parser's current token is contained in a token set
    fn at_ts(&self, kinds: TokenSet<Self::Kind>) -> bool {
        kinds.contains(self.cur())
    }

    /// Look ahead at a token and get its kind.
    fn nth(&mut self, n: usize) -> Self::Kind
    where
        Self::Source: NthToken<'source>,
    {
        self.source_mut().nth(n)
    }

    /// Checks if a token lookahead is something
    fn nth_at(&mut self, n: usize, kind: Self::Kind) -> bool
    where
        Self::Source: NthToken<'source>,
    {
        self.nth(n) == kind
    }

    /// Consume the current token if `kind` matches.
    fn bump(&mut self, kind: Self::Kind) {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        self.do_bump(kind)
    }

    /// Consume any token but cast it as a different kind
    fn bump_remap(&mut self, kind: Self::Kind) {
        self.do_bump(kind);
    }

    /// Bumps the current token regardless of its kind and advances to the next token.
    fn bump_any(&mut self) {
        let kind = self.cur();
        assert_ne!(kind, Self::EOF);

        self.do_bump(kind);
    }

    /// Consumes the current token if `kind` matches and lexes the next token using the
    /// specified `context.
    fn bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext<'source>>::Context,
    ) where
        Self::Source: BumpWithContext<'source>,
    {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        self.do_bump_with_context(kind, context);
    }

    fn do_bump_with_context(
        &mut self,
        kind: Self::Kind,
        context: <Self::Source as BumpWithContext<'source>>::Context,
    ) where
        Self::Source: BumpWithContext<'source>,
    {
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().skipping {
            self.source_mut().skip_as_trivia_with_context(context);
        } else {
            self.source_mut().bump_with_context(context);
        }
    }

    fn do_bump(&mut self, kind: Self::Kind) {
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().skipping {
            self.source_mut().skip_as_trivia();
        } else {
            self.source_mut().bump();
        }
    }

    /// Consume the next token if `kind` matches.
    fn eat(&mut self, kind: Self::Kind) -> bool {
        if !self.at(kind) {
            return false;
        }

        self.do_bump(kind);

        true
    }

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
    fn expect(&mut self, kind: Self::Kind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(expected_token(kind));
            false
        }
    }

    /// Allows parsing an unsupported syntax as skipped trivia tokens.
    fn parse_as_skipped_trivia_tokens<P>(&mut self, parse: P)
    where
        P: FnOnce(&mut Self),
    {
        let events_pos = self.context().events.len();
        self.context_mut().skipping = true;
        parse(self);
        self.context_mut().skipping = false;

        // Truncate any start/finish events
        self.context_mut().events.truncate(events_pos);
    }

    /// Add a diagnostic
    fn error(&mut self, err: impl ToDiagnostic<Self>) {
        let err = err.into_diagnostic(self);

        // Don't report another diagnostic if the last diagnostic is at the same position of the current one
        if let Some(previous) = self.context().diagnostics.last() {
            if previous.file_id == err.file_id {
                match (&err.diagnostic_range(), &previous.diagnostic_range()) {
                    (Some(err_range), Some(previous_range))
                        if err_range.start() == previous_range.start() =>
                    {
                        return;
                    }
                    _ => {}
                }
            }
        }
        self.context_mut().diagnostics.push(err)
    }

    /// Creates a new diagnostic. Pass the message and the range where the error occurred
    #[must_use]
    fn err_builder(&self, message: impl Display, span: impl AsSpan) -> ParseDiagnostic {
        ParseDiagnostic::new(self.context().file_id, message, span)
    }

    /// Bump and add an error event
    fn err_and_bump(&mut self, err: impl ToDiagnostic<Self>, unknown_syntax_kind: Self::Kind) {
        let m = self.start();
        self.bump_any();
        m.complete(self, unknown_syntax_kind);
        self.error(err);
    }

    /// Returns the kind of the last bumped token.
    fn last(&self) -> Option<Self::Kind> {
        self.context()
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { kind, .. } => Some(*kind),
                _ => None,
            })
    }

    /// Returns the end offset of the last bumped token.
    fn last_end(&self) -> Option<TextSize> {
        self.context()
            .events
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { end, .. } => Some(*end),
                _ => None,
            })
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    fn start(&mut self) -> Marker {
        let pos = self.context().events.len() as u32;
        let start = self.source().position();
        self.context_mut().push_event(Event::tombstone());
        Marker::new(pos, start)
    }
}

pub trait Rewind {
    type Checkpoint;

    #[must_use]
    fn checkpoint(&self) -> Self::Checkpoint;

    fn rewind(&mut self, checkpoint: Self::Checkpoint);
}

/// An abstraction for syntax tree implementations
pub trait TreeSink {
    type Kind: SyntaxKind;

    /// Adds new token to the current branch.
    fn token(&mut self, kind: Self::Kind, end: TextSize);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: Self::Kind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    /// Emit errors
    fn errors(&mut self, errors: Vec<ParseDiagnostic>);
}
