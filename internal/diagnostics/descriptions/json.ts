import {createDiagnosticsCategory} from "./index";
import {markup} from "@internal/markup";

// @internal/codec-json
export const json = createDiagnosticsCategory({
	SINGLE_QUOTE_USAGE: {message: markup`You can only use double quoted strings`},
	TRAILING_COMMA_VALUE: {
		message: markup`Trailing comma is only allowed after a value`,
	},
	UNCLOSED_STRING: {message: markup`Unclosed string`},
	UNCLOSED_BLOCK_COMMENT: {message: markup`Unclosed block comment`},
	MISTAKEN_ARRAY_IDENTITY: {
		message: markup`Trying to use an array element as an object property. Did you mean to make an object?`,
	},
	REDUNDANT_COMMA: {message: markup`Redundant comma`},
	EMPTY_INPUT_IN_JSON: {message: markup`Empty input`},
	PROPERTY_KEY_UNQUOTED_IN_JSON: {
		message: markup`Property keys must be quoted in JSON`,
	},
	IMPLICIT_OBJECT_IN_JSON: {
		message: markup`Objects must be wrapped in curly braces in JSON`,
	},
	COMMENTS_IN_JSON: {message: markup`Comments aren't allowed in JSON`},
	TRAILING_COMMA_IN_JSON: {
		message: markup`Trailing commas aren't allowed in JSON`,
	},
	REGEX_IN_JSON: {message: markup`Regular expressions aren't allowed in JSON`},
	UNKNOWN_WORD_IN_JSON: (word: string) => ({
		message: markup`${word} isn't a valid JSON word`,
	}),
	STRING_NEWLINES_IN_JSON: {
		message: markup`Newlines aren't allowed in JSON, you insert a newline by escaping it like this "\\n"`,
	},
	UNDEFINED_IN_JSON: {
		message: markup`undefined isn't allowed in JSON, you could use null instead`,
	},
	BIGINT_IN_JSON: {message: markup`Bigints aren't allowed in JSON`},
	NUMERIC_SEPARATORS_IN_JSON: {
		message: markup`Numeric separators are not allowed in JSON`,
	},
});
