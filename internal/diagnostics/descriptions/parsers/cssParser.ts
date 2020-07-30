import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

export const cssParser = createDiagnosticsCategory({
	INVALID_BLOCK_START: {message: markup`Invalid block start`},
	INVALID_DECLARATION: {message: markup`Invalid declaration`},
	INVALID_ESCAPE: {message: markup`Invalid escape sequence`},
	UNEXPECTED_TOKEN: {message: markup`Unexpected token`},
	UNTERMINATED_AT_RULE: {message: markup`Unterminated at-rule`},
	UNTERMINATED_BLOCK: {message: markup`Unterminated block`},
	UNTERMINATED_FUNCTION: {message: markup`Unterminated function`},
	UNTERMINATED_STRING: {message: markup`Unterminated string`},
	UNTERMINATED_URL: {message: markup`Unterminated URL`},
});
