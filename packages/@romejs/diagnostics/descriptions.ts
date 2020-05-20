/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticAdvice,
	DiagnosticAdviceAction,
	DiagnosticBlessedMessage,
	DiagnosticDescription,
	DiagnosticLocation,
	DiagnosticSuppression,
} from "./types";
import {escapeMarkup, markup} from "@romejs/string-markup";
import stringDiff from "@romejs/string-diff";
import {buildDuplicateLocationAdvice, buildSuggestionAdvice} from "./helpers";
import {SourceLocation} from "@romejs/parser-core";
import {DiagnosticCategory} from "./categories";
import {ResolverQueryResponseNotFound} from "@romejs/core/master/fs/Resolver";
import {UnknownNumber} from "@romejs/ob1";
import {toKebabCase} from "@romejs/string-utils";

type DiagnosticMetadataString = Omit<Partial<DiagnosticDescription>, "message"> & {
	message: string;
};

// The purpose of this is so that we're explicit whenever we want to create a diagnostic message outside of this file
export function createBlessedDiagnosticMessage(
	value: string,
): DiagnosticBlessedMessage {
	return {
		type: "PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE",
		value,
	};
}

function join(conjunction: string, items: Array<string>): string {
	if (items.length === 0) {
		return "";
	} else if (items.length === 1) {
		return items[0];
	} else {
		const popped = items.pop()!;
		return [...items, `${conjunction} ${popped}`].join(", ");
	}
}

function andJoin(items: Array<string>): string {
	return join("and", items);
}
andJoin;

function orJoin(items: Array<string>): string {
	return join("or", items);
}

function addEmphasis(items: Array<string>): Array<string> {
	return items.map((item) => `<emphasis>${item}</emphasis>`);
}

// rome-ignore lint/js/noExplicitAny
type InputMessagesFactory = (...params: Array<any>) => DiagnosticMetadataString;

type InputMessagesCategory = {
	[key: string]: string | DiagnosticMetadataString | InputMessagesFactory;
};

type InputMessages = {
	[category: string]: InputMessagesCategory;
};

type OuputMessagesFactoryReturn<Ret extends DiagnosticMetadataString> = Omit<
	Ret,
	"message" | "advice"
> & {
	advice: DiagnosticAdvice;
	message: DiagnosticBlessedMessage;
};

type OutputMessagesFactory<Func extends InputMessagesFactory> = (
	...params: Parameters<Func>
) => OuputMessagesFactoryReturn<ReturnType<Func>>;

type OutputMessagesValue<Value> = Value extends string
	? {
			message: DiagnosticBlessedMessage;
			advice: DiagnosticAdvice;
		}
	: Value extends DiagnosticMetadataString
		? OuputMessagesFactoryReturn<Value>
		: Value extends InputMessagesFactory
			? OutputMessagesFactory<Value>
			: never;

type OutputMessagesCategory<Input extends InputMessagesCategory> = {
	[Key in keyof Input]: OutputMessagesValue<Input[Key]>
};

type OutputMessages<Input extends InputMessages> = {
	[Key in keyof Input]: OutputMessagesCategory<Input[Key]>
};

// This is a lot of gross meta programming
function createMessages<Input extends InputMessages>(
	messages: Input,
): OutputMessages<Input> {
	// rome-ignore lint/js/noExplicitAny
	const out: OutputMessages<Input> = ({} as any);

	for (const categoryName in messages) {
		// rome-ignore lint/js/noExplicitAny
		const category: OutputMessagesCategory<any> = {};
		out[categoryName] = category;

		const inputCategory = messages[categoryName];
		for (const key in inputCategory) {
			const value = inputCategory[key];

			if (typeof value === "string") {
				category[key] = {
					advice: [],
					message: createBlessedDiagnosticMessage(value),
				};
			} else if (typeof value === "function") {
				// rome-ignore lint/js/noExplicitAny
				const callback: InputMessagesFactory = (value as any);

				category[key] = function(...params) {
					const {message, ...ret} = callback(...params);
					return {
						advice: [],
						...ret,
						message: createBlessedDiagnosticMessage(message),
					};
				};
			} else {
				// rome-ignore lint/js/noExplicitAny
				const {message, ...obj} = (value as any);
				category[key] = {
					advice: [],
					...obj,
					message: createBlessedDiagnosticMessage(message),
				};
			}
		}
	}

	return out;
}

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

export const descriptions = createMessages({
	FLAGS: {
		UNSUPPORTED_SHORTHANDS: `Shorthand flags are not supported`,
		INCORRECT_CASED_FLAG: (flag: string) => ({
			message: "Incorrect cased flag name",
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`Use <emphasis>${toKebabCase(flag)}</emphasis> instead`,
				},
			],
		}),
		INCORRECT_ARG_COUNT: (excessive: boolean, message: string) => ({
			message: excessive ? "Too many arguments" : "Missing arguments",
			advice: [
				{
					type: "log",
					category: "info",
					text: message,
				},
			],
		}),
		DISALLOWED_REVIEW_FLAG: (key: string) => ({
			message: `Flag <emphasis>${key}</emphasis> is not allowed with <emphasis>review</emphasis>`,
		}),
		DISALLOWED_REQUEST_FLAG: (key: string) => ({
			message: `This command does not support the <emphasis>${key}</emphasis> flag`,
		}),
		UNKNOWN_ACTION: (action: string) => ({
			message: `Unknown action ${action}`,
		}),
		NO_FILES_FOUND: (noun: undefined | string) => ({
			message: noun === undefined
				? "No files found"
				: `No files to ${noun} found`,
		}),
	},
	// @romejs/parser-core
	PARSER_CORE: {
		EXPECTED_SPACE: "Expected no space between",
		EXPECTED_EOF: "Expected end of file",
		UNEXPECTED_EOF: "Unexpected end of file",
		UNEXPECTED: (type: string) => ({
			message: markup`Unexpected ${type}`,
		}),
		UNEXPECTED_CHARACTER: (char: string) => ({
			message: markup`Unexpected character <emphasis>${char}</emphasis>`,
		}),
		EXPECTED_TOKEN: (got: string, expected: string) => {
			return {
				message: markup`Expected token ${expected} but got ${got}`,
			};
		},
	},
	// @romejs/codec-js-regexp
	REGEX_PARSER: {
		INVALID_CAPTURE_GROUP_MODIFIER: "Invalid capture group modifier",
		UNCLOSED_GROUP: "Unclosed group",
		UNOPENED_GROUP: "Unopened group",
		INVALID_QUANTIFIER_TARGET: "Invalid target for quantifier",
		UNKNOWN_REGEX_PART: "Unknown regex part",
		REVERSED_CHAR_SET_RANGE: "Range values reversed. Start char code is greater than end char code",
		UNCLOSED_CHAR_SET: "Unclosed character set",
		DUPLICATE_FLAG: "Duplicate regular expression flag",
		INVALID_FLAG: "Invalid regular expression flag",
		REVERSED_QUANTIFIER_RANGE: "Quantifier minimum is greater than maximum",
		NO_TARGET_QUANTIFIER: "Nothing to repeat",
		INVALID_NAMED_CAPTURE: "Invalid named capture referenced",
		UNCLOSED_NAMED_CAPTURE: "Unclosed named capture",
	},
	// @romejs/codec-json
	JSON: {
		SINGLE_QUOTE_USAGE: "You can only use double quoted strings",
		TRAILING_COMMA_VALUE: "Trailing comma is only allowed after a value",
		UNCLOSED_STRING: "Unclosed string",
		UNCLOSED_BLOCK_COMMENT: "Unclosed block comment",
		MISTAKEN_ARRAY_IDENTITY: "Trying to use an array element as an object property. Did you mean to make an object?",
		REDUNDANT_COMMA: "Redundant comma",
		EMPTY_INPUT_IN_JSON: "Empty input",
		PROPERTY_KEY_UNQUOTED_IN_JSON: "Property keys must be quoted in JSON",
		IMPLICIT_OBJECT_IN_JSON: "Objects must be wrapped in curly braces in JSON",
		COMMENTS_IN_JSON: "Comments aren't allowed in JSON",
		TRAILING_COMMA_IN_JSON: "Trailing commas aren't allowed in JSON",
		REGEX_IN_JSON: "Regular expressions aren't allowed in JSON",
		UNKNOWN_WORD_IN_JSON: (word: string) => ({
			message: markup`${word} isn't a valid JSON word`,
		}),
		STRING_NEWLINES_IN_JSON: 'Newlines aren\'t allowed in JSON, you insert a newline by escaping it like this "\\n"',
		UNDEFINED_IN_JSON: "undefined isn't allowed in JSON, you could use null instead",
		BIGINT_IN_JSON: "Bigints aren't allowed in JSON",
		NUMERIC_SEPARATORS_IN_JSON: "Numeric separators are not allowed in JSON",
	},
	// @romejs/codec-semver
	SEMVER: {
		MISSING_MINOR_VERSION: "A minor number is required for a version",
		MISSING_PATCH_VERSION: "A patch number is required for a version",
		EXCESSIVE_VERSION_PARTS: "Too many parts for version",
		INVALID_QUANTIFIER_PART: "Invalid version qualifier part",
		WILDCARD_IN_VERSION: "Wildcard aren't allowed in a hard version",
		INVALID_VERSION_NUMBER: "This isn't a valid version part, expected a number",
		INVALID_RANGE: "A semver range can only be defined with versions",
		BARE_PIPE_WITHOUT_LOOSE: "Bare pipes are only allowed in loose mode",
		UNEXPECTED_WORD: (word: string) => ({
			message: markup`Unexpected word <emphasis>${word}</emphasis>`,
		}),
		UNKNOWN_START: "Unknown start of atom",
		EXPECTED_VERSION: "Unexpected value for version",
	},
	V8: {
		SYNTAX_ERROR: (message: string) => ({message, category: "v8/syntaxError"}),
	},
	// @romejs/core/master/commands/lint.ts
	LINT_COMMAND: {
		INVALID_DECISION_ACTION: (action: string) => ({
			message: markup`<emphasis>${action}</emphasis> is not a valid decision action`,
		}),
		INVALID_DECISION_PART_COUNT: (i: number) => ({
			message: `Segment ${i} contains an invalid number of decision parts`,
		}),
	},
	// @romejs/js-compiler
	LINT: {
		JSX_A11Y_TABINDEX_NO_POSITIVE: {
			category: "lint/jsx-a11y/tabindexNoPositive",
			message: "Avoid positive integer values for <emphasis>tabIndex</emphasis>.",
		},
		JSX_A11Y_MOUSE_EVENTS_HAVE_KEY_EVENTS: (
			mouseEvent: string,
			keyboardEvent: string,
		) => ({
			category: "lint/jsx-a11y/mouseEventsHaveKeyEvents",
			message: `The mouse event <emphasis>${mouseEvent}</emphasis> should be paired with the event <emphasis>${keyboardEvent}</emphasis>`,
		}),
		JSX_A11Y_MEDIA_HAS_CAPTION: {
			category: "lint/jsx-a11y/mediaHasCaption",
			message: "<emphasis>audio</emphasis> and <emphasis>video</emphasis> elements should have <emphasis>track</emphasis> for captions",
		},
		REACT_NO_WILL_UPDATE_SET_STATE: {
			category: "lint/react/noWillUpdateSetState",
			message: "Avoid <emphasis>this.setState</emphasis> in <emphasis>componentWillUpdate</emphasis>",
		},
		JSX_A11Y_ANCHOR_HAS_CONTENT: {
			category: "lint/jsx-a11y/anchorHasContent",
			message: "Anchor must have content and the content must be accessible by a screen reader.",
		},
		JSX_A11Y_LANG: (value: string, suggestions: Array<string>) => ({
			category: "lint/jsx-a11y/lang",
			message: `The <emphasis>lang</emphasis> attribute must have a valid value.`,
			advice: buildSuggestionAdvice(value, suggestions),
		}),
		JSX_A11Y_ALT_TEXT: {
			category: "lint/jsx-a11y/altText",
			message: "<emphasis>img</emphasis>, <emphasis>area</emphasis>, <emphasis>input type='image'</emphasis>, <emphasis>object</emphasis> must have alt text",
		},
		JSX_A11Y_HEADING_HAS_CONTENT: {
			category: "lint/jsx-a11y/headingHasContent",
			message: "Headings must have content and the content must be accessible by a screen reader.",
		},
		JSX_A11Y_HTML_HAS_LANG: {
			category: "lint/jsx-a11y/htmlHasLang",
			message: `<emphasis>html</emphasis> elements must have a <emphasis>lang prop</emphasis>.`,
		},
		JSX_A11Y_IFRAME_HAS_TITLE: {
			category: "lint/jsx-a11y/iframeHasTitle",
			message: `<emphasis>iframe</emphasis> elements should have a <emphasis>title prop</emphasis>.`,
		},
		JSX_A11Y_IMG_REDUNDANT_ALT: {
			category: "lint/jsx-a11y/imgRedundantAlt",
			message: `<emphasis>img</emphasis> element alt descriptions must not contain "image", "picture", or "photo"`,
		},
		JSX_A11Y_NO_ACCESS_KEY: {
			category: "lint/jsx-a11y/noAccessKey",
			message: "The <emphasis>accessKey</emphasis> prop is not allowed. Inconsistencies between keyboard shortcuts and keyboard comments used by screenreader and keyboard only users create a11y complications.",
		},
		JSX_A11Y_NO_AUTOFOCUS: {
			category: "lint/jsx-a11y/noAutofocus",
			message: "The <emphasis>autoFocus</emphasis> prop should not be used, as it can reduce usability and accessibility for users.",
		},
		JSX_A11Y_NO_DISTRACTING_ELEMENTS: (element: string) => ({
			category: "lint/jsx-a11y/noDistractingElements",
			message: `Do not use ${element} elements as they can create visual accessibility issues and are deprecated.`,
		}),
		JSX_A11Y_NO_ONCHANGE: {
			category: "lint/jsx-a11y/noOnchange",
			message: `<emphasis>onBlur</emphasis> should be used in favor of <emphasis>onChange</emphasis>. Only use <emphasis>onChange</emphasis> if absolutely necessary without negatively affecting keyboard only or screen reader users.`,
		},
		JSX_A11Y_NO_TARGET_BLANK: {
			category: "lint/jsx-a11y/noTargetBlank",
			message: `Using <emphasis>target="_blank"</emphasis> without <emphasis>rel="noreferrer"</emphasis> is a security risk.`,
		},
		JSX_A11Y_NO_SCOPE: {
			category: "lint/jsx-a11y/scope",
			message: "The <emphasis>scope</emphasis> prop can only be used on <emphasis>th</emphasis> elements.",
		},
		REACT_JSX_KEY: (origin: string) => ({
			category: "lint/react/jsxKey",
			message: `Missing the "key" prop for element in ${origin}`,
		}),
		REACT_JSX_NO_COMMENT_TEXT: {
			category: "lint/react/jsxNoCommentText",
			message: "Comments inside children should be placed in braces",
		},
		REACT_NO_CHILDREN_PROP: {
			category: "lint/react/noChildrenProp",
			message: "children should not be passed as a prop",
		},
		REACT_NO_DANGER: {
			category: "lint/react/noDanger",
			message: "dangerouslySetInnerHTML should be avoided",
		},
		REACT_NO_DANGER_WITH_CHILDREN: {
			category: "lint/react/noDangerWithChildren",
			message: "Only set one of <emphasis>children</emphasis> or <emphasis>props.dangerouslySetInnerHTML</emphasis>.",
		},
		REACT_NO_DID_UPDATE_SET_STATE: {
			category: "lint/react/noDidUpdateSetState",
			message: "Avoid this.setState in componentDidUpdate",
		},
		REACT_NO_FIND_DOM_NODE: {
			category: "lint/react/noFindDOMNode",
			message: "Do not use findDOMNode",
		},
		REACT_REACT_IN_JSX_SCOPE: {
			category: "lint/react/reactInJsxScope",
			message: `<emphasis>React</emphasis> must be in scope when using JSX`,
		},
		REACT_STYLE_PROP_OBJECT: {
			category: "lint/react/stylePropObject",
			message: "<emphasis>style</emphasis> property value must be an object.",
		},
		REACT_VOID_DOM_ELEMENTS_NO_CHILDREN: (
			element: string,
			properties: Array<string>,
		) => ({
			category: "lint/react/voidDomElementsNoChildren",
			message: markup`<emphasis>${element}</emphasis> is a void element tag and must not have <emphasis>${orJoin(
				properties,
			)}</emphasis>.`,
		}),
		JS_IMPORT_DEFAULT_BASENAME: (prev: string, basename: string) => ({
			category: "lint/js/importDefaultBasename",
			message: markup`When importing the default, use the basename <emphasis>${basename}</emphasis>`,
			advice: [
				{
					type: "log",
					category: "info",
					text: "If you really meant this then use this instead",
				},
				{
					type: "code",
					code: markup`import {default as ${prev}}`,
				},
			],
		}),
		JS_NO_COMMA_OPERATOR: {
			category: "lint/js/noCommaOperator",
			message: "Avoid usage of the comma operator. It can lead to easy mistakes and ambiguous code.",
			advice: [
				{
					type: "log",
					category: "info",
					text: "If you want multiple expressions then break it up.",
				},
			],
		},
		JS_NEGATION_ELSE: {
			category: "lint/js/negationElse",
			message: "Invert the blocks when you have a negation test",
		},
		JS_DUPLICATE_IMPORT_SOURCE: (seenLocation: DiagnosticLocation) => ({
			category: "lint/js/duplicateImportSource",
			message: "This module has already been imported",
			advice: [
				{
					type: "log",
					category: "info",
					text: "Previously imported here",
				},
				{
					type: "frame",
					location: seenLocation,
				},
			],
		}),
		JS_PREFER_BLOCK_STATEMENT: {
			category: "lint/js/preferBlockStatements",
			message: "Block statements are preferred in this position",
		},
		JS_PREFER_TEMPLATE: {
			category: "lint/js/preferTemplate",
			message: "Template literals are preferred over string concatenation",
		},
		JS_PREFER_WHILE: {
			category: "lint/js/preferWhile",
			message: "A while loop should be used over a for loop",
		},

		JS_UNSAFE_NEGATION: {
			category: "lint/js/unsafeNegation",
			message: "Unsafe usage of negation operator in left side of binary expression",
		},
		JS_UNUSED_VARIABLES: (kind: string, name: string) => ({
			category: "lint/js/unusedVariables",
			message: markup`Unused ${kind} <emphasis>${name}</emphasis>`,
		}),
		JS_UNDECLARED_VARIABLES: (name: string) => ({
			category: "lint/js/undeclaredVariables",
			message: markup`Undeclared variable <emphasis>${name}</emphasis>`,
		}),
		JS_VARIABLE_CAMEL_CASE: (name: string, camelCaseName: string) => ({
			category: "lint/js/camelCase",
			message: markup`Variable <emphasis>${name}</emphasis> should be camel cased as <emphasis>${camelCaseName}</emphasis>`,
		}),
		JS_IDENTIFIER_CAMEL_CASE: (name: string, camelCaseName: string) => ({
			category: "lint/js/camelCase",
			message: markup`Identifier <emphasis>${name}</emphasis> should be camel cased as <emphasis>${camelCaseName}</emphasis>`,
		}),
		JS_CASE_SINGLE_STATEMENT: {
			category: "lint/js/caseSingleStatement",
			message: "A switch case should only have a single statement. If you want more then wrap it in a block.",
		},
		JS_CONFUSING_LANGUAGE: (
			description: string,
			word: string,
			suggestion: string,
			advice: DiagnosticAdvice,
		) => ({
			category: "lint/js/confusingLanguage",
			message: description,
			advice: [
				...advice,
				{
					type: "log",
					category: "info",
					text: markup`Consider using <emphasis>${suggestion}</emphasis> instead`,
				},
			],
		}),
		JS_DOUBLE_EQUALS: {
			category: "lint/js/doubleEquals",
			message: "Use === instead of ==",
			advice: [
				{
					type: "log",
					category: "info",
					text: "== is only allowed when comparing against null",
				},
			],
		},
		JS_EMPTY_MATCHES: {
			category: "lint/js/emptyMatches",
			message: "The expression can return empty matches, and may match infinitely in some use cases",
		},
		JS_NEGATE_DOUBLE_EQUALS: {
			category: "lint/js/doubleEquals",
			message: "Use !== instead of !=",
			advice: [
				{
					type: "log",
					category: "info",
					text: "!= is only allowed when comparing against null",
				},
			],
		},
		JS_NO_CATCH_ASSIGN: {
			category: "lint/js/noCatchAssign",
			message: "Don't reassign catch parameters",
		},
		JS_SPARSE_ARRAY: {
			category: "lint/js/sparseArray",
			message: "Your array contains an empty slot",
		},
		JS_SINGLE_VAR_DECLARATOR: {
			category: "lint/js/singleVarDeclarator",
			message: "Declare each variable separately",
		},
		JS_PREFER_FUNCTION_DECLARATIONS: {
			category: "lint/js/preferFunctionDeclarations",
			message: "Use a function declaration instead of a const function",
		},
		JS_NO_VAR: {
			category: "lint/js/noVar",
			message: "Variable declarations using `var` are disallowed, use `let` or `const` instead.",
		},
		JS_NO_SHORTHAND_ARRAY_TYPE: {
			category: "lint/js/noShorthandArrayType",
			message: escapeMarkup("Use Array<T> instead of shorthand T[]"),
		},
		JS_NO_UNSAFE_FINALLY: (type: string) => ({
			category: "lint/js/noUnsafeFinally",
			message: markup`Unsafe usage of ${type}.`,
		}),
		JS_NO_TEMPLATE_CURLY_IN_STRING: {
			category: "lint/js/noTemplateCurlyInString",
			message: `Unexpected template string expression.`,
		},
		JS_NO_SHADOW_RESTRICTED_NAMES: (name: string) => ({
			category: "lint/js/noShadowRestrictedNames",
			message: markup`Shadowing of global property <emphasis>${name}</emphasis>`,
			advice: [
				{
					type: "log",
					category: "info",
					text: "Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.",
				},
			],
		}),
		JS_NO_MULTIPLE_SPACES_IN_REGEX_LITERAL: (count: number) => ({
			category: "lint/js/noMultipleSpacesInRegularExpressionLiterals",
			message: "Unclear multiple spaces in regular expression",
			advice: [
				{
					type: "log",
					category: "info",
					text: `It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {${String(
						count,
					)}}/`,
				},
			],
		}),
		JS_NO_LABEL_VAR: {
			category: "lint/js/noLabelVar",
			message: "Labels should not be variable names",
		},
		JS_NO_IMPORT_ASSIGN: (name: string) => ({
			category: "lint/js/noImportAssign",
			message: markup`<emphasis>${name}</emphasis> is read-only`,
		}),
		JS_NO_EXTRA_BOOLEAN_CAST: {
			category: "lint/js/noExtraBooleanCast",
			message: `Redundant double negation.`,
		},
		JS_NO_FUNCTION_ASSIGN: {
			category: "lint/js/noFunctionAssign",
			message: "Reassignment of function declaration",
		},
		JS_NO_EMPTY_CHAR_SET: {
			category: "lint/js/noEmptyCharacterClass",
			message: "Empty character classes in regular expressions are not allowed",
		},
		JS_NO_DUPLICATE_KEYS: (key: string) => ({
			category: "lint/js/noDuplicateKeys",
			message: markup`Duplicate key <emphasis>${key}</emphasis>`,
		}),
		JS_NO_POSIX_IN_REGULAR_EXPRESSION: {
			category: "lint/js/noPosixInRegularExpression",
			message: "POSIX Character Classes and Collating Sequences are not supported in ECMAscript Regular Expressions",
		},
		JS_NO_DUPLICATE_CASE: (value: string) => ({
			category: "lint/js/noDuplicateCase",
			message: markup`Duplicate case <emphasis>${value}</emphasis> not allowed.`,
		}),
		JS_NO_DUPE_ARGS: (name: string) => ({
			category: "lint/js/noDupeArgs",
			message: markup`Duplicate argument <emphasis>${name}</emphasis> in function definition`,
		}),
		JS_NO_DELETE: {
			category: "lint/js/noDelete",
			message: `Unexpected 'delete' operator.`,
		},
		JS_NO_DELETE_VARS: {
			category: "lint/js/noDeleteVars",
			message: "Variables should not be deleted.",
		},
		JS_NO_DEBUGGER: {
			category: "lint/js/noDebugger",
			message: "Unexpected 'debugger' statement",
		},
		JS_NO_COND_ASSIGN: {
			category: "lint/js/noCondAssign",
			message: "Cannot assign variable in loop condition",
		},
		JS_NO_COMPARE_NEG_ZERO: (op: string) => ({
			category: "lint/js/noCompareNegZero",
			message: `Do not use the '${op}' operator to compare against -0`,
			fixable: op === "===",
		}),
		JS_NO_ASYNC_PROMISE_EXECUTOR: {
			category: "lint/js/noAsyncPromiseExecutor",
			message: "Promise executor functions should not be async.",
		},
		JS_GETTER_RETURN: (got: string) => ({
			category: "lint/js/getterReturn",
			message: `Expected a 'return' at end of a getter method but got ${got}`,
		}),
		JS_NO_SETTER_RETURN: {
			category: "lint/js/noSetterReturn",
			message: `Setter cannot return a value`,
		},
		JS_EMPTY_BLOCKS: {
			category: "lint/js/emptyBlocks",
			message: "Empty block",
		},
		JS_NO_ARGUMENTS: {
			category: "lint/js/noArguments",
			message: "Use the rest parameters instead of 'arguments'",
		},
		JS_DUPLICATE_REGEX_GROUP_NAME: (name: string) => ({
			category: "lint/js/noDuplicateGroupNamesInRegularExpressions",
			message: markup`Duplicate group name <emphasis>${name}</emphasis> in regular expression`,
		}),
		JS_NO_REFERENCE_TO_NON_EXISTING_GROUP: (name: string) => ({
			category: "lint/js/noReferenceToNonExistingGroup",
			message: markup`Reference to non-existent group <emphasis>"${name}"</emphasis>`,
		}),
		JS_DEFAULT_EXPORT_SAME_BASENAME: (
			{
				defaultName,
				defaultType,
				actualFilename,
				correctFilename,
			}: {
				defaultName: string;
				defaultType: string;
				actualFilename: string;
				correctFilename: string;
			},
		) => {
			let adviceMessage = "";

			if (defaultName === "*default*") {
				adviceMessage += "The";
			} else {
				adviceMessage += `Filename should be <emphasis>${correctFilename}</emphasis> or the`;
			}

			adviceMessage += ` ${defaultType} name should be <emphasis>${actualFilename}</emphasis>`;

			return {
				category: "lint/js/defaultExportSameBasename",
				message: `Filename and the name of a default ${defaultType} should match`,
				advice: [
					{
						type: "log",
						category: "info",
						text: adviceMessage,
					},
				],
			};
		},
		JS_RESTRICTED_GLOBALS: (globalName) => ({
			category: "lint/js/restrictedGlobals",
			message: markup`The use of the existing global variable <emphasis>${globalName}</emphasis> is not allowed. Use local variable instead.`,
		}),
		JS_SORT_EXPORT_SPECIFIERS: {
			category: "lint/js/sortImportExportSpecifiers",
			message: `Specifiers of the export declaration should be sorted alphabetically.`,
		},
		JS_SORT_IMPORT_SPECIFIERS: {
			category: "lint/js/sortImportExportSpecifiers",
			message: `Specifiers of the import declaration should be sorted alphabetically.`,
		},
		PENDING_FIXES: (
			relativeFilename: string,
			original: string,
			formatted: string,
		) => ({
			category: "lint/pendingFixes",
			message: "Pending formatting and recommended autofixes",
			advice: [
				{
					type: "diff",
					diff: stringDiff(original, formatted),
				},
				({
					type: "action",
					command: "lint",
					shortcut: "f",
					instruction: "To apply fixes and formatting run",
					noun: "Apply fixes and format",
					args: [relativeFilename],
					commandFlags: {
						save: true,
					},
				} as DiagnosticAdviceAction),
				({
					type: "action",
					hidden: true,
					command: "lint",
					shortcut: "o",
					instruction: "To format this file without any fixes run",
					noun: "Only format",
					args: [relativeFilename],
					commandFlags: {
						format: true,
					},
				} as DiagnosticAdviceAction),
			],
		}),
		TS_NO_EXPLICIT_ANY: {
			category: "lint/ts/noExplicitAny",
			message: "Unexpected any. Specify a different type.",
		},
	},
	PROJECT_MANAGER: {
		NO_VCS: (rootConfigLocation: undefined | DiagnosticLocation) => ({
			category: "projectManager/vscMissing",
			message: "Can't find any version control for this project",
			advice: rootConfigLocation === undefined
				? [
						{
							type: "log",
							category: "info",
							text: "Version control root was set to the project root as it was not configured. To configure a different folder run",
						},
						{
							type: "command",
							command: "rome config set-directory vcs.root DIRECTORY_HERE",
						},
					]
				: [
						{
							type: "log",
							category: "info",
							text: "Version control root was set here",
						},
						{
							type: "frame",
							location: rootConfigLocation,
						},
					],
		}),
		DUPLICATE_PACKAGE: (packageName: string, existing: string) => ({
			category: "projectManager/nameCollision",
			message: markup`Duplicate package name <emphasis>${packageName}</emphasis>`,
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`Defined already by <filelink target="${existing}" />`,
				},
			],
		}),
		NOT_FOUND: {
			category: "projectManager/missing",
			message: `Couldn't find a project`,
			advice: [
				{
					type: "log",
					category: "info",
					text: "Run <command>rome init</command> in this folder to initialize a project",
				},
			],
		},
		INCORRECT_CONFIG_FILENAME: (validFilenames: Array<string>) => ({
			category: "projectManager/incorrectConfigFilename",
			message: markup`Invalid rome config filename, <emphasis>${validFilenames.join(
				" or ",
			)}</emphasis> are the only valid filename`,
		}),
	},
	FORMAT: {
		DISABLED: {
			category: "format/disabled",
			message: "Format is disabled for this project",
			// TODO advice and better error message
		},
	},
	// @romejs/js-compiler
	COMPILER: {
		CLASSES_UNSUPPORTED: {
			category: "compile/classes",
			message: "The classes transform doesn't know how to transform this",
		},
		JSX_NOT_XML: {
			category: "compile/jsx",
			message: "JSX is not XML",
		},
	},
	// @romejs/string-escape
	STRING_ESCAPE: {
		NOT_ENOUGH_CODE_POINTS: "Not enough code point digits",
		INVALID_STRING_CHARACTER: "Invalid string character (U+0000 to U+001F)",
		INVALID_HEX_DIGIT_FOR_ESCAPE: "Invalid hex digit for unicode escape",
	},
	ANALYZE_DEPENDENCIES: {
		CJS_EXPORT_IN_ES: {
			category: "analyzeDependencies/cjsExportInES",
			message: "You cannot use CommonJS exports in an ES module",
		},
	},
	// @romejs/string-markup
	STRING_MARKUP: {
		UNCLOSED_STRING: "Unclosed string",
		EXPECTED_CLOSING_TAG_NAME: "Expected closing tag name",
		UNKNOWN_START: "Unknown child start",
		EXPECTED_ATTRIBUTE_NAME: "Expected attribute name",
		INCORRECT_CLOSING_TAG_NAME: (expected: string, got: string) => ({
			message: markup`Expected to close ${expected} but found ${got}`,
		}),
		UNCLOSED_TAG: (tagName: string, openLocation: DiagnosticLocation) => ({
			message: markup`Unclosed ${tagName} tag`,
			advice: [
				{type: "log", category: "info", text: "Tag started here"},
				{
					type: "frame",
					location: openLocation,
				},
			],
		}),
		INVALID_ATTRIBUTE_NAME_FOR_TAG: (
			tagName: string,
			attributeName: string,
			validAttributes: Array<string>,
		) => ({
			message: markup`<emphasis>${attributeName}</emphasis> is not a valid attribute name for <emphasis>${tagName}</emphasis>`,
			advice: buildSuggestionAdvice(attributeName, validAttributes),
		}),
		UNKNOWN_TAG_NAME: (tagName: string) => ({
			message: markup`Unknown tag name <emphasis>${tagName}</emphasis>`,
		}),
		RESTRICTED_CHILD: (
			tagName: string,
			allowedParents: Array<string>,
			gotParentName: string = "none",
		) => ({
			message: markup`The tag <emphasis>${tagName}</emphasis> should only appear as a child of ${orJoin(
				addEmphasis(allowedParents),
			)} not <emphasis>${gotParentName}</emphasis>`,
		}),
		RESTRICTED_PARENT: (
			tagName: string,
			allowedChildren: Array<string>,
			gotChildName: string,
		) => ({
			message: markup`The tag <emphasis>${tagName}</emphasis> should only contain the tags ${orJoin(
				addEmphasis(allowedChildren),
			)} not <emphasis>${gotChildName}</emphasis>`,
		}),
		RESTRICTED_PARENT_TEXT: (tagName: string) => ({
			message: markup`The tag <emphasis>${tagName}</emphasis> should not contain any text`,
		}),
	},
	// @romejs/path-match
	PATH_MATCH: {
		INVALID_PATTERN_SEGMENT_PART: "Invalid pattern segment part",
		INVALID_PATH_SEGMENT: "Invalid path segment",
	},
	TESTS: {
		CANCELLED: {
			category: "tests/cancelled",
			message: "Test was cancelled",
		},
		UNDECLARED: {
			message: "No tests declared in this file",
			category: "tests/noneDeclared",
		},
		LOGS: (advice: DiagnosticAdvice) => ({
			message: "Test file produced console logs",
			category: "tests/logs",
			advice: [
				...advice,
				{
					type: "log",
					category: "info",
					text: "Only visible when this test file contains failures",
				},
			],
		}),
	},
	SUPPRESSIONS: {
		UNUSED: (suppression: DiagnosticSuppression) => {
			let description = "";
			if (suppression.startLine === suppression.endLine) {
				description = `line ${suppression.startLine}`;
			} else {
				description += `lines ${suppression.startLine} to ${suppression.endLine}`;
			}

			return {
				message: "Unused suppression. Did not hide any errors.",
				category: "suppressions/unused",
				advice: [
					{
						type: "log",
						category: "info",
						text: `This suppression should hide <emphasis>${description}</emphasis>`,
					},
				],
			};
		},
		MISSING_SPACE: {
			category: "suppressions/missingSpace",
			message: "Missing space between prefix and suppression categories",
		},
		MISSING_TARGET: {
			category: "suppressions/missingTarget",
			message: "We could not find a target for this suppression",
		},
		DUPLICATE: (category: string) => ({
			category: "suppressions/duplicate",
			message: markup`Duplicate suppression category <emphasis>${category}</emphasis>`,
		}),
	},
	SNAPSHOTS: {
		MISSING_NEWLINE_AFTER_CODE_BLOCK: "Newline required after code block",
		MISSING_NEWLINE_BEFORE_CODE_BLOCK: "Newline required before code block end",
		UNCLOSED_CODE_BLOCK: "Unclosed code block",
		EXPECTED_CODE_BLOCK_AFTER_HEADING: "Expected a code block after this heading",
		REDUNDANT: {
			category: "tests/snapshots/redundant",
			message: "Snapshot should not exist",
		},
		MISSING: {
			category: "tests/snapshots/missing",
			message: "Snapshot does not exist",
		},
		INCORRECT: (expected: string, got: string) => ({
			category: "tests/snapshots/incorrect",
			message: "Snapshots do not match",
			advice: [
				{
					type: "diff",
					diff: stringDiff(expected, got),
				},
			],
		}),
		INLINE_COLLISION: {
			category: "tests/snapshots/inlineCollision",
			message: "Trying to update this inline snapshot multiple times",
			advice: [
				{
					type: "log",
					category: "info",
					text: "<emphasis>t.inlineSnapshot</emphasis> can only be called once. Did you call it in a loop?",
				},
			],
		},
		INLINE_MISSING_RECEIVED: {
			category: "tests/snapshots/inlineMissingReceived",
			message: "This inline snapshot call does not have a received argument",
		},
		INLINE_FROZEN: {
			category: "tests/snapshots/frozen",
			message: "Inline snapshot cannot be updated as snapshots are frozen",
		},
		FROZEN: {
			category: "tests/snapshots/frozen",
			message: "Snapshot cannot be updated as snapshots are frozen",
		},
		INLINE_BAD_MATCH: {
			category: "tests/snapshots/incorrect",
			message: "Inline snapshots do not match",
		},
	},
	BUNDLER: {
		TOP_LEVEL_AWAIT_IN_LEGACY: {
			category: "bundler/topLevelAwait",
			message: "This module contains a top level await which isn't supported in wrapper mode",
		},
		DETECTED_CYCLE: (
			localName: string,
			target: string,
			culprit: string,
			path: Array<string>,
		) => {
			function formatPart(part: string, index?: number): string {
				const tagged = `<filelink target="${part}" />`;
				if (part === culprit) {
					return `<highlight i="0" legend>${tagged}</highlight>`;
				} else if (part === target) {
					return `<highlight i="1" legend>${tagged}</highlight>`;
				} else if (index === 0) {
					return `${tagged} <inverse>ENTRY</inverse>`;
				} else {
					return tagged;
				}
			}

			return {
				category: "bundler/moduleCycle",
				message: `The variable <emphasis>${localName}</emphasis> won't be initialized yet`,
				advice: [
					{
						type: "log",
						category: "info",
						text: "This is because the module it belongs to wont be executed yet. This is due to a circular dependency creating a module cycle.",
					},
					{
						type: "log",
						category: "info",
						text: `The likely cause is the file ${formatPart(culprit)} that was required by ${formatPart(
							target,
						)} which created a circular dependency:`,
					},
					{
						type: "list",
						reverse: true,
						ordered: true,
						list: path.map(formatPart),
					},
				],
			};
		},
	},
	RESOLVER: {
		NOT_FOUND: (
			responseType: ResolverQueryResponseNotFound["type"],
			source: string,
			location: DiagnosticLocation,
		) => {
			let messagePrefix = "";
			let category: DiagnosticCategory = "resolver/notFound";

			switch (responseType) {
				case "UNSUPPORTED": {
					messagePrefix = `Unsupported`;
					category = "resolver/unsupported";
					break;
				}
				case "MISSING": {
					messagePrefix = `Cannot find`;
					break;
				}
				case "FETCH_ERROR": {
					messagePrefix = "Failed to fetch";
					category = "resolver/fetchFailed";
					break;
				}
			}

			return {
				message: messagePrefix +
				markup` <emphasis>${source}</emphasis> from <filelink emphasis target="${location.filename}" />`,
				category,
			};
		},
		IMPORT_TYPE_MISMATCH: (
			exportName: string,
			source: string,
			importedAsKing: string,
			actualKind: string,
			exportLoc: undefined | SourceLocation,
		) => ({
			category: "resolver/importTypeMismatch",
			message: `The export <emphasis>${exportName}</emphasis> in <filelink emphasis target="${source}" /> was incorrectly imported as a <emphasis>${importedAsKing}</emphasis> when it's actually a <emphasis>${actualKind}</emphasis>`,
			advice: exportLoc && [
				{
					type: "log",
					category: "info",
					text: `Export was defined here in <filelink emphasis target="${exportLoc.filename}" />`,
				},
				{
					type: "frame",
					location: exportLoc,
				},
			],
		}),
		UNKNOWN_EXPORT: (
			name: string,
			source: string,
			exportedNames: Array<string>,
			formatExportedName: (
				name: string,
			) => {
				location: undefined | DiagnosticLocation;
				source: undefined | string;
			},
		) => ({
			message: `Couldn't find export <emphasis>${name}</emphasis> in <filelink emphasis target="${source}" />`,
			category: "resolver/unknownExport",
			advice: exportedNames.length === 0
				? [
						{
							type: "log",
							category: "info",
							text: "This file doesn't have any exports",
						},
					]
				: buildSuggestionAdvice(
						name,
						exportedNames,
						{
							formatItem: (name) => {
								const {location, source} = formatExportedName(name);

								if (location !== undefined) {
									if (location.start === undefined) {
										name = markup`<filelink target="${location.filename}">${name}</filelink>`;
									} else {
										name = markup`<filelink target="${location.filename}" line="${location.start.line}" column="${location.start.column}">${name}</filelink>`;
									}
								}

								if (source !== undefined) {
									name += markup` <dim>(from <filelink target="${source}" />)</dim>`;
								}

								return name;
							},
						},
					),
		}),
		UNKNOWN_EXPORT_POSSIBLE_UNEXPORTED_LOCAL: (
			name: string,
			source: string,
			location: SourceLocation,
		) => ({
			message: markup`Couldn't find export <emphasis>${name}</emphasis> in <filelink emphasis target="${source}" />`,
			category: "resolver/unknownExport",
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`However we found a matching local variable in <filelink emphasis target="${location.filename}" />. Did you forget to export it?`,
				},
				{
					type: "frame",
					location,
				},
			],
		}),
	},
	SPDX: {
		UNKNOWN_LICENSE: (id: string, knownLicenses: Array<string>) => ({
			message: markup`Unknown SPDX license <emphasis>${id}</emphasis>`,
			advice: buildSuggestionAdvice(id, knownLicenses),
		}),
		VALID_LICENSE_WITH_MISSING_DASH: (possibleCorrectLicense: string) => ({
			message: `Missing dash between SPDX license name and version`,
			advice: [
				{
					type: "log",
					category: "info",
					text: `Did you mean <emphasis>${possibleCorrectLicense}</emphasis>?`,
				},
			],
		}),
		WITH_RIGHT_LICENSE_ONLY: "Only a license id can be on the right side of a WITH",
		OPERATOR_NOT_BETWEEN_EXPRESSION: "Can only use AND/OR in between an expression",
		PLUS_NOT_AFTER_LICENSE: "A plus can only come after a license id",
		UNOPENED_PAREN: "Nothing open to close",
	},
	// @romejs/js-parser
	JS_PARSER: {
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
		UNWRAPPED_ADJACENT_JHX: `Adjacent JSX elements must be wrapped in an enclosing tag. Did you want a JSX fragment <>...</>?`,
		CONFUSED_OR: "Unexpected ||, did you mean just |?",
		INVALID_ASSIGNMENT_TARGET: "Not a valid assignment target",
		IMPORT_KIND_SPECIFIER_ON_IMPORT_DECLARATION_WITH_KIND: "The `type` and `typeof` keywords on named imports can only be used on regular `import` statements. It cannot be used with `import type` or `import typeof` statements",
		DESTRUCTURING_IN_IMPORT: "ES2015 named imports do not destructure. Use another statement for destructuring after the import.",
		IMPORT_TYPE_STAR: "import * is not allowed",
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
		JSX_EXPECTED_CLOSING_FRAGMENT_TAG: (
			name: string,
			openingLoc: SourceLocation,
		) => ({
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
	},
	// @romejs/js-analysis
	TYPE_CHECK: {
		NOT_CALLABLE: {
			category: "typeCheck/uncallable",
			message: `This type isn't callable`,
		},
		INCOMPATIBILITY: (upper: string, originLoc: undefined | SourceLocation) => ({
			category: "typeCheck/incompatible",
			message: "Type incompatibility found",
			advice: [
				{
					type: "log",
					category: "error",
					text: `This type is incompatible with expected type of`,
				},
				originLoc === undefined
					? {
							type: "log",
							category: "info",
							text: upper,
						}
					: {
							type: "frame",
							location: {
								...originLoc,
								marker: upper,
							},
						},
			],
		}),
		UNKNOWN_IMPORT: (
			importedName: string,
			source: string,
			possibleNames: Array<string>,
		) => ({
			category: "typeCheck/unknownImport",
			message: `Unknown import '${importedName}' in '${source}'`,
			advice: buildSuggestionAdvice(importedName, possibleNames),
		}),
		UNKNOWN_PROP: (key: string, possibleNames: Array<string>) => ({
			message: markup`Property ${key} not found in`,
			category: "typeCheck/unknownProperty",
			advice: buildSuggestionAdvice(key, possibleNames),
		}),
		UNDECLARED_VARIABLE: (name: string, possibleNames: Array<string>) => ({
			category: "typeCheck/undeclaredVariable",
			message: markup`Undeclared variable ${name}`,
			advice: buildSuggestionAdvice(name, possibleNames),
		}),
		NOT_EXHAUSTIVE: (only: string, target: string) => ({
			category: "typeCheck/notExhaustive",
			//message += `but allows ${this.extraenous.map(type => this.utils.humanize(type)).join(' | ')}`;
			message: `Expected only a ${only} but got ${target}`,
		}),
		MISSING_CONDITION: (missing: Array<string>) => ({
			category: "typeCheck/missingCondition",
			message: `Missing the conditions ${missing.join(", ")}`,
		}),
	},
	// @romejs/consume
	CONSUME: {
		SET_PROPERTY_NON_OBJECT: "Attempted to set a property on a non-object",
		EXPECTED_JSON_VALUE: "Expected a JSON value",
		EXPECTED_OBJECT: "Expected object",
		EXPECTED_ARRAY: "Expected array",
		EXPECTED_DATE: "Expected a date",
		EXPECTED_BOOLEAN: "Expected a boolean",
		EXPECTED_STRING: "Expected a string",
		EXPECTED_BIGINT: "Expected a bigint",
		EXPECTED_NUMBER: "Expected a number",
		EXPECTED_URL: "Expected a URL",
		EXPECTED_VALID_NUMBER: "Expected valid number",
		EXPECTED_ABSOLUTE_PATH: "Expected an absolute file path",
		EXPECTED_RELATIVE_PATH: "Expected a relative file path",
		EXPECTED_EXPLICIT_RELATIVE_PATH: "Expected an explicit relative file path. This is one that starts with <emphasis>./</emphasis> or <emphasis>../</emphasis>",
		INVALID: "Invalid value",
		EXPECTED_NUMBER_BETWEEN: (min: UnknownNumber, max: UnknownNumber) => ({
			message: `Expected number between ${min} and ${max}`,
		}),
		EXPECTED_NUMBER_HIGHER: (num: UnknownNumber) => ({
			message: `Expected number higher than ${num}`,
		}),
		EXPECTED_NUMBER_LOWER: (num: UnknownNumber) => ({
			message: `Expected number lower than ${num}`,
		}),
		INVALID_STRING_SET_VALUE: (value: string, validValues: Array<string>) => ({
			message: markup`Invalid value <emphasis>${value}</emphasis>`,
			advice: [
				{
					type: "log",
					category: "info",
					text: "Possible values are",
				},
				{
					type: "list",
					list: validValues.map((str) => escapeMarkup(str)),
				},
			],
		}),
		UNUSED_PROPERTY: (key: string, type: string, knownProperties: Array<string>) => ({
			message: markup`Unknown <emphasis>${key}</emphasis> ${type}`,
			advice: buildSuggestionAdvice(
				key,
				knownProperties,
				{
					ignoreCase: true,
				},
			),
		}),
	},
	// @romejs/codec-js-manifest
	MANIFEST: {
		TOO_MANY_HASH_PARTS: "Too many hashes",
		MISSING_HOSTED_GIT_USER: "Missing user",
		MISSING_HOSTED_GIT_REPO: "Missing repo",
		TOO_MANY_HOSTED_GIT_PARTS: "Expected only 2 parts",
		EMPTY_NPM_PATTERN: "Missing rest of npm dependency pattern",
		TOO_MANY_NPM_PARTS: "Too many @ signs",
		STRING_BIN_WITHOUT_NAME: "A string bin is only allowed if the manifest has a name property",
		MISSING_REPO_URL: "Missing repo URL",
		MIXED_EXPORTS_PATHS: "Cannot mix a root conditional export with relative paths",
		NAME_EXCEEDS: `cannot exceed 214 characters`,
		INVALID_NAME_START: `cannot start with a dot or underscore`,
		ORG_WITH_NO_PACKAGE_NAME: `contains an org but no package name`,
		ORG_TOO_MANY_PARTS: `contains too many name separators`,
		REDUNDANT_ORG_NAME_START: "Redundant <emphasis>@</emphasis> in org name",
		INVALID_NAME_CHAR: (char: string) => ({
			message: markup`The character <emphasis>${char}</emphasis> isn't allowed`,
		}),
		INCORRECT_CASING: (typoKey: string, correctKey: string) => ({
			message: `${typoKey} has incorrect casing, should be ${correctKey}`,
		}),
		INCORRECT_CAMEL_CASING: (typoKey: string, correctKey: string) => ({
			message: `${typoKey} isn't correctly camel cased when it should be ${correctKey}`,
		}),
		TYPO: (typoKey: string, correctKey: string) => ({
			message: `${typoKey} is a typo of ${correctKey}`,
		}),
	},
	// @romejs/project
	PROJECT_CONFIG: {
		BOOLEAN_CATEGORY: (enabled: boolean) => ({
			message: `Expected an object here but got a boolean`,
			advice: [
				{
					type: "log",
					category: "info",
					text: `You likely wanted \`{"enabled": ${String(enabled)}}\` instead`,
				},
			],
		}),
		RECURSIVE_CONFIG: "Recursive config",
	},
});
