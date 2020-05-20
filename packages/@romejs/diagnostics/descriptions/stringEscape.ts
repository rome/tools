import {createDiagnosticsCategory} from "./index";

// @romejs/string-escape
export const stringEscape = createDiagnosticsCategory({
	NOT_ENOUGH_CODE_POINTS: "Not enough code point digits",
	INVALID_STRING_CHARACTER: "Invalid string character (U+0000 to U+001F)",
	INVALID_HEX_DIGIT_FOR_ESCAPE: "Invalid hex digit for unicode escape",
});
