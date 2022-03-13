use crate::token_source::TokenSourceCheckpoint;
use crate::{CompletedMarker, Event, Marker, Parser, TextSize, ToDiagnostic};
use rome_js_syntax::{JsSyntaxKind, TextRange};
use rslint_errors::Diagnostic;

pub(crate) struct RewriteParser<'p, 's> {
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

    pub fn start(&mut self) -> RewriteMarker {
        let pos = self.inner.events.len() as u32;
        self.inner.push_event(Event::tombstone(self.offset));
        RewriteMarker(Marker::new(pos, self.offset))
    }

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

    pub fn finish(mut self) {
        self.skip_trivia(false); // Skip the leading trivia up to the current token.
        assert_eq!(
            self.offset,
            self.inner.tokens.position(),
            "Rewrite didn't consume all tokens"
        );
    }

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
    pub fn range(&self, p: &RewriteParser) -> TextRange {
        self.0.range(p.inner)
    }

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
