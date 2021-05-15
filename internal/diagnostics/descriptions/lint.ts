/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	DiagnosticAdvice,
	DiagnosticLanguage,
	DiagnosticLocation,
} from "../types";
import {StaticMarkup, markup} from "@internal/markup";
import {stringDiffCompressed} from "@internal/string-diff";
import {buildSuggestionAdvice} from "../helpers";
import {addEmphasis, createDiagnosticsCategory, orJoin} from "./index";
import {DIAGNOSTIC_CATEGORIES} from "../categories";

export const lint = createDiagnosticsCategory({
	TS_PREFER_TS_EXPECT_ERROR: {
		category: DIAGNOSTIC_CATEGORIES["lint/ts/useTsExpectError"],
		message: markup`Prefer @ts-expect-error to get notified when suppression is no longer necessary.`,
	},
	CSS_NO_DUPLICATE_CUSTOM_PROPERTIES: {
		category: DIAGNOSTIC_CATEGORIES["lint/css/noDuplicateCustomProperties"],
		message: markup`Duplicate custom properties inside the same block can lead to unwanted styles`,
	},
	A11_Y_USE_HEADING_CONTENT: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useHeadingContent"],
		message: markup`Provide screen reader accessible content when using <emphasis>heading</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`All headings on a page should have content that is accessible to screen readers.`,
			},
		],
	},
	A11_Y_NO_ON_CHANGE: (goodEventName: string, badEventName: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noOnChange"],
		message: markup`Provide an <emphasis>${goodEventName}</emphasis> event instead of an <emphasis>${badEventName}</emphasis> event unless absolutely necessary.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The ${goodEventName} event is more declarative and reliable for indicating input changes when using keyboard navigation.`,
			},
		],
	}),
	A11_Y_USE_KEY_WITH_MOUSE_EVENTS: (mouseEvent: string, keyboardEvent: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useKeyWithMouseEvents"],
		message: markup`Pair the <emphasis>${mouseEvent}</emphasis> mouse event with the <emphasis>${keyboardEvent}</emphasis> keyboard event.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.`,
			},
		],
	}),
	A11_Y_USE_KEY_WITH_CLICK_EVENTS: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useKeyWithClickEvents"],
		message: markup`Pair the <emphasis>onClick</emphasis> mouse event with the <emphasis>onKeyUp</emphasis>, the <emphasis>onKeyDown</emphasis>, or the <emphasis>onKeyPress</emphasis> keyboard event.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation.`,
			},
		],
	},
	A11_Y_USE_ANCHOR_CONTENT: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useAnchorContent"],
		message: markup`Provide screen reader accessible content when using <emphasis>anchor</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`All links on a page should have content that is accessible to screen readers.`,
			},
		],
	},
	A11_Y_NO_TARGET_BLANK: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noTargetBlank"],
		message: markup`Avoid using <emphasis>target="_blank"</emphasis> without <emphasis>rel="noreferrer"</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Opening external links in new tabs without rel="noreferrer" is a security risk. See <hyperlink target="https://html.spec.whatwg.org/multipage/links.html#link-type-noopener" /> for more details.`,
			},
		],
	},
	A11_Y_NO_ACCESS_KEY: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noAccessKey"],
		message: markup`Avoid the <emphasis>accessKey</emphasis> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Assigning keyboard shortcuts using the accessKey attribute leads to inconsistent keyboard actions across applications.`,
			},
		],
	},
	CSS_NO_IMPORTANT_IN_KEYFRAME: {
		category: DIAGNOSTIC_CATEGORIES["lint/css/noImportantInKeyframes"],
		message: markup`Using <emphasis>!important</emphasis> within keyframes declarations is completely ignored in some browsers.`,
	},
	A11_Y_NO_REDUNDANT_ALT: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noRedundantAlt"],
		message: markup`Avoid the words "image", "picture", or "photo" in <emphasis>img</emphasis> element alt text.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Screen readers announce img elements as "images", so it is not necessary to redeclare this in alternative text.`,
			},
		],
	},
	A11_Y_USE_ARIA_PROPS: (attribute: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useAriaProps"],
		message: markup`<emphasis>${attribute}</emphasis> is an invalid ARIA attribute.`,
	}),
	A11_Y_NO_NONINTERACTIVE_TABINDEX: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noNoninteractiveTabindex"],
		message: markup`Do not use <emphasis>tabIndex</emphasis> on an element that is not interactive.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Adding non-interactive elements to the keyboard navigation flow can confuse users.`,
			},
		],
	},
	A11_Y_NO_POSITIVE_TABINDEX: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noPositiveTabindex"],
		message: markup`Avoid positive integer values for the <emphasis>tabIndex</emphasis> attribute.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Elements with a positive tab index override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.`,
			},
		],
	},
	A11_Y_NO_SVG_WITHOUT_TITLE: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noSvgWithoutTitle"],
		message: markup`Alternative text <emphasis>title</emphasis> element cannot be empty`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`For accessibility purposes, <emphasis>SVGs</emphasis> should have an alternative text, provided via <emphasis>title</emphasis> element.`,
			},
		],
	},
	A11_Y_USE_ARIA_PROPTYPES: (
		attributeName: string,
		values?: Array<string | boolean>,
	) => {
		let advice: DiagnosticAdvice[] = [];
		if (values) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`The supported values for the <emphasis>${attributeName}</emphasis> attribute are: ${values.reduce(
					(str, value) => {
						str.push(typeof value === "boolean" ? String(value) : `"${value}"`);
						return str;
					},
					[] as string[],
				).join(", ")}`,
			});
		}
		return {
			category: DIAGNOSTIC_CATEGORIES["lint/a11y/useAriaProptypes"],
			message: markup`The value of the ARIA attribute <emphasis>${attributeName}</emphasis> is not correct.`,
			advice,
		};
	},
	A11_Y_USE_MEDIA_CAPTION: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useMediaCaption"],
		message: markup`Provide a <emphasis>track</emphasis> for captions when using <emphasis>audio</emphasis> or <emphasis>video</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Captions support users with hearing-impairments. They should be a transcription or translation of the dialogue, sound effects, musical cues, and other relevant audio information.`,
			},
		],
	},
	A11_Y_NO_ARIA_UNSUPPORTED_ELEMENTS: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noAriaUnsupportedElements"],
		message: markup`Avoid the <emphasis>role</emphasis> attribute and <emphasis>aria-*</emphasis> attributes when using <emphasis>meta</emphasis>, <emphasis>html</emphasis>, <emphasis>script</emphasis>, and <emphasis>style</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Using roles on elements that do not support them can cause issues with screen readers.`,
			},
		],
	},
	A11_Y_NO_NONINTERACTIVE_ELEMENT_TO_INTERACTIVE_ROLE: (element: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noNoninteractiveElementToInteractiveRole"],
		message: markup`The HTML element <emphasis>${element}</emphasis> is non-interactive and should not have an interactive role.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Replace <emphasis>${element}</emphasis> with a div or a span.`,
			},
		],
	}),
	A11_Y_NO_DISTRACTING_ELEMENTS: (element: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noDistractingElements"],
		message: markup`Avoid using deprecated ${element} elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Deprecated ${element} are difficult to read and distract attention away from page content, especially for users with visual impairments.`,
			},
		],
	}),
	A11Y_NO_AUTOFOCUS: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noAutofocus"],
		message: markup`Avoid the <emphasis>autoFocus</emphasis> attribute.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Automatically focusing elements overrides natural page content focus order, causing issues for keyboard-only navigation.`,
			},
		],
	},
	A11Y_NO_SCOPE: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noHeaderScope"],
		message: markup`Avoid using the <emphasis>scope</emphasis> attribute on elements other than <emphasis>th</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Using the scope attribute incorrectly on tables makes them difficult to navigate using the keyboard.`,
			},
		],
	},
	JS_NO_SINGLE_CHAR_REGEX_ALTERNATIVES: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noSingleCharRegexAlternatives"],
		message: markup`No single character alternations in regular expressions. Use a character class instead.`,
	},
	JS_NO_UNNECESSARY_CONTINUE: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUnnecessaryContinue"],
		message: markup`Unnecessary <emphasis>continue</emphasis> statement`,
	},
	JS_PREFER_OPTIONAL_CHAINING: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/preferOptionalChaining"],
		message: markup`Prefer optional chaining to manual checks.`,
	},
	HTML_USE_CLOSING_NON_VOID: {
		category: DIAGNOSTIC_CATEGORIES["lint/html/useClosingNonVoid"],
		message: markup`Non-void HTML elements cannot be self-closing. This is valid when using JSX, but not when using HTML.`,
	},
	JSX_USE_SELF_CLOSING_ELEMENTS: {
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/useSelfClosingElements"],
		message: markup`JSX elements without children should be marked as self-closing. In JSX, it is valid for any element to be self-closing.`,
	},
	JS_NO_SHOUTY_CONSTANTS: (constantLocation: undefined | DiagnosticLocation) => {
		const advice: DiagnosticAdvice[] = [
			{
				type: "log",
				category: "info",
				text: markup`You should avoid declaring constants with a string that's the same value as the variable name. It introduces a level of unnecessary indirection when it's only two additional characters to inline.`,
			},
		];

		if (constantLocation !== undefined) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`This constant is declared here`,
			});

			advice.push({
				type: "frame",
				location: constantLocation,
			});
		}

		return {
			category: DIAGNOSTIC_CATEGORIES["lint/js/noShoutyConstants"],
			message: markup`Redundant constant reference`,
			advice,
		};
	},
	JS_NO_UNUSED_TEMPLATE_LITERAL: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUnusedTemplateLiteral"],
		message: markup`Do not use template literals if interpolation and special-character handling are not needed.`,
	},
	JSX_NO_IMPLICIT_BOOLEAN: {
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/noImplicitBoolean"],
		message: markup`Use explicit boolean values for boolean JSX props.`,
	},
	JS_NO_NESTED_TERNARY: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noNestedTernary"],
		message: markup`Nesting ternary expressions can make code more difficult to understand.`,
	},
	JSX_USE_J_S_X_FILE_EXTENSION: (ext: string, basename: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/useJSXFileExtension"],
		message: markup`Files with the <emphasis>${ext}</emphasis> extension cannot contain JSX elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Change the <emphasis>${basename}${ext}</emphasis> file extension to <emphasis>.jsx</emphasis> or <emphasis>.tsx</emphasis>.`,
			},
		],
	}),
	TS_PREFER_INTERFACES: {
		category: DIAGNOSTIC_CATEGORIES["lint/ts/useInterfaces"],
		message: markup`Use an interface instead of an object type alias`,
	},
	JSX_NO_PROP_SPREADING: {
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/noPropSpreading"],
		message: markup`Avoid using property spreading in JSX components.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Explicit JSX attributes enhance the readability of code by clearly indicating which props are accepted by a given element.`,
			},
		],
	},
	REACT_NO_ARRAY_INDEX_KEY: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noArrayIndexKey"],
		message: markup`Avoid using array index as key property in an element.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`See <hyperlink target="https://reactjs.org/docs/lists-and-keys.html#keys" /> for more information.`,
			},
		],
	},
	REACT_NO_THIS_IN_SFC: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noThisInSFC"],
		message: markup`Avoid using <emphasis>this</emphasis> in stateless functional components.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The <emphasis>this</emphasis> keyword has no binding in functional components. Use hooks instead.`,
			},
		],
	},

	JSX_USE_PASCAL_CASE: (oldName: string, newName: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/usePascalCase"],
		message: markup`Switch <emphasis>${oldName}</emphasis> to <emphasis>${newName}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`User-defined JSX components should be defined and referenced in PascalCase.`,
			},
		],
	}),
	REACT_NO_USELESS_FRAGMENT: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noUselessFragment"],
		message: markup`Avoid using unnecessary <emphasis>Fragment</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`A <emphasis>Fragment</emphasis> is redundant if it contains only one child, or if it is the child of a html element, and is not a keyed fragment.`,
			},
		],
	},
	REACT_NO_ACCESS_STATE_IN_SET_STATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noAccessStateInSetState"],
		message: markup`Avoid using <emphasis>this.state</emphasis> within a <emphasis>this.setState</emphasis> call.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Batched state calls could result in unexpected errors due to stale state data.`,
			},
		],
	},
	A11Y_NO_REDUNDANT_ROLES: (role: string, element: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noRedundantRoles"],
		message: markup`Using the role attribute <emphasis>${role}</emphasis> on the <emphasis>${element}</emphasis> element is redundant.`,
	}),
	A11Y_ANCHOR_IS_VALID: (message: StaticMarkup) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useValidAnchor"],
		message,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Anchor elements should only be used for default section or page navigation.`,
			},
		],
	}),
	JSX_NO_DUPLICATE_PROPS: (key: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/noDuplicateProps"],
		message: markup`Avoid duplicate component props. Check the <emphasis>${key}</emphasis> prop.`,
	}),
	REACT_NO_STRING_REFS: (details: StaticMarkup) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/react/noStringRefs"],
		message: markup`Using ${details} is a deprecated pattern.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`See <hyperlink target="https://reactjs.org/docs/refs-and-the-dom.html#legacy-api-string-refs" /> for more information.`,
			},
		],
	}),
	REACT_USE_FRAGMENT_SYNTAX: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/useFragmentSyntax"],
		message: markup`Use shorthand syntax for <emphasis>Fragment</emphasis> elements instead of standard syntax.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Shorthand fragment syntax saves keystrokes and is only unapplicable when keys are required.`,
			},
		],
	},
	REACT_NO_REDUNDANT_SHOULD_COMPONENT_UPDATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noRedundantShouldComponentUpdate"],
		message: markup`Do not implement <emphasis>shouldComponentUpdate</emphasis> when extending <emphasis>React.PureComponent</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`When the shouldComponentUpdate method is implemented, extending React.PureComponent provides no benefit.`,
			},
		],
	},
	REACT_NO_UNSAFE: (oldMethod: string, newMethod: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/react/noUnsafe"],
		message: markup`The <emphasis>${oldMethod}</emphasis> method is unsafe for use in async rendering. Use the <emphasis>${newMethod}</emphasis> method instead.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`See <hyperlink target="https://reactjs.org/blog/2018/03/27/update-on-async-rendering.html" /> for more information.`,
			},
		],
	}),
	REACT_NO_DID_MOUNT_SET_STATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noDidMountSetState"],
		message: markup`Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentDidMount</emphasis> method.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Updating state after mounting causes a second render that can cause visual layout thrashing.`,
			},
		],
	},
	REACT_USE_BUTTON_TYPE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/useButtonType"],
		message: markup`Provide an explicit <emphasis>type</emphasis> prop on <emphasis>button</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The default button type of "submit" causes page reloads and is not typical behavior in a React application.`,
			},
		],
	},
	REACT_NO_WILL_UPDATE_SET_STATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noWillUpdateSetState"],
		message: markup`Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentWillUpdate</emphasis> method.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Updating state immediately before a scheduled render causes a second render that can cause visual layout thrashing.`,
			},
		],
	},
	A11Y_LANG: (value: string, suggestions: string[]) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useValidLang"],
		message: markup`Provide a valid value for the <emphasis>lang</emphasis> attribute.`,
		advice: buildSuggestionAdvice(value, suggestions),
	}),
	A11Y_ALT_TEXT: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useAltText"],
		message: markup`Provide <emphasis>alt</emphasis> text when using <emphasis>img</emphasis>, <emphasis>area</emphasis>, <emphasis>input type='image'</emphasis>, and <emphasis>object</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Meaningful alternative text on elements helps users relying on screen readers to understand content's purpose within a page.`,
			},
		],
	},
	A11Y_HTML_USE_LANG: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useHtmlLang"],
		message: markup`Provide a <emphasis>lang</emphasis> attribute when using the <emphasis>html</emphasis> element.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Setting a lang attribute on HTML elements configures the language used by screen readers when no user default is specified.`,
			},
		],
	},
	A11Y_IFRAME_USE_TITLE: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useIframeTitle"],
		message: markup`Provide a <emphasis>title</emphasis> attribute when using <emphasis>iframe</emphasis> elements.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Screen readers rely on the title set on an iframe to describe the content being displayed.`,
			},
		],
	},
	A11Y_NO_ACCESS_KEY: {
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/noAccessKey"],
		message: markup`Avoid the <emphasis>accessKey</emphasis> attribute to reduce inconsistencies between keyboard shortcuts and screen reader keyboard comments.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Assigning keyboard shortcuts using the accessKey attribute leads to inconsistent keyboard actions across applications.`,
			},
		],
	},
	A11Y_ROLE_HAS_REQUIRED_ARIA_PROPS: (
		roleName: string,
		missingAttributes: string[],
	) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/a11y/useAriaPropsForRole"],
		message: markup`The element with the <emphasis>${roleName}</emphasis> ARIA role does not have the required ARIA attributes.`,
		advice: missingAttributes.map((missingAttribute) => {
			return {
				type: "log",
				category: "info",
				text: markup`Missing aria attribute: ${missingAttribute}`,
			};
		}),
	}),
	REACT_USE_KEY: (origin: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/react/useKey"],
		message: markup`Provide a <emphasis>key</emphasis> prop with a unique value for each element in <emphasis>${origin}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Keys help React identify which items have changed, are added, or are removed.`,
			},
		],
	}),
	JSX_NO_COMMENT_TEXT: {
		category: DIAGNOSTIC_CATEGORIES["lint/jsx/noCommentText"],
		message: markup`Wrap <emphasis>comments</emphasis> inside children within <emphasis>braces</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`JavaScript comment sequences are not supported by JSX and result in unwanted characters on-screen.`,
			},
		],
	},
	REACT_NO_CHILDREN_PROP: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noChildrenProp"],
		message: markup`Avoid passing <emphasis>children</emphasis> using a prop.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The canonical way to pass children in React is to use JSX elements or additional arguments to React.createElement.`,
			},
		],
	},
	REACT_NO_DANGER: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noDanger"],
		message: markup`Avoid passing content using the <emphasis>dangerouslySetInnerHTML</emphasis> prop.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Setting content using code can expose users to cross-site scripting (XSS) attacks.`,
			},
		],
	},
	REACT_NO_DANGER_WITH_CHILDREN: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noDangerWithChildren"],
		message: markup`Avoid passing both <emphasis>children</emphasis> and the <emphasis>dangerouslySetInnerHTML</emphasis> prop.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Setting HTML content will inadvertently override any passed children in React.`,
			},
		],
	},
	REACT_NO_DID_UPDATE_SET_STATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noDidUpdateSetState"],
		message: markup`Avoid calling <emphasis>this.setState</emphasis> in the <emphasis>componentDidUpdate</emphasis> method.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Updating state immediately after a previous update causes a second render that can cause visual layout thrashing.`,
			},
		],
	},
	REACT_NO_DIRECT_MUTATION_STATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noDirectMutationState"],
		message: markup`Avoid mutating <emphasis>this.state</emphasis> directly.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Calling <emphasis>setState()</emphasis> after mutating <emphasis>this.state</emphasis> directly may replace the mutation you made. The only place you may set <emphasis>this.state</emphasis> directly is in a constructor of a react class component.`,
			},
		],
	},
	REACT_NO_FIND_DOM_NODE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noFindDOMNode"],
		message: markup`Avoid using the <emphasis>findDOMNode</emphasis> function.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`React plans to deprecate the findDOMNode function entirely since it prevents internal optimizations. Use callback refs instead.`,
			},
		],
	},
	REACT_USE_SORT_COMP: (
		right: string,
		wrong: string,
		position: "before" | "after",
	) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/react/useSortComp"],
		message: markup`<emphasis>${wrong}</emphasis> should be placed ${position} <emphasis>${right}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`When creating React components it is more convenient to always follow the same organisation for method order to help you easily find lifecycle methods, event handlers, etc.`,
			},
		],
	}),
	REACT_USE_STYLE_PROP_OBJECT: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/useStylePropObject"],
		message: markup`The <emphasis>style</emphasis> prop value must be an object.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`React will ignore non-object style props, even valid JSON strings.`,
			},
		],
	},
	REACT_USE_RENDER_RETURN: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/useRenderReturn"],
		message: markup`The <emphasis>render</emphasis> method on a component must return content.`,
	},
	REACT_NO_RENDER_RETURN_VALUE: {
		category: DIAGNOSTIC_CATEGORIES["lint/react/noRenderReturnValue"],
		message: markup`Do not depend on the return value from <emphasis>ReactDOM.render()</emphasis>.`,
	},
	REACT_NO_VOID_ELEMENTS_WITH_CHILDREN: (element: string, properties: string[]) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/react/noVoidElementsWithChildren"],
		message: markup`<emphasis>${element}</emphasis> is a void element tag and must not have <emphasis>${orJoin(
			properties.map((name) => markup`${name}`),
		)}</emphasis>.`,
	}),
	JS_USE_DEFAULT_IMPORT_BASENAME: (prev: string, basenames: string[]) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/useDefaultImportBasename"],
		message: markup`Use the basename ${orJoin(
			addEmphasis(basenames.map((basename) => markup`${basename}`)),
		)} when importing the default.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`If you really meant to use a named import, use the following:`,
			},
			{
				type: "code",
				language: "js",
				sourceText: `import {default as ${prev}}`,
			},
		],
	}),
	JS_NO_COMMA_OPERATOR: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noCommaOperator"],
		message: markup`<emphasis>Avoid the comma operator</emphasis>. It can lead to easy mistakes and ambiguous code.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`If you want multiple expressions, then break it up.`,
			},
		],
	},
	JS_NO_NEGATION_ELSE: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noNegationElse"],
		message: markup`<emphasis>Invert blocks</emphasis> when performing a negation test.`,
	},
	JS_NO_DUPLICATE_IMPORT_SOURCE: (seenLocation: DiagnosticLocation) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDuplicateImportSource"],
		message: markup`This module has <emphasis>already been imported</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Previously imported here`,
			},
			{
				type: "frame",
				location: seenLocation,
			},
		],
	}),
	JS_PREFER_BLOCK_STATEMENT: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useBlockStatements"],
		message: markup`<emphasis>Block statements</emphasis> are preferred in this position.`,
	},
	JS_USE_TEMPLATE: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useTemplate"],
		message: markup`<emphasis>Template literals</emphasis> are preferred over <emphasis>string concatenation</emphasis>.`,
	},
	JS_USE_WHILE: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useWhile"],
		message: markup`Use <emphasis>while</emphasis> loops instead of <emphasis>for</emphasis> loops.`,
	},
	JS_UNSAFE_NEGATION: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUnsafeNegation"],
		message: markup`The <emphasis>negation operator is used unsafely</emphasis> on the left side of this binary expression.`,
	},
	JS_NO_UNUSED_VARIABLES: (kind: string, name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUnusedVariables"],
		categoryValue: name,
		message: markup`The ${kind} variable <emphasis>${name}</emphasis> is unused.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Unused variables are dead code and usually the result of incomplete refactoring.`,
			},
		],
	}),
	JS_NO_UNDECLARED_VARIABLES: (name: string, bindingsInScope: string[]) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUndeclaredVariables"],
		categoryValue: name,
		message: markup`The <emphasis>${name}</emphasis> variable is undeclared`,
		advice: buildSuggestionAdvice(name, bindingsInScope),
	}),
	JS_VARIABLE_CAMEL_CASE: (name: string, camelCaseName: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/useCamelCase"],
		message: markup`The <emphasis>${name}</emphasis> variable should be camel cased as <emphasis>${camelCaseName}</emphasis>.`,
	}),
	JS_USE_SINGLE_CASE_STATEMENT: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useSingleCaseStatement"],
		message: markup`A switch case should only have a single statement. If you want more, then wrap it in a block.`,
	},
	/*JS_NO_CONFUSING_LANGUAGE: (
		message: StaticMarkup,
		suggestion: string,
		advice: DiagnosticAdvice,
	) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noConfusingLanguage"],
		message,
		advice: [
			...advice,
			{
				type: "log",
				category: "info",
				text: markup`Consider using <emphasis>${suggestion}</emphasis> instead`,
			},
		],
	}),*/
	JS_NO_DOUBLE_EQUALS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDoubleEquals"],
		message: markup`Use <emphasis>===</emphasis> instead of <emphasis>==</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`== is only allowed when comparing against null.`,
			},
		],
	},
	REGEX_NO_EMPTY_MATCHES: {
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noEmptyMatches"],
		message: markup`This expression can return <emphasis>empty matches</emphasis>, and may match infinitely in some use cases.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Strengthen the regular expression so that empty matches are not possible.`,
			},
		],
	},
	JS_NEGATE_DOUBLE_EQUALS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDoubleEquals"],
		message: markup`Use <emphasis>!==</emphasis> instead of <emphasis>!=</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`!= is only allowed when comparing against null.`,
			},
		],
	},
	JS_NO_CATCH_ASSIGN: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noCatchAssign"],
		message: markup`Do not <emphasis>reassign catch parameters</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use a local variable instead.`,
			},
		],
	},
	JS_NO_SPARSE_ARRAY: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noSparseArray"],
		message: markup`This <emphasis>array</emphasis> contains an <emphasis>empty slot</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Sparse arrays without values for some items can lead to confusion.`,
			},
		],
	},
	JS_USE_SINGLE_VAR_DECLARATOR: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useSingleVarDeclarator"],
		message: markup`Declare variables separately.`,
	},
	JS_USE_FUNCTION_DECLARATIONS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useFunctionDeclarations"],
		message: markup`Use a <emphasis>function declaration</emphasis> instead of a <emphasis>const function</emphasis>.`,
	},
	JS_NO_VAR: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noVar"],
		message: markup`Variable declarations using <emphasis>var</emphasis> are disallowed.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use let or const instead.`,
			},
		],
	},
	TS_PREFER_SHORTHAND_ARRAY_TYPE: {
		category: DIAGNOSTIC_CATEGORIES["lint/ts/preferShorthandArrayType"],
		message: markup`Use <emphasis>shorthand T[] syntax</emphasis> instead of <emphasis>Array${"<T>"} syntax</emphasis>.`,
	},
	JS_NO_UNSAFE_FINALLY: (type: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noUnsafeFinally"],
		message: markup`Using <emphasis>${type}</emphasis> inside a <emphasis>finally</emphasis> clause is unsafe.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Do not use control flow statements inside finally clauses.`,
			},
		],
	}),
	JS_NO_TEMPLATE_CURLY_IN_STRING: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noTemplateCurlyInString"],
		message: markup`This string contains an <emphasis>unexpected template string</emphasis> expression.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Using template string expressions in regular strings is usually a typo.`,
			},
		],
	},
	JS_NO_SHADOW_RESTRICTED_NAMES: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noShadowRestrictedNames"],
		message: markup`Do not shadow the global <emphasis>${name}</emphasis> property.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global.`,
			},
		],
	}),
	REGEX_NO_MULTIPLE_SPACES_IN_REGEX_LITERAL: (count: number) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noMultipleSpacesInRegularExpressionLiterals"],
		message: markup`This <emphasis>regular expression</emphasis> contains unclear uses of <emphasis>multiple spaces</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`It's hard to visually count the amount of spaces, it's clearer if you use a quantifier instead. eg / {${String(
					count,
				)}}/`,
			},
		],
	}),
	JS_NO_LABEL_VAR: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noLabelVar"],
		message: markup`Do not use the ${name} variable name as a label.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Creating a label with the same name as an in-scope variable leads to confusion.`,
			},
		],
	}),
	JS_NO_IMPORT_ASSIGN: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noImportAssign"],
		message: markup`The imported variable <emphasis>${name}</emphasis> is read-only.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use a local variable instead of reassigning an import.`,
			},
		],
	}),
	JS_NO_EXTRA_BOOLEAN_CAST: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noExtraBooleanCast"],
		message: markup`Avoid <emphasis>redundant double-negation</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`It is not necessary to use double-negation when a value will already be coerced to a boolean.`,
			},
		],
	},
	JS_NO_FUNCTION_ASSIGN: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noFunctionAssign"],
		message: markup`Do not <emphasis>reassign a function declaration</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use a local variable instead.`,
			},
		],
	},
	REGEX_NO_EMPTY_CHAR_SET: {
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noEmptyCharacterClass"],
		message: markup`Do not use <emphasis>empty character classes in regular expressions</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Empty character classes are usually typos.`,
			},
		],
	},
	JS_NO_DUPLICATE_KEYS: (key: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDuplicateKeys"],
		message: markup`Avoid duplicate component key. Check the <emphasis>${key}</emphasis> key.`,
	}),
	REGEX_NO_POSIX_IN_REGULAR_EXPRESSION: {
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noPosixInRegularExpression"],
		message: markup`Do not use POSIX character classes and collating sequences.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`This functionality is not supported in JavaScript regular expressions.`,
			},
		],
	},
	JS_NO_DUPLICATE_CASE: (value: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDuplicateCase"],
		message: markup`Do not duplicate the <emphasis>${value}</emphasis> case.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Duplicated switch logic paths are hard to follow and usually typos.`,
			},
		],
	}),
	JS_NO_DUPE_ARGS: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDupeArgs"],
		message: markup`Avoid duplicate function arguments. Check the <emphasis>${name}</emphasis> argument.`,
	}),
	JS_NO_DELETE: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDelete"],
		message: markup`This is an unexpected use of the <emphasis>delete</emphasis> operator.`,
	},
	JS_NO_DELETE_VARS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDeleteVars"],
		message: markup`This is an invalid use of the <emphasis>delete</emphasis> operator.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Only object properties can be deleted.`,
			},
		],
	},
	JS_NO_DEBUGGER: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noDebugger"],
		message: markup`This is an unexpected use of the <emphasis>debugger</emphasis> statement.`,
	},
	JS_NO_COND_ASSIGN: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noCondAssign"],
		message: markup`Do not assign <emphasis>variables in loop conditions</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`It is a common typo to mistype an equality operator as an assignment operator.`,
			},
		],
	},
	JS_NO_COMPARE_NEG_ZERO: (op: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noCompareNegZero"],
		message: markup`Do not use the <emphasis>${op}</emphasis> operator to compare against <emphasis>-0</emphasis>.`,
		fixable: op === "===",
	}),
	JS_NO_ASYNC_PROMISE_EXECUTOR: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noAsyncPromiseExecutor"],
		message: markup`<emphasis>Promise executor functions</emphasis> should not be <emphasis>async</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`This can lead to lost errors and unnecessary indirection.`,
			},
		],
	},
	JS_NO_GETTER_RETURN: (got: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noGetterReturn"],
		message: markup`<emphasis>Return a value at the end of a getter method</emphasis> instead of ${got}.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Getters that do not return values are either typos or should not be getters.`,
			},
		],
	}),
	JS_NO_SETTER_RETURN: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noSetterReturn"],
		message: markup`Do not <emphasis>return a value</emphasis> at the end of a <emphasis>setter method</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Setters that return values are either typos or should not be setters.`,
			},
		],
	},
	JS_NO_EMPTY_BLOCKS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noEmptyBlocks"],
		message: markup`Avoid <emphasis>empty logic blocks</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Empty logic blocks usually result from incomplete refactoring.`,
			},
		],
	},
	JS_NO_ARGUMENTS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/noArguments"],
		message: markup`Use the <emphasis>rest parameters</emphasis> instead of <emphasis>arguments</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Arguments does not have Array.prototype methods and can be inconvenient to use.`,
			},
		],
	},
	REGEX_DUPLICATE_REGEX_GROUP_NAME: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noDuplicateGroupNamesInRegularExpressions"],
		message: markup`Avoid duplicate group names. Check the <emphasis>${name}</emphasis> group.`,
	}),
	REGEX_NO_REFERENCE_TO_NON_EXISTING_GROUP: (name: string) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/regex/noReferenceToNonExistingGroup"],
		message: markup`Avoid nonexistent group names. Check the <emphasis>${name}</emphasis> group.`,
	}),
	JS_USE_DEFAULT_EXPORT_BASENAME: (
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
		let adviceMessage;

		if (defaultName === "*default*") {
			adviceMessage = markup`The`;
		} else {
			adviceMessage = markup`The filename should be <emphasis>${correctFilename}</emphasis> or the`;
		}

		adviceMessage = markup`${adviceMessage} ${defaultType} name should be <emphasis>${actualFilename}</emphasis>.`;

		return {
			category: DIAGNOSTIC_CATEGORIES["lint/js/useDefaultExportBasename"],
			message: markup`The filename and the name of a default ${defaultType} should match.`,
			advice: [
				{
					type: "log",
					category: "info",
					text: adviceMessage,
				},
			],
		};
	},
	JS_NO_RESTRICTED_GLOBALS: (globalName) => ({
		category: DIAGNOSTIC_CATEGORIES["lint/js/noRestrictedGlobals"],
		message: markup`Do not use the global variable <emphasis>${globalName}</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use a local variable instead.`,
			},
		],
	}),
	JS_SORT_EXPORT_SPECIFIERS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useSortedSpecifiers"],
		message: markup`The specifiers of the export declaration should be sorted alphabetically.`,
	},
	JS_SORT_IMPORT_SPECIFIERS: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useSortedSpecifiers"],
		message: markup`The specifiers of the import declaration should be sorted alphabetically.`,
	},
	PENDING_FIXES: (
		relativeFilename: undefined | string,
		language: DiagnosticLanguage,
		original: string,
		formatted: string,
	) => {
		const advice: DiagnosticAdvice[] = [
			{
				type: "diff",
				language,
				diff: stringDiffCompressed(original, formatted),
			},
		];
		const verboseAdvice: DiagnosticAdvice[] = [];

		if (relativeFilename !== undefined) {
			verboseAdvice.push({
				type: "action",
				command: "check",
				suggestedKeyboardShortcut: "f",
				description: markup`Apply fixes and format`,
				args: [relativeFilename],
				commandFlags: {
					apply: true,
				},
			});

			verboseAdvice.push({
				type: "action",
				command: "check",
				suggestedKeyboardShortcut: "o",
				description: markup`Only format without any fixes`,
				args: [relativeFilename],
				commandFlags: {
					format: true,
				},
			});
		}

		return {
			category: DIAGNOSTIC_CATEGORIES["lint/pendingFixes"],
			message: markup`Pending formatting and safe fixes`,
			advice,
			verboseAdvice,
		};
	},
	TS_NO_EXPLICIT_ANY: {
		category: DIAGNOSTIC_CATEGORIES["lint/ts/noExplicitAny"],
		message: markup`Avoid using the <emphasis>any</emphasis> type.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Using nonspecific types defeats the purpose of using TypeScript.`,
			},
		],
	},
	JS_USE_SIMPLIFIED_LOGICAL_EXPRESSION: {
		category: DIAGNOSTIC_CATEGORIES["lint/js/useSimplifiedLogicalExpression"],
		message: markup`Logical expression contains unnecessary complexity.`,
	},
	TS_USE_SIMPLIFIED_BOOLEAN_EXPRESSION: {
		category: DIAGNOSTIC_CATEGORIES["lint/ts/useSimplifiedBooleanExpression"],
		message: markup`Boolean expression contains unnecessary complexity.`,
	},
});
