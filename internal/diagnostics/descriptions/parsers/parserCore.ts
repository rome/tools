import {createDiagnosticsCategory} from "../index";
import {markup} from "@internal/markup";

// @internal/parser-core
export const parserCore = createDiagnosticsCategory({
	UNEXPECTED_SPACE: {message: markup`Expected no space between`},
	UNEXPECTED_NEWLINE: {message: markup`Expected no lines between`},
	EXPECTED_NEWLINE: {message: markup`Expected a line between between`},
	EXPECTED_EOF: {message: markup`Expected end of file`},
	UNEXPECTED_EOF: {message: markup`Unexpected end of file`},
	UNEXPECTED: (type: string) => ({
		message: markup`Unexpected ${type}`,
	}),
	EXPECTED_COUNT: (name: string, expected: number, got: number) => {
		return {
			message: markup`Not enough ${name} characters, expected ${expected} but found ${got}`,
		};
	},
	INVALID_COUNT_CHAR: (name: string) => {
		return {
			message: markup`Invalid ${name} character`,
		};
	},
	UNEXPECTED_CHARACTER: (char: string, tokenType?: string) => {
		let message = markup`Unexpected character <emphasis>${char}</emphasis>`;
		if (tokenType !== undefined) {
			message = markup`${message} <dim>(${tokenType})</dim>`;
		}
		return {message};
	},
	UNEXPECTED_CHARACTERS: (str: string, tokenType?: string) => {
		return {
			message: markup`Unexpected ${tokenType ?? "characters"} <emphasis>${str}</emphasis>`,
		};
	},
	EXPECTED_TOKEN: (got: string, expected: string) => {
		return {
			message: markup`Expected token ${expected} but got ${got}`,
		};
	},
});
