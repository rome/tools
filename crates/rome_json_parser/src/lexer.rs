use std::{iter::Peekable, ops::Range};

use logos::{Logos, Span};
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
    // #[regex(r#"\."#)]
    // Text1,
    #[regex(r#"-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?"#)]
    Number,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
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

#[derive(Debug)]
pub struct Lexer<'a> {
    tokens_with_span: Vec<(JsonSyntaxKind, TextRange)>,
    source: &'a str,
    file_id: usize,
    none_trivia_cursor: usize,
    non_trivia_index_list: Vec<usize>,
}

impl<'a> Lexer<'a> {
    // let lexer = Lexer::from_str(source, file_id);
    pub fn from_str(source: &'a str, file_id: usize) -> Self {
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
        // dbg!(&tokens_with_span);
        Self {
            tokens_with_span,
            source,
            file_id,
            none_trivia_cursor: 0,
            non_trivia_index_list,
        }
    }

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
    pub fn advance(&mut self) {
        self.none_trivia_cursor += 1;
    }

    pub fn file_id(&self) -> usize {
        self.file_id
    }

    pub fn current_token(&self) -> JsonSyntaxKind {
        self.current()
            .map(|item| item.0.into())
            .unwrap_or_else(|| JsonSyntaxKind::EOF)
    }

    fn current(&self) -> Option<&(JsonSyntaxKind, TextRange)> {
        self.non_trivia_index_list
            .get(self.none_trivia_cursor)
            .map(|i| &self.tokens_with_span[*i])
    }

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

    pub fn current_range(&self) -> TextRange {
        match self.current() {
            Some((_, range)) => *range,
            None => TextRange::new(
                TextSize::from(self.source.len() as u32),
                TextSize::from(self.source.len() as u32),
            ),
        }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn finish(self) -> Vec<Diagnostic> {
        // self.inner.finish()
        vec![]
    }

    #[inline]
    pub fn current_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor))
            .unwrap_or(&self.tokens_with_span.len())
    }

    #[inline]
    pub fn next_none_trivia_cursor(&self) -> usize {
        *(self.non_trivia_index_list.get(self.none_trivia_cursor + 1))
            .unwrap_or(&self.tokens_with_span.len())
    }
    // pub fn lookahead(&self) -> It {
    //     todo!()
    // }
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
        // let text = self.inner.slice();
        let span = self.inner.span();

        Some((kind, span))
    }
}
