//! Token definitions for the lexer

use crate::{JsSyntaxKind, TextSize};
use rome_js_syntax::TextRange;

/// A single raw token such as `>>` or `||` or `"abc"`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token {
    /// The kind of token this is.
    pub kind: JsSyntaxKind,
    /// Indicates if there is a newline between this token and the previous non-trivia token.
    pub after_newline: bool,
    /// Offset from the start of the file, in bytes.
    pub offset: TextSize,
    /// How long the token is in bytes. For tokens with escape sequences
    /// like strings with `\uXXXX` escapes, the length is the raw length, not considering the char backed by the escape.
    pub len: TextSize,
}

impl Token {
    /// Create a new token which has an exact length of 1.
    pub fn single(kind: JsSyntaxKind, offset: usize) -> Self {
        Self {
            kind,
            len: TextSize::from(1),
            offset: TextSize::from(offset as u32),
            after_newline: false,
        }
    }

    /// Create a new token which has a specific length.
    pub fn new(kind: JsSyntaxKind, len: usize) -> Self {
        Self {
            kind,
            len: TextSize::from(len as u32),
            offset: TextSize::from(0),
            after_newline: false,
        }
    }

    /// Range from the start of the file, in bytes.
    #[inline(always)]
    pub fn range(&self) -> TextRange {
        TextRange::at(self.offset, self.len)
    }

    /// Same as [Token::range()].start.
    #[inline(always)]
    pub fn start(&self) -> TextSize {
        self.offset
    }

    /// Same as [Token::range()].end.
    #[inline(always)]
    pub fn end(&self) -> TextSize {
        self.offset + self.len
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
