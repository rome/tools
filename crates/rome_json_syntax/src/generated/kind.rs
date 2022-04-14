//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum JsonSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file.May have trivia attached"]
    EOF,
    COMMA,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    JSON_STRING_LITERAL,
    JSON_NUMBER_LITERAL,
    JSON_BOOL_LITERAL,
    JSON_NULL_LITERAL,
    WHITESPACE,
    JSON_STATEMENT,
    JSON_OBJECT_STATEMENT,
    JSON_OBJECT_VALUE_LIST,
    JSON_OBJECT_VALUE,
    JSON_ARRAY_STATEMENT,
    JSON_ARRAY_VALUE_LIST,
    JSON_DATA_VALUE,
    JSON_DATA_LITERAL_EXPRESSION,
    JSON_STRING_LITERAL_EXPRESSION,
    JSON_NUMBER_LITERAL_EXPRESSION,
    JSON_BOOLEAN_LITERAL_EXPRESSION,
    JSON_NULL_LITERAL_EXPRESSION,
    #[doc(hidden)]
    __LAST,
}
use self::JsonSyntaxKind::*;
impl JsonSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            COMMA | L_CURLY | R_CURLY | L_BRACK | R_BRACK => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            JSON_STRING_LITERAL | JSON_NUMBER_LITERAL | JSON_BOOL_LITERAL | JSON_NULL_LITERAL => {
                true
            }
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            JSON_OBJECT_VALUE_LIST | JSON_ARRAY_VALUE_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<JsonSyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            COMMA => ",",
            L_CURLY => "'{'",
            R_CURLY => "'}'",
            L_BRACK => "'['",
            R_BRACK => "']'",
            JSON_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [,] => { $ crate :: JsonSyntaxKind :: COMMA } ; ['{'] => { $ crate :: JsonSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: JsonSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: JsonSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: JsonSyntaxKind :: R_BRACK } ; [ident] => { $ crate :: JsonSyntaxKind :: IDENT } ; [EOF] => { $ crate :: JsonSyntaxKind :: EOF } ; [#] => { $ crate :: JsonSyntaxKind :: HASH } ; }
