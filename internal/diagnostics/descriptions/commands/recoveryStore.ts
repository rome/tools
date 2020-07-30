import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import stringDiff from "@internal/string-diff";

export const recoveryStore = createDiagnosticsCategory({
	NOT_FOUND: (storeId: string) => ({
		message: markup`Could not find recovery store <emphasis>${storeId}</emphasis>`,
		category: "recoveryStore/notFound",
	}),
	DIFF: (original: string, artifact: string) => ({
		message: markup`Differences between saved and current file`,
		category: "recoveryStore/diff",
		advice: [
			{
				type: "diff",
				language: "unknown",
				diff: stringDiff(original, artifact),
				legend: {
					add: "Saved file",
					delete: "Current file",
				},
			},
		],
	}),
});
