use crate::{LexContext, Lexer, LexerCheckpoint, LexerReturn, ReLexContext, TextRange, TokenFlags};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxKind::EOF};
use rslint_errors::Diagnostic;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// The source of tokens for the parser
#[derive(Debug)]
pub struct BufferedLexer<'t> {
    lookahead: VecDeque<Lookahead>,

    /// Stores the information about the current token in case the `lexer` is at least one token ahead.
    current: Option<LexerCheckpoint>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    inner: Lexer<'t>,
}

impl<'t> BufferedLexer<'t> {
    pub fn new(lexer: Lexer<'t>) -> BufferedLexer<'t> {
        BufferedLexer {
            inner: lexer,
            current: None,
            lookahead: VecDeque::new(),
        }
    }

    #[inline(always)]
    pub fn next_token(&mut self, context: LexContext) -> LexerReturn {
        self.current = None;

        if !context.is_regular() {
            self.reset_lookahead();
        } else if let Some(next) = self.lookahead.pop_front() {
            let kind = next.checkpoint.current_kind;
            self.current = Some(next.checkpoint);
            return LexerReturn::new(kind, next.diagnostic);
        }

        self.inner.next_token(context)
    }

    /// Returns the kind of the current token
    #[inline(always)]
    pub fn current(&self) -> JsSyntaxKind {
        if let Some(current) = &self.current {
            current.current_kind
        } else {
            self.inner.current()
        }
    }

    #[inline(always)]
    pub fn current_range(&self) -> TextRange {
        if let Some(current) = &self.current {
            TextRange::new(current.current_start, current.position)
        } else {
            self.inner.current_range()
        }
    }

    /// Tests if there's a line break before the current token.
    #[inline(always)]
    pub fn has_preceding_line_break(&self) -> bool {
        if let Some(current) = &self.current {
            current
                .current_flags
                .contains(TokenFlags::PRECEDING_LINE_BREAK)
        } else {
            self.inner.has_preceding_line_break()
        }
    }

    #[inline]
    pub fn has_unicode_escape(&self) -> bool {
        if let Some(current) = &self.current {
            current.current_flags.contains(TokenFlags::UNICODE_ESCAPE)
        } else {
            self.inner.has_unicode_escape()
        }
    }

    /// Returns the source text
    #[inline]
    pub fn source(&self) -> &'t str {
        self.inner.source()
    }

    pub fn checkpoint(&self) -> LexerCheckpoint {
        if let Some(current) = &self.current {
            current.clone()
        } else {
            self.inner.checkpoint()
        }
    }

    pub fn rewind(&mut self, checkpoint: LexerCheckpoint) {
        if self.inner.position as u32 == u32::from(checkpoint.position) {
            return;
        }

        self.inner.rewind(checkpoint);
        self.lookahead.clear();
        self.current = None;
    }

    fn reset_lookahead(&mut self) {
        if let Some(current) = self.current.take() {
            self.inner.rewind(current);
            self.lookahead.clear();
        }
    }

    /// Re-lexs the current token in the given context
    /// See [Lexer::re_lex]
    pub fn re_lex(&mut self, context: ReLexContext) -> LexerReturn {
        let current_kind = self.current();
        let current_checkpoint = self.inner.checkpoint();

        if let Some(current) = self.current.take() {
            self.inner.rewind(current);
        }

        let LexerReturn {
            kind: new_kind,
            diagnostic,
        } = self.inner.re_lex(context);

        if new_kind != current_kind {
            // The token has changed, clear the lookahead
            self.lookahead.clear();
        } else {
            // It's still the same kind. So let's move the lexer back to the position it was before re-lexing
            // and keep the lookahead as is.
            self.inner.rewind(current_checkpoint);
        }

        LexerReturn::new(new_kind, diagnostic)
    }

    #[inline(always)]
    pub fn lookahead<'s>(&'s mut self) -> LookaheadIterator<'s, 't> {
        LookaheadIterator::new(self)
    }
}

#[derive(Debug)]
pub struct LookaheadIterator<'l, 't> {
    buffered: &'l mut BufferedLexer<'t>,
    nth: usize,
}

impl<'l, 't> LookaheadIterator<'l, 't> {
    fn new(lexer: &'l mut BufferedLexer<'t>) -> Self {
        Self {
            buffered: lexer,
            nth: 0,
        }
    }
}

impl<'l, 't> Iterator for LookaheadIterator<'l, 't> {
    type Item = LookaheadToken;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lookbehind = &self.buffered.lookahead;
        self.nth += 1;

        if let Some(lookbehind) = lookbehind.get(self.nth - 1) {
            let lookahead = LookaheadToken::from(lookbehind);
            return Some(lookahead);
        }

        let lexer = &mut self.buffered.inner;

        if lexer.current() == EOF {
            return None;
        }

        if self.buffered.current.is_none() {
            self.buffered.current = Some(lexer.checkpoint());
        }

        let LexerReturn { diagnostic, .. } = lexer.next_token(LexContext::Regular);

        let checkpoint = lexer.checkpoint();
        self.buffered.lookahead.push_back(Lookahead {
            checkpoint,
            diagnostic,
        });

        Some(LookaheadToken {
            kind: lexer.current(),
            range: lexer.current_range(),
            flags: lexer.current_flags,
        })
    }
}

impl<'l, 't> FusedIterator for LookaheadIterator<'l, 't> {}

// TODO duplicate checkpoint fields. Implement `From<Lexer>` and `Into<Checkpoint>`
#[derive(Debug)]
pub struct Lookahead {
    checkpoint: LexerCheckpoint,
    diagnostic: Option<Box<Diagnostic>>,
}

#[derive(Debug)]
pub struct LookaheadToken {
    kind: JsSyntaxKind,
    range: TextRange,
    flags: TokenFlags,
}

impl LookaheadToken {
    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn has_preceding_line_break(&self) -> bool {
        self.flags.contains(TokenFlags::PRECEDING_LINE_BREAK)
    }
}

impl From<&Lookahead> for LookaheadToken {
    fn from(behind: &Lookahead) -> Self {
        LookaheadToken {
            kind: behind.checkpoint.current_kind,
            range: TextRange::new(behind.checkpoint.current_start, behind.checkpoint.position),
            flags: behind.checkpoint.current_flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::buffered_lexer::BufferedLexer;
    use crate::{LexContext, Lexer, TextRange, TextSize};
    use rome_js_syntax::JsSyntaxKind::{JS_NUMBER_LITERAL, NEWLINE, WHITESPACE};
    use rome_js_syntax::T;

    #[test]
    fn without_lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        buffered.next_token(LexContext::Regular);
        assert_eq!(buffered.current(), T![let]);
        assert!(!buffered.has_preceding_line_break());
        assert_eq!(
            buffered.current_range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![ident]);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, NEWLINE);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);
        assert_eq!(
            buffered.next_token(LexContext::Regular).kind,
            JS_NUMBER_LITERAL
        );
        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![EOF]);
    }

    #[test]
    fn lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        buffered.next_token(LexContext::Regular);
        assert_eq!(buffered.current(), T![let]);
        assert!(!buffered.has_preceding_line_break());
        assert_eq!(
            buffered.current_range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        {
            let lookahead = buffered.lookahead().map(|l| l.kind).collect::<Vec<_>>();

            assert_eq!(
                lookahead,
                vec![
                    WHITESPACE,
                    T![ident],
                    NEWLINE,
                    WHITESPACE,
                    T![=],
                    WHITESPACE,
                    JS_NUMBER_LITERAL,
                    T![EOF]
                ]
            );
        }

        assert_eq!(buffered.current(), T![let]);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);

        {
            let mut lookahead = buffered.lookahead();
            let nth1 = lookahead.next().unwrap();
            let nth2 = lookahead.next().unwrap();
            let nth3 = lookahead.next().unwrap();
            let nth4 = lookahead.next().unwrap();

            assert_eq!(nth1.range().start(), TextSize::from(4));
            assert_eq!(nth1.kind(), T![ident]);
            assert_eq!(nth2.range().start(), TextSize::from(5));
            assert_eq!(nth2.kind(), NEWLINE);
            assert_eq!(nth3.kind(), WHITESPACE);
            assert_eq!(nth4.kind(), T![=]);
            assert!(nth4.has_preceding_line_break());
        }

        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![ident]);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, NEWLINE);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next_token(LexContext::Regular).kind, WHITESPACE);
        assert_eq!(
            buffered.next_token(LexContext::Regular).kind,
            JS_NUMBER_LITERAL
        );
        assert_eq!(buffered.next_token(LexContext::Regular).kind, T![EOF]);
    }
}
