import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/parser-core
export const parserCore = createDiagnosticsCategory({
	EXPECTED_SPACE: {message: markup`Expected no space between`},
	EXPECTED_EOF: {message: markup`Expected end of file`},
	UNEXPECTED_EOF: {message: markup`Unexpected end of file`},
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
