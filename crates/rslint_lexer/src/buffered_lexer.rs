use crate::{LexContext, LexedToken, Lexer, LexerCheckpoint, ReLexContext, TextRange, TokenFlags};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxKind::EOF};
use rslint_errors::Diagnostic;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// Wrapper around a [Lexer] that supports lookahead.
///
/// The underlying [Lexer] only supports inspecting the current token and lexing the next token.
/// However, the current token is often not enough for the Parser to decide what the next node is,
/// it often needs information about the next non-trivia tokens.
///
/// The [BufferedLexer] adds support for lookahead by caching the lexed tokens and keeping track
/// of the current position (and what the `nth` token is). This means, that every token
/// only gets lexed once except if the buffer cached some lookahead tokens and:
///
/// * `next_token` is called with a context other than [LexContext::default()].
/// * the lexer gets rewinded to a previous position
/// * re-lexing the current token changes the kind of the token. That means,
///   that any following token may turn out to be different as well, thus, it's necessary to clear the
///   lookahead cache.
#[derive(Debug)]
pub struct BufferedLexer<'l> {
    /// Cache storing the lookahead tokens. That are, all tokens between the `current` token and
    /// the "current" of the [Lexer]. This is because the [Lexer]'s current token points to the
    /// furthest requested lookahead token.
    ///
    /// For example for the following source `let a = 2;`. The `current` token of the inner [Lexer] and
    /// of the [BufferedLexer] after one call to `next_token` is the `let` token. However, the `current`
    /// token diverges if the [BufferedLexer] performs lookahead. Let's say you do a lookahead of 4 (`=` token).
    /// Now, the [BufferedLexer] calls [Lexer::next_token] four times, moving the [Lexer]'s `current`
    /// token to the `=`. However, the `current` of the [BufferedLexer] still points to the `let` token.
    /// That's why the [BufferedLexer] stores the following information:
    /// * `current`: `let` (information about the `current` token from the consumer perspective)
    /// * `lookahead`: [WHITESPACE, IDENT: 'a', WHITESPACE]. The tokens that have been lexed to
    ///    answer the "lookahead 4" request but haven't been returned yet.
    /// * [Lexer::current]: Points to `=`
    lookahead: VecDeque<Lookahead>,

    /// Stores the information of the current token in case the `lexer` is at least one token ahead.
    current: Option<LexerCheckpoint>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    inner: Lexer<'l>,
}

impl<'l> BufferedLexer<'l> {
    /// Creates a new [BufferedLexer] wrapping the passed in [Lexer].
    pub fn new(lexer: Lexer<'l>) -> Self {
        Self {
            inner: lexer,
            current: None,
            lookahead: VecDeque::new(),
        }
    }

    /// Returns the kind of the next token and any associated diagnostic.
    ///
    /// [See `Lexer.next_token`](Lexer::next_token)
    #[inline(always)]
    pub fn next_token(&mut self, context: LexContext) -> LexedToken {
        // Reset the lookahead if the context isn't the regular context because it's highly likely
        // that the lexer will return a different token.
        if !context.is_regular() {
            self.reset_lookahead();
        }
        // Retrieve the next token from the lookahead cache if it isn't empty
        else if let Some(next) = self.lookahead.pop_front() {
            let kind = next.checkpoint.current_kind;

            // Store the lookahead as the current token if the lookahead isn't empty (in which case,
            // the lexer is still at least one token ahead).

            if self.lookahead.is_empty() {
                self.current = None;
            } else {
                self.current = Some(next.checkpoint);
            }

            return LexedToken::new(kind, next.diagnostic);
        }

        // The [BufferedLexer] and [Lexer] are now both at the same position. Clear the cached
        // current token and lex out the next token.
        self.current = None;
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

    /// Returns the range of the current token
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
            current.has_preceding_line_break()
        } else {
            self.inner.has_preceding_line_break()
        }
    }

    /// Returns true if the current token is an identifier, and it contains any unicode escape sequences
    #[inline]
    pub fn has_unicode_escape(&self) -> bool {
        if let Some(current) = &self.current {
            current.has_unicode_escape()
        } else {
            self.inner.has_unicode_escape()
        }
    }

    /// Returns the source text
    #[inline]
    pub fn source(&self) -> &'l str {
        self.inner.source()
    }

    /// Creates a checkpoint representing the current lexer state. Allows rewinding
    /// the lexer to this position later on.
    pub fn checkpoint(&self) -> LexerCheckpoint {
        if let Some(current) = &self.current {
            current.clone()
        } else {
            self.inner.checkpoint()
        }
    }

    /// Rewinds the lexer to the state stored in the checkpoint.
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

    /// Re-lex the current token in the given context
    /// See [Lexer::re_lex]
    pub fn re_lex(&mut self, context: ReLexContext) -> LexedToken {
        let current_kind = self.current();
        let current_checkpoint = self.inner.checkpoint();

        if let Some(current) = self.current.take() {
            self.inner.rewind(current);
        }

        let LexedToken {
            kind: new_kind,
            diagnostic,
        } = self.inner.re_lex(context);

        if new_kind != current_kind {
            // The token has changed, clear the lookahead
            self.lookahead.clear();
        } else if !self.lookahead.is_empty() {
            // It's still the same kind. So let's move the lexer back to the position it was before re-lexing
            // and keep the lookahead as is.
            self.current = Some(self.inner.checkpoint());
            self.inner.rewind(current_checkpoint);
        }

        LexedToken::new(new_kind, diagnostic)
    }

    /// Returns an iterator over the tokens following the current token to perform lookahead.
    /// For example, what's the 3rd token after the current token?
    #[inline(always)]
    pub fn lookahead<'s>(&'s mut self) -> LookaheadIterator<'s, 'l> {
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

        // Is the `nth` token already in the cache, then return it
        if let Some(lookbehind) = lookbehind.get(self.nth - 1) {
            let lookahead = LookaheadToken::from(lookbehind);
            return Some(lookahead);
        }

        let lexer = &mut self.buffered.inner;

        // We're already at the end, calling next now only returns `EOF` again. End the iterator.
        if lexer.current() == EOF {
            return None;
        }

        // Store the current token before moving the inner lexer forward if we haven't done so.
        // Necessary to prevent that [BufferedLexer::current] moves forward when performing lookahead.
        if self.buffered.current.is_none() {
            self.buffered.current = Some(lexer.checkpoint());
        }

        let LexedToken { diagnostic, .. } = lexer.next_token(LexContext::default());

        // Lex the next token and cache it in the lookahead cache. Needed to cache it right away
        // because of the diagnostic.
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

#[derive(Debug)]
pub struct Lookahead {
    /// Checkpoint containing the information about the token.
    checkpoint: LexerCheckpoint,
    /// Associated diagnostic for this lookahead token
    diagnostic: Option<Box<Diagnostic>>,
}

impl Lookahead {
    fn kind(&self) -> JsSyntaxKind {
        self.checkpoint.current_kind
    }

    fn range(&self) -> TextRange {
        TextRange::new(self.checkpoint.current_start, self.checkpoint.position)
    }

    fn flags(&self) -> TokenFlags {
        self.checkpoint.current_flags
    }
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
        self.flags.has_preceding_line_break()
    }
}

impl From<&Lookahead> for LookaheadToken {
    fn from(ahead: &Lookahead) -> Self {
        LookaheadToken {
            kind: ahead.kind(),
            range: ahead.range(),
            flags: ahead.flags(),
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

        buffered.next_token(LexContext::default());
        assert_eq!(buffered.current(), T![let]);
        assert!(!buffered.has_preceding_line_break());
        assert_eq!(
            buffered.current_range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::default()).kind, T![ident]);
        assert_eq!(buffered.next_token(LexContext::default()).kind, NEWLINE);
        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::default()).kind, T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);
        assert_eq!(
            buffered.next_token(LexContext::default()).kind,
            JS_NUMBER_LITERAL
        );
        assert_eq!(buffered.next_token(LexContext::default()).kind, T![EOF]);
    }

    #[test]
    fn lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        buffered.next_token(LexContext::default());
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
        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);

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

        assert_eq!(buffered.next_token(LexContext::default()).kind, T![ident]);
        assert_eq!(buffered.next_token(LexContext::default()).kind, NEWLINE);
        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);
        assert_eq!(buffered.next_token(LexContext::default()).kind, T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next_token(LexContext::default()).kind, WHITESPACE);
        assert_eq!(
            buffered.next_token(LexContext::default()).kind,
            JS_NUMBER_LITERAL
        );
        assert_eq!(buffered.next_token(LexContext::default()).kind, T![EOF]);
    }
}
