import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const recoveryStore = createDiagnosticsCategory({
	NOT_FOUND: (storeId: string) => ({
		message: markup`Could not find recovery store <emphasis>${storeId}</emphasis>`,
		category: DIAGNOSTIC_CATEGORIES["recoveryStore/notFound"],
	}),
	DIFF: (original: string, artifact: string) => ({
		message: markup`Differences between saved and current file`,
		category: DIAGNOSTIC_CATEGORIES["recoveryStore/diff"],
		advice: [
			{
				type: "diff-strings",
				language: "unknown",
				before: original,
				after: artifact,
				legend: {
					add: "Saved file",
					delete: "Current file",
				},
			},
		],
	}),
});
