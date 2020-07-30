import {markup} from "@internal/markup";
import {createDiagnosticsCategory} from "./index";

// @internal/commit-parser
export const commitParser = createDiagnosticsCategory({
	UNEXPECTED_TOKEN: {message: markup`Unexpected commit token`},
	EMPTY_SCOPE: {message: markup`Empty commit scope`},
	MISSING_DESCRIPTION: {message: markup`Missing commit description`},
	MISSING_TYPE: {message: markup`Missing commit type`},
});
