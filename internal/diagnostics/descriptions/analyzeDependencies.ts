import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const analyzeDependencies = createDiagnosticsCategory({
	CJS_EXPORT_IN_ES: {
		category: DIAGNOSTIC_CATEGORIES["analyzeDependencies/cjsExportInES"],
		message: markup`You cannot use CommonJS exports in an ES module`,
	},
});
