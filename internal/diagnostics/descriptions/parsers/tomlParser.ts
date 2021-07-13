import {createDiagnosticsCategory} from "@internal/diagnostics";
import {markup} from "@internal/markup";

export const toml = createDiagnosticsCategory({
	UNCLOSED_STRING: {message: markup`Unclosed string`},
	DUPLICATE_DECLARATION: {message: markup`Duplicate key`},
	BAD_ARRAY_TYPE: {message: markup`Bad array value type`},
	BAD_TABLE_TYPE: {message: markup`Bad table value type`},
	TRAILING_NUMBER_UNDERSCORE: {message: markup`Trailing number underscore`},
	LEADING_NUMBER_UNDERSCORE: {message: markup`Leading number underscore`},
	DOUBLE_NUMBER_UNDERSCORE: {message: markup`Double number underscore`},
	TRAILING_KEY_DOT: {message: markup`Trailing key dot`},
	EXCESSIVE_PLUS: {message: markup`Excessive plus`},
	EXCESSIVE_MINUS: {message: markup`Excessive minus`},
	UNKNOWN_WORD: {message: markup`Unknown word`},
	UNKNOWN_ARRAY_SEPARATOR: {message: markup`Unknown array separator`},
	TRAILING_INLINE_TABLE_COMMA: {message: markup`Trailing inline table comma`},
	INVALID_KEY_CHAR: (char: string) => ({
		message: markup`The character <emphasis>${char}</emphasis> isn't a valid character in a key`,
	}),
	NO_VALUE_FOR_KEY: (keyName: string) => ({
		message: markup`The key <emphasis>${keyName}</emphasis> doesn't have any value`,
	}),
	VALUE_NOT_RECOGNISED: (keyName: string) => ({
		message: markup`Unable to parse the value associated to key <emphasis>${keyName}</emphasis>`,
	}),
	UNCLOSED_VALUE: (value: string) => ({
		message: markup`Text string <emphasis>${value}<emphasis> doesn't have a closing quote.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Add a quote (") at the end`,
			},
		],
	}),
});
