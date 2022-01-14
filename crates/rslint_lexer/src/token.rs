//! Token definitions for the lexer

use crate::JsSyntaxKind;

/// A single raw token such as `>>` or `||` or `"abc"`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token this is.
    pub kind: JsSyntaxKind,
    /// Indicates if there is a newline between this token and the previous non-trivia token.
    pub after_newline: bool,
    /// Offset from the start of the file, in bytes.
    pub offset: u32,
    /// How long the token is in bytes. For tokens with escape sequences
    /// like strings with `\uXXXX` escapes, the length is the raw length, not considering the char backed by the escape.
    pub len: u32,
}

impl Token {
    /// Create a new token which has an exact length of 1.
    pub fn single(kind: JsSyntaxKind, offset: usize) -> Self {
        Self {
            kind,
            len: 1,
            offset: offset as u32,
            after_newline: false,
        }
    }

    /// Create a new token which has a specific length.
    pub fn new(kind: JsSyntaxKind, len: usize) -> Self {
        Self {
            kind,
            len: len as u32,
            offset: 0,
            after_newline: false,
        }
    }

    /// Range from the start of the file, in bytes.
    #[inline(always)]
    pub fn range(&self) -> std::ops::Range<usize> {
        let end = self.offset as usize + self.len as usize;
        (self.offset as usize)..end
    }

    /// Same as [Token::range()].start.
    #[inline(always)]
    pub fn start(&self) -> usize {
        self.offset as usize
    }

    /// Same as [Token::range()].end.
    #[inline(always)]
    pub fn end(&self) -> usize {
        self.offset as usize + self.len as usize
    }
}

macro_rules! tok {
    ($tok:tt) => {
        (Token::new(T![$tok], stringify!($tok).len()), None)
    };
    ($tok:ident, $len:expr) => {
        (Token::new(JsSyntaxKind::$tok, $len), None)
    };
}
