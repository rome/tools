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
	REACT_NO_USELESS_FRAGMENT: {
		category: "lint/react/noUselessFragment",
		message: "Avoid using unnecessary <emphasis>Fragment</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "A <emphasis>Fragment</emphasis> is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed fragment.",
			},
		],
	},
	REACT_NO_ACCESS_STATE_IN_SET_STATE: {
		category: "lint/react/noAccessStateInSetState",
		message: "Avoid using <emphasis>this.state</emphasis> within a <emphasis>this.setState</emphasis> call.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Batched state calls could result in unexpected errors due to stale state data.",
			},
		],
	},
	JSX_A11Y_NO_REDUNDANT_ROLES: {
		category: "lint/jsx-a11y/noRedundantRoles",
		message: "Using the role attribute on the HTML element is redundant, the HTML element is semantically enough.",
	},
	JSX_A11Y_ANCHOR_IS_VALID: (message: string) => ({
		category: "lint/jsx-a11y/anchorIsValid",
		message,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Anchor elements should only be used for default section or page navigation.",
			},
		],
	}),
	JSX_A11Y_NO_NONINTERACTIVE_TABINDEX: {
		category: "lint/jsx-a11y/noNoninteractiveTabindex",
		message: "Do not use <emphasis>tabIndex</emphasis> on an element that is not interactive",
	},
	JSX_A11Y_ARIA_PROPS: (attribute: string) => ({
		category: "lint/jsx-a11y/ariaProps",
		message: `<emphasis>${attribute}</emphasis> is an invalid ARIA attribute.`,
	}),
	JSX_A11Y_CLICK_EVENTS_HAVE_KEY_EVENTS: {
		category: "lint/jsx-a11y/clickEventsHaveKeyEvents",
		message: "Pair the <emphasis>onClick</emphasis> mouse event with the <emphasis>onKeyUp</emphasis>, the <emphasis>onKeyDown</emphasis>, or the <emphasis>onKeyPress</emphasis> keyboard event.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.",
			},
		],
	},
	REACT_JSX_NO_DUPLICATE_PROPS: (key: string) => ({
		category: "lint/react/jsxNoDuplicateProps",
		message: `Avoid duplicate component props. Check the <emphasis>${key}</emphasis> prop.`,
	}),
	REACT_NO_STRING_REFS: (details: string) => ({
		category: "lint/react/noStringRefs",
		message: `Using ${details} is a deprecated pattern.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: 'See <hyperlink target="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs" /> for more information.',
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
		advice: [
			{
				type: "log",
				category: "info",
				text: "When the shouldComponentUpdate method is implemented, extending React.PureComponent provides no benefit.",
			},
		],
	},
	REACT_NO_UNSAFE: (oldMethod: string, newMethod: string) => ({
		category: "lint/react/noUnsafe",
		message: `The <emphasis>${oldMethod}</emphasis> method is unsafe for use in async rendering. Use the <emphasis>${newMethod}</emphasis> method instead.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: 'See <hyperlink target="https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html" /> for more information.',
			},
		],
	}),
	REACT_NO_DID_MOUNT_SET_STATE: {
		category: "lint/react/noDidMountSetState",
		message: "Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentDidMount</emphasis> method.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Updating state after mounting causes a second render that can cause visual layout thrashing.",
			},
		],
	},
	REACT_BUTTON_HAS_TYPE: {
		category: "lint/react/buttonHasType",
		message: "Provide an explicit <emphasis>type</emphasis> prop on <emphasis>button</emphasis> elements.",
		advice: [
			{
				type: "log",
				category: "info",
				text: 'The default button type of "submit" causes page reloads and is not typical behavior in a React application.',
			},
		],
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
		message: "Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentWillUpdate</emphasis> method.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Updating state immediately before a scheduled render causes a second render that can cause visual layout thrashing.",
			},
		],
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
	JSX_A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS: (
		roleName: string,
		missingAttributes: Array<string>,
	) => ({
		category: "lint/jsx-a11y/roleHasRequiredAriaProps",
		message: `The element with the <emphasis>${roleName}</emphasis> ARIA role does not have the required ARIA attributes.`,
		advice: missingAttributes.map((missingAttribute) => {
			return {
				type: "log",
				category: "info",
				text: `Missing aria attribute: ${missingAttribute}`,
			};
		}),
	}),
	REACT_JSX_KEY: (origin: string) => ({
		category: "lint/react/jsxKey",
		message: `Provide a <emphasis>key</emphasis> prop with a unique value for each element in <emphasis>${origin}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Keys help React identify which items have changed, are added, or are removed.",
			},
		],
	}),
	REACT_JSX_NO_COMMENT_TEXT: {
		category: "lint/react/jsxNoCommentText",
		message: "Wrap <emphasis>comments</emphasis> inside children within <emphasis>braces</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "JavaScript comment sequences are not supported by JSX and result in unwanted characters on-screen.",
			},
		],
	},
	REACT_NO_CHILDREN_PROP: {
		category: "lint/react/noChildrenProp",
		message: "Avoid passing <emphasis>children</emphasis> using a prop.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "The canonical way to pass children in React is to use JSX elements or additional arguments to React.createElement.",
			},
		],
	},
	REACT_NO_DANGER: {
		category: "lint/react/noDanger",
		message: "Avoid passing content using the <emphasis>dangerouslySetInnerHTML</emphasis> prop.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Setting content using code can expose users to cross-site scripting (XSS) attacks.",
			},
		],
	},
	REACT_NO_DANGER_WITH_CHILDREN: {
		category: "lint/react/noDangerWithChildren",
		message: "Avoid passing both <emphasis>children</emphasis> and the <emphasis>dangerouslySetInnerHTML</emphasis> prop.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Setting HTML content will inadvertently override any passed children in React.",
			},
		],
	},
	REACT_NO_DID_UPDATE_SET_STATE: {
		category: "lint/react/noDidUpdateSetState",
		message: "Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentDidUpdate</emphasis> method.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Updating state immediately after a previous update causes a second render that can cause visual layout thrashing.",
			},
		],
	},
	REACT_NO_FIND_DOM_NODE: {
		category: "lint/react/noFindDOMNode",
		message: "Avoid using the <emphasis>findDOMNode</emphasis> function.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "React plans to deprecate the findDOMNode function entirely since it prevents internal optimizations. Use callback refs instead.",
			},
		],
	},
	REACT_REACT_IN_JSX_SCOPE: {
		category: "lint/react/reactInJsxScope",
		message: "<emphasis>React</emphasis> must be in scope when using JSX.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "The React JSX parser must be available in modules that use JSX syntax.",
			},
		],
	},
	REACT_STYLE_PROP_OBJECT: {
		category: "lint/react/stylePropObject",
		message: "The <emphasis>style</emphasis> prop value must be an object.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "React will ignore non-object style props, even valid JSON strings.",
			},
		],
	},
	REACT_VOID_DOM_ELEMENTS_NO_CHILDREN: (
		element: string,
		properties: Array<string>,
	) => ({
		category: "lint/react/voidDomElementsNoChildren",
		message: `<emphasis>${element}</emphasis> is a void element tag and must not have <emphasis>${orJoin(
			properties,
		)}</emphasis>.`,
	}),
	JS_IMPORT_DEFAULT_BASENAME: (prev: string, basename: string) => ({
		category: "lint/js/importDefaultBasename",
		message: `Use the basename <emphasis>${basename}</emphasis> when importing the default.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "If you really meant to use a named import, use the following:",
			},
			{
				type: "code",
				code: `import {default as ${prev}}`,
			},
		],
	}),
	JS_NO_COMMA_OPERATOR: {
		category: "lint/js/noCommaOperator",
		message: "<emphasis>Avoid the comma operator</emphasis>. It can lead to easy mistakes and ambiguous code.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "If you want multiple expressions, then break it up.",
			},
		],
	},
	JS_NEGATION_ELSE: {
		category: "lint/js/negationElse",
		message: "<emphasis>Invert blocks</emphasis> when performing a negation test.",
	},
	JS_DUPLICATE_IMPORT_SOURCE: (seenLocation: DiagnosticLocation) => ({
		category: "lint/js/duplicateImportSource",
		message: "This module has <emphasis>already been imported</emphasis>.",
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
		message: "<emphasis>Block statements</emphasis> are preferred in this position.",
	},
	JS_PREFER_TEMPLATE: {
		category: "lint/js/preferTemplate",
		message: "<emphasis>Template literals</emphasis> are preferred over <emphasis>string concatenation</emphasis>.",
	},
	JS_PREFER_WHILE: {
		category: "lint/js/preferWhile",
		message: "Use <emphasis>while</emphasis> loops instead of <emphasis>for</emphasis> loops.",
	},
	JS_UNSAFE_NEGATION: {
		category: "lint/js/unsafeNegation",
		message: "The <emphasis>negation operator is used unsafely</emphasis> on the left side of this binary expression.",
	},
	JS_UNUSED_VARIABLES: (kind: string, name: string) => ({
		category: "lint/js/unusedVariables",
		message: markup`The ${kind} variable <emphasis>${name}</emphasis> is unused.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Unused variables are dead code and usually result from incomplete refactoring.",
			},
		],
	}),
	JS_UNDECLARED_VARIABLES: (name: string) => ({
		category: "lint/js/undeclaredVariables",
		message: markup`The <emphasis>${name}</emphasis> variable is undeclared.`,
	}),
	JS_VARIABLE_CAMEL_CASE: (name: string, camelCaseName: string) => ({
		category: "lint/js/camelCase",
		message: markup`The <emphasis>${name}</emphasis> variable should be camel cased as <emphasis>${camelCaseName}</emphasis>.`,
	}),
	JS_IDENTIFIER_CAMEL_CASE: (name: string, camelCaseName: string) => ({
		category: "lint/js/camelCase",
		message: markup`The <emphasis>${name}</emphasis> identifier should be camel cased as <emphasis>${camelCaseName}</emphasis>.`,
	}),
	JS_CASE_SINGLE_STATEMENT: {
		category: "lint/js/caseSingleStatement",
		message: "A switch case should only have a single statement. If you want more, then wrap it in a block.",
	},
	JS_CONFUSING_LANGUAGE: (
		description: string,
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
		message: "Use <emphasis>===</emphasis> instead of <emphasis>==</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "== is only allowed when comparing against null.",
			},
		],
	},
	JS_EMPTY_MATCHES: {
		category: "lint/js/emptyMatches",
		message: "This expression can return <emphasis>empty matches</emphasis>, and may match infinitely in some use cases.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Strengthen the regular expression so that empty matches are not possible.",
			},
		],
	},
	JS_NEGATE_DOUBLE_EQUALS: {
		category: "lint/js/doubleEquals",
		message: "Use <emphasis>!==</emphasis> instead of <emphasis>!=</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "!= is only allowed when comparing against null.",
			},
		],
	},
	JS_NO_CATCH_ASSIGN: {
		category: "lint/js/noCatchAssign",
		message: "Do not <emphasis>reassign catch parameters</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Use a local variable instead.",
			},
		],
	},
	JS_SPARSE_ARRAY: {
		category: "lint/js/sparseArray",
		message: "This <emphasis>array</emphasis> contains an <emphasis>empty slot</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Sparse arrays without values for some items can lead to confusion.",
			},
		],
	},
	JS_SINGLE_VAR_DECLARATOR: {
		category: "lint/js/singleVarDeclarator",
		message: "Declare variables separately.",
	},
	JS_PREFER_FUNCTION_DECLARATIONS: {
		category: "lint/js/preferFunctionDeclarations",
		message: "Use a <emphasis>function declaration</emphasis> instead of a <emphasis>const function</emphasis>.",
	},
	JS_NO_VAR: {
		category: "lint/js/noVar",
		message: "Variable declarations using <emphasis>var</emphasis> are disallowed.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Use let or const instead.",
			},
		],
	},
	JS_NO_SHORTHAND_ARRAY_TYPE: {
		category: "lint/js/noShorthandArrayType",
		message: escapeMarkup(
			"Use <emphasis>Array<T> syntax</emphasis> instead of <emphasis>shorthand T[] syntax</emphasis>.",
		),
	},
	JS_NO_UNSAFE_FINALLY: (type: string) => ({
		category: "lint/js/noUnsafeFinally",
		message: markup`Using <emphasis>${type}</emphasis> inside a <emphasis>finally</emphasis> clause is unsafe.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Do not use control flow statements inside finally clauses.",
			},
		],
	}),
	JS_NO_TEMPLATE_CURLY_IN_STRING: {
		category: "lint/js/noTemplateCurlyInString",
		message: "This string contains an <emphasis>unexpected template string</emphasis> expression.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Using template string expressions in regular strings is usually a typo.",
			},
		],
	},
	JS_NO_SHADOW_RESTRICTED_NAMES: (name: string) => ({
		category: "lint/js/noShadowRestrictedNames",
		message: markup`Do not shadow the global <emphasis>${name}</emphasis> property.`,
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
		message: "This <emphasis>regular expression</emphasis> contains unclear uses of <emphasis>multiple spaces</emphasis>.",
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
	JS_NO_LABEL_VAR: (name: string) => ({
		category: "lint/js/noLabelVar",
		message: `Do not use the ${name} variable name as a label.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Creating a label with the same name as an in-scope variable leads to confusion.",
			},
		],
	}),
	JS_NO_IMPORT_ASSIGN: (name: string) => ({
		category: "lint/js/noImportAssign",
		message: markup`The imported variable <emphasis>${name}</emphasis> is read-only.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Use a local variable instead of reassigning an import.",
			},
		],
	}),
	JS_NO_EXTRA_BOOLEAN_CAST: {
		category: "lint/js/noExtraBooleanCast",
		message: "Avoid <emphasis>redundant double-negation</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "It is not necessary to use double-negation when a value will already be coerced to a boolean.",
			},
		],
	},
	JS_NO_FUNCTION_ASSIGN: {
		category: "lint/js/noFunctionAssign",
		message: "Do not <emphasis>reassign a function declaration</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Use a local variable instead.",
			},
		],
	},
	JS_NO_EMPTY_CHAR_SET: {
		category: "lint/js/noEmptyCharacterClass",
		message: "Do not use <emphasis>empty character classes in regular expressions</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Empty character classes are usually typos.",
			},
		],
	},
	JS_NO_DUPLICATE_KEYS: (key: string) => ({
		category: "lint/js/noDuplicateKeys",
		message: `Avoid duplicate component key. Check the <emphasis>${key}</emphasis> key.`,
	}),
	JS_NO_POSIX_IN_REGULAR_EXPRESSION: {
		category: "lint/js/noPosixInRegularExpression",
		message: "Do not use POSIX character classes and collating sequences.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "This functionality is not supported in JavaScript regular expressions.",
			},
		],
	},
	JS_NO_DUPLICATE_CASE: (value: string) => ({
		category: "lint/js/noDuplicateCase",
		message: markup`Do not duplicate the <emphasis>${value}</emphasis> case.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Duplicated switch logic paths are hard to follow and usually typos.",
			},
		],
	}),
	JS_NO_DUPE_ARGS: (name: string) => ({
		category: "lint/js/noDupeArgs",
		message: `Avoid duplicate function arguments. Check the <emphasis>${name}</emphasis> argument.`,
	}),
	JS_NO_DELETE: {
		category: "lint/js/noDelete",
		message: "This is an unexpected use of the <emphasis>delete</emphasis> operator.",
	},
	JS_NO_DELETE_VARS: {
		category: "lint/js/noDeleteVars",
		message: "This is an invalid use of the <emphasis>delete</emphasis> operator.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Only object properties can be deleted.",
			},
		],
	},
	JS_NO_DEBUGGER: {
		category: "lint/js/noDebugger",
		message: "This is an unexpected use of the <emphasis>debugger</emphasis> statement.",
	},
	JS_NO_COND_ASSIGN: {
		category: "lint/js/noCondAssign",
		message: "Do not assign <emphasis>variables in loop conditions</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "It is a common typo to mistype an equality operator as an assignment operator.",
			},
		],
	},
	JS_NO_COMPARE_NEG_ZERO: (op: string) => ({
		category: "lint/js/noCompareNegZero",
		message: `Do not use the <emphasis>${op}</emphasis> operator to compare against <emphasis>-0</emphasis>.`,
		fixable: op === "===",
	}),
	JS_NO_ASYNC_PROMISE_EXECUTOR: {
		category: "lint/js/noAsyncPromiseExecutor",
		message: "<emphasis>Promise executor functions</emphasis> should not be <emphasis>async</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "This can lead to lost errors and unnecessary indirection.",
			},
		],
	},
	JS_GETTER_RETURN: (got: string) => ({
		category: "lint/js/getterReturn",
		message: `<emphasis>Return a value at the end of a getter method</emphasis> instead of ${got}.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Getters that do not return values are either typos or should not be getters.",
			},
		],
	}),
	JS_NO_SETTER_RETURN: {
		category: "lint/js/noSetterReturn",
		message: "Do not <emphasis>return a value</emphasis> at the end of a <emphasis>setter method</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Setters that return values are either typos or should not be setters.",
			},
		],
	},
	JS_EMPTY_BLOCKS: {
		category: "lint/js/emptyBlocks",
		message: "Avoid <emphasis>empty logic blocks</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Empty logic blocks usually result from incomplete refactoring.",
			},
		],
	},
	JS_NO_ARGUMENTS: {
		category: "lint/js/noArguments",
		message: "Use the <emphasis>rest parameters</emphasis> instead of <emphasis>arguments</emphasis>.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Arguments does not have Array.prototype methods and can be inconvenient to use.",
			},
		],
	},
	JS_DUPLICATE_REGEX_GROUP_NAME: (name: string) => ({
		category: "lint/js/noDuplicateGroupNamesInRegularExpressions",
		message: `Avoid duplicate group names. Check the <emphasis>${name}</emphasis> group.`,
	}),
	JS_NO_REFERENCE_TO_NON_EXISTING_GROUP: (name: string) => ({
		category: "lint/js/noReferenceToNonExistingGroup",
		message: `Avoid nonexistent group names. Check the <emphasis>${name}</emphasis> group.`,
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
			adviceMessage += `The filename should be <emphasis>${correctFilename}</emphasis> or the`;
		}

		adviceMessage += ` ${defaultType} name should be <emphasis>${actualFilename}</emphasis>.`;

		return {
			category: "lint/js/defaultExportSameBasename",
			message: `The filename and the name of a default ${defaultType} should match.`,
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
		message: markup`Do not use the global variable <emphasis>${globalName}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: "Use a local variable instead.",
			},
		],
	}),
	JS_SORT_EXPORT_SPECIFIERS: {
		category: "lint/js/sortImportExportSpecifiers",
		message: "The specifiers of the export declaration should be sorted alphabetically.",
	},
	JS_SORT_IMPORT_SPECIFIERS: {
		category: "lint/js/sortImportExportSpecifiers",
		message: "The specifiers of the import declaration should be sorted alphabetically.",
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
		message: "Avoid using the <emphasis>any</emphasis> type.",
		advice: [
			{
				type: "log",
				category: "info",
				text: "Using nonspecific types defeats the purpose of using TypeScript.",
			},
		],
	},
});
