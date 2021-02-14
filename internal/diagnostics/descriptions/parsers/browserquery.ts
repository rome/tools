import {createDiagnosticsCategory} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const browserquery = createDiagnosticsCategory({
	EXPECTED_OPERATOR_OR_VERSION: {
		message: markup`Expected an operator such as <emphasis>></emphasis>`
	},
});
