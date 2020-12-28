import {createDiagnosticsCategory} from "./index";
import {DiagnosticSuppression} from "../types";
import {markup} from "@internal/markup";
import {joinCategoryName} from "../helpers";

export const suppressions = createDiagnosticsCategory({
	UNUSED: (suppression: DiagnosticSuppression) => {
		let description = markup``;
		if (suppression.startLine === suppression.endLine) {
			description = markup`on <emphasis>line ${suppression.startLine}</emphasis>`;
		} else {
			description = markup`between <emphasis>lines ${suppression.startLine} and ${suppression.endLine}</emphasis>`;
		}

		return {
			message: markup`Unused <emphasis>${joinCategoryName(suppression)}</emphasis> suppression`,
			category: "suppressions/unused",
			advice: [
				{
					type: "log",
					category: "info",
					text: markup`This suppression should have hidden a diagnostic ${description}`,
				},
			],
		};
	},
	MISSING_SPACE: {
		category: "suppressions/missingSpace",
		message: markup`Missing space between prefix and suppression categories`,
	},
	EMPTY: {
		category: "suppressions/empty",
		message: markup`This suppression comment doesn't include any categories to suppress!`,
	},
	MISSING_TARGET: {
		category: "suppressions/missingTarget",
		message: markup`We could not find a target for this suppression`,
	},
	MISSING_EXPLANATION: {
		category: "suppressions/missingExplanation",
		message: markup`Suppression comments must have an explanation`,
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
