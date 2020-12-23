import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {DiagnosticLocation} from "../../types";

// @internal/html-parser
export const htmlParser = createDiagnosticsCategory({
	UNCLOSED_STRING: {message: markup`Unclosed string`},
	EXPECTED_CLOSING_TAG_NAME: (tagName) => ({
		message: markup`Expected closing tag <emphasis>${tagName}</emphasis>.`,
	}),
	UNKNOWN_START: {message: markup`Unknown child start`},
	EXPECTED_ATTRIBUTE_NAME: {message: markup`Expected attribute name`},
	INCORRECT_CLOSING_TAG_NAME: (expected: string, got: string) => ({
		message: markup`Expected to close tag <emphasis>${expected}</emphasis> but found tag <emphasis>${got}</emphasis>`,
	}),
	TAGEND_NOT_FOUND: (tagName: string) => ({
		message: markup`The tag <emphasis>${tagName}</emphasis> doesn't have an end.`,
	}),
	TAGNAME_NOT_FOUND: () => ({}),
	INVALID_ATTRIBUTE_NAME: {
		message: markup`The name of the attribute is not valid and should be wrapped in double quotes.`,
	},
	UNOPENED_TAG: {message: markup`Ending tag with no opening tag`},
	UNSUPPORTED_DOCTYPE: (value: string) => ({
		message: markup`The !DOCTYPE value <emphasis>${value}</emphasis> is not supported.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The supported value should be <emphasis>html</emphasis>`,
			},
			{
				type: "log",
				category: "warn",
				text: markup`Other DOCTYPEs are deprecated and not supported by Rome as they activate the old "quirk-mode", which is an old standard.`,
			},
		],
	}),
	UNCLOSED_TAG: (tagName: string, openLocation: DiagnosticLocation) => ({
		message: markup`Unclosed ${tagName} tag`,
		advice: [
			{type: "log", category: "info", text: markup`Tag started here`},
			{
				type: "frame",
				location: openLocation,
			},
		],
	}),
});
