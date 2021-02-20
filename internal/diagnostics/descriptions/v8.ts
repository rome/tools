import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";
import {DIAGNOSTIC_CATEGORIES} from "@internal/diagnostics";

export const v8 = createDiagnosticsCategory({
	SYNTAX_ERROR: (message: string) => ({
		message: markup`${message}`,
		category: DIAGNOSTIC_CATEGORIES["v8/syntaxError"],
	}),
});
