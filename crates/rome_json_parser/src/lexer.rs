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

    #[regex(r#""([^"\\]|\\u[0-9a-f][0-9a-f][0-9a-f][0-9a-f]|\\["\\/bfnrt])*""#)]
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
            TokenKind::String => JsonSyntaxKind::JSON_STRING,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::NewLine => JsonSyntaxKind::NEWLINE,
            TokenKind::Error => JsonSyntaxKind::JSON_UNKNOWN,
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
            TokenKind::String => JsonSyntaxKind::JSON_STRING,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::NewLine => JsonSyntaxKind::NEWLINE,
            TokenKind::Error => JsonSyntaxKind::JSON_UNKNOWN,
        }
    }
}

pub struct Lexer<'a> {
    tokens_with_span: Vec<(TokenKind, Span)>,
    source: &'a str,
    file_id: usize,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    // let lexer = Lexer::from_str(source, file_id);
    pub fn from_str(source: &'a str, file_id: usize) -> Self {
        let lexer = LogosLexer::new(source);
        Self {
            tokens_with_span: lexer.into_iter().collect::<Vec<_>>(),
            source,
            file_id,
            cursor: 0,
        }
    }

    pub fn next_with_token(&mut self) -> JsonSyntaxKind {
        let ret = match self.tokens_with_span.get(self.cursor) {
            Some((kind, _)) => kind.into(),
            None => JsonSyntaxKind::EOF,
        };
        self.cursor += 1;
        ret
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn file_id(&self) -> usize {
        self.file_id
    }

    pub fn current_token(&self) -> JsonSyntaxKind {
        self.current()
            .map(|item| item.0.into())
            .unwrap_or(JsonSyntaxKind::EOF)
    }

    fn current(&self) -> Option<&(TokenKind, Span)> {
        self.tokens_with_span.get(self.cursor)
    }

    pub fn current_range(&self) -> TextRange {
        match self.current() {
            Some((_, range)) => TextRange::new(
                TextSize::from(range.start as u32),
                TextSize::from(range.end as u32),
            ),
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
