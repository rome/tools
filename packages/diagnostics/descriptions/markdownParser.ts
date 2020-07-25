import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/cli-layout";

// @romefrontend/markdown-parser
export const markdownParser = createDiagnosticsCategory({
	INVALID_SEQUENCE: {message: markup`Invalid sequence`},
});
