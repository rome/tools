import {createDiagnosticsCategory} from "./index";
import {markup} from "@romejs/string-markup";

// @romejs/parser-core
export const parserCore = createDiagnosticsCategory({
	EXPECTED_SPACE: "Expected no space between",
	EXPECTED_EOF: "Expected end of file",
	UNEXPECTED_EOF: "Unexpected end of file",
	UNEXPECTED: (type: string) => ({
		message: markup`Unexpected ${type}`,
	}),
	UNEXPECTED_CHARACTER: (char: string) => ({
		message: markup`Unexpected character <emphasis>${char}</emphasis>`,
	}),
	EXPECTED_TOKEN: (got: string, expected: string) => {
		return {
			message: markup`Expected token ${expected} but got ${got}`,
		};
	},
});
