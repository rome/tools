import {createDiagnosticsCategory} from "./index";

export const analyzeDependencies = createDiagnosticsCategory({
	CJS_EXPORT_IN_ES: {
		category: "analyzeDependencies/cjsExportInES",
		message: "You cannot use CommonJS exports in an ES module",
	},
});
