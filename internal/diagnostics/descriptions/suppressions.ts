import {createDiagnosticsCategory} from "./index";
import {DiagnosticSuppression} from "../types";
import {markup} from "@internal/markup";
import {buildSuggestionAdvice, formatCategoryDescription} from "../helpers";
import {
	DIAGNOSTIC_CATEGORIES,
	VALID_DIAGNOSTIC_CATEGORIES,
} from "../categories";

export const suppressions = createDiagnosticsCategory({
	UNUSED: (suppression: DiagnosticSuppression) => {
		let description = markup``;
		if (suppression.startLine === suppression.endLine) {
			description = markup`on <emphasis>line ${suppression.startLine}</emphasis>`;
		} else {
			description = markup`between <emphasis>lines ${suppression.startLine} and ${suppression.endLine}</emphasis>`;
		}

		return {
			message: markup`Unused <emphasis>${formatCategoryDescription(suppression)}</emphasis> suppression`,
			category: DIAGNOSTIC_CATEGORIES["suppressions/unused"],
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
		category: DIAGNOSTIC_CATEGORIES["suppressions/missingSpace"],
		message: markup`Missing space between prefix and suppression categories`,
	},
	EMPTY: {
		category: DIAGNOSTIC_CATEGORIES["suppressions/empty"],
		message: markup`This suppression comment doesn't include any categories to suppress!`,
	},
	MISSING_TARGET: {
		category: DIAGNOSTIC_CATEGORIES["suppressions/missingTarget"],
		message: markup`We could not find a target for this suppression`,
	},
	MISSING_EXPLANATION: {
		category: DIAGNOSTIC_CATEGORIES["suppressions/missingExplanation"],
		message: markup`Suppression comments must have an explanation`,
	},
	INVALID_CATEGORY_NAME: (category: string) => ({
		category: DIAGNOSTIC_CATEGORIES["suppressions/invalidCategory"],
		message: markup`Unknown category <emphasis>${category}</emphasis>`,
		advice: buildSuggestionAdvice(
			category,
			Array.from(VALID_DIAGNOSTIC_CATEGORIES),
		),
	}),
	DUPLICATE: (category: string) => ({
		category: DIAGNOSTIC_CATEGORIES["suppressions/duplicate"],
		message: markup`Duplicate suppression category <emphasis>${category}</emphasis>`,
	}),
	OVERLAP: (category: string) => ({
		category: DIAGNOSTIC_CATEGORIES["suppressions/overlap"],
		message: markup`overlap suppression category <emphasis>${category}</emphasis>`,
	}),
	INCORRECT_SUPPRESSION_START: {
		category: DIAGNOSTIC_CATEGORIES["suppressions/incorrectSuppressionStart"],
		message: markup`This looks like a suppression comment typo. Did you mean <emphasis>rome-ignore</emphasis> instead?`,
	},
});
