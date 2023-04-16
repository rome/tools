//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum MdSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file.May have trivia attached"]
    EOF,
    COLON,
    BACKTICK,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    H1,
    H2,
    H3,
    H4,
    H5,
    MD_STRING_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    MULTILINE_COMMENT,
    MD_ROOT,
    MD_HEADING,
    MD_TEXT,
    MD_ELEMENT_LIST,
    MD_BOGUS,
    #[doc(hidden)]
    __LAST,
}
use self::MdSyntaxKind::*;
impl MdSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            COLON | BACKTICK | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK | H1
            | H2 | H3 | H4 | H5 => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            MD_STRING_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            MD_ELEMENT_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<MdSyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            COLON => ":",
            BACKTICK => "`",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            H1 => "#",
            H2 => "##",
            H3 => "###",
            H4 => "####",
            H5 => "#####",
            MD_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [:] => { $ crate :: MdSyntaxKind :: COLON } ; ['`'] => { $ crate :: MdSyntaxKind :: BACKTICK } ; ['('] => { $ crate :: MdSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: MdSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: MdSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: MdSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: MdSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: MdSyntaxKind :: R_BRACK } ; [#] => { $ crate :: MdSyntaxKind :: H1 } ; [##] => { $ crate :: MdSyntaxKind :: H2 } ; [###] => { $ crate :: MdSyntaxKind :: H3 } ; [####] => { $ crate :: MdSyntaxKind :: H4 } ; [#####] => { $ crate :: MdSyntaxKind :: H5 } ; [ident] => { $ crate :: MdSyntaxKind :: IDENT } ; [EOF] => { $ crate :: MdSyntaxKind :: EOF } ; [#] => { $ crate :: MdSyntaxKind :: HASH } ; }
