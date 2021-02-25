import {createDiagnosticsCategory} from "../index";
import {buildSuggestionAdvice} from "../../helpers";
import {markup} from "@internal/markup";

export const cssParser = createDiagnosticsCategory({
	INVALID_BLOCK_START: {message: markup`Invalid block start.`},
	INVALID_DECLARATION: {message: markup`Invalid declaration.`},
	INVALID_ESCAPE: {message: markup`Invalid escape sequence.`},
	UNEXPECTED_TOKEN: {message: markup`Unexpected token.`},
	UNTERMINATED_AT_RULE: {message: markup`Unterminated at-rule.`},
	UNTERMINATED_BLOCK: {message: markup`Unterminated block.`},
	UNTERMINATED_FUNCTION: {message: markup`Unterminated function.`},
	UNTERMINATED_STRING: {message: markup`Unterminated string.`},
	UNTERMINATED_URL: {message: markup`Unterminated URL.`},
	EXPECTED_IDENTIFIER: {message: markup`Expected an identifier.`},
	EXPECTED_SELECTOR: {message: markup`Expected a selector.`},
	EXPECTED_LBRACKET: {
		message: markup`Expected a left curly bracket <emphasis>{</emphasis>.`,
	},
	UNEXPECTED_EMPTY_SELECTOR: {message: markup`Unexpected empty selectors.`},
	EXPECTED_CLOSING_ATTRIBUTE_SELECTOR: {
		message: markup`Expected to close attribute selector with a right square bracket <emphasis>]</emphasis>.`,
	},
	UNKNOWN_ATTRIBUTE_MATCHER: (matcher: string, validMatchers: string[]) => ({
		message: markup`Unknown attribute matcher.`,
		advice: buildSuggestionAdvice(matcher, validMatchers, {minRating: 0}),
	}),
	UNKNOWN_ATTRIBUTE_MODIFIER: {message: markup`Unknown attribute modifier.`},
	INVALID_CUSTOM_PROPERTY: {
		message: markup`Invalid custom property found inside the "var" function.`,
	},
	MISSING_KEYFRAME_NAME: {
		message: markup`The keyframe must have a name.`,
	},
	INVALID_IDENTIFIER: (ident: string, invalidIdents: string[]) => ({
		message: markup`The identifier <emphasis>${ident}</emphasis> can't be used here.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`In this position, the words ${invalidIdents.map((i) =>
					markup`<emphasis>${i}</emphasis>, `
				)} are CSS-wide keywords, so they are reserved.`,
			},
		],
	}),
	UNKNOWN_KEYFRAME_SELECTOR_NAME: {
		message: markup`The selector name of the keyframe is not correct.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The only valid values are percentages, and the keywords <emphasis>from</emphasis> and <emphasis>to</emphasis>.`,
			},
		],
	},

	INVALID_KEYFRAME_SELECTOR_NAME: (
		wrongKeyword: string,
		validMatchers: string[],
	) => ({
		message: markup`The keyword <emphasis>${wrongKeyword}</emphasis> is not accepted as valid keyframe name.`,
		advice: buildSuggestionAdvice(
			wrongKeyword,
			validMatchers,
			{minRating: 0, ignoreCase: false},
		),
	}),

	URL_FUNCTION_TOO_MANY_PARAMETERS: {
		message: markup`The function <emphasis>url()</emphasis> can only accept one parameter.`,
	},

	URL_FUNCTION_INVALID_VALUE: {
		message: markup`The function <emphasis>url()</emphasis> can only accept strings.`,
	},

	CALC_VALUE_NOT_RECOGNISED: {
		message: markup`The function <emphasis>calc()</emphasis> doesn't this token`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`<emphasis>calc()</emphasis> supports only numbers, percentages, dimensions or other sums.`,
			},
		],
	},

	CALC_MISSING_LEFT_SPACE: {
		message: markup`Missing left space.`,
	},

	CALC_MISSING_RIGHT_SPACE: {
		message: markup`Missing right space.`,
	},

	CALC_OPERATOR_ADD_OR_PLUS_NEEDED: {
		message: markup`An operator is needed.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Consider adding <emphasis>+ or -</emphasis>`,
			},
		],
	},

	CALC_OPERATOR_TIMES_OR_MOD_NEEDED: {
		message: markup`An operator is needed.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Consider adding <emphasis>* or /</emphasis>`,
			},
		],
	},

	CALC_UNTERMITED_SUM: {
		message: markup`The character <emphasis>)</emphasis> is missing, the expression is not completed.`,
	},

	CALC_INCORRECT_NUMBER_VALUE: {
		message: markup`Incorrect character, expected a number or a parenthesis`,
	},
	EXPECTED_ID_HASH: {
		message: markup`Expected to use identifier after <emphasis>#</emphasis>.`,
	},
});
