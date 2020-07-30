import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/markdown-parser
export const markdownParser = createDiagnosticsCategory({
	INVALID_SEQUENCE: {message: markup`Invalid sequence`},
});
