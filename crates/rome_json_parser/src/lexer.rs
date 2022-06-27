use std::ops::Range;

use logos::Logos;
use rome_diagnostics::Diagnostic;
use rome_json_syntax::{JsonSyntaxKind, TextRange, TextSize, T};

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("null")]
    Null,

    #[regex(r#""([^"\\]|\\u[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]|\\["\\/bfnrt])*""#)]
    String,

    #[regex(r#"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?"#)]
    Number,

    #[regex(r"[ \t\f]+")]
    Whitespace,

    #[regex(r"\r\n|\n")]
    NewLine,

    #[error]
    Error,
    // Root,

    // Array,
    // Object,
}

impl From<TokenKind> for JsonSyntaxKind {
    fn from(kind: TokenKind) -> Self {
        match kind {
            TokenKind::LeftBrace => T!['{'],
            TokenKind::RightBrace => T!['}'],
            TokenKind::Colon => T![:],
            TokenKind::Comma => T![,],
            TokenKind::LeftBracket => T!['['],
            TokenKind::RightBracket => T![']'],
            TokenKind::True => T![true],
            TokenKind::False => T![false],
            TokenKind::Null => T![null],
            TokenKind::String => JsonSyntaxKind::JSON_STRING_LITERAL,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER_LITERAL,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::NewLine => JsonSyntaxKind::NEWLINE,
            TokenKind::Error => JsonSyntaxKind::ERROR_TOKEN,
        }
    }
}

impl From<&TokenKind> for JsonSyntaxKind {
    fn from(kind: &TokenKind) -> Self {
        match kind {
            TokenKind::LeftBrace => T!['{'],
            TokenKind::RightBrace => T!['}'],
            TokenKind::Colon => T![:],
            TokenKind::Comma => T![,],
            TokenKind::LeftBracket => T!['['],
            TokenKind::RightBracket => T![']'],
            TokenKind::True => T![true],
            TokenKind::False => T![false],
            TokenKind::Null => T![null],
            TokenKind::String => JsonSyntaxKind::JSON_STRING_LITERAL,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER_LITERAL,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::NewLine => JsonSyntaxKind::NEWLINE,
            TokenKind::Error => JsonSyntaxKind::ERROR_TOKEN,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Lexer<'a> {
    /// Vector of tokens, with it's [JsonSyntaxKind] and [TextRange].
    tokens_with_span: Vec<(JsonSyntaxKind, TextRange)>,
    /// Source code of tokens
    source: &'a str,
    /// Custom file_id
    file_id: usize,
    /// Current cursor of non_trivia_index_list, we use such way to make our get next non trivia token function faster in parser phase.
    /// If we want to get the first non trivia token, we need to get the index of the first non trivia token in `token_with_span` vector.
    /// then use the index to get `(kind, range)` from `token_with_span` vector.
    /// ```rs
    /// let index = lexer.non_trivia_index_list[lexer.none_trivia_index_cursor];
    /// let (token_kind, token_range) = lexer.token_with_span[index];
    /// ```
    /// The benefit of doing so is we could easily collecting trivia token between latest none trivia token and next none trivia token by 
    /// just iterating `tokens_with_span` vector from index of latest none trivia token to index of next none trivia token.
    /// ```rs
    /// let left = lexer.current_none_trivia_cursor();
    /// let right = lexer.next_none_trivia_cursor();
    /// for index in (left + 1)..right {
    ///   let trivia_token = lexer.token_with_span[index];
    ///   // do something here.
    /// }
    /// ```
    none_trivia_cursor: usize,
    /// We collect all index of non-trivia in the `token_with_span` ahead of time.
    non_trivia_index_list: Vec<usize>,
}

#[allow(unused)]
impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, file_id: usize) -> Self {
        let lexer = LogosLexer::new(source);
        let mut tokens_with_span = vec![];
        let mut non_trivia_index_list = vec![];
        for (i, (kind, range)) in lexer.into_iter().enumerate() {
            tokens_with_span.push((
                kind.into(),
                TextRange::new(
                    TextSize::from(range.start as u32),
                    TextSize::from(range.end as u32),
                ),
            ));
            if !matches!(kind, TokenKind::Whitespace | TokenKind::NewLine) {
                non_trivia_index_list.push(i);
            }
        }
        Self {
            tokens_with_span,
            source,
            file_id,
            none_trivia_cursor: 0,
            non_trivia_index_list,
        }
    }

    /// Get nth non trivia token_kind from latest_cursor, return `current_token_kind` if n = 0.
    pub fn nth(&self, n: usize) -> JsonSyntaxKind {
        let index = self.none_trivia_cursor + n;
        match self.non_trivia_index_list.get(index) {
            Some(i) => {
                let (kind, _) = &self.tokens_with_span[*i];
                *kind
            }
            None => JsonSyntaxKind::EOF,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.tokens_with_span.len()
    }

    /// Get the next non trivia token and pointed the latest to it.
    pub fn advance(&mut self) {
        self.none_trivia_cursor += 1;
    }

    pub fn file_id(&self) -> usize {
        self.file_id
    }

    /// Get current token_kind of latest non trivia token.
    pub fn current_token_kind(&self) -> JsonSyntaxKind {
        self.current()
            .map(|item| item.0.into())
            .unwrap_or_else(|| JsonSyntaxKind::EOF)
    }
    
    /// Get current (token_kind, range) of latest non trivia token.
    fn current(&self) -> Option<&(JsonSyntaxKind, TextRange)> {
        self.non_trivia_index_list
            .get(self.none_trivia_cursor)
            .map(|i| &self.tokens_with_span[*i])
    }

    /// Get any (token_kind, range) at given index, including both trivia and none trivia tokens.
    pub fn token_at(&self, i: usize) -> (JsonSyntaxKind, TextRange) {
        *self.tokens_with_span.get(i).unwrap_or({
            &(
                JsonSyntaxKind::EOF,
                TextRange::new(
                    TextSize::from(self.source.len() as u32),
                    TextSize::from(self.source.len() as u32),
                ),
            )
        })
    }

    /// Get text range of latest none trivia token.
    pub fn current_range(&self) -> TextRange {
        match self.current() {
            Some((_, range)) => *range,
            None => TextRange::new(
                TextSize::from(self.source.len() as u32),
                TextSize::from(self.source.len() as u32),
            ),
        }
    }

    /// Get source code of lexing phase.
    pub fn source(&self) -> &'a str {
        self.source
    }

    /// Get all diagnostics of lexing phase.
    pub fn finish(self) -> Vec<Diagnostic> {
        // self.inner.finish()
        vec![]
    }

    /// Get index of `tokens_with_span` vector of latest none trivia token.
    #[inline]
    pub fn current_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor))
            .unwrap_or(&self.tokens_with_span.len())
    }

    /// Get index of `tokens_with_span` vector of next none trivia token.
    #[inline]
    pub fn next_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor + 1))
            .unwrap_or(&self.tokens_with_span.len())
    }
}

pub struct LogosLexer<'a> {
    inner: logos::Lexer<'a, TokenKind>,
}

impl<'a> LogosLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: TokenKind::lexer(input),
        }
    }
}

impl<'a> Iterator for LogosLexer<'a> {
    type Item = (TokenKind, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let span = self.inner.span();

        Some((kind, span))
    }
}
