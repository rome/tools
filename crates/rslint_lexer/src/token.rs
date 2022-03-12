//! Token definitions for the lexer

use bitflags::bitflags;

bitflags! {
    pub struct TokenFlags: u8 {
        /// Indicates that there has been a line break between the last non-trivia token
        const PRECEDING_LINE_BREAK = 1 << 0;

        /// Indicates that an identifier contains an unicode escape sequence
        const UNICODE_ESCAPE = 1 << 1;
    }
}

macro_rules! lexer_return {
    ($tok:ident) => {
        $crate::LexerReturn {
            kind: $crate::JsSyntaxKind::$tok,
            diagnostic: None,
        }
    };
    ($tok:tt) => {
        $crate::LexerReturn {
            kind: T![$tok],
            diagnostic: None,
        }
    };
}
