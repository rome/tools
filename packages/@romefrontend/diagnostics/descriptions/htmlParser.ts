import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/string-markup";
import {DiagnosticLocation} from "../types";

// @romefrontend/html-parser
export const htmlParser = createDiagnosticsCategory({
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
});
