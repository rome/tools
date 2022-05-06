use std::{iter::Peekable, ops::Range};

use logos::Logos;
use rome_diagnostics::Diagnostic;
use rome_json_syntax::{JsonSyntaxKind, TextRange, TextSize};
use rome_rowan::SyntaxKind;

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
    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[error]
    Error,
    // Root,

    // Array,
    // Object,
}

impl Into<JsonSyntaxKind> for TokenKind {
    fn into(self) -> JsonSyntaxKind {
        match self {
            TokenKind::LeftBrace => JsonSyntaxKind::L_CURLY,
            TokenKind::RightBrace => JsonSyntaxKind::R_CURLY,
            TokenKind::Colon => JsonSyntaxKind::COLON,
            TokenKind::Comma => JsonSyntaxKind::COLON,
            TokenKind::LeftBracket => JsonSyntaxKind::L_BRACK,
            TokenKind::RightBracket => JsonSyntaxKind::R_BRACK,
            TokenKind::True => JsonSyntaxKind::TRUE_KW,
            TokenKind::False => JsonSyntaxKind::FALSE_KW,
            TokenKind::Null => JsonSyntaxKind::NULL_KW,
            TokenKind::String => JsonSyntaxKind::JSON_STRING_LITERAL,
            TokenKind::Number => JsonSyntaxKind::JSON_NUMBER_LITERAL,
            TokenKind::Whitespace => JsonSyntaxKind::WHITESPACE,
            TokenKind::Error => JsonSyntaxKind::ERROR_TOKEN,
        }
    }
}

pub struct Lexer<'a> {
    inner: Peekable<LogosLexer<'a>>,
    source: &'a str,
    file_id: usize,
    cache_slot: Option<TokenKind>
}

impl<'a> Lexer<'a> {
    // pub fn new(input: &'a str) -> Self {
    //     Self {
    //         inner: TokenKind::lexer(input).peekable(),
    //     }
    // }

    // let lexer = Lexer::from_str(source, file_id);
    pub fn from_str(source: &'a str, file_id: usize) -> Self {
        Self {
            inner: LogosLexer::new(source).peekable(),
            source,
            file_id,
            cache_slot: None
        }
    }

    pub fn next_token(&mut self) -> JsonSyntaxKind {
        // self.cache_slot.take().unwrap_or_else(|| {
            
        // };
        // self.inner
        //     .next()
        //     .map(|(kind, _)| kind.into())
        //     .unwrap_or(JsonSyntaxKind::EOF)
        JsonSyntaxKind::COLON
    }

    fn current_token() {

    }

    pub fn current(&mut self) -> JsonSyntaxKind {
        match self.inner.peek() {
            Some((kind, _)) => (*kind).into(),
            None => JsonSyntaxKind::EOF,
        }
    }
    pub fn current_range(&mut self) -> TextRange {
        match self.inner.next() {
            Some((_, range)) => TextRange::new(
                range.start.try_into().unwrap(),
                range.end.try_into().unwrap(),
            ),
            None => TextRange::new(
                self.source.len().try_into().unwrap(),
                self.source.len().try_into().unwrap(),
            ),
        }
    }
    pub fn has_preceding_line_break(&self) -> bool {
        todo!()
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

// struct LogosLexer<'a> {

// }

// impl<'a> Iterator for Lexer<'a> {
//     type Item = (TokenKind, &'a str);

//     fn next(&mut self) -> Option<Self::Item> {
//         let kind = self.inner.next()?;

//         Some((kind, text))
//     }
// }

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
