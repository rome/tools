use rome_js_syntax::JsSyntaxKind;
use rome_js_syntax::JsSyntaxKind::EOF;
use rome_rowan::TextSize;
use rslint_errors::file::FileId;
use rslint_errors::Diagnostic;
use rslint_lexer::buffered_lexer::BufferedLexer;
use rslint_lexer::{LexContext, Lexer, LexerCheckpoint, LexerReturn, ReLexContext, TextRange};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
pub struct Trivia {
    kind: JsSyntaxKind,
    range: TextRange,
    trailing: bool,
}

impl Trivia {
    fn new(kind: JsSyntaxKind, range: TextRange, trailing: bool) -> Self {
        Self {
            kind,
            range,
            trailing,
        }
    }
    /// Returns the kind of the token
    pub fn kind(&self) -> JsSyntaxKind {
        self.kind
    }

    /// Returns the token's length in bytes
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    pub fn offset(&self) -> TextSize {
        self.range.start()
    }

    pub fn trailing(&self) -> bool {
        self.trailing
    }
}

// TODO add detached mode. Throw if any operation is called in detached mode in debug build
/// The source of tokens for the parser
pub struct TokenSource<'l> {
    lexer: BufferedLexer<'l>,

    /// A list of the tokens including whitespace.
    pub(super) trivia: Vec<Trivia>,

    non_trivia_lookahead: VecDeque<Lookahead>,
    lookahead_offset: usize,
}

#[derive(Debug, Copy, Clone)]
struct Lookahead {
    kind: JsSyntaxKind,
    after_newline: bool,
}

impl<'l> TokenSource<'l> {
    /// Creates a new token source.
    pub fn new(lexer: BufferedLexer<'l>) -> TokenSource<'l> {
        TokenSource {
            lexer,
            trivia: vec![],
            lookahead_offset: 0,
            non_trivia_lookahead: VecDeque::new(),
        }
    }

    /// Creates a new token source for the given string
    pub fn from_str(source: &'l str, file_id: FileId) -> TokenSource<'l> {
        let lexer = Lexer::from_str(source, file_id);
        let buffered = BufferedLexer::new(lexer);
        TokenSource::new(buffered)
    }

    /// Moves the token source to the first non-trivia token. Returns the lexing error
    /// for the skipped trivia and the now current token
    pub fn initialize(&mut self) -> Vec<Diagnostic> {
        self.next_non_trivia_token(LexContext::Regular, true)
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: LexContext, first_token: bool) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        let mut processed_tokens = 0;
        let mut trailing = !first_token;

        self.non_trivia_lookahead.pop_front();

        loop {
            let LexerReturn { kind, diagnostic } = self.lexer.next_token(context);
            processed_tokens += 1;

            if let Some(diagnostic) = diagnostic {
                diagnostics.push(*diagnostic);
            }

            if !kind.is_trivia() {
                break;
            }

            if kind == JsSyntaxKind::NEWLINE {
                trailing = false;
            }

            self.trivia
                .push(Trivia::new(kind, self.current_range(), trailing));
        }

        if self.lookahead_offset != 0 {
            debug_assert!(self.lookahead_offset >= processed_tokens);
            self.lookahead_offset -= processed_tokens;
        }

        diagnostics
    }

    /// Returns the kind of the current non-trivia token
    #[inline(always)]
    pub fn current(&self) -> JsSyntaxKind {
        self.lexer.current()
    }

    /// Returns the range of the current non-trivia token
    #[inline(always)]
    pub fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    /// Gets the kind of the nth non-trivia token
    #[inline(always)]
    pub fn nth(&mut self, n: usize) -> JsSyntaxKind {
        if n == 0 {
            self.current()
        } else {
            self.lookahead(n).map_or(EOF, |lookahead| lookahead.kind)
        }
    }

    /// Returns true if the current token is preceded by a line break
    #[inline(always)]
    pub fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    #[inline(always)]
    pub fn has_unicode_escape(&self) -> bool {
        self.lexer.has_unicode_escape()
    }

    /// Returns true if the nth non-trivia token is preceded by a line break
    #[inline(always)]
    pub fn has_nth_preceding_line_break(&mut self, n: usize) -> bool {
        if n == 0 {
            self.has_preceding_line_break()
        } else {
            self.lookahead(n)
                .map_or(false, |lookahead| lookahead.after_newline)
        }
    }

    #[inline(always)]
    fn lookahead(&mut self, n: usize) -> Option<Lookahead> {
        assert_ne!(n, 0);

        if let Some(lookahead) = self.non_trivia_lookahead.get(n - 1) {
            return Some(*lookahead);
        }

        let mut iter = self.lexer.lookahead().skip(self.lookahead_offset);

        let mut remaining = n - self.non_trivia_lookahead.len();
        while let Some(item) = iter.next() {
            self.lookahead_offset += 1;

            if !item.kind().is_trivia() {
                remaining -= 1;

                let lookahead = Lookahead {
                    after_newline: item.has_preceding_line_break(),
                    kind: item.kind(),
                };

                self.non_trivia_lookahead.push_back(lookahead);

                if remaining == 0 {
                    return Some(lookahead);
                }
            }
        }

        None
    }

    pub fn rewind(&mut self, checkpoint: TokenSourceCheckpoint) {
        assert!(self.trivia.len() >= checkpoint.trivia_len as usize);
        self.trivia.truncate(checkpoint.trivia_len as usize);
        self.lexer.rewind(checkpoint.lexer);
        self.non_trivia_lookahead.clear();
        self.lookahead_offset = 0;
    }

    pub fn checkpoint(&self) -> TokenSourceCheckpoint {
        TokenSourceCheckpoint {
            trivia_len: self.trivia.len() as u32,
            lexer: self.lexer.checkpoint(),
        }
    }

    /// Returns the source text
    #[inline(always)]
    pub fn source(&self) -> &'l str {
        self.lexer.source()
    }

    /// Bumps the current token and moves the parser to the next non-trivia token
    #[inline(always)]
    pub fn bump(&mut self, context: LexContext) -> Vec<Diagnostic> {
        if self.current() == EOF {
            vec![]
        } else {
            self.next_non_trivia_token(context, false)
        }
    }

    pub fn re_lex(&mut self, mode: ReLexContext) -> LexerReturn {
        self.non_trivia_lookahead.clear();
        self.lookahead_offset = 0;
        let current_kind = self.current();

        let LexerReturn {
            kind: new_kind,
            diagnostic,
        } = self.lexer.re_lex(mode);

        // Only need to clear the lookahead cache when the token did change
        if current_kind != new_kind {
            self.non_trivia_lookahead.clear();
            self.lookahead_offset = 0;
        }

        LexerReturn::new(new_kind, diagnostic)
    }

    /// Returns the byte offset of the current token from the start of the source document
    #[inline(always)]
    pub fn position(&self) -> TextSize {
        self.current_range().start()
    }

    pub fn finish(self) -> Vec<Trivia> {
        self.trivia
    }
}

#[derive(Debug)]
pub struct TokenSourceCheckpoint {
    lexer: LexerCheckpoint,
    /// A `u32` should be enough because `TextSize` is also limited to `u32`.
    /// The worst case is a document where every character is its own token. This would
    /// result in `u32::MAX` tokens
    trivia_len: u32,
}

impl TokenSourceCheckpoint {
    /// byte offset in the source text
    pub fn offset(&self) -> TextSize {
        self.lexer.position()
    }

    pub fn trivia<'s>(&self, token_source: &'s TokenSource) -> &'s [Trivia] {
        &token_source.trivia[self.trivia_len as usize..]
    }
}
