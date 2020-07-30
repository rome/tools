import {createDiagnosticsCategory} from "../index";
import {DiagnosticAdvice, DiagnosticLocation} from "../../types";
import {SourceLocation} from "@internal/parser-core";
import {buildDuplicateLocationAdvice} from "../../helpers";
import {markup} from "@internal/markup";

function buildJSXOpeningAdvice(
	name: string,
	openingLoc: SourceLocation,
): DiagnosticAdvice {
	return [
		{
			type: "log",
			category: "info",
			text: name === ""
				? markup`Originated from this opening tag`
				: markup`Originated from opening tag of <emphasis>${name}</emphasis>`,
		},
		{
			type: "frame",
			location: openingLoc,
		},
	];
}

// @internal/js-parser
export const jsParser = createDiagnosticsCategory({
	UNTERMINATED_BLOCK_COMMENT: {message: markup`Unterminated comment`},
	UNTERMINATED_JSX_STRING: {message: markup`Unterminated string constant`},
	INVALID_UNICODE_ESCAPE: {message: markup`Invalid Unicode escape`},
	EXPECTED_UNICODE_ESCAPE: {
		message: markup`Expecting Unicode escape sequence \\uXXXX`,
	},
	BAD_HEX_ESCAPE: {message: markup`Bad character escape sequence`},
	OCTAL_IN_STRICT_MODE: {message: markup`Octal literal in strict mode`},
	UNTERMINATED_TEMPLATE: {message: markup`Unterminated template`},
	UNTERMINATED_STRING: {message: markup`Unterminated string constant`},
	OUT_OF_BOUND_CODE_POINT: {message: markup`Code point out of bounds`},
	IDENTIFIER_AFTER_NUMBER: {message: markup`Identifier directly after number`},
	OCTAL_BIGINT: {message: markup`A bigint can't be an octal`},
	DECIMAL_BIGINT: {message: markup`A bigint can't have a decimal`},
	INVALID_NUMBER: {message: markup`Invalid number`},
	LEGACY_OCTAL_IN_STRICT_MODE: {
		message: markup`Legacy octal literals are not allowed in strict mode`,
	},
	INVALID_INT_TOKEN: {message: markup`Invalid or unexpected int token`},
	UNICODE_ESCAPE_IN_REGEX_FLAGS: {
		message: markup`Regular expression flags can't contain unicode escapes`,
	},
	UNTERMINATED_REGEX: {message: markup`Unterminated regular expression`},
	DANGLING_BACKSLASH_IN_REGEX: {
		message: markup`Dangling backslash in a regular expression`,
	},
	EXPECTED_RELATIONAL_OPERATOR: {message: markup`Expected relational operator`},
	UNEXPECTED_SPACE: {message: markup`Unexpected space`},
	EXPECTED_SEMI_OR_LINE_TERMINATOR: {
		message: markup`Expected a semicolon or a line terminator`,
	},
	GET_SET_CLASS_CONSTRUCTOR: {
		message: markup`Constructor can't have get/set modifier`,
	},
	ASYNC_CLASS_CONSTRUCTOR: {message: markup`Constructor cannot be async`},
	GENERATOR_CLASS_CONSTRUCTOR: {
		message: markup`Constructor cannot be a generator`,
	},
	DUPLICATE_CLASS_CONSTRUCTOR: {
		message: markup`Duplicate constructor in the same class`,
	},
	UNKNOWN_CLASS_PROPERTY_START: {message: markup`Unknown class property start`},
	CLASS_STATIC_PROTOTYPE_PROPERTY: {
		message: markup`Classes may not have static property named prototype`,
	},
	CLASS_PRIVATE_FIELD_NAMED_CONSTRUCTOR: {
		message: markup`Classes may not have a private field named '#constructor'`,
	},
	CLASS_PROPERTY_NAME_CONSTRUCTOR: {
		message: markup`Classes may not have a non-static field named 'constructor'`,
	},
	PROTO_PROP_REDEFINITION: {message: markup`Redefinition of __proto__ property`},
	MISSING_CONDITIONAL_SEPARATOR: {
		message: markup`Missing conditional expression consequent separator`,
	},
	WRAP_EXPONENTIATION: {
		message: markup`Illegal expression. Wrap left hand side or entire exponentiation in parentheses.`,
	},
	DELETE_LOCAL_VARIABLE_IN_STRICT: {
		message: markup`Deleting local variable in strict mode`,
	},
	DELETE_PRIVATE_FIELD: {
		message: markup`Deleting a private field is not allowed`,
	},
	TAGGED_TEMPLATE_IN_OPTIONAL_CHAIN: {
		message: markup`Tagged Template Literals are not allowed in optionalChain`,
	},
	YIELD_NAME_IN_GENERATOR: {
		message: markup`Can not use 'yield' as identifier inside a generator`,
	},
	AWAIT_NAME_IN_ASYNC: {
		message: markup`Can not use 'await' as identifier inside an async function`,
	},
	EMPTY_PARENTHESIZED_EXPRESSION: {
		message: markup`Parenthesized expression didnt contain anything`,
	},
	AWAIT_IN_ASYNC_PARAMS: {
		message: markup`await is not allowed in async function parameters`,
	},
	YIELD_IN_GENERATOR_PARAMS: {
		message: markup`yield is not allowed in generator parameters`,
	},
	FLOW_TYPE_CAST_IN_TS: {
		message: markup`Flow type cast expressions aren't allowed in TypeScript`,
	},
	PARENTHESIZED_FUNCTION_PARAMS: {
		message: markup`Function parameters can't be parenthesized`,
	},
	NEW_WITH_TYPESCRIPT_TYPE_ARGUMENTS_NO_PARENS: {
		message: markup`In TypeScript, a new expression with type arguments must have parens`,
	},
	INVALID_TEMPLATE_ESCAPE: {
		message: markup`Invalid escape sequence in template`,
	},
	EXPECTED_IDENTIFIER: {message: markup`Expected an identifier`},
	IMPORT_EXACT_ARGUMENTS: {
		message: markup`import() requires exactly one argument`,
	},
	IMPORT_TRAILING_COMMA: {
		message: markup`Trailing comma is disallowed inside import(...) arguments`,
	},
	IMPORT_SPREAD: {message: markup`Spread is not allowed in import()`},
	IMPORT_NEW_CALLEE: {message: markup`Cannot use new with import(...)`},
	SUPER_OUTSIDE_METHOD: {
		message: markup`super is only allowed in object methods and classes`,
	},
	INVALID_SUPER_SUFFIX: {message: markup`Invalid super suffix operator`},
	AWAIT_OUTSIDE_ASYNC: {
		message: markup`Can't use await outside of an async function`,
	},
	AWAIT_STAR: {
		message: markup`await* has been removed from the async functions proposal. Use Promise.all() instead.`,
	},
	NEW_TARGET_OUTSIDE_CLASS: {
		message: markup`new.target can only be used in functions or class properties`,
	},
	MULTIPLE_DESTRUCTURING_RESTS: {
		message: markup`Cannot have multiple rest elements when destructuring`,
	},
	TRAILING_COMMA_AFTER_REST: {
		message: markup`A trailing comma is not permitted after the rest element`,
	},
	GETTER_WITH_PARAMS: {message: markup`getter should have no parameters`},
	SETTER_WITH_REST: {
		message: markup`setter function argument must not be a rest parameter`,
	},
	SETTER_NOT_ONE_PARAM: {message: markup`setter should have exactly one param`},
	ASYNC_GETTER_SETTER: {message: markup`An object setter/getter can't be async`},
	GENERATOR_GETTER_SETTER: {
		message: markup`An object setter/getter can't be a generator`,
	},
	ARGUMENTS_IN_CLASS_FIELD: {
		message: markup`'arguments' is not allowed in class field initializer`,
	},
	NON_SIMPLE_PARAM_IN_EXPLICIT_STRICT_FUNCTION: {
		message: markup`Non-simple parameter in strict mode`,
	},
	STRICT_DIRECTIVE_IN_NON_SIMPLE_PARAMS: {
		message: markup`Illegal 'use strict' directive in function with non-simple parameter list`,
	},
	OBJECT_PROPERTY_WITH_TYPE_PARAMETERS: {
		message: markup`Object property cannot have type parameters`,
	},
	ILLEGAL_VARIANCE: {message: markup`Variance is not allowed here`},
	OBJECT_METHOD_IN_PATTERN: {
		message: markup`Object methods aren't allowed in object patterns`,
	},
	IMPORT_META_OUTSIDE_MODULE: {
		message: markup`import.meta may only appear in a module`,
	},
	EXPECTED_ARROW_AFTER_ASYNC_TYPE_PARAMS: {
		message: markup`Expected arrow because we are a possible async arrow and type annotated parameters were present`,
	},
	INVALID_OBJECT_PATTERN_PROP: {
		message: markup`Invalid property node for object pattern`,
	},
	ASYNC_OBJECT_METHOD_LINE_BREAK: {
		message: markup`There shouldn't be any newlines between async and the rest of the function`,
	},
	SPACE_BETWEEN_PRIVATE_HASH: {
		message: markup`Unexpected space between # and identifier`,
	},
	CONFUSING_CALL_ARGUMENT: {
		message: markup`Function parameter type annotation? Possibly forgot curlies around an object. Possibly forgot async keyword.`,
	},
	EXPECTED_ARROW_AFTER_TYPE_PARAMS: {
		message: markup`Expected an arrow function after this type parameter declaration`,
	},
	REQUIRED_CLASS_NAME: {message: markup`Class name is required`},
	JSX_ELEM_TYPE_ARGUMENTS_OUTSIDE_TS: {
		message: markup`JSX element type arguments are only allowed in TS`,
	},
	UNWRAPPED_ADJACENT_JHX: {
		message: markup`Adjacent JSX elements must be wrapped in an enclosing tag. Did you want a JSX fragment \\<>...\\</>?`,
	},
	CONFUSED_OR: {message: markup`Unexpected ||, did you mean just |?`},
	INVALID_ASSIGNMENT_TARGET: {message: markup`Not a valid assignment target`},
	IMPORT_KIND_SPECIFIER_ON_IMPORT_DECLARATION_WITH_KIND: {
		message: markup`The \`type\` and \`typeof\` keywords on named imports can only be used on regular \`import\` statements. It cannot be used with \`import type\` or \`import typeof\` statements`,
	},
	DESTRUCTURING_IN_IMPORT: {
		message: markup`ES2015 named imports do not destructure. Use another statement for destructuring after the import.`,
	},
	IMPORT_MISSING_SOURCE: {message: markup`import missing a source`},
	EXPORT_TYPE_NAMESPACE: {
		message: markup`Can't have a type export namespacer specifier`,
	},
	EXPORT_MISSING_FROM: {message: markup`Expected \`from\` for an export node`},
	EXPORT_FROM_NOT_STRING: {message: markup`Export from only allows strings`},
	BINDING_MEMBER_EXPRESSION: {message: markup`Binding member expression`},
	INVALID_OBJECT_PATTERN_PROPERTY: {
		message: markup`Not a valid assignment object pattern property`,
	},
	OBJECT_PATTERN_CANNOT_CONTAIN_METHODS: {
		message: markup`Object pattern cannot contains methods`,
	},
	INVALID_ASSIGNMENT_PATTERN_OPERATOR: {
		message: markup`Only '=' operator can be used for specifying default value.`,
	},
	INVALID_OBJECT_REST_ARGUMENT: {
		message: markup`Invalid rest operator's argument`,
	},
	INVALID_EXPORT_DEFAULT: {
		message: markup`Only expressions, functions or classes are allowed as the \`default\` export.`,
	},
	INVALID_EXPORT_DECLARATION: {message: markup`Invalid export declaration`},
	DESTRUCTURING_REST_ELEMENT_NOT_LAST: {
		message: markup`The rest element has to be the last element when destructuring`,
	},
	REST_INVALID_ARGUMENT: {message: markup`Invalid rest operator's argument`},
	EXPORT_ASYNC_NO_FUNCTION_KEYWORD: {
		message: markup`Started with \`export async\` so we expected to receive an async function but no function keyword was found`,
	},
	TYPE_CAST_WITHOUT_ANNOTATION: {
		message: markup`Type cast expression has no type annotation. Did you mean for this to be a function parameter?`,
	},
	TYPE_CAST_CANNOT_BE_OPTIONAL: {
		message: markup`Type cast expressions cannot be optional. Did you mean for this to be a function parameter?`,
	},
	TYPE_CAST_EXPECTED_PARENS: {
		message: markup`The type cast expression is expected to be wrapped with parentheses`,
	},
	INVALID_ASYNC_ARROW_WITH_TYPE_PARAMS: {
		message: markup`Invalid async arrow with type parameters`,
	},
	TYPE_NUMERIC_LITERAL_PLUS: {
		message: markup`Numeric literal type annotations cannot stand with a +, omit it instead`,
	},
	TYPE_NUMERIC_LITERAL_EXPECTED: {
		message: markup`Unexpected token, expected "number"`,
	},
	JSX_INVALID_ATTRIBUTE_VALUE: {
		message: markup`JSX attribute value should be either an expression or a quoted JSX text`,
	},
	JSX_UNCLOSED_SELF_CLOSING_TAG: {message: markup`Unclosed JSX element open`},
	JSX_UNCLOSED_CLOSING_TAG: {message: markup`Unclosed JSX element close`},
	JSX_EMPTY_ATTRIBUTE_VALUE: {
		message: markup`JSX attribute cannot be an empty expression`,
	},
	JSX_UNKNOWN_IDENTIFIER_TOKEN: {message: markup`Unknown JSX identifier token`},
	TS_IMPORT_ARG_NOT_STRING: {
		message: markup`Argument in a type import must be a string literal`,
	},
	TS_CONSTANT_NOT_LITERAL: {
		message: markup`Only literal values are allowed as a constant type`,
	},
	TS_INVALID_SIGNATURE_BINDING_NODE: {
		message: markup`Invalid node in signature binding list`,
	},
	TS_REQUIRED_FOLLOWS_OPTIONAL: {
		message: markup`A required element cannot follow an optional element.`,
	},
	TS_TEMPLATE_LITERAL_WITH_SUBSTITUION: {
		message: markup`Template literal types cannot have any substitution`,
	},
	TS_UNKNOWN_NON_ARRAY_START: {message: markup`Unknown TS non array type start`},
	TS_INVALID_READONLY_MODIFIER: {
		message: markup`'readonly' type modifier is only permitted on array and tuple literal types.`,
	},
	TS_EXTERNAL_MODULE_REFERENCE_ARG_NOT_STRING: {
		message: markup`TypeScript require() must have a single string argument`,
	},
	TS_UNKNOWN_DECLARE_START: {message: markup`Unknown TypeScript declare start`},
	TS_UNEXPECTED_CAST_IN_PARAMETER_POSITION: {
		message: markup`Unexpected type cast in parameter position`,
	},
	TS_DISABLED_BUT_ACCESSIBILITY_OR_READONLY: {
		message: markup`Accessibility and readonly syntax found but TS is not enabled`,
	},
	TS_PARAMETER_PROPERTY_BINDING_PATTERN: {
		message: markup`A parameter property may not be declared using a binding pattern.`,
	},
	TS_TUPLE_ELEMENT_LABEL_INCORRECT: {
		message: markup`Only an identifier can be a tuple element label but this is something more complex`,
	},
	TS_TUPLE_ELEMENT_OPTIONAL_REST: {
		message: markup`A tuple member cannot be both optional and rest.`,
	},
	TS_TUPLE_ELEMENT_OPTIONAL_TRAILING: {
		message: markup`A labeled tuple element is declared as optional with a question mark after the name and before the colon, rather than after the type.`,
	},
	TYPE_ANNOTATION_AFTER_ASSIGNMENT: {
		message: markup`Type annotations must come before default assignments, e.g. instead of \`age = 25: number\` use \`age: number = 25\``,
	},
	TYPE_BINDING_PARAMETER_OPTIONAL: {
		message: markup`A binding pattern parameter cannot be optional in an implementation signature.`,
	},
	ILLEGAL_FUNCTION_IN_STRICT: {
		message: markup`In strict mode code, functions can only be declared at top level or inside a block`,
	},
	ILLEGAL_FUNCTION_IN_NON_STRICT: {
		message: markup`In non-strict mode code, functions can only be declared at top level, inside a block, or as the body of an if statement`,
	},
	ILLEGAL_GENERATOR_DEFINITION: {
		message: markup`Generators can only be declared at the top level or inside a block`,
	},
	ILLEGAL_ASYNC_DEFINITION: {
		message: markup`Async functions can only be declared at the top level or inside a block`,
	},
	LEXICAL_DECLARATION_IN_SINGLE_STATEMENT_CONTEXT: {
		message: markup`Lexical declaration cannot appear in a single-statement context`,
	},
	IMPORT_EXPORT_MUST_TOP_LEVEL: {
		message: markup`'import' and 'export' may only appear at the top level`,
	},
	REGULAR_FOR_AWAIT: {
		message: markup`Can't have an await on a regular for loop`,
	},
	RETURN_OUTSIDE_FUNCTION: {message: markup`'return' outside of function`},
	MULTIPLE_DEFAULT_CASE: {message: markup`Multiple default clauses`},
	SWITCH_STATEMENT_OUTSIDE_CASE: {
		message: markup`Statement outside of a case or default block`,
	},
	NEWLINE_AFTER_THROW: {message: markup`Illegal newline after throw`},
	TRY_MISSING_FINALLY_OR_CATCH: {
		message: markup`Missing catch or finally clause`,
	},
	INVALID_LABEL_DECLARATION: {message: markup`Invalid labeled declaration`},
	WITH_IN_STRICT: {message: markup`'with' in strict mode`},
	OCTAL_IN_STRICT: {message: markup`Octal literal in strict mode`},
	FOR_IN_OF_WITH_INITIALIZER: {
		message: markup`Loop variable declaration may not have an initializer`,
	},
	CONST_WITHOUT_INITIALIZER: {
		message: markup`A constant must have an initializer`,
	},
	COMPLEX_BINDING_WITHOUT_INITIALIZER: {
		message: markup`Complex binding patterns require an initialization value`,
	},
	ACCESSOR_WITH_TYPE_PARAMS: {
		message: markup`An accessor cannot have type parameters`,
	},
	UNEXPECTED_SPREAD: {message: markup`Unexpected spread`},
	DUPLICATE_LABEL: (label: string, loc: undefined | SourceLocation) => ({
		message: markup`Label <emphasis>${label}</emphasis> is already declared`,
		advice: buildDuplicateLocationAdvice([loc]),
	}),
	UNKNOWN_LABEL: (label: undefined | string) => ({
		message: label === undefined
			? markup`No loop label found`
			: markup`Unknown label <emphasis>${label}</emphasis>`,
	}),
	IMPORT_EXPORT_IN_SCRIPT: (manifestPath: string) => ({
		message: markup`<emphasis>import</emphasis> and <emphasis>export</emphasis> can only appear in a module`,
		advice: [
			// TODO this advice is pointless if you have syntax extensions enabled
			{
				type: "log",
				category: "info",
				text: markup`Change the extension to <emphasis>.mjs</emphasis> to turn this file into a module`,
			},
			{
				type: "log",
				category: "info",
				text: markup`Add <emphasis>"type": "module"</emphasis> to your <filelink emphasis target="${manifestPath}" />`,
			},
		],
	}),
	SUPER_CALL_OUTSIDE_CONSTRUCTOR: {
		message: markup`super() is only valid inside a class constructor of a subclass`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Maybe a typo in the method name ('constructor') or not extending another class?`,
			},
		],
	},
	JSX_DISABLED: {
		message: markup`JSX syntax isn't enabled`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Are you using <emphasis>TypeScript</emphasis>? Change the file extension to <emphasis>.tsx</emphasis>`,
			},
			{
				type: "log",
				category: "info",
				text: markup`Are you using <emphasis>Flow</emphasis>? Add a <emphasis>@flow</emphasis> comment annotation to the top of the file`,
			},
			{
				type: "log",
				category: "info",
				text: markup`Not using either? Change the file extension to <emphasis>.jsx</emphasis>`,
			},
			// TODO you can also add `@jsx whatever` at the top of a file
		],
	},
	JSX_IN_TS_EXTENSION: {
		message: markup`JSX isn't allowed in regular TypeScript files`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Change the file extension to <emphasis>.tsx</emphasis> to enable JSX support`,
			},
		],
	},
	INVALID_PARENTEHSIZED_LVAL: (patternType: undefined | "object" | "array") => {
		const message = markup`Invalid parenthesized binding`;
		if (patternType === "object") {
			return {
				message,
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Did you use \`({a}) = 0\` instead of \`({a} = 0)\`?`,
					},
				],
			};
		} else if (patternType === "array") {
			return {
				message,
				advice: [
					{
						type: "log",
						category: "info",
						text: markup`Did you use \`([a]) = 0\` instead of \`([a] = 0)\`?`,
					},
				],
			};
		} else {
			return {
				message,
				advice: [],
			};
		}
	},
	EXPECTED_COMMA_SEPARATOR: (context: string) => ({
		message: markup`Expected a comma to separate items in ${context}`,
	}),
	INVALID_LEFT_HAND_SIDE: (context: string) => ({
		message: markup`Invalid left-hand side in ${context}`,
	}),
	TS_EMPTY_LIST: (descriptor: string) => ({
		message: markup`${descriptor} list cannot be empty`,
	}),
	JSX_EXPECTED_CLOSING_TAG: (name: string, openingLoc: SourceLocation) => ({
		message: markup`Expected a corresponding JSX closing tag for <emphasis>${name}</emphasis>`,
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_EXPECTED_CLOSING_FRAGMENT_TAG: (name: string, openingLoc: SourceLocation) => ({
		message: markup`Expected JSX closing fragment tag`,
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_UNKNOWN_CHILD_START: (name: string, openingLoc: SourceLocation) => ({
		message: markup`Unknown JSX children start`,
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_UNCLOSED_ELEMENT: (name: string, openingLoc: SourceLocation) => ({
		message: markup`Unclosed JSX element`,
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	TS_REQUIRED: (label: string) => ({
		message: markup`A ${label} is only valid inside of a TypeScript file`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`To enable <emphasis>TypeScript</emphasis> support, the file extension should end in <emphasis>.ts</emphasis> or <emphasis>.tsx</emphasis>`,
			},
		],
	}),
	DUPLICATE_EXPORT: (name: string, existing: SourceLocation) => ({
		message: name === "default"
			? markup`Only one default export allowed per module.`
			: markup`\`${name}\` has already been exported. Exported identifiers must be unique.`,
		advice: buildDuplicateLocationAdvice([existing]),
	}),
	NEW_IN_OPTIONAL_CHAIN: (responsiblePointer?: DiagnosticLocation) => ({
		message: markup`constructors in/after an Optional Chain are not allowed`,
		advice: responsiblePointer && [
			{
				type: "log",
				category: "info",
				text: markup`Optional chain member responsible`,
			},
			{
				type: "frame",
				location: responsiblePointer,
			},
		],
	}),
	UNKNOWN_EXPRESSION_ATOM_START: (context: string) => ({
		message: markup`Unknown start to an ${context}`,
	}),
	INVALID_META_PROPERTY: (metaName: string, propertyName: string) => ({
		message: markup`The only valid meta property for ${metaName} is ${metaName}.${propertyName}`,
	}),
	ARGUMENT_CLASH_IN_STRICT: (name: string, loc: undefined | SourceLocation) => ({
		message: markup`Argument <emphasis>${name}</emphasis> name clash in strict mode`,
		advice: buildDuplicateLocationAdvice([loc]),
	}),
	RESERVED_WORD: (word: string) => ({
		message: markup`${word} is a reserved word`,
	}),
	UNEXPECTED_KEYWORD: (keyword: string) => ({
		message: markup`Unexpected keyword ${keyword}`,
	}),
	UNEXPECTED_TOKEN: (
		expected: undefined | string,
		possibleShiftMistake: boolean,
	) => ({
		message: expected === undefined
			? markup`Unexpected token`
			: markup`Unexpected token, expected ${expected}`,
		advice: possibleShiftMistake
			? [
					{
						type: "log",
						category: "info",
						text: markup`Did you accidently hold shift?`,
					},
				]
			: [],
	}),
	EXPECTED_CLOSING: (name: string, char: string, location: DiagnosticLocation) => ({
		message: markup`Unclosed ${name}`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`We expected to find the closing character <emphasis>${char}</emphasis> here`,
			},
			{
				type: "frame",
				location,
			},
		],
	}),
	EXPECTED_KEYWORD: (keyword: string) => ({
		message: markup`Expected keyword ${keyword}`,
	}),
	ESCAPE_SEQUENCE_IN_WORD: (word: string) => ({
		message: markup`${word} can't contain a unicode escape`,
	}),
	EXPECTED_ENABLE_SYNTAX: (syntaxName: string) => ({
		message: markup`Expected ${syntaxName} syntax to be enabled`,
	}),
	UNEXPECTED_HASH: (exclamationFollowed: boolean) => ({
		message: markup`Unexpected character #`,
		advice: exclamationFollowed
			? [
					{
						type: "log",
						category: "info",
						text: markup`Did you want to write a hashbang? A hashbang can only be the first thing in a file.`,
					},
				]
			: [],
	}),
	UNEXPECTED_UNICODE_CHARACTER: (
		char: string,
		unicodeName: string,
		equivalentChar: string,
		equivalentName: string,
	) => ({
		message: markup`Unexpected Unicode character '<emphasis>${char}</emphasis>' (<emphasis>${unicodeName}</emphasis>)`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Did you mean '<emphasis>${equivalentChar}</emphasis>' (<emphasis>${equivalentName}</emphasis>)? Both characters look the same, but are not.`,
			},
		],
	}),
	EXPECTED_NUMBER_IN_RADIX: (radix: number) => ({
		message: markup`Expected number in radix ${String(radix)}`,
	}),
	INVALID_IDENTIFIER_NAME: (name: string) => ({
		message: markup`Invalid identifier ${name}`,
	}),
	ESCAPE_SEQUENCE_IN_KEYWORD: (keyword: string) => ({
		message: markup`Escape sequence in keyword ${keyword}`,
	}),
});
