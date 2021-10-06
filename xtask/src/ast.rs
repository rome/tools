//! Definitions for the ECMAScript AST used for codegen
//! Based on the rust analyzer parser and ast definitions

pub(crate) struct KindsSrc<'a> {
	pub(crate) punct: &'a [(&'a str, &'a str)],
	pub(crate) keywords: &'a [&'a str],
	pub(crate) literals: &'a [&'a str],
	pub(crate) tokens: &'a [&'a str],
	pub(crate) nodes: &'a [&'a str],
}

pub(crate) const KINDS_SRC: KindsSrc = KindsSrc {
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
	],
	literals: &["NUMBER", "STRING", "REGEX"],
	tokens: &[
		"HASH", // #
		"TEMPLATE_CHUNK",
		"DOLLARCURLY", // ${
		"BACKTICK",
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
		"TS_INFER",
	],
};

pub(crate) struct AstSrc<'a> {
	pub(crate) tokens: &'a [&'a str],
	pub(crate) nodes: &'a [AstNodeSrc<'a>],
	pub(crate) enums: &'a [AstEnumSrc<'a>],
}

pub(crate) struct AstNodeSrc<'a> {
	pub(crate) name: &'a str,
	pub(crate) fields: &'a [Field<'a>],
	pub(crate) doc: &'a str,
}

pub(crate) enum Field<'a> {
	Token(&'a str),
	Node { name: &'a str, src: FieldSrc<'a> },
}

pub(crate) enum FieldSrc<'a> {
	Shorthand,
	Optional(&'a str),
	Many(&'a str),
}

pub(crate) struct AstEnumSrc<'a> {
	pub(crate) name: &'a str,
	pub(crate) variants: &'a [&'a str],
	pub(crate) doc: &'a str,
}

macro_rules! ast_nodes {
    ($(
        $(#[doc = $doc:literal])*
        struct $name:ident {
            $($field_name:ident $(![$($token:tt)*])? $(: $ty:tt)?),*$(,)?
        }
    )*) => {
        [$(
            AstNodeSrc {
                name: stringify!($name),
                fields: &[
                    $(field!($(T![$($token)*])? $field_name $($ty)?)),*
                ],
                doc: concat!($($doc, "\n"),*)
            }
        ),*]
    };
}

macro_rules! field {
    (T![$($token:tt)*] T) => {
        Field::Token(stringify!($($token)*))
    };
    ($field_name:ident) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Shorthand }
    };
    ($field_name:ident [$ty:ident]) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Many(stringify!($ty)) }
    };
    ($field_name:ident $ty:ident) => {
        Field::Node { name: stringify!($field_name), src: FieldSrc::Optional(stringify!($ty)) }
    };
}

macro_rules! ast_enums {
    ($(
        $(#[doc = $doc:literal])*
        enum $name:ident {
            $($variant:ident),*$(,)?
        }
    )*) => {
        [$(
            AstEnumSrc {
                name: stringify!($name),
                variants: &[$(stringify!($variant)),*],
                doc: concat!($($doc, "\n"),*)
            }
        ),*]
    };
}

/// Data used by codegen for generating ast nodes and SyntaxKind enums.  
/// Comments represent definitions which are manually created since they are either unique enough
/// or special enough to generate definitions for manually.
pub(crate) const AST_SRC: AstSrc = AstSrc {
	tokens: &["Whitespace", "Comment", "String"],
	nodes: &ast_nodes! {
		// TODO: move this down once ts is done -------------

		/// A type signifying any type (Any)
		struct TsAny { T![ident] }
		/// A type signifying an unknown type (Unknown)
		struct TsUnknown { T![ident] }
		/// A type signifying a number (number)
		struct TsNumber { T![ident] }
		/// A type signifying a JavaScript Object, any non-primitive (object)
		struct TsObject { T![ident] }
		/// A type signifying a boolean (boolean)
		struct TsBoolean { T![ident] }
		/// A type signifying a JavaScript bigint (big integer) (bigint)
		struct TsBigint { T![ident] }
		/// A type signifying a JavaScript string (string)
		struct TsString { T![ident] }
		/// A type signifying a JavaScript Symbol (symbol)
		struct TsSymbol { T![ident] }
		/// A type signifying no type (void)
		struct TsVoid { T![ident] }
		/// A type signifying a JavaScript undefined value (undefined)
		struct TsUndefined { T![ident] }
		/// A type signifying a JavaScript null value (null)
		struct TsNull { T![ident] }
		/// A type signifying a function never returns (never)
		struct TsNever { T![ident] }

		/// A type signifying JavaScript's `this`
		struct TsThis { T![this] }
		/// A type represented by a literal value
		struct TsLiteral {
			/* - for numbers */
			lit: Literal
		}

		struct TsTemplate {
			/* backtick */
			elements: [TsTemplateElement],
			/* chunks */
			/* backtick */
		}

		struct TsTemplateElement {
			/* dollarcurly */
			ty: TsType,
			T!['}']
		}

		/// A type guard which performs a runtime check to guarantee the type of something in a scope
		///
		/// ```ts
		/// function isFish(pet: Fish | Bird): pet is Fish {
		///    return (pet as Fish).swim !== undefined;
		/// }
		/// ```
		///
		/// It could also be an assertion function:
		///
		/// ```ts
		/// function check(cond: any): asserts condition { /* */ }
		/// ```
		///
		/// <https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions>
		/// <https://www.typescriptlang.org/docs/handbook/advanced-types.html#user-defined-type-guards>
		struct TsPredicate {
			/* asserts */
			lhs: TsThisOrName,
			/* is */
			rhs: TsType
		}

		/// A type with a fixed number of elements with known types
		///
		/// ```ts
		/// let x: [number, ...string[]];
		/// let y: [foo: number, ...bar: string[]];
		/// ```
		///
		/// <https://www.typescriptlang.org/docs/handbook/basic-types.html#tuple>
		struct TsTuple {
			T!['['],
			elements: TsTupleElement,
			T![']']
		}

		/// An individual tuple element, this could be a rest element and could be named:
		/// e.g. `number`, `foo: number`, `...number`, or `...foo: number`
		struct TsTupleElement {
			T![...],
			T![ident],
			T![?],
			T![:],
			ty: TsType
		}

		/// A parenthesized type
		///
		/// ```ts
		/// let x: (Foo);
		/// ```
		struct TsParen {
			T!['('],
			ty: TsType,
			T![')']
		}

		/// A reference to a type which may or may not have type arguments. e.g. `Foo`, `Foo<Bar>`, `Foo<Bar, Baz>`
		struct TsTypeRef {
			name: TsEntityName,
			type_args: TsTypeArgs
		}

		/// A full path to a type from a namespace. e.g. `foo.bar` or `foo.bar.baz`
		struct TsQualifiedPath {
			lhs: TsEntityName,
			T![.],
			rhs: TsTypeName
		}

		struct TsTypeName {
			T![ident]
		}

		/// A type which allows the creation of new types from existing ones
		/// by mapping over property types.
		///
		/// ```ts
		/// type Readonly<T> = {
		///   readonly [P in keyof T]: T[P];
		/// }
		/// ```
		///
		/// <https://www.typescriptlang.org/docs/handbook/advanced-types.html#mapped-types>
		// TODO: this is kind of ugly, especially the : then + then - then ?
		struct TsMappedType {
			T!['{'],
			readonly_modifier: TsMappedTypeReadonly,
			param: TsMappedTypeParam,
			T![:],
			T![+],
			T![-],
			T![?],
			ty: TsType,
			T![;],
			T!['}']
		}

		struct TsMappedTypeParam {
			T!['['],
			name: TsTypeName,
			T![ident],
			ty: TsType,
			/* as */
			/* alias */
			T![']']
		}

		/// An optional readonly modifier applied to mapped types
		struct TsMappedTypeReadonly {
			T![+],
			T![-],
			T![readonly]
		}

		struct TsTypeQuery {
			T![typeof],
			expr: TsTypeQueryExpr
		}

		struct TsImport {
			T![import],
			T!['('],
			/* arg */
			T![')'],
			T![.],
			qualifier: TsEntityName,
			type_args: TsTypeArgs
		}

		struct TsTypeArgs {
			T![<],
			args: [TsType],
			T![>]
		}

		struct TsArray {
			ty: TsType,
			T!['['],
			T![']']
		}

		struct TsIndexedArray {
			ty: TsType,
			T!['['],
			/* index: TsType */
			T![']']
		}

		struct TsTypeOperator {
			/* operator */
			ty: TsType
		}

		struct TsIntersection {
			types: [TsType]
		}

		struct TsUnion {
			types: [TsType]
		}

		struct TsTypeParams {
			T![<],
			params: [TsTypeParam],
			T![>]
		}

		struct TsFnType {
			params: ParameterList,
			T![=>],
			return_type: TsType
		}

		struct TsConstructorType {
			T![new],
			params: ParameterList,
			T![=>],
			return_type: TsType
		}

		struct TsExtends {
			T![extends],
			ty: TsType
		}

		struct TsConditionalType {
			ty: TsType,
			extends: TsExtends,
			T![?],
			/* cons */
			T![:],
			/* alt */
		}

		struct TsConstraint {
			T![extends],
			ty: TsType
		}

		struct TsTypeParam {
			T![ident],
			constraint: TsConstraint,
			default: TsDefault
		}

		struct TsDefault {
			T![=],
			ty: TsType
		}

		struct TsNonNull {
			// FIXME: Should this allow super! too?
			target: Expr,
			T![!]
		}

		struct TsAssertion {
			expr: Expr,
			T![ident],
			T![<],
			ty: TsType,
			T![>]
		}

		struct TsTypeAliasDecl {
			/* type */
			T![ident],
			type_params: TsTypeParams,
			T![=],
			ty: TsType
		}

		/// A TypeScript const assertion either as `foo as const` or `<const>foo`
		struct TsConstAssertion {
			expr: Expr,
			T![ident],
			T![<],
			T![const],
			T![>]
		}

		struct TsEnum {
			T![ident],
			T![const],
			T![enum],
			T!['{'],
			members: [TsEnumMember],
			T!['}']
		}

		struct TsEnumMember {
			T![ident],
			T![=],
			value: Expr
		}

		struct TsNamespaceDecl {
			T![declare],
			/* namespace */
			T![.],
			T![ident],
			body: TsNamespaceBody
		}

		struct TsModuleBlock {
			T!['{'],
			items: [ModuleItem],
			T!['}']
		}

		struct TsModuleDecl {
			T![declare],
			T![global],
			T![module],
			T![.],
			T![ident],
			body: TsNamespaceBody
		}

		struct TsConstructorParam {
			/* accessibility */
			T![readonly],
			pat: Pattern
		}

		struct TsCallSignatureDecl {
			type_params: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType,
		}

		struct TsConstructSignatureDecl {
			T![new],
			type_params: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType,
		}

		struct TsIndexSignature {
			decorators: [TsDecorator],
			T![readonly],
			T!['['],
			pat: SinglePattern,
			T![']'],
			T![:],
			ty: TsType
		}

		struct TsMethodSignature {
			T![readonly],
			key: Expr,
			T![?],
			type_params: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType
		}

		struct TsPropertySignature {
			T![readonly],
			prop: Expr,
			T![?],
			T![:],
			ty: TsType,
		}

		struct TsExprWithTypeArgs {
			item: TsEntityName,
			type_params: TsTypeArgs
		}

		struct TsInterfaceDecl {
			T![declare],
			/* interface */
			T![ident],
			type_params: TsTypeParams,
			T![extends],
			extends: [TsExprWithTypeArgs],
			T!['{'],
			members: [TsTypeElement],
			T!['}']
		}

		struct TsObjectType {
			T!['{'],
			members: [TsTypeElement],
			T!['}']
		}

		struct TsImportEqualsDecl {
			T![import],
			T![export],
			T![ident],
			T![=],
			module: TsModuleRef,
			T![;]
		}

		struct TsExternalModuleRef {
			T![require],
			T!['('],
			/* string */
			T![')']
		}

		struct TsExportAssignment {
			T![export],
			T![=],
			expr: Expr,
			T![;]
		}

		struct TsNamespaceExportDecl {
			T![export],
			T![as],
			T![namespace],
			T![ident],
			T![;]
		}

		struct TsDecorator {
			T![@],
			expr: Expr,
		}

		struct TsInfer {
			T![infer],
			T![ident]
		}

		// --------------------------------------------------
		struct Script {
			T![shebang],
			items: [Stmt],
		}

		struct Module {
			T![shebang],
			items: [ModuleItem],
		}

		struct ImportDecl {
			T![import],
			T![type],
			imports: [ImportClause],
			T![from],
			/* source */
			T![assert],
			asserted_object: ObjectExpr,
			T![;]
		}

		struct ImportStringSpecifier { /* string */ }

		struct WildcardImport {
			T![*],
			T![as],
			T![ident]
		}

		struct NamedImports {
			T!['{'],
			specifiers: [Specifier],
			T!['}']
		}

		struct Specifier {
			/* manual impl */
		}

		struct ExportDecl {
			decorators: [TsDecorator],
			T![export],
			T![type],
			decl: Decl
		}

		struct ExportNamed {
			T![export],
			T![type],
			T!['{'],
			specifiers: [Specifier],
			T!['}'],
			T![from],
			/* source */
		}

		struct ExportWildcard {
			T![export],
			T![type],
			T![*],
			T![as],
			T![ident],
			T![from],
			/* source */
		}

		struct ExportDefaultDecl {
			decorators: [TsDecorator],
			T![export],
			T![type],
			T![default],
			decl: DefaultDecl
		}

		struct ExportDefaultExpr {
			T![export],
			T![type],
			T![default],
			expr: Expr
		}

		struct Literal { /*LiteralToken*/ }

		struct BlockStmt {
			T!['{'],
			stmts: [Stmt],
			T!['}'],
		}

		struct VarDecl {
			T![var],
			/* let */
			T![const],
			declared: [Declarator],
			T![;],
		}

		struct Declarator {
			pattern: Pattern,
			T![!], // ts (definite)
			T![=],
			value: Expr,
		}

		struct EmptyStmt {
			T![;],
		}

		struct ExprStmt {
			expr: Expr,
		}

		struct IfStmt {
			T![if],
			condition: Condition,
			/* cons */
			T![else],
			/* alt */
		}

		struct Condition {
			T!['('],
			condition: Expr,
			T![')'],
		}

		struct DoWhileStmt {
			T![do],
			cons: Stmt,
			T![while],
			condition: Condition,
			T![;],
		}

		struct WhileStmt {
			T![while],
			condition: Condition,
			cons: Stmt,
		}

		struct ForStmt {
			T![for],
			T!['('],
			init: ForStmtInit,
			/* semicolon */
			test: ForStmtTest,
			/* semicolon */
			update: ForStmtUpdate,
			T![')'],
			cons: Stmt,
		}

		struct ForStmtInit {
			inner: ForHead
		}

		struct ForStmtTest {
			expr: Expr
		}

		struct ForStmtUpdate {
			expr: Expr
		}

		struct ForInStmt {
			T![for],
			T!['('],
			left: ForStmtInit,
			T![in],
			right: Expr,
			T![')'],
			cons: Stmt,
		}

		struct ForOfStmt {
			T![for],
			T![await],
			T!['('],
			left: ForStmtInit,
			/* of */
			right: Expr,
			T![')'],
			cons: Stmt
		}

		struct ContinueStmt {
			T![continue],
			T![ident],
			T![;],
		}

		struct BreakStmt {
			T![break],
			T![ident],
			T![;],
		}

		struct ReturnStmt {
			T![return],
			value: Expr,
			T![;],
		}

		struct WithStmt {
			T![with],
			condition: Condition,
			cons: Stmt,
		}

		struct SwitchStmt {
			T![switch],
			test: Condition,
			T!['{'],
			cases: [SwitchCase],
			T!['}'],
		}

		struct CaseClause {
			T![case],
			test: Expr,
			T![:],
			cons: [Stmt],
		}

		struct DefaultClause {
			T![default],
			T![:],
			cons: [Stmt]
		}

		struct LabelledStmt {
			label: Name,
			T![:],
			stmt: Stmt,
		}

		struct ThrowStmt {
			T![throw],
			exception: Expr,
			T![;],
		}

		struct TryStmt {
			T![try],
			test: BlockStmt,
			handler: CatchClause,
			finalizer: Finalizer,
		}

		struct CatchClause {
			T![catch],
			T!['('],
			error: Pattern,
			T![')'],
			cons: BlockStmt
		}

		struct Finalizer {
			T![finally],
			cons: BlockStmt
		}

		struct DebuggerStmt {
			T![debugger],
			T![;],
		}

		struct FnDecl {
			decorators: [TsDecorator],
			T![ident],
			T![async],
			T![function],
			T![*],
			name: Name,
			type_parameters: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType,
			body: BlockStmt,
		}

		struct Name { T![ident] }

		struct NameRef { T![ident] }

		struct ParameterList {
			T!['('],
			parameters: [Pattern],
			T![')'],
		}

		struct ThisExpr {
			T![this],
		}

		struct ArrayExpr {
			T!['['],
			elements: [ExprOrSpread],
			T![']'],
		}

		struct ObjectExpr {
			T!['{'],
			props: [ObjectProp],
			T!['}'],
		}

		struct SpreadProp {
			T![...],
			value: Expr,
		}

		struct InitializedProp {
			key: Name,
			T![=],
			value: Expr,
		}

		struct IdentProp {
			name: Name
		}

		struct LiteralProp {
			/* key */
			T![:]
			/* value */
		}

		struct Getter {
			decorators: [TsDecorator],
			T![get],
			key: PropName,
			parameters: ParameterList,
			body: BlockStmt,
		}

		struct Setter {
			decorators: [TsDecorator],
			T![set],
			key: PropName,
			parameters: ParameterList,
			body: BlockStmt,
		}

		struct GroupingExpr {
			T!['('],
			inner: Expr,
			T![')'],
		}

		struct BracketExpr {
			T![super],
			/* object */
			/* optional chain */
			T!['['],
			/* prop */
			T![']'],
		}

		struct DotExpr {
			T![super],
			object: Expr,
			/* optional chain */
			T![.],
			prop: Name,
		}

		struct NewExpr {
			T![new],
			type_args: TsTypeArgs,
			object: Expr,
			arguments: ArgList,
		}

		struct ArgList {
			T!['('],
			// TODO: Change this to expr or spread
			args: [Expr],
			T![')'],
		}

		struct CallExpr {
			callee: Expr,
			type_args: TsTypeArgs,
			/* optional chain */
			arguments: ArgList,
		}

		struct SuperCall {
			T![super],
			arguments: ArgList
		}

		struct ImportCall {
			T![import],
			// This only takes one arg, it doesnt take an ArgList
			T!['('],
			argument: Expr,
			T![')']
		}

		struct NewTarget {
			T![new],
			T![.],
			/* target */
		}

		struct ImportMeta {
			T![import],
			T![.],
			/* meta */
		}

		struct UnaryExpr {
			/* Prefix op */
			Expr,
		}

		struct BinExpr {
			/* Binop */
		}

		struct CondExpr {
			/* test */
			T![?],
			/* cons */
			T![:],
			/* alt */
		}

		struct AssignExpr {
			/* lhs: PatternOrExpr, */
			/* AssignOp */
			/* rhs: Expr */
		}

		struct SequenceExpr {
			exprs: [Expr],
		}

		struct Template {
			tag: Expr,
			/* backtick */
			/* chunks */
			elements: [TemplateElement],
			/* backtick */
		}

		struct TemplateElement {
			/* dollarcurly */
			expr: Expr,
			T!['}']
		}

		struct SpreadElement {
			T![...],
			element: Expr
		}

		struct ArrayPattern {
			decorators: [TsDecorator],
			T!['['],
			elements: [Pattern],
			T![']'],
			T![!],
			T![:],
			ty: TsType
		}

		struct ObjectPattern {
			decorators: [TsDecorator],
			T!['{'],
			elements: [ObjectPatternProp],
			T!['}'],
			T![!],
			T![:],
			ty: TsType
		}

		struct RestPattern {
			decorators: [TsDecorator],
			T![...],
			pat: Pattern,
			T![:],
			ty: TsType
		}

		struct AssignPattern {
			decorators: [TsDecorator],
			key: Pattern,
			T![:],
			ty: TsType,
			T![=],
			value: Expr
		}

		struct KeyValuePattern {
			key: PropName,
			T![:],
			/* pattern */
		}

		struct ComputedPropertyName {
			T!['['],
			prop: Expr,
			T![']']
		}

		struct SinglePattern {
			decorators: [TsDecorator],
			name: Name,
			T![?],
			T![!],
			T![:],
			ty: TsType
		}

		struct ArrowExpr {
			T![async],
			type_params: TsTypeParams,
			params: ArrowExprParams,
			T![:],
			return_type: TsType,
			T![=>],
			/* ExprOrBlock */
		}

		struct YieldExpr {
			T![yield],
			T![*],
			value: Expr
		}

		struct FnExpr {
			T![async],
			T![function],
			T![*],
			name: Name,
			type_params: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType,
			body: BlockStmt,
		}

		struct Method {
			decorators: [TsDecorator],
			T![static],
			T![async],
			T![*],
			name: PropName,
			type_params: TsTypeParams,
			parameters: ParameterList,
			T![:],
			return_type: TsType,
			body: BlockStmt
		}

		struct TsAccessibility { /* modifier */ }

		struct PrivateProp {
			decorators: [TsDecorator],
			T![declare],
			T![abstract],
			T![static],
			accessibility: TsAccessibility,
			key: PrivateName,
			T![?],
			T![!],
			T![:],
			ty: TsType,
			T![;]
		}

		struct ClassProp {
			decorators: [TsDecorator],
			T![declare],
			T![abstract],
			T![static],
			accessibility: TsAccessibility,
			key: PropName,
			T![?],
			T![!],
			T![:],
			ty: TsType,
			T![;]
		}

		struct Constructor {
			decorators: [TsDecorator],
			accessibility: TsAccessibility,
			name: PropName,
			type_params: TsTypeParams,
			parameters: ConstructorParameters,
			body: BlockStmt
		}

		struct ConstructorParameters {
			T!['('],
			parameters: ConstructorParamOrPat,
			T![')']
		}

		struct ClassDecl {
			decorators: [TsDecorator],
			T![abstract],
			T![class],
			name: Name,
			type_params: TsTypeParams,
			T![extends],
			parent: Expr,
			parent_type_args: TsTypeArgs,
			T![implements],
			implements: [TsExprWithTypeArgs],
			body: ClassBody
		}

		struct ClassExpr {
			T![class],
			name: Name,
			type_params: TsTypeParams,
			T![extends],
			parent: Expr,
			parent_type_args: TsTypeArgs,
			T![implements],
			implements: [TsExprWithTypeArgs],
			body: ClassBody
		}

		struct ClassBody {
			T!['{'],
			elements: [ClassElement],
			T!['}']
		}

		struct AwaitExpr {
			T![await],
			expr: Expr
		}

		struct PrivateName {
			T![#],
			name: Name
		}

		struct PrivatePropAccess {
			lhs: Expr,
			T![.],
			rhs: PrivateName
		}

		struct ExprPattern {
			expr: Expr
		}
	},
	enums: &ast_enums! {
		enum ObjectProp {
			LiteralProp,
			Getter,
			Setter,
			SpreadProp,
			InitializedProp,
			IdentProp,
			Method
		}

		enum Pattern {
			SinglePattern,
			RestPattern,
			AssignPattern,
			ObjectPattern,
			ArrayPattern,
			ExprPattern
		}

		enum SwitchCase {
			CaseClause,
			DefaultClause,
		}

		enum ObjectPatternProp {
			AssignPattern,
			KeyValuePattern,
			RestPattern,
			SinglePattern,
		}

		enum ArrowExprParams {
			Name,
			ParameterList
		}

		enum MethodDefinition {
			Method,
			Getter,
			Setter
		}

		enum ClassElement {
			EmptyStmt,
			Method,
			PrivateProp,
			ClassProp,
			Constructor,
			TsIndexSignature,
			Getter,
			Setter
		}

		enum ImportClause {
			WildcardImport,
			NamedImports,
			Name,
			ImportStringSpecifier,
		}

		enum DefaultDecl {
			FnDecl,
			ClassDecl
		}

		/*
		enum ModuleItem {
			Stmt,
			ImportDeclaration,
			ExportDeclaration
		}
		*/

		/*
		enum ExprOrSpread {
			Expr,
			SpreadElement
		}
		*/

		/*
		enum StmtListItem {
			STMT,
			DECLARATION
		}
		*/

		enum Decl {
			FnDecl,
			ClassDecl,
			VarDecl,
			TsEnum,
			TsTypeAliasDecl,
			TsNamespaceDecl,
			TsModuleDecl,
			TsInterfaceDecl
		}

		/*
		enum ForHead {
			VAR_STMT,
			EXPR
		}
		*/

		enum Expr {
			ArrowExpr,
			Literal,
			Template,
			NameRef,
			ThisExpr,
			ArrayExpr,
			ObjectExpr,
			GroupingExpr,
			BracketExpr,
			DotExpr,
			NewExpr,
			CallExpr,
			UnaryExpr,
			BinExpr,
			CondExpr,
			AssignExpr,
			SequenceExpr,
			FnExpr,
			ClassExpr,
			NewTarget,
			ImportMeta,
			SuperCall,
			ImportCall,
			YieldExpr,
			AwaitExpr,
			PrivatePropAccess,
			TsNonNull,
			TsAssertion,
			TsConstAssertion
		}

		/// Either a single type reference or a fully qualified path
		enum TsEntityName {
			TsTypeName,
			TsQualifiedPath
		}

		/// A TypeScript type
		enum TsType {
			TsAny,
			TsUnknown,
			TsNumber,
			TsObject,
			TsBoolean,
			TsBigint,
			TsString,
			TsSymbol,
			TsVoid,
			TsUndefined,
			TsNull,
			TsNever,
			TsThis,
			TsLiteral,
			TsPredicate,
			TsTuple,
			TsParen,
			TsTypeRef,
			TsTemplate,
			TsMappedType,
			TsImport,
			TsArray,
			TsIndexedArray,
			TsTypeOperator,
			TsIntersection,
			TsUnion,
			TsFnType,
			TsConstructorType,
			TsConditionalType,
			TsObjectType,
			TsInfer
		}

		enum TsThisOrName {
			TsThis,
			TsTypeName
		}

		enum TsNamespaceBody {
			TsModuleBlock,
			TsNamespaceDecl
		}

		enum TsTypeElement {
			TsCallSignatureDecl,
			TsConstructSignatureDecl,
			TsPropertySignature,
			TsMethodSignature,
			TsIndexSignature
		}
	},
};
