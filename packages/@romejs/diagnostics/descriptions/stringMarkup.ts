import {addEmphasis, createDiagnosticsCategory, orJoin} from "./index";
import {DiagnosticLocation} from "../types";
import {markup} from "@romejs/string-markup";
import {buildSuggestionAdvice} from "../helpers";

// @romejs/string-markup
export const stringMarkup = createDiagnosticsCategory({
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
});
