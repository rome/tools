use crate::diagnostic::{expected_token, ParseDiagnostic, ToDiagnostic};
use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::{BumpWithContext, NthToken, TokenSource, Trivia};
use rome_console::fmt::Display;
use rome_diagnostics::location::AsSpan;
use rome_diagnostics::FileId;
use rome_rowan::{SyntaxKind, TextRange, TextSize};
use std::ops::{Deref, DerefMut};

pub mod diagnostic;
pub mod event;
mod marker;
pub mod prelude;
mod token_set;
pub mod token_source;

pub use marker::{CompletedMarker, Marker};
pub use token_set::TokenSet;

pub struct Parser<'source, L: LanguageParser> {
    file_id: FileId,
    source: L::Source<'source>,
    language_parser: L,
    events: Vec<Event<L::Kind>>,
    skipping: bool,
    diagnostics: Vec<ParseDiagnostic>,
}

impl<'source, L> Parser<'source, L>
where
    L: LanguageParser,
{
    pub fn from_language(file_id: FileId, source: L::Source<'source>, language_parser: L) -> Self {
        Self {
            file_id,
            language_parser,
            source,
            skipping: false,
            events: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    /// Gets the source text of a range
    ///
    /// # Panics
    ///
    /// If the range is out of bounds
    pub fn text(&self, span: TextRange) -> &'source str {
        &self.source.text()[span]
    }

    pub fn source(&self) -> &L::Source<'source> {
        &self.source
    }

    pub fn source_mut(&mut self) -> &mut L::Source<'source> {
        &mut self.source
    }

    pub fn events(&self) -> &[Event<L::Kind>] {
        &self.events
    }

    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    pub fn truncate_diagnostics(&mut self, length: usize) {
        self.diagnostics.truncate(length)
    }

    /// Gets the current token kind of the parser
    pub fn cur(&self) -> L::Kind {
        self.source.current()
    }

    /// Gets the range of the current token
    pub fn cur_range(&self) -> TextRange {
        self.source.current_range()
    }

    /// Get the source code of the parser's current token.
    pub fn cur_text(&self) -> &'source str {
        &self.source.text()[self.cur_range()]
    }

    /// Checks if the parser is currently at a specific token
    pub fn at(&self, kind: L::Kind) -> bool {
        self.cur() == kind
    }

    /// Check if the parser's current token is contained in a token set
    pub fn at_ts(&self, kinds: TokenSet<L::Kind>) -> bool {
        kinds.contains(self.cur())
    }

    /// Consume the current token if `kind` matches.
    pub fn bump(&mut self, kind: L::Kind) {
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
    pub fn bump_remap(&mut self, kind: L::Kind) {
        self.do_bump(kind);
    }

    /// Bumps the current token regardless of its kind and advances to the next token.
    pub fn bump_any(&mut self) {
        let kind = self.cur();
        assert_ne!(kind, L::EOF);

        self.do_bump(kind);
    }

    fn do_bump(&mut self, kind: L::Kind) {
        let kind = match self.language_parser.validate_token(kind, self) {
            Ok(kind) => kind,
            Err((kind, diagnostic)) => {
                self.error(diagnostic);
                kind
            }
        };

        self.push_token(kind, self.cur_range().end());

        if self.skipping {
            self.source.skip_as_trivia();
        } else {
            self.source.bump();
        }
    }

    /// Consume the next token if `kind` matches.
    #[inline]
    pub fn eat(&mut self, kind: L::Kind) -> bool {
        if !self.at(kind) {
            return false;
        }

        self.do_bump(kind);

        true
    }

    /// Try to eat a specific token kind, if the kind is not there then adds an error to the events stack.
    pub fn expect(&mut self, kind: L::Kind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(expected_token(kind));
            false
        }
    }

    /// Allows parsing an unsupported syntax as skipped trivia tokens.
    pub fn parse_as_skipped_trivia_tokens<P>(&mut self, parse: P)
    where
        P: FnOnce(&mut Self),
    {
        let events_pos = self.events.len();
        self.skipping = true;
        parse(self);
        self.skipping = false;

        // Truncate any start/finish events
        self.events.truncate(events_pos);
    }

    /// Add a diagnostic
    pub fn error(&mut self, err: impl ToDiagnostic<L>) {
        let err = err.into_diagnostic(self);

        // Don't report another diagnostic if the last diagnostic is at the same position of the current one
        if let Some(previous) = self.diagnostics.last() {
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
        self.diagnostics.push(err)
    }

    /// Creates a new diagnostic. Pass the message and the range where the error occurred
    #[must_use]
    pub fn err_builder(&self, message: impl Display, span: impl AsSpan) -> ParseDiagnostic {
        ParseDiagnostic::new(self.file_id, message, span)
    }

    /// Bump and add an error event
    pub fn err_and_bump(&mut self, err: impl ToDiagnostic<L>, unknown_syntax_kind: L::Kind) {
        let m = self.start();
        self.bump_any();
        m.complete(self, unknown_syntax_kind);
        self.error(err);
    }

    /// Returns the kind of the last bumped token.
    pub fn last(&self) -> Option<L::Kind> {
        self.events.iter().rev().find_map(|event| match event {
            Token { kind, .. } => Some(*kind),
            _ => None,
        })
    }

    /// Returns the end offset of the last bumped token.
    pub fn last_end(&self) -> Option<TextSize> {
        self.events.iter().rev().find_map(|event| match event {
            Token { end, .. } => Some(*end),
            _ => None,
        })
    }

    /// Starts a new node in the syntax tree. All nodes and tokens
    /// consumed between the `start` and the corresponding `Marker::complete`
    /// belong to the same node.
    pub fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        let start = self.source.position();
        self.push_event(Event::tombstone());
        Marker::new(pos, start)
    }

    pub fn push_token(&mut self, kind: L::Kind, end: TextSize) {
        self.push_event(Token { kind, end });
    }

    pub fn push_event(&mut self, event: Event<L::Kind>) {
        self.events.push(event)
    }

    /// Splits the events into two at the given `position`. Returns a newly allocated vector containing the
    /// elements from `[position;len]`.
    ///
    /// ## Safety
    /// The method is marked as `unsafe` to discourage its usage. Removing events can lead to
    /// corrupted events if not done carefully.
    pub unsafe fn split_off_events(&mut self, position: usize) -> Vec<Event<L::Kind>> {
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

    /// Consume the parser and returns the list of events, the source text's trivia, and the diagnostics.
    pub fn finish(mut self) -> (Vec<Event<L::Kind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
        let (trivia, source_diagnostics) = self.source.finish();
        self.diagnostics.extend(source_diagnostics);
        (self.events, trivia, self.diagnostics)
    }
}

impl<'source, L> Deref for Parser<'source, L>
where
    L: LanguageParser,
{
    type Target = L;

    fn deref(&self) -> &Self::Target {
        &self.language_parser
    }
}

impl<'source, L> DerefMut for Parser<'source, L>
where
    L: LanguageParser,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.language_parser
    }
}

impl<'source, L> Parser<'source, L>
where
    L: LanguageParser,
    L::Source<'source>: BumpWithContext<'source>,
{
    /// Consumes the current token if `kind` matches and lexes the next token using the
    /// specified `context.
    pub fn bump_with_context(
        &mut self,
        kind: L::Kind,
        context: <L::Source<'source> as BumpWithContext<'source>>::Context,
    ) {
        assert_eq!(
            kind,
            self.cur(),
            "expected {:?} but at {:?}",
            kind,
            self.cur()
        );

        let kind = match self.language_parser.validate_token(kind, self) {
            Ok(kind) => kind,
            Err((kind, diagnostic)) => {
                self.error(diagnostic);
                kind
            }
        };

        self.push_token(kind, self.cur_range().end());

        if self.skipping {
            self.source.skip_as_trivia_with_context(context);
        } else {
            self.source.bump_with_context(context);
        }
    }
}

impl<'source, L> Parser<'source, L>
where
    L: LanguageParser,
    L::Source<'source>: NthToken<'source>,
{
    /// Look ahead at a token and get its kind.
    pub fn nth(&mut self, n: usize) -> L::Kind
    where
        L::Source<'source>: NthToken<'source>,
    {
        self.source.nth(n)
    }

    /// Checks if a token lookahead is something
    pub fn nth_at(&mut self, n: usize, kind: L::Kind) -> bool
    where
        L::Source<'source>: NthToken<'source>,
    {
        self.nth(n) == kind
    }
}

pub trait LanguageParser {
    type Kind: SyntaxKind;
    type Source<'source>: TokenSource<'source, Kind = Self::Kind>;

    const EOF: Self::Kind;

    fn validate_token(
        &self,
        kind: Self::Kind,
        _p: &Parser<Self>,
    ) -> Result<Self::Kind, (Self::Kind, ParseDiagnostic)>
    where
        Self: Sized,
    {
        Ok(kind)
    }
}

pub trait Rewind {
    type Checkpoint;

    #[must_use]
    fn checkpoint(&self) -> Self::Checkpoint;

    fn rewind(&mut self, checkpoint: Self::Checkpoint);
}

impl<'source, L> Rewind for Parser<'source, L>
where
    L: Rewind + LanguageParser,
    L::Source<'source>: Rewind,
{
    type Checkpoint = ParserCheckpoint<L::Checkpoint, <L::Source<'source> as Rewind>::Checkpoint>;

    /// Rewind the parser back to a previous position in time
    fn rewind(&mut self, checkpoint: Self::Checkpoint) {
        let Self::Checkpoint {
            token_source,
            event_pos,
            errors_pos,
            language_checkpoint,
        } = checkpoint;
        self.source_mut().rewind(token_source);
        self.drain_events(self.cur_event_pos() - event_pos);
        self.diagnostics.truncate(errors_pos as usize);
        self.language_parser.rewind(language_checkpoint);
    }

    /// Get a checkpoint representing the progress of the parser at this point in time
    #[must_use]
    fn checkpoint(&self) -> Self::Checkpoint {
        Self::Checkpoint {
            token_source: self.source().checkpoint(),
            event_pos: self.cur_event_pos(),
            errors_pos: self.diagnostics.len() as u32,
            language_checkpoint: self.language_parser.checkpoint(),
        }
    }
}

/// A structure signifying the Parser progress at one point in time
#[derive(Debug)]
pub struct ParserCheckpoint<LanguageCheckpoint, SourceCheckpoint> {
    event_pos: usize,
    /// The length of the errors list at the time the checkpoint was created.
    /// Safety: The parser only supports files <= 4Gb. Storing a `u32` is sufficient to store one error
    /// for each single character in the file, which should be sufficient for any realistic file.
    errors_pos: u32,
    language_checkpoint: LanguageCheckpoint,
    token_source: SourceCheckpoint,
}

impl<L, S> ParserCheckpoint<L, S> {
    pub fn event_position(&self) -> usize {
        self.event_pos
    }

    pub fn take_token_source_checkpoint(self) -> S {
        self.token_source
    }
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
