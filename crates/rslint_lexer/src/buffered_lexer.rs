use crate::{LexMode, Lexer, LexerReturn, LexerState, TextRange, TextSize};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxKind::EOF};
use rslint_errors::Diagnostic;
use std::collections::VecDeque;
use std::iter::FusedIterator;

/// The source of tokens for the parser
#[derive(Debug)]
pub struct BufferedLexer<'t> {
    /// Stores the information about the tokens between the `lexer`s current token and
    /// current position of the buffered token.
    lookbehind: VecDeque<Lookbehind>,

    /// Underlying lexer. May be ahead if iterated with lookahead
    lexer: Lexer<'t>,
}

impl<'t> BufferedLexer<'t> {
    pub fn new(lexer: Lexer<'t>) -> BufferedLexer<'t> {
        BufferedLexer {
            lexer,
            lookbehind: VecDeque::new(),
        }
    }

    #[inline(always)]
    pub fn next(&mut self) -> LexerReturn {
        self.lookbehind.pop_front();

        if let Some(next) = self.lookbehind.front_mut() {
            // Fine to take the diagnostic, it isn't needed anymore. Avoids cloning
            let diagnostic = next.diagnostic.take();
            LexerReturn::new(next.kind, diagnostic)
        } else {
            let lexer = &mut self.lexer;
            let result = lexer.next();
            self.lookbehind.push_back(Lookbehind {
                kind: result.kind,
                range: lexer.current_range(),
                after_newline: lexer.has_preceding_line_break(),
                diagnostic: None, // diagnostic has already been consumed.
            });

            result
        }
    }

    /// Returns the kind of the current token
    #[inline(always)]
    pub fn current(&self) -> JsSyntaxKind {
        if let Some(current) = &self.lookbehind.front() {
            current.kind
        } else {
            self.lexer.current()
        }
    }

    #[inline(always)]
    pub fn current_range(&self) -> TextRange {
        if let Some(current) = &self.lookbehind.front() {
            current.range
        } else {
            self.lexer.current_range()
        }
    }

    /// Tests if there's a line break before the current token.
    #[inline(always)]
    pub fn has_preceding_line_break(&self) -> bool {
        if let Some(current) = &self.lookbehind.front() {
            current.after_newline
        } else {
            self.lexer.has_preceding_line_break()
        }
    }

    /// Returns the source text
    #[inline]
    pub fn source(&self) -> &'t str {
        self.lexer.source()
    }

    pub fn checkpoint(&self) -> BufferedLexerCheckpoint {
        if let Some(current) = self.lookbehind.front() {
            BufferedLexerCheckpoint {
                position: current.range.end(),
                current_start: current.range.start(),
                current_kind: current.kind,
                current_after_new_line: current.after_newline,
                // TODO take correct lexer state
                // state: self.lexer.state.clone(),
            }
        } else {
            BufferedLexerCheckpoint {
                position: TextSize::from(self.lexer.position as u32),
                current_start: TextSize::from(self.lexer.current_start as u32),
                current_kind: self.lexer.current_kind,
                current_after_new_line: self.lexer.current_after_new_line,
                // state: self.lexer.state.clone(),
            }
        }
    }

    pub fn rewind(&mut self, checkpoint: BufferedLexerCheckpoint) {
        assert!(self.lexer.position >= u32::from(checkpoint.position) as usize);
        self.lexer.position = u32::from(checkpoint.position) as usize;
        self.lexer.current_start = u32::from(checkpoint.current_start) as usize;
        self.lexer.current_kind = checkpoint.current_kind;
        self.lexer.current_after_new_line = checkpoint.current_after_new_line;

        self.lookbehind.clear();

        self.lookbehind.push_back(Lookbehind {
            kind: self.lexer.current(),
            range: self.lexer.current_range(),
            after_newline: self.lexer.has_preceding_line_break(),
            diagnostic: None, // Not needed for the current token
        });
    }

    pub fn re_lex(&mut self, mode: LexMode) -> LexerReturn {
        if let Some(current) = self.lookbehind.pop_front() {
            self.lexer.current_start = u32::from(current.range.start()) as usize;
            self.lexer.position = u32::from(current.range.end()) as usize;
            // TODO restore `after_newline` too?
            self.lexer.current_after_new_line = current.after_newline;
            self.lexer.current_kind = current.kind;
        }

        self.lookbehind.clear();
        let result = self.lexer.re_lex(mode);

        self.lookbehind.push_back(Lookbehind {
            kind: result.kind,
            range: self.lexer.current_range(),
            after_newline: self.lexer.has_preceding_line_break(),
            diagnostic: None, // diagnostic has already been consumed.
        });

        result
    }

    #[inline(always)]
    pub fn lookahead<'s>(&'s mut self) -> LookaheadIterator<'s, 't> {
        LookaheadIterator::new(self)
    }
}

#[derive(Debug)]
pub struct LookaheadIterator<'l, 't> {
    buffered: &'l mut BufferedLexer<'t>,
    nth: u32,
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
    type Item = Lookahead;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let lookbehind = &self.buffered.lookbehind;
        self.nth += 1;

        if let Some(lookbehind) = lookbehind.get(self.nth as usize) {
            let lookahead = Lookahead::from(lookbehind);
            return Some(lookahead);
        }

        let lexer = &mut self.buffered.lexer;
        if lexer.current() == EOF {
            return None;
        }

        let lex_return = lexer.next();

        let lookbehind = Lookbehind {
            kind: lex_return.kind,
            range: lexer.current_range(),
            after_newline: lexer.has_preceding_line_break(),
            diagnostic: lex_return.diagnostic,
        };

        let lookahead = Lookahead::from(&lookbehind);
        self.buffered.lookbehind.push_back(lookbehind);
        self.nth += 1;
        Some(lookahead)
    }
}

impl<'l, 't> FusedIterator for LookaheadIterator<'l, 't> {}

#[derive(Debug)]
pub struct Lookbehind {
    kind: JsSyntaxKind,
    range: TextRange,
    after_newline: bool,
    diagnostic: Option<Box<Diagnostic>>,
}

#[derive(Debug)]
pub struct Lookahead {
    kind: JsSyntaxKind,
    range: TextRange,
    after_newline: bool,
}

impl Lookahead {
    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn has_preceding_line_break(&self) -> bool {
        self.after_newline
    }
}

impl From<&Lookbehind> for Lookahead {
    fn from(behind: &Lookbehind) -> Self {
        Lookahead {
            kind: behind.kind,
            range: behind.range,
            after_newline: behind.after_newline,
        }
    }
}

#[derive(Debug)]
pub struct BufferedLexerCheckpoint {
    position: TextSize,
    current_start: TextSize,
    current_kind: JsSyntaxKind,
    current_after_new_line: bool,
    // TODO remove
    // state: LexerState,
}

#[cfg(test)]
mod tests {
    use crate::buffered_lexer::BufferedLexer;
    use crate::{Lexer, TextRange, TextSize};
    use rome_js_syntax::JsSyntaxKind::{JS_NUMBER_LITERAL, NEWLINE, WHITESPACE};
    use rome_js_syntax::T;

    #[test]
    fn without_lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        buffered.next();
        assert_eq!(buffered.current(), T![let]);
        assert!(!buffered.has_preceding_line_break());
        assert_eq!(
            buffered.current_range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), T![ident]);
        assert_eq!(buffered.next().kind(), NEWLINE);
        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), JS_NUMBER_LITERAL);
        assert_eq!(buffered.next().kind(), T![EOF]);
    }

    #[test]
    fn lookahead() {
        let lexer = Lexer::from_str("let a\n = 5", 0);
        let mut buffered = BufferedLexer::new(lexer);

        buffered.next();
        assert_eq!(buffered.current(), T![let]);
        assert!(!buffered.has_preceding_line_break());
        assert_eq!(
            buffered.current_range(),
            TextRange::at(TextSize::from(0), TextSize::from(3))
        );

        {
            let mut lookahead = buffered.lookahead();

            let nth1 = lookahead.next().unwrap();
            let nth2 = lookahead.next().unwrap();
            let nth3 = lookahead.next().unwrap();

            assert_eq!(nth1.kind(), WHITESPACE);
            assert_eq!(nth2.kind(), T![ident]);
            assert_eq!(nth3.kind(), NEWLINE);
        }

        assert_eq!(buffered.current(), T![let]);
        assert_eq!(buffered.next().kind(), WHITESPACE);

        assert_eq!(
            buffered
                .lookbehind
                .iter()
                .map(|entry| entry.kind)
                .collect::<Vec<_>>(),
            vec![WHITESPACE, T![ident], NEWLINE]
        );

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

        assert_eq!(buffered.next().kind(), T![ident]);
        assert_eq!(buffered.next().kind(), NEWLINE);
        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), T![=]);
        assert!(buffered.has_preceding_line_break());
        assert_eq!(buffered.next().kind(), WHITESPACE);
        assert_eq!(buffered.next().kind(), JS_NUMBER_LITERAL);
        assert_eq!(buffered.next().kind(), T![EOF]);
    }
}
