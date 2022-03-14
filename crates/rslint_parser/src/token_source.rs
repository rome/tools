use rome_js_syntax::JsSyntaxKind;
use rome_js_syntax::JsSyntaxKind::EOF;
use rome_rowan::TextSize;
use rslint_errors::file::FileId;
use rslint_errors::Diagnostic;
use rslint_lexer::buffered_lexer::BufferedLexer;
use rslint_lexer::{LexContext, Lexer, LexerCheckpoint, LexerReturn, ReLexContext, TextRange};
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum TriviaKind {
    Newline,
    Whitespace,
    Comment,
    MultilineComment,
}

impl TriviaKind {
    /// Returns true if this is a new line trivia
    pub fn is_newline(&self) -> bool {
        matches!(self, TriviaKind::Newline)
    }
}

impl TryFrom<JsSyntaxKind> for TriviaKind {
    type Error = ();

    fn try_from(value: JsSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsSyntaxKind::NEWLINE => Ok(TriviaKind::Newline),
                JsSyntaxKind::WHITESPACE => Ok(TriviaKind::Whitespace),
                JsSyntaxKind::COMMENT => Ok(TriviaKind::Comment),
                JsSyntaxKind::MULTILINE_COMMENT => Ok(TriviaKind::MultilineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}

/// A comment or a whitespace trivia in the source code.
#[derive(Debug, Copy, Clone)]
pub struct Trivia {
    /// The kind of the trivia token.
    kind: TriviaKind,

    /// The range of the trivia in the source text
    range: TextRange,

    /// Whatever this is the trailing or leading trivia of a non-trivia token.
    trailing: bool,
}

impl Trivia {
    fn new(kind: TriviaKind, range: TextRange, trailing: bool) -> Self {
        Self {
            kind,
            range,
            trailing,
        }
    }
    /// Returns the kind of the token
    pub fn kind(&self) -> TriviaKind {
        self.kind
    }

    /// Returns the token's length in bytes
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns the byte offset of the trivia in the source text
    pub fn offset(&self) -> TextSize {
        self.range.start()
    }

    /// Returns `true` if this is the trailing trivia of a non-trivia token or false otherwise.
    pub fn trailing(&self) -> bool {
        self.trailing
    }
}

/// Token source for the parser that skips over any non-trivia token.
pub struct TokenSource<'l> {
    lexer: BufferedLexer<'l>,

    /// List of the skipped trivia. Needed to construct the CST and compute the non-trivia token offsets.
    pub(super) trivia: Vec<Trivia>,

    /// Cache for the non-trivia token lookahead. For example for the source `let a = 10;` if the
    /// [TokenSource]'s currently positioned at the start of the file (`let`). The `nth(2)` non-trivia token,
    /// as returned by the [TokenSource], is the `=` token but retrieving it requires skipping over the
    /// two whitespace trivia tokens (first between `let` and `a`, second between `a` and `=`).
    /// The [TokenSource] state then is:
    ///
    /// * `non_trivia_lookahead`: [IDENT: 'a', EQ]
    /// * `lookahead_offset`: 4 (the `=` is the 4th token after the `let` keyword)
    non_trivia_lookahead: VecDeque<Lookahead>,

    /// Offset of the last cached lookahead token from the current [BufferedLexer] token.
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
        self.next_non_trivia_token(LexContext::default(), true)
    }

    #[inline]
    fn next_non_trivia_token(&mut self, context: LexContext, first_token: bool) -> Vec<Diagnostic> {
        let mut diagnostics = vec![];
        let mut processed_tokens = 0;
        let mut trailing = !first_token;

        // Drop the last cached lookahead, we're now moving past it
        self.non_trivia_lookahead.pop_front();

        loop {
            let LexerReturn { kind, diagnostic } = self.lexer.next_token(context);
            processed_tokens += 1;

            if let Some(diagnostic) = diagnostic {
                diagnostics.push(*diagnostic);
            }

            let trivia_kind = TriviaKind::try_from(kind);

            match trivia_kind {
                Err(_) => break,
                Ok(trivia_kind) => {
                    // Trivia after and including the newline is considered the leading trivia of the next token
                    if trivia_kind.is_newline() {
                        trailing = false;
                    }

                    self.trivia
                        .push(Trivia::new(trivia_kind, self.current_range(), trailing));
                }
            }
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

        // Return the cached token if any
        if let Some(lookahead) = self.non_trivia_lookahead.get(n - 1) {
            return Some(*lookahead);
        }

        // Jump right to where we've left of last time rather than going through all tokens again.
        let iter = self.lexer.lookahead().skip(self.lookahead_offset);
        let mut remaining = n - self.non_trivia_lookahead.len();

        for item in iter {
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
            if !context.is_regular() {
                self.lookahead_offset = 0;
                self.non_trivia_lookahead.clear();
            }

            self.next_non_trivia_token(context, false)
        }
    }

    pub fn re_lex(&mut self, mode: ReLexContext) -> LexerReturn {
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

    /// Ends this token source and returns the source text's trivia
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
    pub(crate) fn current_start(&self) -> TextSize {
        self.lexer.current_start()
    }

    pub(crate) fn trivia_position(&self) -> usize {
        self.trivia_len as usize
    }
}
