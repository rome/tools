import {createDiagnosticsCategory} from "./index";
import {DiagnosticSuppression} from "../types";
import {markup} from "@internal/markup";

export const suppressions = createDiagnosticsCategory({
	UNUSED: (suppression: DiagnosticSuppression) => {
		let description = "";
		if (suppression.startLine === suppression.endLine) {
			description = `line ${suppression.startLine}`;
		} else {
			description += `lines ${suppression.startLine} to ${suppression.endLine}`;
		}

		return {
			message: markup`Unused suppression. Did not hide any errors.`,
			category: "suppressions/unused",
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`This suppression should hide <emphasis>${description}</emphasis>`,
				},
			],
		};
	},
	MISSING_SPACE: {
		category: "suppressions/missingSpace",
		message: markup`Missing space between prefix and suppression categories`,
	},
	MISSING_TARGET: {
		category: "suppressions/missingTarget",
		message: markup`We could not find a target for this suppression`,
	},
	DUPLICATE: (category: string) => ({
		category: "suppressions/duplicate",
		message: markup`Duplicate suppression category <emphasis>${category}</emphasis>`,
	}),
	OVERLAP: (category: string) => ({
		category: "suppressions/overlap",
		message: markup`overlap suppression category <emphasis>${category}</emphasis>`,
	}),
	INCORRECT_SUPPRESSION_START: {
		category: "suppressions/incorrectSuppressionStart",
		message: markup`This looks like a suppression comment typo. Did you mean <emphasis>rome-ignore</emphasis> instead?`,
	},
});
