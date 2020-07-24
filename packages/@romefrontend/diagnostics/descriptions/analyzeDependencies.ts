import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/cli-layout";

export const analyzeDependencies = createDiagnosticsCategory({
	CJS_EXPORT_IN_ES: {
		category: "analyzeDependencies/cjsExportInES",
		message: markup`You cannot use CommonJS exports in an ES module`,
	},
});
