import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/string-escape
export const stringEscape = createDiagnosticsCategory({
	INVALID_STRING_CHARACTER: {
		message: markup`Invalid string character (U+0000 to U+001F)`,
	},
	TOML_INVALID_UNICODE_POINT: {
		message: markup`Invalid unicode codepoint`,
	},
	TOML_INVALID_ESCAPE: {
		message: markup`Unknown character escape`,
	},
	TOML_NEWLINE_IN_SINGLE_QUOTE_STRING: {
		message: markup`Cannot have newlines in a single quote string`,
	},
});
