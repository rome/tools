//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
	#[doc(hidden)]
	TOMBSTONE,
	EOF,
	#[doc = r" Polymorph list type"]
	LIST,
	SEMICOLON,
	COMMA,
	L_PAREN,
	R_PAREN,
	L_CURLY,
	R_CURLY,
	L_BRACK,
	R_BRACK,
	L_ANGLE,
	R_ANGLE,
	TILDE,
	QUESTION,
	QUESTION2,
	QUESTIONDOT,
	AMP,
	PIPE,
	PLUS,
	PLUS2,
	STAR,
	STAR2,
	SLASH,
	CARET,
	PERCENT,
	DOT,
	DOT2,
	COLON,
	EQ,
	EQ2,
	EQ3,
	FAT_ARROW,
	BANG,
	NEQ,
	NEQ2,
	MINUS,
	MINUS2,
	LTEQ,
	GTEQ,
	PLUSEQ,
	MINUSEQ,
	PIPEEQ,
	AMPEQ,
	CARETEQ,
	SLASHEQ,
	STAREQ,
	PERCENTEQ,
	AMP2,
	PIPE2,
	SHL,
	SHR,
	USHR,
	SHLEQ,
	SHREQ,
	USHREQ,
	AMP2EQ,
	PIPE2EQ,
	STAR2EQ,
	QUESTION2EQ,
	AT,
	BACKTICK,
	AWAIT_KW,
	BREAK_KW,
	CASE_KW,
	CATCH_KW,
	CLASS_KW,
	CONST_KW,
	CONTINUE_KW,
	DEBUGGER_KW,
	DEFAULT_KW,
	DELETE_KW,
	DO_KW,
	ELSE_KW,
	ENUM_KW,
	EXPORT_KW,
	EXTENDS_KW,
	FALSE_KW,
	FINALLY_KW,
	FOR_KW,
	FUNCTION_KW,
	IF_KW,
	IN_KW,
	INSTANCEOF_KW,
	INTERFACE_KW,
	IMPORT_KW,
	IMPLEMENTS_KW,
	NEW_KW,
	NULL_KW,
	PACKAGE_KW,
	PRIVATE_KW,
	PROTECTED_KW,
	PUBLIC_KW,
	RETURN_KW,
	SUPER_KW,
	SWITCH_KW,
	THIS_KW,
	THROW_KW,
	TRY_KW,
	TRUE_KW,
	TYPEOF_KW,
	VAR_KW,
	VOID_KW,
	WHILE_KW,
	WITH_KW,
	YIELD_KW,
	READONLY_KW,
	KEYOF_KW,
	UNIQUE_KW,
	DECLARE_KW,
	ABSTRACT_KW,
	STATIC_KW,
	ASYNC_KW,
	TYPE_KW,
	FROM_KW,
	AS_KW,
	REQUIRE_KW,
	NAMESPACE_KW,
	ASSERT_KW,
	MODULE_KW,
	GLOBAL_KW,
	INFER_KW,
	GET_KW,
	SET_KW,
	OF_KW,
	TARGET_KW,
	NEVER_KW,
	UNKNOWN_KW,
	ANY_KW,
	UNDEFINED_KW,
	LET_KW,
	FLOAT_KW,
	NUMBER_KW,
	JS_NUMBER_LITERAL_TOKEN,
	JS_BIG_INT_LITERAL_TOKEN,
	JS_STRING_LITERAL_TOKEN,
	JS_REGEX_LITERAL_TOKEN,
	HASH,
	TEMPLATE_CHUNK,
	DOLLARCURLY,
	ERROR_TOKEN,
	IDENT,
	WHITESPACE,
	COMMENT,
	JS_SHEBANG,
	JS_ROOT,
	JS_DIRECTIVE,
	ERROR,
	JS_BLOCK_STATEMENT,
	JS_FUNCTION_BODY,
	JS_VARIABLE_DECLARATION_STATEMENT,
	JS_VARIABLE_DECLARATION,
	JS_VARIABLE_DECLARATOR,
	JS_EQUAL_VALUE_CLAUSE,
	JS_EMPTY_STATEMENT,
	EXPR,
	JS_EXPRESSION_STATEMENT,
	JS_IF_STATEMENT,
	JS_ELSE_CLAUSE,
	JS_DO_WHILE_STATEMENT,
	JS_WHILE_STATEMENT,
	FOR_STMT,
	FOR_IN_STMT,
	JS_CONTINUE_STATEMENT,
	JS_BREAK_STATEMENT,
	JS_RETURN_STATEMENT,
	JS_WITH_STATEMENT,
	JS_SWITCH_STATEMENT,
	JS_CASE_CLAUSE,
	JS_DEFAULT_CLAUSE,
	JS_LABELED_STATEMENT,
	JS_THROW_STATEMENT,
	JS_TRY_STATEMENT,
	JS_TRY_FINALLY_STATEMENT,
	JS_CATCH_CLAUSE,
	JS_CATCH_DECLARATION,
	JS_FINALLY_CLAUSE,
	JS_DEBUGGER_STATEMENT,
	JS_FUNCTION_DECLARATION,
	JS_PARAMETER_LIST,
	JS_REST_PARAMETER,
	TS_TYPE_ANNOTATION,
	JS_IDENTIFIER_BINDING,
	NAME,
	JS_REFERENCE_IDENTIFIER_EXPRESSION,
	PARAMETER_LIST,
	JS_THIS_EXPRESSION,
	JS_ARRAY_EXPRESSION,
	JS_ARRAY_HOLE,
	JS_COMPUTED_MEMBER_NAME,
	JS_LITERAL_MEMBER_NAME,
	JS_OBJECT_EXPRESSION,
	JS_PROPERTY_OBJECT_MEMBER,
	JS_GETTER_OBJECT_MEMBER,
	JS_SETTER_OBJECT_MEMBER,
	JS_METHOD_OBJECT_MEMBER,
	JS_PARENTHESIZED_EXPRESSION,
	NEW_EXPR,
	JS_FUNCTION_EXPRESSION,
	BRACKET_EXPR,
	DOT_EXPR,
	CALL_EXPR,
	JS_UNARY_EXPRESSION,
	JS_PRE_UPDATE_EXPRESSION,
	JS_POST_UPDATE_EXPRESSION,
	JS_BINARY_EXPRESSION,
	JS_LOGICAL_EXPRESSION,
	JS_CONDITIONAL_EXPRESSION,
	ASSIGN_EXPR,
	JS_SEQUENCE_EXPRESSION,
	ARG_LIST,
	JS_STRING_LITERAL,
	JS_NUMBER_LITERAL,
	JS_BIG_INT_LITERAL,
	JS_BOOLEAN_LITERAL,
	JS_NULL_LITERAL,
	JS_REGEX_LITERAL,
	TEMPLATE,
	TEMPLATE_ELEMENT,
	CONDITION,
	SPREAD_ELEMENT,
	SUPER_CALL,
	JS_IMPORT_CALL_EXPRESSION,
	NEW_TARGET,
	IMPORT_META,
	JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
	JS_SPREAD,
	INITIALIZED_PROP,
	OBJECT_PATTERN,
	ARRAY_PATTERN,
	ASSIGN_PATTERN,
	REST_PATTERN,
	KEY_VALUE_PATTERN,
	COMPUTED_PROPERTY_NAME,
	FOR_OF_STMT,
	SINGLE_PATTERN,
	JS_ARROW_FUNCTION_EXPRESSION,
	JS_YIELD_EXPRESSION,
	JS_CLASS_DECLARATION,
	JS_CLASS_EXPRESSION,
	JS_EXTENDS_CLAUSE,
	JS_PRIVATE_CLASS_MEMBER_NAME,
	JS_CONSTRUCTOR_CLASS_MEMBER,
	JS_CONSTRUCTOR_PARAMETER_LIST,
	JS_CONSTRUCTOR_PARAMETER,
	JS_PROPERTY_CLASS_MEMBER,
	JS_METHOD_CLASS_MEMBER,
	JS_GETTER_CLASS_MEMBER,
	JS_SETTER_CLASS_MEMBER,
	JS_EMPTY_CLASS_MEMBER,
	IMPORT_DECL,
	EXPORT_DECL,
	EXPORT_NAMED,
	EXPORT_DEFAULT_DECL,
	EXPORT_DEFAULT_EXPR,
	EXPORT_WILDCARD,
	WILDCARD_IMPORT,
	NAMED_IMPORTS,
	SPECIFIER,
	JS_AWAIT_EXPRESSION,
	FOR_STMT_TEST,
	FOR_STMT_UPDATE,
	FOR_STMT_INIT,
	PRIVATE_NAME,
	PRIVATE_PROP_ACCESS,
	IMPORT_STRING_SPECIFIER,
	EXPR_PATTERN,
	TS_ANY,
	TS_UNKNOWN,
	TS_NUMBER,
	TS_OBJECT,
	TS_BOOLEAN,
	TS_BIGINT,
	TS_STRING,
	TS_SYMBOL,
	TS_VOID,
	TS_UNDEFINED,
	TS_NULL,
	TS_NEVER,
	TS_THIS,
	TS_LITERAL,
	TS_PREDICATE,
	TS_TUPLE,
	TS_TUPLE_ELEMENT,
	TS_PAREN,
	TS_TYPE_REF,
	TS_QUALIFIED_PATH,
	TS_TYPE_NAME,
	TS_TEMPLATE,
	TS_TEMPLATE_ELEMENT,
	TS_MAPPED_TYPE,
	TS_MAPPED_TYPE_PARAM,
	TS_MAPPED_TYPE_READONLY,
	TS_TYPE_QUERY,
	TS_TYPE_QUERY_EXPR,
	TS_IMPORT,
	TS_TYPE_ARGS,
	TS_ARRAY,
	TS_INDEXED_ARRAY,
	TS_TYPE_OPERATOR,
	TS_INTERSECTION,
	TS_UNION,
	TS_TYPE_PARAMS,
	TS_FN_TYPE,
	TS_CONSTRUCTOR_TYPE,
	TS_IMPLEMENTS_CLAUSE,
	TS_EXTENDS,
	TS_CONDITIONAL_TYPE,
	TS_CONSTRAINT,
	TS_DEFAULT,
	TS_TYPE_PARAM,
	TS_NON_NULL,
	TS_ASSERTION,
	TS_CONST_ASSERTION,
	TS_ENUM,
	TS_ENUM_MEMBER,
	TS_TYPE_ALIAS_DECL,
	TS_NAMESPACE_DECL,
	TS_MODULE_BLOCK,
	TS_MODULE_DECL,
	TS_CONSTRUCTOR_PARAM,
	TS_CALL_SIGNATURE_DECL,
	TS_CONSTRUCT_SIGNATURE_DECL,
	TS_INDEX_SIGNATURE,
	TS_METHOD_SIGNATURE,
	TS_PROPERTY_SIGNATURE,
	TS_INTERFACE_DECL,
	TS_ACCESSIBILITY,
	TS_OBJECT_TYPE,
	TS_EXPR_WITH_TYPE_ARGS,
	TS_IMPORT_EQUALS_DECL,
	TS_MODULE_REF,
	TS_EXTERNAL_MODULE_REF,
	TS_EXPORT_ASSIGNMENT,
	TS_NAMESPACE_EXPORT_DECL,
	TS_DECORATOR,
	DEFAULT_CASE,
	TS_INFER,
	NULL,
	UNDEFINED,
	PROP_NAME,
	STMT,
	DECL,
	PATTERN,
	TS_ENTITY_NAME,
	BOOLEAN,
	BIG_INT_VALUE,
	JS_UNKNOWN_EXPRESSION,
	JS_UNKNOWN_STATEMENT,
	JS_UNKNOWN_PATTERN,
	JS_UNKNOWN_MEMBER,
	JS_UNKNOWN_BINDING,
	JS_UNKNOWN_ASSIGNMENT_TARGET,
	#[doc(hidden)]
	__LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
	pub fn is_keyword(self) -> bool {
		match self {
			AWAIT_KW | BREAK_KW | CASE_KW | CATCH_KW | CLASS_KW | CONST_KW | CONTINUE_KW
			| DEBUGGER_KW | DEFAULT_KW | DELETE_KW | DO_KW | ELSE_KW | ENUM_KW | EXPORT_KW
			| EXTENDS_KW | FALSE_KW | FINALLY_KW | FOR_KW | FUNCTION_KW | IF_KW | IN_KW
			| INSTANCEOF_KW | INTERFACE_KW | IMPORT_KW | IMPLEMENTS_KW | NEW_KW | NULL_KW
			| PACKAGE_KW | PRIVATE_KW | PROTECTED_KW | PUBLIC_KW | RETURN_KW | SUPER_KW
			| SWITCH_KW | THIS_KW | THROW_KW | TRY_KW | TRUE_KW | TYPEOF_KW | VAR_KW | VOID_KW
			| WHILE_KW | WITH_KW | YIELD_KW | READONLY_KW | KEYOF_KW | UNIQUE_KW | DECLARE_KW
			| ABSTRACT_KW | STATIC_KW | ASYNC_KW | TYPE_KW | FROM_KW | AS_KW | REQUIRE_KW
			| NAMESPACE_KW | ASSERT_KW | MODULE_KW | GLOBAL_KW | INFER_KW | GET_KW | SET_KW
			| OF_KW | TARGET_KW | NEVER_KW | UNKNOWN_KW | ANY_KW | UNDEFINED_KW | LET_KW
			| FLOAT_KW | NUMBER_KW => true,
			_ => false,
		}
	}
	pub fn is_punct(self) -> bool {
		match self {
			SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
			| L_ANGLE | R_ANGLE | TILDE | QUESTION | QUESTION2 | QUESTIONDOT | AMP | PIPE
			| PLUS | PLUS2 | STAR | STAR2 | SLASH | CARET | PERCENT | DOT | DOT2 | COLON | EQ
			| EQ2 | EQ3 | FAT_ARROW | BANG | NEQ | NEQ2 | MINUS | MINUS2 | LTEQ | GTEQ | PLUSEQ
			| MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
			| SHL | SHR | USHR | SHLEQ | SHREQ | USHREQ | AMP2EQ | PIPE2EQ | STAR2EQ
			| QUESTION2EQ | AT | BACKTICK => true,
			_ => false,
		}
	}
	pub fn is_literal(self) -> bool {
		match self {
			JS_NUMBER_LITERAL_TOKEN
			| JS_BIG_INT_LITERAL_TOKEN
			| JS_STRING_LITERAL_TOKEN
			| JS_REGEX_LITERAL_TOKEN => true,
			_ => false,
		}
	}
	pub fn is_before_expr(self) -> bool {
		match self {
			BANG | L_PAREN | L_BRACK | L_CURLY | SEMICOLON | COMMA | COLON | QUESTION | PLUS2
			| MINUS2 | TILDE | CASE_KW | DEFAULT_KW | DO_KW | ELSE_KW | RETURN_KW | THROW_KW
			| NEW_KW | EXTENDS_KW | YIELD_KW | IN_KW | TYPEOF_KW | VOID_KW | DELETE_KW | PLUSEQ
			| MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AMP2 | PIPE2
			| SHLEQ | SHREQ | USHREQ | EQ | FAT_ARROW | MINUS | PLUS => true,
			_ => false,
		}
	}
	pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
		let kw = match ident {
			"await" => AWAIT_KW,
			"break" => BREAK_KW,
			"case" => CASE_KW,
			"catch" => CATCH_KW,
			"class" => CLASS_KW,
			"const" => CONST_KW,
			"continue" => CONTINUE_KW,
			"debugger" => DEBUGGER_KW,
			"default" => DEFAULT_KW,
			"delete" => DELETE_KW,
			"do" => DO_KW,
			"else" => ELSE_KW,
			"enum" => ENUM_KW,
			"export" => EXPORT_KW,
			"extends" => EXTENDS_KW,
			"false" => FALSE_KW,
			"finally" => FINALLY_KW,
			"for" => FOR_KW,
			"function" => FUNCTION_KW,
			"if" => IF_KW,
			"in" => IN_KW,
			"instanceof" => INSTANCEOF_KW,
			"interface" => INTERFACE_KW,
			"import" => IMPORT_KW,
			"implements" => IMPLEMENTS_KW,
			"new" => NEW_KW,
			"null" => NULL_KW,
			"package" => PACKAGE_KW,
			"private" => PRIVATE_KW,
			"protected" => PROTECTED_KW,
			"public" => PUBLIC_KW,
			"return" => RETURN_KW,
			"super" => SUPER_KW,
			"switch" => SWITCH_KW,
			"this" => THIS_KW,
			"throw" => THROW_KW,
			"try" => TRY_KW,
			"true" => TRUE_KW,
			"typeof" => TYPEOF_KW,
			"var" => VAR_KW,
			"void" => VOID_KW,
			"while" => WHILE_KW,
			"with" => WITH_KW,
			"yield" => YIELD_KW,
			"readonly" => READONLY_KW,
			"keyof" => KEYOF_KW,
			"unique" => UNIQUE_KW,
			"declare" => DECLARE_KW,
			"abstract" => ABSTRACT_KW,
			"static" => STATIC_KW,
			"async" => ASYNC_KW,
			"type" => TYPE_KW,
			"from" => FROM_KW,
			"as" => AS_KW,
			"require" => REQUIRE_KW,
			"namespace" => NAMESPACE_KW,
			"assert" => ASSERT_KW,
			"module" => MODULE_KW,
			"global" => GLOBAL_KW,
			"infer" => INFER_KW,
			"get" => GET_KW,
			"set" => SET_KW,
			"of" => OF_KW,
			"target" => TARGET_KW,
			"never" => NEVER_KW,
			"unknown" => UNKNOWN_KW,
			"any" => ANY_KW,
			"undefined" => UNDEFINED_KW,
			"let" => LET_KW,
			"float" => FLOAT_KW,
			"number" => NUMBER_KW,
			_ => return None,
		};
		Some(kw)
	}
	pub fn from_char(c: char) -> Option<SyntaxKind> {
		let tok = match c {
			';' => SEMICOLON,
			',' => COMMA,
			'(' => L_PAREN,
			')' => R_PAREN,
			'{' => L_CURLY,
			'}' => R_CURLY,
			'[' => L_BRACK,
			']' => R_BRACK,
			'<' => L_ANGLE,
			'>' => R_ANGLE,
			'~' => TILDE,
			'?' => QUESTION,
			'&' => AMP,
			'|' => PIPE,
			'+' => PLUS,
			'*' => STAR,
			'/' => SLASH,
			'^' => CARET,
			'%' => PERCENT,
			'.' => DOT,
			':' => COLON,
			'=' => EQ,
			'!' => BANG,
			'-' => MINUS,
			'@' => AT,
			'`' => BACKTICK,
			_ => return None,
		};
		Some(tok)
	}
	pub fn to_string(&self) -> Option<&str> {
		let tok = match self {
			SEMICOLON => ";",
			COMMA => ",",
			L_PAREN => "'('",
			R_PAREN => "')'",
			L_CURLY => "'{'",
			R_CURLY => "'}'",
			L_BRACK => "'['",
			R_BRACK => "']'",
			L_ANGLE => "<",
			R_ANGLE => ">",
			TILDE => "~",
			QUESTION => "?",
			QUESTION2 => "??",
			QUESTIONDOT => "?.",
			AMP => "&",
			PIPE => "|",
			PLUS => "+",
			PLUS2 => "++",
			STAR => "*",
			STAR2 => "**",
			SLASH => "/",
			CARET => "^",
			PERCENT => "%",
			DOT => ".",
			DOT2 => "...",
			COLON => ":",
			EQ => "=",
			EQ2 => "==",
			EQ3 => "===",
			FAT_ARROW => "=>",
			BANG => "!",
			NEQ => "!=",
			NEQ2 => "!==",
			MINUS => "-",
			MINUS2 => "--",
			LTEQ => "<=",
			GTEQ => ">=",
			PLUSEQ => "+=",
			MINUSEQ => "-=",
			PIPEEQ => "|=",
			AMPEQ => "&=",
			CARETEQ => "^=",
			SLASHEQ => "/=",
			STAREQ => "*=",
			PERCENTEQ => "%=",
			AMP2 => "&&",
			PIPE2 => "||",
			SHL => "<<",
			SHR => ">>",
			USHR => ">>>",
			SHLEQ => "<<=",
			SHREQ => ">>=",
			USHREQ => ">>>=",
			AMP2EQ => "&&=",
			PIPE2EQ => "||=",
			STAR2EQ => "**=",
			QUESTION2EQ => "??=",
			AT => "@",
			BACKTICK => "'`'",
			_ => return None,
		};
		Some(tok)
	}
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: SyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: SyntaxKind :: R_CURLY } ; ['['] => { $ crate :: SyntaxKind :: L_BRACK } ; [']'] => { $ crate :: SyntaxKind :: R_BRACK } ; [<] => { $ crate :: SyntaxKind :: L_ANGLE } ; [>] => { $ crate :: SyntaxKind :: R_ANGLE } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [?] => { $ crate :: SyntaxKind :: QUESTION } ; [??] => { $ crate :: SyntaxKind :: QUESTION2 } ; [?.] => { $ crate :: SyntaxKind :: QUESTIONDOT } ; [&] => { $ crate :: SyntaxKind :: AMP } ; [|] => { $ crate :: SyntaxKind :: PIPE } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [++] => { $ crate :: SyntaxKind :: PLUS2 } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [**] => { $ crate :: SyntaxKind :: STAR2 } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [^] => { $ crate :: SyntaxKind :: CARET } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [...] => { $ crate :: SyntaxKind :: DOT2 } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [==] => { $ crate :: SyntaxKind :: EQ2 } ; [===] => { $ crate :: SyntaxKind :: EQ3 } ; [=>] => { $ crate :: SyntaxKind :: FAT_ARROW } ; [!] => { $ crate :: SyntaxKind :: BANG } ; [!=] => { $ crate :: SyntaxKind :: NEQ } ; [!==] => { $ crate :: SyntaxKind :: NEQ2 } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [--] => { $ crate :: SyntaxKind :: MINUS2 } ; [<=] => { $ crate :: SyntaxKind :: LTEQ } ; [>=] => { $ crate :: SyntaxKind :: GTEQ } ; [+=] => { $ crate :: SyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: SyntaxKind :: MINUSEQ } ; [|=] => { $ crate :: SyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: SyntaxKind :: AMPEQ } ; [^=] => { $ crate :: SyntaxKind :: CARETEQ } ; [/=] => { $ crate :: SyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: SyntaxKind :: STAREQ } ; [%=] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [&&] => { $ crate :: SyntaxKind :: AMP2 } ; [||] => { $ crate :: SyntaxKind :: PIPE2 } ; [<<] => { $ crate :: SyntaxKind :: SHL } ; [>>] => { $ crate :: SyntaxKind :: SHR } ; [>>>] => { $ crate :: SyntaxKind :: USHR } ; [<<=] => { $ crate :: SyntaxKind :: SHLEQ } ; [>>=] => { $ crate :: SyntaxKind :: SHREQ } ; [>>>=] => { $ crate :: SyntaxKind :: USHREQ } ; [&&=] => { $ crate :: SyntaxKind :: AMP2EQ } ; [||=] => { $ crate :: SyntaxKind :: PIPE2EQ } ; [**=] => { $ crate :: SyntaxKind :: STAR2EQ } ; [??=] => { $ crate :: SyntaxKind :: QUESTION2EQ } ; [@] => { $ crate :: SyntaxKind :: AT } ; ['`'] => { $ crate :: SyntaxKind :: BACKTICK } ; [await] => { $ crate :: SyntaxKind :: AWAIT_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [case] => { $ crate :: SyntaxKind :: CASE_KW } ; [catch] => { $ crate :: SyntaxKind :: CATCH_KW } ; [class] => { $ crate :: SyntaxKind :: CLASS_KW } ; [const] => { $ crate :: SyntaxKind :: CONST_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [debugger] => { $ crate :: SyntaxKind :: DEBUGGER_KW } ; [default] => { $ crate :: SyntaxKind :: DEFAULT_KW } ; [delete] => { $ crate :: SyntaxKind :: DELETE_KW } ; [do] => { $ crate :: SyntaxKind :: DO_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [enum] => { $ crate :: SyntaxKind :: ENUM_KW } ; [export] => { $ crate :: SyntaxKind :: EXPORT_KW } ; [extends] => { $ crate :: SyntaxKind :: EXTENDS_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [finally] => { $ crate :: SyntaxKind :: FINALLY_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [function] => { $ crate :: SyntaxKind :: FUNCTION_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [instanceof] => { $ crate :: SyntaxKind :: INSTANCEOF_KW } ; [interface] => { $ crate :: SyntaxKind :: INTERFACE_KW } ; [import] => { $ crate :: SyntaxKind :: IMPORT_KW } ; [implements] => { $ crate :: SyntaxKind :: IMPLEMENTS_KW } ; [new] => { $ crate :: SyntaxKind :: NEW_KW } ; [null] => { $ crate :: SyntaxKind :: NULL_KW } ; [package] => { $ crate :: SyntaxKind :: PACKAGE_KW } ; [private] => { $ crate :: SyntaxKind :: PRIVATE_KW } ; [protected] => { $ crate :: SyntaxKind :: PROTECTED_KW } ; [public] => { $ crate :: SyntaxKind :: PUBLIC_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [super] => { $ crate :: SyntaxKind :: SUPER_KW } ; [switch] => { $ crate :: SyntaxKind :: SWITCH_KW } ; [this] => { $ crate :: SyntaxKind :: THIS_KW } ; [throw] => { $ crate :: SyntaxKind :: THROW_KW } ; [try] => { $ crate :: SyntaxKind :: TRY_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [typeof] => { $ crate :: SyntaxKind :: TYPEOF_KW } ; [var] => { $ crate :: SyntaxKind :: VAR_KW } ; [void] => { $ crate :: SyntaxKind :: VOID_KW } ; [while] => { $ crate :: SyntaxKind :: WHILE_KW } ; [with] => { $ crate :: SyntaxKind :: WITH_KW } ; [yield] => { $ crate :: SyntaxKind :: YIELD_KW } ; [readonly] => { $ crate :: SyntaxKind :: READONLY_KW } ; [keyof] => { $ crate :: SyntaxKind :: KEYOF_KW } ; [unique] => { $ crate :: SyntaxKind :: UNIQUE_KW } ; [declare] => { $ crate :: SyntaxKind :: DECLARE_KW } ; [abstract] => { $ crate :: SyntaxKind :: ABSTRACT_KW } ; [static] => { $ crate :: SyntaxKind :: STATIC_KW } ; [async] => { $ crate :: SyntaxKind :: ASYNC_KW } ; [type] => { $ crate :: SyntaxKind :: TYPE_KW } ; [from] => { $ crate :: SyntaxKind :: FROM_KW } ; [as] => { $ crate :: SyntaxKind :: AS_KW } ; [require] => { $ crate :: SyntaxKind :: REQUIRE_KW } ; [namespace] => { $ crate :: SyntaxKind :: NAMESPACE_KW } ; [assert] => { $ crate :: SyntaxKind :: ASSERT_KW } ; [module] => { $ crate :: SyntaxKind :: MODULE_KW } ; [global] => { $ crate :: SyntaxKind :: GLOBAL_KW } ; [infer] => { $ crate :: SyntaxKind :: INFER_KW } ; [get] => { $ crate :: SyntaxKind :: GET_KW } ; [set] => { $ crate :: SyntaxKind :: SET_KW } ; [of] => { $ crate :: SyntaxKind :: OF_KW } ; [target] => { $ crate :: SyntaxKind :: TARGET_KW } ; [never] => { $ crate :: SyntaxKind :: NEVER_KW } ; [unknown] => { $ crate :: SyntaxKind :: UNKNOWN_KW } ; [any] => { $ crate :: SyntaxKind :: ANY_KW } ; [undefined] => { $ crate :: SyntaxKind :: UNDEFINED_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [float] => { $ crate :: SyntaxKind :: FLOAT_KW } ; [number] => { $ crate :: SyntaxKind :: NUMBER_KW } ; [ident] => { $ crate :: SyntaxKind :: IDENT } ; [js_shebang] => { $ crate :: SyntaxKind :: JS_SHEBANG } ; [js_string_literal_token] => { $ crate :: SyntaxKind :: JS_STRING_LITERAL_TOKEN } ; [js_number_literal_token] => { $ crate :: SyntaxKind :: JS_NUMBER_LITERAL_TOKEN } ; [js_big_int_literal_token] => { $ crate :: SyntaxKind :: JS_BIG_INT_LITERAL_TOKEN } ; [js_regex_literal_token] => { $ crate :: SyntaxKind :: JS_REGEX_LITERAL_TOKEN } ; [#] => { $ crate :: SyntaxKind :: HASH } ; }
