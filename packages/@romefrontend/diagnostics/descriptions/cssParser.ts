import {createDiagnosticsCategory} from "./index";

export const cssParser = createDiagnosticsCategory({
	INVALID_BLOCK_START: "Invalid block start",
	INVALID_DECLARATION: "Invalid declaration",
	INVALID_ESCAPE: "Invalid escape sequence",
	UNEXPECTED_TOKEN: "Unexpected token",
	UNTERMINATED_AT_RULE: "Unterminated at-rule",
	UNTERMINATED_BLOCK: "Unterminated block",
	UNTERMINATED_FUNCTION: "Unterminated function",
	UNTERMINATED_STRING: "Unterminated string",
	UNTERMINATED_URL: "Unterminated URL",
});
