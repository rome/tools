import {createDiagnosticsCategory} from "./index";
import {DiagnosticSuppression} from "../types";
import {markup} from "@romefrontend/string-markup";

export const suppressions = createDiagnosticsCategory({
	UNUSED: (suppression: DiagnosticSuppression) => {
		let description = "";
		if (suppression.startLine === suppression.endLine) {
			description = `line ${suppression.startLine}`;
		} else {
			description += `lines ${suppression.startLine} to ${suppression.endLine}`;
		}

		return {
			message: "Unused suppression. Did not hide any errors.",
			category: "suppressions/unused",
			advice: [
				{
					type: "log",
					category: "info",
					text: `This suppression should hide <emphasis>${description}</emphasis>`,
				},
			],
		};
	},
	MISSING_SPACE: {
		category: "suppressions/missingSpace",
		message: "Missing space between prefix and suppression categories",
	},
	MISSING_TARGET: {
		category: "suppressions/missingTarget",
		message: "We could not find a target for this suppression",
	},
	DUPLICATE: (category: string) => ({
		category: "suppressions/duplicate",
		message: markup`Duplicate suppression category <emphasis>${category}</emphasis>`,
	}),
});
