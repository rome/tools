use crate::token_source::TokenSourceCheckpoint;
use crate::{CompletedMarker, Event, Marker, Parser, TextSize, ToDiagnostic};
use rome_js_syntax::{JsSyntaxKind, TextRange};
use rslint_errors::Diagnostic;

/// Simplified parser API for when rewriting the AST structure with `rewrite_events`.
///
/// The difference from the regular [Parser] is that the `TokenSource` must be detached during
/// rewriting to avoid lexing previously lexed tokens in a different context. For example for `a[`test`] = "b"`.
/// Template literal elements get lexed in the `TemplateElement` context. However, if the rewriter
/// rewinds the token source then all tokens are lexed in the `LexMode::Regular` which yields
/// complete different results.
///
/// This is why the [RewriteParser] tracks the source offset without relying on the `TokenSource`
/// and explicitly passes the positions to [Marker] and [CompletedMarker]. This further has the
/// benefit that rewriting the events doesn't require re-lexing all tokens as well.
pub(crate) struct RewriteParser<'p, 's> {
    /// The byte offset of the current token from the start of the source
    offset: TextSize,
    inner: &'p mut Parser<'s>,
    trivia_offset: usize,
}

impl<'p, 's> RewriteParser<'p, 's> {
    pub fn new(p: &'p mut Parser<'s>, checkpoint: TokenSourceCheckpoint) -> Self {
        Self {
            inner: p,
            offset: checkpoint.current_start(),
            trivia_offset: checkpoint.trivia_position(),
        }
    }

    /// Starts a marker for a new node.
    pub fn start(&mut self) -> RewriteMarker {
        let pos = self.inner.events.len() as u32;
        self.skip_trivia(false);
        self.inner.push_event(Event::tombstone(self.offset));
        RewriteMarker(Marker::new(pos, self.offset))
    }

    /// Bumps the passed in token
    pub fn bump(&mut self, token: RewriteToken) {
        self.skip_trivia(false);
        self.inner
            .push_token(token.kind, TextRange::at(self.offset, token.length));
        self.offset += token.length;
        self.skip_trivia(true);
    }

    fn skip_trivia(&mut self, trailing: bool) {
        let remaining_trivia = &self.inner.tokens.trivia[self.trivia_offset..];
        for trivia in remaining_trivia {
            if trailing != trivia.trailing() || self.offset != trivia.offset() {
                break;
            }

            self.trivia_offset += 1;
            self.offset += trivia.len();
        }
    }

    /// Finishes the rewriter
    ///
    /// ## Panics
    /// If not all tokens have been consumed or if they have been consumed out of order
    pub fn finish(mut self) {
        self.skip_trivia(false); // Skip the leading trivia up to the current token.
        assert_eq!(
            self.offset,
            self.inner.tokens.position(),
            "Rewrite didn't consume all tokens"
        );
    }

    /// Returns true if the parser is in strict mode
    pub fn is_strict_mode(&self) -> bool {
        self.inner.state.strict().is_some()
    }

    pub fn err_builder(&self, message: &str) -> Diagnostic {
        self.inner.err_builder(message)
    }

    pub fn error(&mut self, diagnostic: impl ToDiagnostic) {
        self.inner.error(diagnostic)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct RewriteToken {
    pub(crate) kind: JsSyntaxKind,
    length: TextSize,
}

impl RewriteToken {
    pub fn new(kind: JsSyntaxKind, length: TextSize) -> Self {
        Self { kind, length }
    }
}

#[derive(Debug)]
pub(crate) struct RewriteMarker(Marker);

impl RewriteMarker {
    /// Completes the node with the specified kind
    pub fn complete(self, p: &mut RewriteParser, kind: JsSyntaxKind) -> RewriteCompletedMarker {
        let mut end_pos = p.inner.last_range().map(|t| t.end()).unwrap_or_default();
        if end_pos < self.0.start {
            end_pos = p.offset;
        }

        RewriteCompletedMarker(self.0.complete_at(p.inner, kind, end_pos))
    }
}

#[derive(Debug)]
pub(crate) struct RewriteCompletedMarker(CompletedMarker);

impl RewriteCompletedMarker {
    /// Returns the range of the marker
    pub fn range(&self, p: &RewriteParser) -> TextRange {
        self.0.range(p.inner)
    }

    /// Returns the source text of the marker
    pub fn text<'a>(&self, p: &'a RewriteParser) -> &'a str {
        self.0.text(p.inner)
    }

    pub fn change_to_unknown(&mut self, p: &mut RewriteParser) {
        self.0.change_to_unknown(p.inner)
    }
}

impl From<RewriteCompletedMarker> for CompletedMarker {
    fn from(inner: RewriteCompletedMarker) -> Self {
        inner.0
    }
}
