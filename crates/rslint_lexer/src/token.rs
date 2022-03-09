//! Token definitions for the lexer

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
