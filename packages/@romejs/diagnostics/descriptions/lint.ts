/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticAdvice,
	DiagnosticAdviceAction,
	DiagnosticLocation,
} from "../types";
import {escapeMarkup, markup} from "@romejs/string-markup";
import stringDiff from "@romejs/string-diff";
import {buildSuggestionAdvice} from "../helpers";
import {createDiagnosticsCategory, orJoin} from "./index";

export const lint = createDiagnosticsCategory({
	REACT_JSX_NO_DUPLICATE_PROPS: (key: string) => ({
		category: "lint/react/jsxNoDuplicateProps",
		message: `React does not support duplicate props. The <emphasis>${key}</emphasis> prop is duplicated.`,
	}),
	REACT_NO_STRING_REFS: (details: string) => ({
		category: "lint/react/noStringRefs",
		message: `Using ${details} is a deprecated pattern.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: 'See <hyperlink target="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs" /> for more information',
			},
		],
	}),
	REACT_JSX_FRAGMENTS: {
		category: "lint/react/jsxFragments",
		message: "Use shorthand syntax for <emphasis>Fragment</emphasis> elements instead of standard syntax.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Shorthand fragment syntax saves keystrokes and is only unapplicable when keys are required.",
			},
		],
	},
	REACT_NO_REDUNDANT_SHOULD_COMPONENT_UPDATE: {
		category: "lint/react/noRedundantShouldComponentUpdate",
		message: "Do not implement <emphasis>shouldComponentUpdate</emphasis> when extending <emphasis>React.PureComponent</emphasis>.",
	},
	REACT_NO_UNSAFE: (oldMethod: string, newMethod: string, details: string) => ({
		category: "lint/react/noUnsafe",
		message: `<emphasis>${oldMethod}</emphasis> is unsafe for use in async rendering. Update the component to use ${newMethod} instead. ${details}`,
	}),
	REACT_NO_DID_MOUNT_SET_STATE: {
		category: "lint/react/noDidMountSetState",
		message: "Avoid <emphasis>this.setState</emphasis> in <emphasis>componentDidMount</emphasis>. This can cause an unexpected second render, which can cause visual layout thrashing.",
	},
	REACT_BUTTON_HAS_TYPE: {
		category: "lint/react/buttonHasType",
		message: `Use an explicit <emphasis>type</emphasis> prop on <emphasis>${escapeMarkup(
			"<button>",
		)}</emphasis> elements.`,
	},
	JSX_A11Y_TABINDEX_NO_POSITIVE: {
		category: "lint/jsx-a11y/tabindexNoPositive",
		message: "Avoid positive integer values for the <emphasis>tabIndex</emphasis> attribute.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Elements with a positive tab index override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.",
			},
		],
	},
	JSX_A11Y_MOUSE_EVENTS_HAVE_KEY_EVENTS: (
		mouseEvent: string,
		keyboardEvent: string,
	) => ({
		category: "lint/jsx-a11y/mouseEventsHaveKeyEvents",
		message: `Pair the <emphasis>${mouseEvent}</emphasis> mouse event with the <emphasis>${keyboardEvent}</emphasis> keyboard event.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.",
			},
		],
	}),
	JSX_A11Y_MEDIA_HAS_CAPTION: {
		category: "lint/jsx-a11y/mediaHasCaption",
		message: "Provide a <emphasis>track</emphasis> for captions when using <emphasis>audio</emphasis> or <emphasis>video</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.",
			},
		],
	},
	REACT_NO_WILL_UPDATE_SET_STATE: {
		category: "lint/react/noWillUpdateSetState",
		message: "Avoid <emphasis>this.setState</emphasis> in <emphasis>componentWillUpdate</emphasis>",
	},
	JSX_A11Y_ARIA_UNSUPPORTED_ELEMENTS: {
		category: "lint/jsx-a11y/ariaUnsupportedElements",
		message: "Avoid the <emphasis>role</emphasis> attribute and <emphasis>aria-*</emphasis> attributes when using <emphasis>meta</emphasis>, <emphasis>html</emphasis>, <emphasis>script</emphasis>, and <emphasis>style</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Using roles on elements that do not support them can cause issues with screen readers.",
			},
		],
	},
	JSX_A11Y_ANCHOR_HAS_CONTENT: {
		category: "lint/jsx-a11y/anchorHasContent",
		message: "Provide screen reader accessible content when using <emphasis>anchor</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "All links on a page should have content that is accessible to screen readers.",
			},
		],
	},
	JSX_A11Y_LANG: (value: string, suggestions: Array<string>) => ({
		category: "lint/jsx-a11y/lang",
		message: "Provide a valid value for the <emphasis>lang</emphasis> attribute.",
		advice: buildSuggestionAdvice(value, suggestions),
	}),
	JSX_A11Y_ALT_TEXT: {
		category: "lint/jsx-a11y/altText",
		message: "Provide <emphasis>alt</emphasis> text when using <emphasis>img</emphasis>, <emphasis>area</emphasis>, <emphasis>input type='image'</emphasis>, and <emphasis>object</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Meaningful alternative text on elements that require it helps users relying on screen readers to understand content's purpose within a page.",
			},
		],
	},
	JSX_A11Y_HEADING_HAS_CONTENT: {
		category: "lint/jsx-a11y/headingHasContent",
		message: "Provide screen reader accessible content when using <emphasis>heading</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "All headings on a page should have content that is accessible to screen readers.",
			},
		],
	},
	JSX_A11Y_HTML_HAS_LANG: {
		category: "lint/jsx-a11y/htmlHasLang",
		message: "Provide a <emphasis>lang</emphasis> attribute when using the <emphasis>html</emphasis> element.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Setting a lang attribute on HTML elements configures the language used by screen readers when no user default is specified.",
			},
		],
	},
	JSX_A11Y_IFRAME_HAS_TITLE: {
		category: "lint/jsx-a11y/iframeHasTitle",
		message: "Provide a <emphasis>title</emphasis> attribute when using <emphasis>iframe</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Screen readers rely on the title set on an iframe to describe the content being displayed.",
			},
		],
	},
	JSX_A11Y_IMG_REDUNDANT_ALT: {
		category: "lint/jsx-a11y/imgRedundantAlt",
		message: 'Avoid the words "image", "picture", or "photo" in <emphasis>img</emphasis> element alt text.',
		advice: [
			{
				type: "log",
				category: "info",
				text: 'Screen readers announce img elements as "images", so it is not necessary to redeclare this in alternative text.',
			},
		],
	},
	JSX_A11Y_NO_ACCESS_KEY: {
		category: "lint/jsx-a11y/noAccessKey",
		message: "Avoid the <emphasis>accessKey</emphasis> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Assigning keyboard shortcuts using the accessKey attribute leads to inconsistent keyboard actions across applications.",
			},
		],
	},
	JSX_A11Y_NO_AUTOFOCUS: {
		category: "lint/jsx-a11y/noAutofocus",
		message: "Avoid the <emphasis>autoFocus</emphasis> attribute.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Automatically focusing elements overrides natural page content focus order, causing issues for keyboard-only navigation.",
			},
		],
	},
	JSX_A11Y_NO_DISTRACTING_ELEMENTS: (element: string) => ({
		category: "lint/jsx-a11y/noDistractingElements",
		message: `Avoid using deprecated ${element} elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: `Deprecated ${element} are difficult to read and distract attention away from page content, especially for users with visual impairments.`,
			},
		],
	}),
	JSX_A11Y_NO_ON_CHANGE: {
		category: "lint/jsx-a11y/noOnChange",
		message: "Provide an <emphasis>onBlur</emphasis> event instead of an <emphasis>onChange</emphasis> event unless absolutely necessary.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "The onBlur event is more declarative and reliable for indicating input changes when using keyboard navigation.",
			},
		],
	},
	JSX_A11Y_NO_TARGET_BLANK: {
		category: "lint/jsx-a11y/noTargetBlank",
		message: 'Avoid using <emphasis>target="_blank"</emphasis> without <emphasis>rel="noreferrer"</emphasis>.',
		advice: [
			{
				type: "log",
				category: "info",
				text: 'Opening external links in new tabs without rel="noreferrer" is a security risk. See <hyperlink target="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener" /> for more details.',
			},
		],
	},
	JSX_A11Y_NO_SCOPE: {
		category: "lint/jsx-a11y/scope",
		message: "Avoid using the <emphasis>scope</emphasis> attribute on elements other than <emphasis>th</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Using the scope attribute incorrectly on tables makes them difficult to navigate using the keyboard.",
			},
		],
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
	PENDING_FIXES: (relativeFilename: string, original: string, formatted: string) => ({
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
});
