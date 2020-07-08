import {createDiagnosticsCategory} from "./index";

export const cssParser = createDiagnosticsCategory({
	INVALID_ESCAPE: "Invalid escape sequence",
	UNTERMINATED_STRING: "Unterminated string",
	UNTERMINATED_URL: "Unterminated URL",
});
