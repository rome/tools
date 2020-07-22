import {createDiagnosticsCategory} from "./index";
import {markup} from "@romefrontend/cli-layout";

export const cssParser = createDiagnosticsCategory({
	INVALID_ESCAPE: {message: markup`Invalid escape sequence`},
	UNTERMINATED_STRING: {message: markup`Unterminated string`},
	UNTERMINATED_URL: {message: markup`Unterminated URL`},
});
