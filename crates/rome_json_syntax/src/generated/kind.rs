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
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_CUSTOM_PROPERTY,
    CSS_SPACE_LITERAL,
    ERROR_TOKEN,
    IDENT,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    CSS_ROOT,
    CSS_ID_SELECTOR_PATTERN,
    CSS_RULE,
    CSS_SELECTOR_LIST,
    CSS_SELECTOR,
    CSS_ANY_FUNCTION,
    CSS_AT_KEYFRAMES,
    CSS_AT_KEYFRAMES_BODY,
    CSS_AT_MEDIA,
    CSS_AT_MEDIA_QUERY,
    CSS_AT_MEDIA_QUERY_CONSEQUENT,
    CSS_AT_MEDIA_QUERY_FEATURE,
    CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN,
    CSS_AT_MEDIA_QUERY_FEATURE_COMPARE,
    CSS_AT_MEDIA_QUERY_FEATURE_PLAIN,
    CSS_AT_MEDIA_QUERY_FEATURE_RANGE,
    CSS_AT_MEDIA_QUERY_RANGE,
    CSS_ATTRIBUTE,
    CSS_ATTRIBUTE_MATCHER,
    CSS_ATTRIBUTE_META,
    CSS_ATTRIBUTE_MODIFIER,
    CSS_ATTRIBUTE_NAME,
    CSS_ATTRIBUTE_SELECTOR_PATTERN,
    CSS_BLOCK,
    CSS_CLASS_SELECTOR_PATTERN,
    CSS_COMBINATOR_SELECTOR_PATTERN,
    CSS_DECLARATION,
    CSS_DIMENSION,
    CSS_IDENTIFIER,
    CSS_KEYFRAMES_BLOCK,
    CSS_KEYFRAMES_SELECTOR,
    CSS_NUMBER,
    CSS_PARAMETER,
    CSS_PERCENTAGE,
    CSS_PSEUDO_CLASS_SELECTOR_PATTERN,
    CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS,
    CSS_RATIO,
    CSS_SIMPLE_FUNCTION,
    CSS_STRING,
    CSS_TYPE_SELECTOR_PATTERN,
    CSS_UNIVERSAL_SELECTOR_PATTERN,
    CSS_VAR_FUNCTION,
    CSS_VAR_FUNCTION_VALUE,
    CSS_ANY_SELECTOR_PATTERN_LIST,
    CSS_AT_KEYFRAMES_ITEM_LIST,
    CSS_AT_MEDIA_QUERY_LIST,
    CSS_ATTRIBUTE_LIST,
    CSS_DECLARATION_LIST,
    CSS_KEYFRAMES_SELECTOR_LIST,
    CSS_PARAMETER_LIST,
    CSS_DECLARATION_IMPORTANT,
    CSS_UNKNOWN,
    #[doc(hidden)]
    __LAST,
}
use self::JsonSyntaxKind::*;
impl JsonSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            CSS_STRING_LITERAL | CSS_NUMBER_LITERAL | CSS_CUSTOM_PROPERTY | CSS_SPACE_LITERAL => {
                true
            }
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            CSS_SELECTOR_LIST
            | CSS_ANY_SELECTOR_PATTERN_LIST
            | CSS_AT_KEYFRAMES_ITEM_LIST
            | CSS_AT_MEDIA_QUERY_LIST
            | CSS_ATTRIBUTE_LIST
            | CSS_DECLARATION_LIST
            | CSS_KEYFRAMES_SELECTOR_LIST
            | CSS_PARAMETER_LIST => true,
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
            L_PAREN => "'('",
            R_PAREN => "')'",
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
macro_rules ! T { [,] => { $ crate :: JsonSyntaxKind :: COMMA } ; ['('] => { $ crate :: JsonSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: JsonSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: JsonSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: JsonSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: JsonSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: JsonSyntaxKind :: R_BRACK } ; [ident] => { $ crate :: JsonSyntaxKind :: IDENT } ; [EOF] => { $ crate :: JsonSyntaxKind :: EOF } ; [#] => { $ crate :: JsonSyntaxKind :: HASH } ; }
