import {createDiagnosticsCategory} from "./index";
import {DiagnosticAdvice, DiagnosticLocation} from "../types";
import {SourceLocation} from "@romejs/parser-core";
import {buildDuplicateLocationAdvice} from "../helpers";
import {markup} from "@romejs/string-markup";

function buildJSXOpeningAdvice(
	name: string,
	openingLoc: SourceLocation,
): DiagnosticAdvice {
	return [
		{
			type: "log",
			category: "info",
			text: name === ""
				? "Originated from this opening tag"
				: `Originated from opening tag of <emphasis>${name}</emphasis>`,
		},
		{
			type: "frame",
			location: openingLoc,
		},
	];
}

// @romejs/js-parser
export const jsParser = createDiagnosticsCategory({
	UNTERMINATED_BLOCK_COMMENT: "Unterminated comment",
	UNTERMINATED_JSX_STRING: "Unterminated string constant",
	INVALID_UNICODE_ESCAPE: "Invalid Unicode escape",
	EXPECTED_UNICODE_ESCAPE: "Expecting Unicode escape sequence \\uXXXX",
	BAD_HEX_ESCAPE: "Bad character escape sequence",
	OCTAL_IN_STRICT_MODE: "Octal literal in strict mode",
	UNTERMINATED_TEMPLATE: "Unterminated template",
	UNTERMINATED_STRING: "Unterminated string constant",
	OUT_OF_BOUND_CODE_POINT: "Code point out of bounds",
	IDENTIFIER_AFTER_NUMBER: "Identifier directly after number",
	OCTAL_BIGINT: "A bigint can't be an octal",
	DECIMAL_BIGINT: "A bigint can't have a decimal",
	INVALID_NUMBER: "Invalid number",
	LEGACY_OCTAL_IN_STRICT_MODE: "Legacy octal literals are not allowed in strict mode",
	INVALID_INT_TOKEN: "Invalid or unexpected int token",
	UNICODE_ESCAPE_IN_REGEX_FLAGS: "Regular expression flags can't contain unicode escapes",
	UNTERMINATED_REGEX: "Unterminated regular expression",
	DANGLING_BACKSLASH_IN_REGEX: "Dangling backslash in a regular expression",
	EXPECTED_RELATIONAL_OPERATOR: "Expected relational operator",
	UNEXPECTED_SPACE: "Unexpected space",
	EXPECTED_SEMI_OR_LINE_TERMINATOR: "Expected a semicolon or a line terminator",
	GET_SET_CLASS_CONSTRUCTOR: "Constructor can't have get/set modifier",
	ASYNC_CLASS_CONSTRUCTOR: "Constructor cannot be async",
	GENERATOR_CLASS_CONSTRUCTOR: "Constructor cannot be a generator",
	DUPLICATE_CLASS_CONSTRUCTOR: "Duplicate constructor in the same class",
	UNKNOWN_CLASS_PROPERTY_START: "Unknown class property start",
	CLASS_STATIC_PROTOTYPE_PROPERTY: "Classes may not have static property named prototype",
	CLASS_PRIVATE_FIELD_NAMED_CONSTRUCTOR: "Classes may not have a private field named '#constructor'",
	CLASS_PROPERTY_NAME_CONSTRUCTOR: "Classes may not have a non-static field named 'constructor'",
	PROTO_PROP_REDEFINITION: "Redefinition of __proto__ property",
	MISSING_CONDITIONAL_SEPARATOR: "Missing conditional expression consequent separator",
	WRAP_EXPONENTIATION: "Illegal expression. Wrap left hand side or entire exponentiation in parentheses.",
	DELETE_LOCAL_VARIABLE_IN_STRICT: "Deleting local variable in strict mode",
	DELETE_PRIVATE_FIELD: "Deleting a private field is not allowed",
	TAGGED_TEMPLATE_IN_OPTIONAL_CHAIN: "Tagged Template Literals are not allowed in optionalChain",
	YIELD_NAME_IN_GENERATOR: "Can not use 'yield' as identifier inside a generator",
	AWAIT_NAME_IN_ASYNC: "Can not use 'await' as identifier inside an async function",
	EMPTY_PARENTHESIZED_EXPRESSION: "Parenthesized expression didnt contain anything",
	AWAIT_IN_ASYNC_PARAMS: "await is not allowed in async function parameters",
	YIELD_IN_GENERATOR_PARAMS: "yield is not allowed in generator parameters",
	FLOW_TYPE_CAST_IN_TS: "Flow type cast expressions aren't allowed in TypeScript",
	PARENTHESIZED_FUNCTION_PARAMS: "Function parameters can't be parenthesized",
	NEW_WITH_TYPESCRIPT_TYPE_ARGUMENTS_NO_PARENS: "In TypeScript, a new expression with type arguments must have parens",
	INVALID_TEMPLATE_ESCAPE: "Invalid escape sequence in template",
	EXPECTED_IDENTIFIER: "Expected an identifier",
	IMPORT_EXACT_ARGUMENTS: "import() requires exactly one argument",
	IMPORT_TRAILING_COMMA: "Trailing comma is disallowed inside import(...) arguments",
	IMPORT_SPREAD: "Spread is not allowed in import()",
	IMPORT_NEW_CALLEE: "Cannot use new with import(...)",
	SUPER_OUTSIDE_METHOD: "super is only allowed in object methods and classes",
	INVALID_SUPER_SUFFIX: "Invalid super suffix operator",
	AWAIT_OUTSIDE_ASYNC: "Can't use await outside of an async function",
	AWAIT_STAR: "await* has been removed from the async functions proposal. Use Promise.all() instead.",
	NEW_TARGET_OUTSIDE_CLASS: "new.target can only be used in functions or class properties",
	MULTIPLE_DESTRUCTURING_RESTS: "Cannot have multiple rest elements when destructuring",
	TRAILING_COMMA_AFTER_REST: "A trailing comma is not permitted after the rest element",
	GETTER_WITH_PARAMS: "getter should have no parameters",
	SETTER_WITH_REST: "setter function argument must not be a rest parameter",
	SETTER_NOT_ONE_PARAM: "setter should have exactly one param",
	ASYNC_GETTER_SETTER: "An object setter/getter can't be async",
	GENERATOR_GETTER_SETTER: "An object setter/getter can't be a generator",
	ARGUMENTS_IN_CLASS_FIELD: "'arguments' is not allowed in class field initializer",
	NON_SIMPLE_PARAM_IN_EXPLICIT_STRICT_FUNCTION: "Non-simple parameter in strict mode",
	STRICT_DIRECTIVE_IN_NON_SIMPLE_PARAMS: "Illegal 'use strict' directive in function with non-simple parameter list",
	OBJECT_PROPERTY_WITH_TYPE_PARAMETERS: "Object property cannot have type parameters",
	ILLEGAL_VARIANCE: "Variance is not allowed here",
	OBJECT_METHOD_IN_PATTERN: "Object methods aren't allowed in object patterns",
	IMPORT_META_OUTSIDE_MODULE: `import.meta may only appear in a module`,
	EXPECTED_ARROW_AFTER_ASYNC_TYPE_PARAMS: "Expected arrow because we are a possible async arrow and type annotated parameters were present",
	INVALID_OBJECT_PATTERN_PROP: "Invalid property node for object pattern",
	ASYNC_OBJECT_METHOD_LINE_BREAK: "There shouldn't be any newlines between async and the rest of the function",
	SPACE_BETWEEN_PRIVATE_HASH: "Unexpected space between # and identifier",
	CONFUSING_CALL_ARGUMENT: "Function parameter type annotation? Possibly forgot curlies around an object. Possibly forgot async keyword.",
	EXPECTED_ARROW_AFTER_TYPE_PARAMS: "Expected an arrow function after this type parameter declaration",
	REQUIRED_CLASS_NAME: "Class name is required",
	JSX_ELEM_TYPE_ARGUMENTS_OUTSIDE_TS: "JSX element type arguments are only allowed in TS",
	UNWRAPPED_ADJACENT_JHX: `Adjacent JSX elements must be wrapped in an enclosing tag. Did you want a JSX fragment \\<>...\\</>?`,
	CONFUSED_OR: "Unexpected ||, did you mean just |?",
	INVALID_ASSIGNMENT_TARGET: "Not a valid assignment target",
	IMPORT_KIND_SPECIFIER_ON_IMPORT_DECLARATION_WITH_KIND: "The `type` and `typeof` keywords on named imports can only be used on regular `import` statements. It cannot be used with `import type` or `import typeof` statements",
	DESTRUCTURING_IN_IMPORT: "ES2015 named imports do not destructure. Use another statement for destructuring after the import.",
	IMPORT_MISSING_SOURCE: "import missing a source",
	EXPORT_TYPE_NAMESPACE: "Can't have a type export namespacer specifier",
	EXPORT_MISSING_FROM: "Expected `from` for an export node",
	EXPORT_FROM_NOT_STRING: "Export from only allows strings",
	BINDING_MEMBER_EXPRESSION: "Binding member expression",
	INVALID_OBJECT_PATTERN_PROPERTY: "Not a valid assignment object pattern property",
	OBJECT_PATTERN_CANNOT_CONTAIN_METHODS: "Object pattern cannot contains methods",
	INVALID_ASSIGNMENT_PATTERN_OPERATOR: "Only '=' operator can be used for specifying default value.",
	INVALID_OBJECT_REST_ARGUMENT: "Invalid rest operator's argument",
	INVALID_EXPORT_DEFAULT: "Only expressions, functions or classes are allowed as the `default` export.",
	INVALID_EXPORT_DECLARATION: "Invalid export declaration",
	DESTRUCTURING_REST_ELEMENT_NOT_LAST: `The rest element has to be the last element when destructuring`,
	REST_INVALID_ARGUMENT: "Invalid rest operator's argument",
	EXPORT_ASYNC_NO_FUNCTION_KEYWORD: "Started with `export async` so we expected to receive an async function but no function keyword was found",
	TYPE_CAST_WITHOUT_ANNOTATION: "Type cast expression has no type annotation. Did you mean for this to be a function parameter?",
	TYPE_CAST_CANNOT_BE_OPTIONAL: "Type cast expressions cannot be optional. Did you mean for this to be a function parameter?",
	TYPE_CAST_EXPECTED_PARENS: "The type cast expression is expected to be wrapped with parentheses",
	INVALID_ASYNC_ARROW_WITH_TYPE_PARAMS: "Invalid async arrow with type parameters",
	TYPE_NUMERIC_LITERAL_PLUS: "Numeric literal type annotations cannot stand with a +, omit it instead",
	TYPE_NUMERIC_LITERAL_EXPECTED: `Unexpected token, expected "number"`,
	JSX_INVALID_ATTRIBUTE_VALUE: "JSX attribute value should be either an expression or a quoted JSX text",
	JSX_UNCLOSED_SELF_CLOSING_TAG: "Unclosed JSX element open",
	JSX_UNCLOSED_CLOSING_TAG: "Unclosed JSX element close",
	JSX_EMPTY_ATTRIBUTE_VALUE: "JSX attribute cannot be an empty expression",
	JSX_UNKNOWN_IDENTIFIER_TOKEN: "Unknown JSX identifier token",
	TS_IMPORT_ARG_NOT_STRING: "Argument in a type import must be a string literal",
	TS_CONSTANT_NOT_LITERAL: "Only literal values are allowed as a constant type",
	TS_INVALID_SIGNATURE_BINDING_NODE: "Invalid node in signature binding list",
	TS_REQUIRED_FOLLOWS_OPTIONAL: "A required element cannot follow an optional element.",
	TS_TEMPLATE_LITERAL_WITH_SUBSTITUION: "Template literal types cannot have any substitution",
	TS_UNKNOWN_NON_ARRAY_START: "Unknown TS non array type start",
	TS_INVALID_READONLY_MODIFIER: "'readonly' type modifier is only permitted on array and tuple literal types.",
	TS_EXTERNAL_MODULE_REFERENCE_ARG_NOT_STRING: "TypeScript require() must have a single string argument",
	TS_UNKNOWN_DECLARE_START: "Unknown TypeScript declare start",
	TS_UNEXPECTED_CAST_IN_PARAMETER_POSITION: "Unexpected type cast in parameter position",
	TS_DISABLED_BUT_ACCESSIBILITY_OR_READONLY: "Accessibility and readonly syntax found but TS is not enabled",
	TS_PARAMETER_PROPERTY_BINDING_PATTERN: "A parameter property may not be declared using a binding pattern.",
	TS_TUPLE_ELEMENT_LABEL_INCORRECT: "Only an identifier can be a tuple element label but this is something more complex",
	TS_TUPLE_ELEMENT_OPTIONAL_REST: "A tuple member cannot be both optional and rest.",
	TS_TUPLE_ELEMENT_OPTIONAL_TRAILING: "A labeled tuple element is declared as optional with a question mark after the name and before the colon, rather than after the type.",
	TYPE_ANNOTATION_AFTER_ASSIGNMENT: "Type annotations must come before default assignments, e.g. instead of `age = 25: number` use `age: number = 25`",
	TYPE_BINDING_PARAMETER_OPTIONAL: "A binding pattern parameter cannot be optional in an implementation signature.",
	ILLEGAL_FUNCTION_IN_STRICT: "In strict mode code, functions can only be declared at top level or inside a block",
	ILLEGAL_FUNCTION_IN_NON_STRICT: "In non-strict mode code, functions can only be declared at top level, inside a block, or as the body of an if statement",
	ILLEGAL_GENERATOR_DEFINITION: "Generators can only be declared at the top level or inside a block",
	ILLEGAL_ASYNC_DEFINITION: "Async functions can only be declared at the top level or inside a block",
	LEXICAL_DECLARATION_IN_SINGLE_STATEMENT_CONTEXT: "Lexical declaration cannot appear in a single-statement context",
	IMPORT_EXPORT_MUST_TOP_LEVEL: "'import' and 'export' may only appear at the top level",
	REGULAR_FOR_AWAIT: "Can't have an await on a regular for loop",
	RETURN_OUTSIDE_FUNCTION: "'return' outside of function",
	MULTIPLE_DEFAULT_CASE: "Multiple default clauses",
	SWITCH_STATEMENT_OUTSIDE_CASE: "Statement outside of a case or default block",
	NEWLINE_AFTER_THROW: "Illegal newline after throw",
	TRY_MISSING_FINALLY_OR_CATCH: "Missing catch or finally clause",
	INVALID_LABEL_DECLARATION: "Invalid labeled declaration",
	WITH_IN_STRICT: "'with' in strict mode",
	OCTAL_IN_STRICT: "Octal literal in strict mode",
	FOR_IN_OF_WITH_INITIALIZER: "Loop variable declaration may not have an initializer",
	CONST_WITHOUT_INITIALIZER: "A constant must have an initializer",
	COMPLEX_BINDING_WITHOUT_INITIALIZER: "Complex binding patterns require an initialization value",
	ACCESSOR_WITH_TYPE_PARAMS: "An accessor cannot have type parameters",
	UNEXPECTED_SPREAD: "Unexpected spread",
	DUPLICATE_LABEL: (label: string, loc: undefined | SourceLocation) => ({
		message: markup`Label <emphasis>${label}</emphasis> is already declared`,
		advice: buildDuplicateLocationAdvice([loc]),
	}),
	UNKNOWN_LABEL: (label: undefined | string) => ({
		message: label === undefined
			? "No loop label found"
			: markup`Unknown label <emphasis>${label}</emphasis>`,
	}),
	IMPORT_EXPORT_IN_SCRIPT: (manifestPath: string) => ({
		message: `<emphasis>import</emphasis> and <emphasis>export</emphasis> can only appear in a module`,
		advice: [
			// TODO this advice is pointless if you have syntax extensions enabled
			{
				type: "log",
				category: "info",
				text: "Change the extension to <emphasis>.mjs</emphasis> to turn this file into a module",
			},
			{
				type: "log",
				category: "info",
				text: `Add <emphasis>"type": "module"</emphasis> to your <filelink emphasis target="${manifestPath}" />`,
			},
		],
	}),
	SUPER_CALL_OUTSIDE_CONSTRUCTOR: {
		message: "super() is only valid inside a class constructor of a subclass",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Maybe a typo in the method name ('constructor') or not extending another class?",
			},
		],
	},
	JSX_DISABLED: {
		message: "JSX syntax isn't enabled",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Are you using <emphasis>TypeScript</emphasis>? Change the file extension to <emphasis>.tsx</emphasis>",
			},
			{
				type: "log",
				category: "info",
				text: "Are you using <emphasis>Flow</emphasis>? Add a <emphasis>@flow</emphasis> comment annotation to the top of the file",
			},
			{
				type: "log",
				category: "info",
				text: "Not using either? Change the file extension to <emphasis>.jsx</emphasis>",
			},
			// TODO you can also add `@jsx whatever` at the top of a file
		],
	},
	JSX_IN_TS_EXTENSION: {
		message: "JSX isn't allowed in regular TypeScript files",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Change the file extension to <emphasis>.tsx</emphasis> to enable JSX support",
			},
		],
	},
	INVALID_PARENTEHSIZED_LVAL: (patternType: undefined | "object" | "array") => ({
		message: "Invalid parenthesized binding",
		advice: patternType === "object"
			? [
					{
						type: "log",
						category: "info",
						text: "Did you use `({a}) = 0` instead of `({a} = 0)`?",
					},
				]
			: patternType === "array"
				? [
						{
							type: "log",
							category: "info",
							text: "Did you use `([a]) = 0` instead of `([a] = 0)`?",
						},
					]
				: [],
	}),
	EXPECTED_COMMA_SEPARATOR: (context: string) => ({
		message: `Expected a comma to separate items in ${context}`,
	}),
	INVALID_LEFT_HAND_SIDE: (context: string) => ({
		message: `Invalid left-hand side in ${context}`,
	}),
	TS_EMPTY_LIST: (descriptor: string) => ({
		message: `${descriptor} list cannot be empty`,
	}),
	JSX_EXPECTED_CLOSING_TAG: (name: string, openingLoc: SourceLocation) => ({
		message: `Expected a corresponding JSX closing tag for <emphasis>${name}</emphasis>`,
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_EXPECTED_CLOSING_FRAGMENT_TAG: (name: string, openingLoc: SourceLocation) => ({
		message: "Expected JSX closing fragment tag",
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_UNKNOWN_CHILD_START: (name: string, openingLoc: SourceLocation) => ({
		message: "Unknown JSX children start",
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	JSX_UNCLOSED_ELEMENT: (name: string, openingLoc: SourceLocation) => ({
		message: "Unclosed JSX element",
		advice: buildJSXOpeningAdvice(name, openingLoc),
	}),
	TS_REQUIRED: (label: string) => ({
		message: `A ${label} is only valid inside of a TypeScript file`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "To enable <emphasis>TypeScript</emphasis> support, the file extension should end in <emphasis>.ts</emphasis> or <emphasis>.tsx</emphasis>",
			},
		],
	}),
	DUPLICATE_EXPORT: (name: string, existing: SourceLocation) => ({
		message: name === "default"
			? "Only one default export allowed per module."
			: `\`${name}\` has already been exported. Exported identifiers must be unique.`,
		advice: buildDuplicateLocationAdvice([existing]),
	}),
	NEW_IN_OPTIONAL_CHAIN: (responsiblePointer?: DiagnosticLocation) => ({
		message: "constructors in/after an Optional Chain are not allowed",
		advice: responsiblePointer && [
			{
				type: "log",
				category: "info",
				text: "Optional chain member responsible",
			},
			{
				type: "frame",
				location: responsiblePointer,
			},
		],
	}),
	UNKNOWN_EXPRESSION_ATOM_START: (context: string) => ({
		message: `Unknown start to an ${context}`,
	}),
	INVALID_META_PROPERTY: (metaName: string, propertyName: string) => ({
		message: `The only valid meta property for ${metaName} is ${metaName}.${propertyName}`,
	}),
	ARGUMENT_CLASH_IN_STRICT: (name: string, loc: undefined | SourceLocation) => ({
		message: markup`Argument <emphasis>${name}</emphasis> name clash in strict mode`,
		advice: buildDuplicateLocationAdvice([loc]),
	}),
	RESERVED_WORD: (word: string) => ({
		message: `${word} is a reserved word`,
	}),
	UNEXPECTED_KEYWORD: (keyword: string) => ({
		message: `Unexpected keyword ${keyword}`,
	}),
	UNEXPECTED_TOKEN: (
		expected: undefined | string,
		possibleShiftMistake: boolean,
	) => ({
		message: expected === undefined
			? "Unexpected token"
			: `Unexpected token, expected ${expected}`,
		advice: possibleShiftMistake
			? [
					{
						type: "log",
						category: "info",
						text: `Did you accidently hold shift?`,
					},
				]
			: [],
	}),
	EXPECTED_CLOSING: (name: string, char: string, location: DiagnosticLocation) => ({
		message: `Unclosed ${name}`,
		advice: [
			{
				type: "log",
				category: "info",
				text: `We expected to find the closing character <emphasis>${char}</emphasis> here`,
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
		message: "Unexpected character #",
		advice: exclamationFollowed
			? [
					{
						type: "log",
						category: "info",
						text: "Did you want to write a hashbang? A hashbang can only be the first thing in a file.",
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
		message: `Expected number in radix ${String(radix)}`,
	}),
	INVALID_IDENTIFIER_NAME: (name: string) => ({
		message: `Invalid identifier ${name}`,
	}),
	ESCAPE_SEQUENCE_IN_KEYWORD: (keyword: string) => ({
		message: `Escape sequence in keyword ${keyword}`,
	}),
});
