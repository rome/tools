//! Definitions for the ECMAScript AST used for codegen
//! Based on the rust analyzer parser and ast definitions

use quote::{format_ident, quote};

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
		"big_int",
		"number",
		"regex",
	],
	literals: &["NUMBER", "BIG_INT", "FLOAT", "STRING", "REGEX"],
	tokens: &[
		"HASH", // #
		"TEMPLATE_CHUNK",
		"DOLLARCURLY", // ${
		"ERROR_TOKEN",
		"IDENT",
		"WHITESPACE",
		"COMMENT",
		"SHEBANG",
	],
	nodes: &[
		"SCRIPT",
		"MODULE",
		"ERROR",
		"BLOCK_STMT",
		"VAR_DECL",
		"DECLARATOR",
		"EMPTY_STMT",
		"EXPR",
		"EXPR_STMT",
		"IF_STMT",
		"DO_WHILE_STMT",
		"WHILE_STMT",
		"FOR_STMT",
		"FOR_IN_STMT",
		"CONTINUE_STMT",
		"BREAK_STMT",
		"RETURN_STMT",
		"WITH_STMT",
		"SWITCH_STMT",
		"CASE_CLAUSE",
		"DEFAULT_CLAUSE",
		"LABELLED_STMT",
		"THROW_STMT",
		"TRY_STMT",
		"CATCH_CLAUSE",
		"FINALIZER",
		"DEBUGGER_STMT",
		"FN_DECL",
		"NAME",
		"NAME_REF",
		"PARAMETER_LIST",
		"THIS_EXPR",
		"ARRAY_EXPR",
		"OBJECT_EXPR",
		"LITERAL_PROP",
		"GETTER",
		"SETTER",
		"GROUPING_EXPR",
		"NEW_EXPR",
		"FN_EXPR",
		"BRACKET_EXPR",
		"DOT_EXPR",
		"CALL_EXPR",
		"UNARY_EXPR",
		"BIN_EXPR",
		"COND_EXPR",
		"ASSIGN_EXPR",
		"SEQUENCE_EXPR",
		"ARG_LIST",
		"LITERAL",
		"TEMPLATE",
		"TEMPLATE_ELEMENT",
		"CONDITION",
		"SPREAD_ELEMENT",
		"SUPER_CALL",
		"IMPORT_CALL",
		"NEW_TARGET",
		"IMPORT_META",
		"IDENT_PROP",
		"SPREAD_PROP",
		"INITIALIZED_PROP",
		"OBJECT_PATTERN",
		"ARRAY_PATTERN",
		"ASSIGN_PATTERN",
		"REST_PATTERN",
		"KEY_VALUE_PATTERN",
		"COMPUTED_PROPERTY_NAME",
		"FOR_OF_STMT",
		"SINGLE_PATTERN",
		"ARROW_EXPR",
		"YIELD_EXPR",
		"CLASS_DECL",
		"CLASS_EXPR",
		"CLASS_BODY",
		"METHOD",
		"IMPORT_DECL",
		"EXPORT_DECL",
		"EXPORT_NAMED",
		"EXPORT_DEFAULT_DECL",
		"EXPORT_DEFAULT_EXPR",
		"EXPORT_WILDCARD",
		"WILDCARD_IMPORT",
		"NAMED_IMPORTS",
		"SPECIFIER",
		"AWAIT_EXPR",
		// These three are just hacks for converting to ast node without
		// having to handle every error recovery case.
		// in the future we might just tag the underlying rowan nodes
		"FOR_STMT_TEST",
		"FOR_STMT_UPDATE",
		"FOR_STMT_INIT",
		"PRIVATE_NAME",
		"CLASS_PROP",
		"PRIVATE_PROP",
		"CONSTRUCTOR",
		"CONSTRUCTOR_PARAMETERS",
		"PRIVATE_PROP_ACCESS",
		"IMPORT_STRING_SPECIFIER",
		"EXPR_PATTERN",
		// TypeScript
		"TS_ANY",
		"TS_UNKNOWN",
		"TS_NUMBER",
		"TS_OBJECT",
		"TS_BOOLEAN",
		"TS_BIGINT",
		"TS_STRING",
		"TS_SYMBOL",
		"TS_VOID",
		"TS_UNDEFINED",
		"TS_NULL",
		"TS_NEVER",
		"TS_THIS",
		"TS_LITERAL",
		"TS_PREDICATE",
		"TS_TUPLE",
		"TS_TUPLE_ELEMENT",
		"TS_PAREN",
		"TS_TYPE_REF",
		"TS_QUALIFIED_PATH",
		"TS_TYPE_NAME",
		"TS_TEMPLATE",
		"TS_TEMPLATE_ELEMENT",
		"TS_MAPPED_TYPE",
		"TS_MAPPED_TYPE_PARAM",
		"TS_MAPPED_TYPE_READONLY",
		"TS_TYPE_QUERY",
		"TS_TYPE_QUERY_EXPR",
		"TS_IMPORT",
		"TS_TYPE_ARGS",
		"TS_ARRAY",
		"TS_INDEXED_ARRAY",
		"TS_TYPE_OPERATOR",
		"TS_INTERSECTION",
		"TS_UNION",
		"TS_TYPE_PARAMS",
		"TS_FN_TYPE",
		"TS_CONSTRUCTOR_TYPE",
		"TS_EXTENDS",
		"TS_CONDITIONAL_TYPE",
		"TS_CONSTRAINT",
		"TS_DEFAULT",
		"TS_TYPE_PARAM",
		"TS_NON_NULL",
		"TS_ASSERTION",
		"TS_CONST_ASSERTION",
		"TS_ENUM",
		"TS_ENUM_MEMBER",
		"TS_TYPE_ALIAS_DECL",
		"TS_NAMESPACE_DECL",
		"TS_MODULE_BLOCK",
		"TS_MODULE_DECL",
		"TS_CONSTRUCTOR_PARAM",
		"TS_CALL_SIGNATURE_DECL",
		"TS_CONSTRUCT_SIGNATURE_DECL",
		"TS_INDEX_SIGNATURE",
		"TS_METHOD_SIGNATURE",
		"TS_PROPERTY_SIGNATURE",
		"TS_INTERFACE_DECL",
		"TS_ACCESSIBILITY",
		"TS_OBJECT_TYPE",
		"TS_EXPR_WITH_TYPE_ARGS",
		"TS_IMPORT_EQUALS_DECL",
		"TS_MODULE_REF",
		"TS_EXTERNAL_MODULE_REF",
		"TS_EXPORT_ASSIGNMENT",
		"TS_NAMESPACE_EXPORT_DECL",
		"TS_DECORATOR",
		"DEFAULT_CASE",
		"TS_INFER",
		"NULL",
		"UNDEFINED",
		"PROP_NAME",
		"STMT",
		"DECL",
		"PATTERN",
		"TS_ENTITY_NAME",
		"BOOLEAN",
		"BIG_INT_VALUE",
	],
};

#[derive(Default, Debug)]
pub struct AstSrc {
	pub tokens: Vec<String>,
	pub nodes: Vec<AstNodeSrc>,
	pub enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub struct AstNodeSrc {
	pub documentation: Vec<String>,
	pub name: String,
	// pub traits: Vec<String>,
	pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
	Token {
		name: String,
		tokens: Option<Vec<String>>,
	},
	Node {
		name: String,
		ty: String,
		optional: bool,
		has_many: bool,
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
	#[allow(dead_code)]
	pub fn is_many(&self) -> bool {
		matches!(self, Field::Node { .. })
	}
	pub fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
		match self {
			Field::Token { name, .. } => {
				let token: proc_macro2::TokenStream = name.parse().unwrap();
				Some(quote! { T![#token] })
			}
			_ => None,
		}
	}

	pub fn extract_tokens(&self) -> Option<proc_macro2::TokenStream> {
		match self {
			Field::Token { tokens, .. } => {
				if let Some(tokens) = tokens {
					let streamed_tokens: Vec<proc_macro2::TokenStream> = tokens
						.iter()
						.map(|token| {
							let token: proc_macro2::TokenStream = token.parse().unwrap();
							quote! {
								T![#token]
							}
						})
						.collect();

					let q = quote! {
							&[#(#streamed_tokens),*]
					};

					Some(q)
				} else {
					None
				}
			}
			_ => None,
		}
	}

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
				if name == "type" {
					format_ident!("ty")
				} else {
					format_ident!("{}", name)
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
}
