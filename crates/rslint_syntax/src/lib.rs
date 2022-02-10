//! A crate for generated Syntax node definitions and utility macros.
//! Both rslint_lexer and rslint_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#[macro_use]
mod generated;

pub use self::generated::JsSyntaxKind;
use self::generated::JsSyntaxKind::*;
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
            | JS_CLASS_STATEMENT
            | JS_CONTINUE_STATEMENT
            | JS_DEBUGGER_STATEMENT
            | JS_DO_WHILE_STATEMENT
            | JS_EMPTY_STATEMENT
            | JS_EXPRESSION_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FUNCTION_STATEMENT
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
            | TS_TYPE_ALIAS_STATEMENT
            | TS_ENUM_STATEMENT
            | TS_INTERFACE_STATEMENT
            | TS_DECLARE_STATEMENT
            | TS_DECLARE_FUNCTION_STATEMENT
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
            | TS_READONLY_PROPERTY_PARAMETER
            | TS_THIS_PARAMETER
            | JS_UNKNOWN_PARAMETER => JS_UNKNOWN_PARAMETER,

            TS_ENUM_MEMBER => JS_UNKNOWN_MEMBER,

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
