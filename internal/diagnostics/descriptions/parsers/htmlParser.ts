import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {DiagnosticLocation} from "../../types";

// @internal/html-parser
export const htmlParser = createDiagnosticsCategory({
	UNCLOSED_STRING: {message: markup`Unclosed string`},
	EXPECTED_CLOSING_TAG_NAME: {message: markup`Expected closing tag name`},
	UNKNOWN_START: {message: markup`Unknown child start`},
	EXPECTED_ATTRIBUTE_NAME: {message: markup`Expected attribute name`},
	INCORRECT_CLOSING_TAG_NAME: (expected: string, got: string) => ({
		message: markup`Expected to close ${expected} but found ${got}`,
	}),
	UNOPENED_TAG: {message: markup`Ending tag with no opening tag`},
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
