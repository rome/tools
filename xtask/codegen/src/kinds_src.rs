//! Definitions for the ECMAScript AST used for codegen
//! Based on the rust analyzer parser and ast definitions

use quote::format_ident;
use std::collections::BTreeMap;

const LANGUAGE_PREFIXES: [&str; 4] = ["js_", "ts_", "jsx_", "tsx_"];

pub struct KindsSrc<'a> {
    pub punct: &'a [(&'a str, &'a str)],
    pub keywords: &'a [&'a str],
    pub literals: &'a [&'a str],
    pub tokens: &'a [&'a str],
    pub nodes: &'a [&'a str],
}

pub const KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (";", "SEMICOLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("<", "L_ANGLE"),
        (">", "R_ANGLE"),
        ("~", "TILDE"),
        ("?", "QUESTION"),
        ("??", "QUESTION2"),
        // These are *not* question AND dot tokens, they are one
        // to distinguish between `? .3134` and `?.` per ecma specs
        ("?.", "QUESTIONDOT"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("+", "PLUS"),
        ("++", "PLUS2"),
        ("*", "STAR"),
        ("**", "STAR2"),
        ("/", "SLASH"),
        ("^", "CARET"),
        ("%", "PERCENT"),
        (".", "DOT"),
        ("...", "DOT2"),
        (":", "COLON"),
        ("=", "EQ"),
        ("==", "EQ2"),
        ("===", "EQ3"),
        ("=>", "FAT_ARROW"),
        ("!", "BANG"),
        ("!=", "NEQ"),
        ("!==", "NEQ2"),
        ("-", "MINUS"),
        ("--", "MINUS2"),
        ("<=", "LTEQ"),
        (">=", "GTEQ"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("|=", "PIPEEQ"),
        ("&=", "AMPEQ"),
        ("^=", "CARETEQ"),
        ("/=", "SLASHEQ"),
        ("*=", "STAREQ"),
        ("%=", "PERCENTEQ"),
        ("&&", "AMP2"),
        ("||", "PIPE2"),
        ("<<", "SHL"),
        (">>", "SHR"),
        (">>>", "USHR"),
        ("<<=", "SHLEQ"),
        (">>=", "SHREQ"),
        (">>>=", "USHREQ"),
        ("&&=", "AMP2EQ"),
        ("||=", "PIPE2EQ"),
        ("**=", "STAR2EQ"),
        ("??=", "QUESTION2EQ"),
        ("@", "AT"),
        ("`", "BACKTICK"),
    ],
    keywords: &[
        "await",
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "debugger",
        "default",
        "delete",
        "do",
        "else",
        "enum",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "function",
        "if",
        "in",
        "instanceof",
        "interface",
        "import",
        "implements",
        "new",
        "null",
        "package",
        "private",
        "protected",
        "public",
        "return",
        "super",
        "switch",
        "this",
        "throw",
        "try",
        "true",
        "typeof",
        "var",
        "void",
        "while",
        "with",
        "yield",
        // contextual keywords
        "readonly",
        "keyof",
        "unique",
        "declare",
        "abstract",
        "static",
        "async",
        "type",
        "from",
        "as",
        "require",
        "namespace",
        "assert",
        "module",
        "global",
        "infer",
        "get",
        "set",
        "of",
        "target",
        "never",
        "unknown",
        "any",
        "undefined",
        "let",
        "float",
        "number",
        "symbol",
        "string",
        "object",
        "boolean",
        "bigint",
        "meta",
        "is",
        "asserts",
    ],
    literals: &[
        "JS_NUMBER_LITERAL",
        "JS_BIG_INT_LITERAL",
        "JS_STRING_LITERAL",
        "JS_REGEX_LITERAL",
    ],
    tokens: &[
        "HASH", // #
        "TEMPLATE_CHUNK",
        "DOLLAR_CURLY", // ${
        "ERROR_TOKEN",
        "IDENT",
        "NEWLINE",
        "WHITESPACE",
        "COMMENT",
        "MULTILINE_COMMENT",
        "JS_SHEBANG",
    ],
    nodes: &[
        "JS_MODULE",
        "JS_MODULE_ITEM_LIST",
        "JS_SCRIPT",
        "JS_EXPRESSION_SNIPPED",
        "JS_DIRECTIVE",
        "JS_DIRECTIVE_LIST",
        "JS_STATEMENT_LIST",
        "JS_BLOCK_STATEMENT",
        "JS_FUNCTION_BODY",
        "JS_VARIABLE_STATEMENT",
        "JS_VARIABLE_DECLARATIONS",
        "JS_VARIABLE_DECLARATION_LIST",
        "JS_VARIABLE_DECLARATION",
        "TS_DEFINITE_VARIABLE_ANNOTATION",
        "JS_INITIALIZER_CLAUSE",
        "JS_EMPTY_STATEMENT",
        "JS_EXPRESSION_STATEMENT",
        "JS_IF_STATEMENT",
        "JS_ELSE_CLAUSE",
        "JS_DO_WHILE_STATEMENT",
        "JS_WHILE_STATEMENT",
        "JS_FOR_STATEMENT",
        "JS_FOR_IN_STATEMENT",
        "JS_FOR_OF_STATEMENT",
        "JS_FOR_VARIABLE_DECLARATION",
        "JS_CONTINUE_STATEMENT",
        "JS_BREAK_STATEMENT",
        "JS_RETURN_STATEMENT",
        "JS_WITH_STATEMENT",
        "JS_SWITCH_STATEMENT",
        "JS_SWITCH_CASE_LIST",
        "JS_CASE_CLAUSE",
        "JS_DEFAULT_CLAUSE",
        "JS_LABELED_STATEMENT",
        "JS_THROW_STATEMENT",
        "JS_TRY_STATEMENT",
        "JS_TRY_FINALLY_STATEMENT",
        "JS_CATCH_CLAUSE",
        "JS_CATCH_DECLARATION",
        "JS_FINALLY_CLAUSE",
        "JS_DEBUGGER_STATEMENT",
        "JS_FUNCTION_STATEMENT",
        "JS_PARAMETERS",
        "JS_PARAMETER_LIST",
        "JS_FORMAL_PARAMETER",
        "JS_REST_PARAMETER",
        "TS_THIS_PARAMETER",
        "TS_PROPERTY_PARAMETER",
        "TS_READONLY_PROPERTY_PARAMETER",
        "TS_TYPE_ANNOTATION",
        "TS_RETURN_TYPE_ANNOTATION",
        "JS_IDENTIFIER_BINDING",
        "JS_IDENTIFIER_EXPRESSION",
        "JS_REFERENCE_IDENTIFIER",
        "JS_NAME",
        "JS_PRIVATE_NAME",
        "JS_THIS_EXPRESSION",
        "JS_ARRAY_EXPRESSION",
        "JS_ARRAY_ELEMENT_LIST",
        "JS_ARRAY_HOLE",
        "JS_COMPUTED_MEMBER_NAME",
        "JS_LITERAL_MEMBER_NAME",
        "JS_OBJECT_EXPRESSION",
        "JS_OBJECT_MEMBER_LIST",
        "JS_PROPERTY_OBJECT_MEMBER",
        "JS_GETTER_OBJECT_MEMBER",
        "JS_SETTER_OBJECT_MEMBER",
        "JS_METHOD_OBJECT_MEMBER",
        "JS_SUPER_EXPRESSION",
        "JS_PARENTHESIZED_EXPRESSION",
        "JS_NEW_EXPRESSION",
        "JS_FUNCTION_EXPRESSION",
        "JS_STATIC_MEMBER_EXPRESSION",
        "JS_COMPUTED_MEMBER_EXPRESSION",
        "JS_CALL_EXPRESSION",
        "JS_UNARY_EXPRESSION",
        "JS_PRE_UPDATE_EXPRESSION",
        "JS_POST_UPDATE_EXPRESSION",
        "JS_BINARY_EXPRESSION",
        "JS_INSTANCEOF_EXPRESSION",
        "JS_IN_EXPRESSION",
        "JS_LOGICAL_EXPRESSION",
        "JS_CONDITIONAL_EXPRESSION",
        "JS_ASSIGNMENT_EXPRESSION",
        "JS_SEQUENCE_EXPRESSION",
        "JS_CALL_ARGUMENTS",
        "JS_CALL_ARGUMENT_LIST",
        "JS_STRING_LITERAL_EXPRESSION",
        "JS_NUMBER_LITERAL_EXPRESSION",
        "JS_BIG_INT_LITERAL_EXPRESSION",
        "JS_BOOLEAN_LITERAL_EXPRESSION",
        "JS_NULL_LITERAL_EXPRESSION",
        "JS_REGEX_LITERAL_EXPRESSION",
        "JS_TEMPLATE",
        "JS_TEMPLATE_ELEMENT",
        "JS_TEMPLATE_CHUNK_ELEMENT",
        "JS_TEMPLATE_ELEMENT_LIST",
        "JS_IMPORT_CALL_EXPRESSION",
        "NEW_TARGET",
        "IMPORT_META",
        "JS_SHORTHAND_PROPERTY_OBJECT_MEMBER",
        "JS_SPREAD",
        "JS_OBJECT_BINDING_PATTERN",
        "JS_ARRAY_BINDING_PATTERN",
        "JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST",
        "JS_BINDING_PATTERN_WITH_DEFAULT",
        "JS_ARRAY_BINDING_PATTERN_REST_ELEMENT",
        "JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST",
        "JS_OBJECT_BINDING_PATTERN_REST",
        "JS_OBJECT_BINDING_PATTERN_PROPERTY",
        "JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY",
        "JS_ARROW_FUNCTION_EXPRESSION",
        "JS_YIELD_EXPRESSION",
        "JS_YIELD_ARGUMENT",
        "JS_CLASS_STATEMENT",
        "JS_CLASS_EXPRESSION",
        "JS_CLASS_MEMBER_LIST",
        "JS_EXTENDS_CLAUSE",
        "JS_PRIVATE_CLASS_MEMBER_NAME",
        "JS_CONSTRUCTOR_CLASS_MEMBER",
        "JS_CONSTRUCTOR_PARAMETER_LIST",
        "JS_CONSTRUCTOR_PARAMETERS",
        "JS_CONSTRUCTOR_PARAMETER",
        "JS_PROPERTY_CLASS_MEMBER",
        "TS_OPTIONAL_PROPERTY_ANNOTATION",
        "TS_DEFINITE_PROPERTY_ANNOTATION",
        "JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER",
        "JS_METHOD_CLASS_MEMBER",
        "JS_GETTER_CLASS_MEMBER",
        "JS_SETTER_CLASS_MEMBER",
        "JS_EMPTY_CLASS_MEMBER",
        "JS_ASSIGNMENT_WITH_DEFAULT",
        "JS_PARENTHESIZED_ASSIGNMENT",
        "JS_IDENTIFIER_ASSIGNMENT",
        "JS_STATIC_MEMBER_ASSIGNMENT",
        "JS_COMPUTED_MEMBER_ASSIGNMENT",
        "JS_ARRAY_ASSIGNMENT_PATTERN",
        "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST",
        "JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT",
        "JS_OBJECT_ASSIGNMENT_PATTERN",
        "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST",
        "JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY",
        "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY",
        "JS_OBJECT_ASSIGNMENT_PATTERN_REST",
        "JS_IMPORT",
        "JS_IMPORT_BARE_CLAUSE",
        "JS_IMPORT_DEFAULT_CLAUSE",
        "JS_IMPORT_NAMESPACE_CLAUSE",
        "JS_IMPORT_NAMED_CLAUSE",
        "JS_NAMED_IMPORT_SPECIFIERS",
        "JS_NAMED_IMPORT_SPECIFIER_LIST",
        "JS_NAMESPACE_IMPORT_SPECIFIER",
        "JS_DEFAULT_IMPORT_SPECIFIER",
        "JS_NAMED_IMPORT_SPECIFIER",
        "JS_SHORTHAND_NAMED_IMPORT_SPECIFIER",
        "JS_IMPORT_ASSERTION",
        "JS_IMPORT_ASSERTION_ENTRY_LIST",
        "JS_IMPORT_ASSERTION_ENTRY",
        "JS_MODULE_SOURCE",
        "JS_EXPORT",
        "JS_EXPORT_VARIABLE_CLAUSE",
        "JS_EXPORT_FUNCTION_CLAUSE",
        "JS_EXPORT_CLASS_CLAUSE",
        "JS_EXPORT_NAMED_CLAUSE",
        "JS_EXPORT_NAMED_SPECIFIER_LIST",
        "JS_EXPORT_NAMED_SHORTHAND_SPECIFIER",
        "JS_EXPORT_NAMED_SPECIFIER",
        "JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE",
        "JS_EXPORT_DEFAULT_FUNCTION_CLAUSE",
        "JS_EXPORT_DEFAULT_CLASS_CLAUSE",
        "JS_EXPORT_FROM_CLAUSE",
        "JS_EXPORT_NAMED_FROM_CLAUSE",
        "JS_EXPORT_NAMED_FROM_SPECIFIER_LIST",
        "JS_EXPORT_NAMED_FROM_SPECIFIER",
        "JS_EXPORT_AS_CLAUSE",
        "JS_LITERAL_EXPORT_NAME",
        "JS_AWAIT_EXPRESSION",
        // TypeScript
        "TS_IDENTIFIER_BINDING",
        "TS_ANY_TYPE",
        "TS_UNKNOWN_TYPE",
        "TS_NUMBER_TYPE",
        "TS_NON_PRIMITIVE_TYPE",
        "TS_BOOLEAN_TYPE",
        "TS_BIGINT_TYPE",
        "TS_STRING_TYPE",
        "TS_SYMBOL_TYPE",
        "TS_VOID_TYPE",
        "TS_UNDEFINED_TYPE",
        "TS_NEVER_TYPE",
        "TS_THIS_TYPE",
        "TS_TYPEOF_TYPE",
        "TS_PARENTHESIZED_TYPE",
        "TS_MAPPED_TYPE",
        "TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE",
        "TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE",
        "TS_MAPPED_TYPE_AS_CLAUSE",
        "TS_TYPE_ALIAS_STATEMENT",
        "TS_QUALIFIED_NAME",
        "TS_REFERENCE_TYPE",
        "TS_UNION_TYPE",
        "TS_UNION_TYPE_VARIANT_LIST",
        "TS_INTERSECTION_TYPE",
        "TS_INTERSECTION_TYPE_ELEMENT_LIST",
        "TS_OBJECT_TYPE",
        "TS_TYPE_MEMBER_LIST",
        "TS_INTERFACE_STATEMENT",
        "TS_INTERFACE_EXTENDS_CLAUSE",
        "TS_PROPERTY_SIGNATURE_TYPE_MEMBER",
        "TS_METHOD_SIGNATURE_TYPE_MEMBER",
        "TS_CALL_SIGNATURE_TYPE_MEMBER",
        "TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER",
        "TS_GETTER_SIGNATURE_TYPE_MEMBER",
        "TS_SETTER_SIGNATURE_TYPE_MEMBER",
        "TS_INDEX_SIGNATURE_TYPE_MEMBER",
        "TS_INDEX_SIGNATURE_PARAMETER",
        "TS_IMPORT_TYPE",
        "TS_IMPORT_TYPE_QUALIFIER",
        "TS_ARRAY_TYPE",
        "TS_INDEXED_ACCESS_TYPE",
        "TS_TUPLE_TYPE",
        "TS_TUPLE_TYPE_ELEMENT_LIST",
        "TS_REST_TUPLE_TYPE_ELEMENT",
        "TS_OPTIONAL_TUPLE_TYPE_ELEMENT",
        "TS_NAMED_TUPLE_TYPE_ELEMENT",
        "TS_TYPE_OPERATOR_TYPE",
        "TS_INFER_TYPE",
        "TS_CONSTRUCTOR_TYPE",
        "TS_FUNCTION_TYPE",
        "TS_TYPE_PREDICATE",
        "TS_TYPE_PARAMETERS",
        "TS_TYPE_PARAMETER_LIST",
        "TS_TYPE_PARAMETER",
        "TS_TYPE_PARAMETER_NAME",
        "TS_TYPE_CONSTRAINT_CLAUSE",
        "TS_DEFAULT_TYPE_CLAUSE",
        "TS_STRING_LITERAL_TYPE",
        "TS_NUMBER_LITERAL_TYPE",
        "TS_BIG_INT_LITERAL_TYPE",
        "TS_BOOLEAN_LITERAL_TYPE",
        "TS_NULL_LITERAL_TYPE",
        "TS_TEMPLATE_LITERAL_TYPE",
        "TS_TEMPLATE_ELEMENT_LIST",
        "TS_TEMPLATE_CHUNK_ELEMENT",
        "TS_TEMPLATE_ELEMENT",
        "TS_TYPE_ARGUMENTS",
        "TS_TYPE_ARGUMENT_LIST",
        "TS_IMPLEMENTS_CLAUSE",
        "TS_TYPE_LIST",
        "TS_EXTENDS",
        "TS_CONDITIONAL_TYPE",
        "TS_NON_NULL_ASSERTION_EXPRESSION",
        "TS_TYPE_ASSERTION_EXPRESSION",
        "TS_AS_EXPRESSION",
        "TS_ENUM_STATEMENT",
        "TS_ENUM_MEMBER_LIST",
        "TS_ENUM_MEMBER",
        "TS_NAME_WITH_TYPE_ARGUMENTS",
        "TS_IMPORT_EQUALS_DECL",
        "TS_MODULE_REF",
        "TS_EXTERNAL_MODULE_REF",
        "TS_DECORATOR",
        // unknown nodes JS
        "JS_UNKNOWN",
        "JS_UNKNOWN_EXPRESSION",
        "JS_UNKNOWN_STATEMENT",
        "JS_UNKNOWN_MEMBER",
        "JS_UNKNOWN_BINDING",
        "JS_UNKNOWN_PARAMETER",
        "JS_UNKNOWN_IMPORT_ASSERTION_ENTRY",
        "JS_UNKNOWN_NAMED_IMPORT_SPECIFIER",
        "JS_UNKNOWN_ASSIGNMENT",
    ],
};

#[derive(Default, Debug)]
pub struct AstSrc {
    pub nodes: Vec<AstNodeSrc>,
    pub unions: Vec<AstEnumSrc>,
    lists: BTreeMap<String, AstListSrc>,
    pub unknowns: Vec<String>,
}

impl AstSrc {
    pub fn push_list(&mut self, name: &str, src: AstListSrc) {
        self.lists.insert(String::from(name), src);
    }

    pub fn lists(&self) -> std::collections::btree_map::Iter<String, AstListSrc> {
        self.lists.iter()
    }

    pub fn is_list(&self, name: &str) -> bool {
        self.lists.contains_key(name)
    }

    /// Sorts all nodes, enums, etc. for a stable code gen result
    pub fn sort(&mut self) {
        // No need to sort lists, they're stored in a btree
        self.nodes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.unions.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        self.unknowns.sort_unstable();

        for union in self.unions.iter_mut() {
            union.variants.sort_unstable();
        }
    }
}

#[derive(Debug)]
pub struct AstListSrc {
    pub element_name: String,
    pub separator: Option<AstListSeparatorConfiguration>,
}

#[derive(Debug)]
pub struct AstListSeparatorConfiguration {
    /// Name of the separator token
    pub separator_token: String,
    /// Whatever the list allows a trailing comma or not
    pub allow_trailing: bool,
}

#[derive(Debug)]
pub struct AstNodeSrc {
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token {
        name: String,
        kind: TokenKind,
        optional: bool,
    },
    Node {
        name: String,
        ty: String,
        optional: bool,
    },
}

#[derive(Debug, Clone)]
pub struct AstEnumSrc {
    pub documentation: Vec<String>,
    pub name: String,
    // pub traits: Vec<String>,
    pub variants: Vec<String>,
}

impl Field {
    pub fn method_name(&self) -> proc_macro2::Ident {
        match self {
            Field::Token { name, .. } => {
                let name = match name.as_str() {
                    ";" => "semicolon",
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    "'`'" => "backtick",
                    "<" => "l_angle",
                    ">" => "r_angle",
                    "=" => "eq",
                    "!" => "excl",
                    "*" => "star",
                    "&" => "amp",
                    "." => "dot",
                    "..." => "dotdotdot",
                    "=>" => "fat_arrow",
                    ":" => "colon",
                    "?" => "question_mark",
                    "+" => "plus",
                    "-" => "minus",
                    "#" => "hash",
                    "@" => "at",
                    "+=" => "add_assign",
                    "-=" => "subtract_assign",
                    "*=" => "times_assign",
                    "%=" => "remainder_assign",
                    "**=" => "exponent_assign",
                    ">>=" => "left_shift_assign",
                    "<<=" => "right_shift_assign",
                    ">>>=" => "unsigned_right_shift_assign",
                    "~" => "bitwise_not",
                    "&=" => "bitwise_and_assign",
                    "|=" => "bitwise_or_assign",
                    "^=" => "bitwise_xor_assign",
                    "&&=" => "bitwise_logical_and_assign",
                    "||=" => "bitwise_logical_or_assign",
                    "??=" => "bitwise_nullish_coalescing_assign",
                    "++" => "increment",
                    "--" => "decrement",
                    "<=" => "less_than_equal",
                    ">=" => "greater_than_equal",
                    "==" => "equality",
                    "===" => "strict_equality",
                    "!=" => "inequality",
                    "!==" => "strict_inequality",
                    "/" => "divide",
                    "%" => "reminder",
                    "**" => "exponent",
                    "<<" => "left_shift",
                    ">>" => "right_shift",
                    ">>>" => "unsigned_right_shift",
                    "|" => "bitwise_or",
                    "^" => "bitwise_xor",
                    "??" => "nullish_coalescing",
                    "||" => "logical_or",
                    "&&" => "logical_and",
                    _ => name,
                };
                format_ident!("{}_token", name)
            }
            Field::Node { name, .. } => {
                let name = name;
                let (prefix, tail) = name.split_once('_').unwrap_or(("", name));
                let final_name = if LANGUAGE_PREFIXES.contains(&prefix) {
                    tail
                } else {
                    name.as_str()
                };

                // this check here is to avoid emitting methods called "type()",
                // where "type" is a reserved word
                if final_name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", final_name)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token { .. } => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Field::Node { optional, .. } => *optional,
            Field::Token { optional, .. } => *optional,
        }
    }
}
