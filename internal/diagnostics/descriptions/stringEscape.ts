import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/string-escape
export const stringEscape = createDiagnosticsCategory({
	NOT_ENOUGH_CODE_POINTS: {message: markup`Not enough code point digits`},
	INVALID_STRING_CHARACTER: {
		message: markup`Invalid string character (U+0000 to U+001F)`,
	},
	INVALID_HEX_DIGIT_FOR_ESCAPE: {
		message: markup`Invalid hex digit for unicode escape`,
	},
});
