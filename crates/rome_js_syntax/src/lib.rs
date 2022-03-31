//! A crate for generated Syntax node definitions and utility macros.
//! Both rome_js_lexer and rome_js_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#[macro_use]
mod generated;
pub mod ast;
pub mod expr_ext;
pub mod modifier_ext;
pub mod numbers;
pub mod stmt_ext;
mod syntax_node;
mod union_ext;
pub mod util;

pub use self::generated::*;
pub use expr_ext::*;
pub use modifier_ext::*;
pub use rome_rowan::{
    SyntaxText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use stmt_ext::*;
pub use syntax_node::*;
pub use util::SyntaxNodeExt;

use crate::JsSyntaxKind::*;
use rome_rowan::RawSyntaxKind;

impl From<u16> for JsSyntaxKind {
    fn from(d: u16) -> JsSyntaxKind {
        assert!(d <= (JsSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, JsSyntaxKind>(d) }
    }
}

impl From<JsSyntaxKind> for u16 {
    fn from(k: JsSyntaxKind) -> u16 {
        k as u16
    }
}

impl JsSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            JsSyntaxKind::NEWLINE
                | JsSyntaxKind::WHITESPACE
                | JsSyntaxKind::COMMENT
                | JsSyntaxKind::MULTILINE_COMMENT
        )
    }

    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (JsSyntaxKind::OF_KW as u16)
            && (self as u16) >= (JsSyntaxKind::BREAK_KW as u16)
    }

    /// Returns `true` for contextual keywords (excluding strict mode contextual keywords)
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u16) >= (JsSyntaxKind::ABSTRACT_KW as u16)
            && (self as u16) <= (JsSyntaxKind::OF_KW as u16)
    }

    /// Returns true for all non-contextual keywords (includes future reserved keywords)
    #[inline]
    pub const fn is_non_contextual_keyword(self) -> bool {
        self.is_keyword() && !self.is_contextual_keyword()
    }

    #[inline]
    pub const fn is_future_reserved_keyword(self) -> bool {
        (self as u16) >= (JsSyntaxKind::IMPLEMENTS_KW as u16)
            && (self as u16) <= (JsSyntaxKind::YIELD_KW as u16)
    }
}

impl rome_rowan::SyntaxKind for JsSyntaxKind {
    fn is_unknown(&self) -> bool {
        matches!(
            self,
            JS_UNKNOWN_STATEMENT
                | JS_UNKNOWN_PARAMETER
                | JS_UNKNOWN_BINDING
                | JS_UNKNOWN_MEMBER
                | JS_UNKNOWN_EXPRESSION
                | JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
                | JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
                | JS_UNKNOWN_ASSIGNMENT
        )
    }

    fn to_unknown(&self) -> JsSyntaxKind {
        match self {
            JS_BLOCK_STATEMENT
            | JS_EXPORT
            | JS_FOR_STATEMENT
            | JS_BREAK_STATEMENT
            | JS_CLASS_DECLARATION
            | JS_CONTINUE_STATEMENT
            | JS_DEBUGGER_STATEMENT
            | JS_DO_WHILE_STATEMENT
            | JS_EMPTY_STATEMENT
            | JS_EXPRESSION_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FUNCTION_DECLARATION
            | JS_IF_STATEMENT
            | JS_IMPORT
            | JS_LABELED_STATEMENT
            | JS_RETURN_STATEMENT
            | JS_SWITCH_STATEMENT
            | JS_THROW_STATEMENT
            | JS_TRY_FINALLY_STATEMENT
            | JS_TRY_STATEMENT
            | JS_VARIABLE_STATEMENT
            | JS_WHILE_STATEMENT
            | JS_WITH_STATEMENT
            | TS_TYPE_ALIAS_DECLARATION
            | TS_ENUM_DECLARATION
            | TS_INTERFACE_DECLARATION
            | TS_DECLARE_STATEMENT
            | TS_DECLARE_FUNCTION_DECLARATION
            | TS_MODULE_DECLARATION
            | TS_GLOBAL_DECLARATION
            | TS_EXTERNAL_MODULE_DECLARATION
            | TS_IMPORT_EQUALS_DECLARATION
            | JS_UNKNOWN_STATEMENT => JS_UNKNOWN_STATEMENT,

            IMPORT_META
            | JS_ARRAY_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_ASSIGNMENT_EXPRESSION
            | JS_AWAIT_EXPRESSION
            | JS_BIG_INT_LITERAL_EXPRESSION
            | JS_BINARY_EXPRESSION
            | JS_BOOLEAN_LITERAL_EXPRESSION
            | JS_CALL_EXPRESSION
            | JS_CLASS_EXPRESSION
            | JS_COMPUTED_MEMBER_EXPRESSION
            | JS_CONDITIONAL_EXPRESSION
            | JS_FUNCTION_EXPRESSION
            | JS_IDENTIFIER_EXPRESSION
            | JS_IMPORT_CALL_EXPRESSION
            | JS_LOGICAL_EXPRESSION
            | JS_NEW_EXPRESSION
            | JS_NULL_LITERAL_EXPRESSION
            | JS_NUMBER_LITERAL_EXPRESSION
            | JS_OBJECT_EXPRESSION
            | JS_PARENTHESIZED_EXPRESSION
            | JS_POST_UPDATE_EXPRESSION
            | JS_PRE_UPDATE_EXPRESSION
            | JS_REGEX_LITERAL_EXPRESSION
            | JS_SEQUENCE_EXPRESSION
            | JS_STATIC_MEMBER_EXPRESSION
            | JS_STRING_LITERAL_EXPRESSION
            | JS_SUPER_EXPRESSION
            | JS_THIS_EXPRESSION
            | JS_UNARY_EXPRESSION
            | JS_YIELD_EXPRESSION
            | NEW_TARGET
            | JS_TEMPLATE
            | TS_AS_EXPRESSION
            | TS_TYPE_ASSERTION_EXPRESSION
            | TS_NON_NULL_ASSERTION_EXPRESSION
            | JSX_TAG_EXPRESSION
            | JS_UNKNOWN_EXPRESSION => JS_UNKNOWN_EXPRESSION,

            JS_OBJECT_BINDING_PATTERN
            | JS_ARRAY_BINDING_PATTERN
            | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
            | JS_BINDING_PATTERN_WITH_DEFAULT
            | JS_IDENTIFIER_BINDING
            | JS_OBJECT_BINDING_PATTERN_PROPERTY
            | JS_OBJECT_BINDING_PATTERN_REST
            | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            | JS_UNKNOWN_BINDING => JS_UNKNOWN_BINDING,

            JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
            | JS_EMPTY_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_GETTER_OBJECT_MEMBER
            | JS_METHOD_CLASS_MEMBER
            | JS_METHOD_OBJECT_MEMBER
            | JS_PRIVATE_CLASS_MEMBER_NAME
            | JS_PROPERTY_CLASS_MEMBER
            | JS_PROPERTY_OBJECT_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_SETTER_OBJECT_MEMBER
            | JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
            | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER
            | TS_GETTER_SIGNATURE_CLASS_MEMBER
            | TS_SETTER_SIGNATURE_CLASS_MEMBER
            | TS_INDEX_SIGNATURE_CLASS_MEMBER
            | TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
            | JS_UNKNOWN_MEMBER => JS_UNKNOWN_MEMBER,

            JS_ARRAY_ASSIGNMENT_PATTERN
            | JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
            | JS_COMPUTED_MEMBER_ASSIGNMENT
            | JS_IDENTIFIER_ASSIGNMENT
            | JS_OBJECT_ASSIGNMENT_PATTERN
            | JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
            | JS_OBJECT_ASSIGNMENT_PATTERN_REST
            | JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
            | JS_PARENTHESIZED_ASSIGNMENT
            | JS_STATIC_MEMBER_ASSIGNMENT
            | TS_AS_ASSIGNMENT
            | TS_NON_NULL_ASSERTION_ASSIGNMENT
            | TS_TYPE_ASSERTION_ASSIGNMENT
            | JS_UNKNOWN_ASSIGNMENT => JS_UNKNOWN_ASSIGNMENT,

            JS_NAMED_IMPORT_SPECIFIER
            | JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
            | JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => JS_UNKNOWN_NAMED_IMPORT_SPECIFIER,

            JS_IMPORT_ASSERTION_ENTRY | JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
                JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
            }

            JS_FORMAL_PARAMETER
            | JS_REST_PARAMETER
            | TS_PROPERTY_PARAMETER
            | TS_THIS_PARAMETER
            | JS_UNKNOWN_PARAMETER => JS_UNKNOWN_PARAMETER,

            _ => JS_UNKNOWN,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }
}

impl TryFrom<JsSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: JsSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                JsSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                JsSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                JsSyntaxKind::MULTILINE_COMMENT => Ok(TriviaPieceKind::MultiLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
