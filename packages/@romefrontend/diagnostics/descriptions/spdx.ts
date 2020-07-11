import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/string-markup";
import {buildSuggestionAdvice} from "../helpers";

// @romefrontend/codec-spdx-license
export const spdx = createDiagnosticsCategory({
	UNKNOWN_LICENSE: (id: string, knownLicenses: Array<string>) => ({
		message: markup`Unknown SPDX license <emphasis>${id}</emphasis>`,
		advice: buildSuggestionAdvice(id, knownLicenses),
	}),
	VALID_LICENSE_WITH_MISSING_DASH: (possibleCorrectLicense: string) => ({
		message: "Missing dash between SPDX license name and version",
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
});
