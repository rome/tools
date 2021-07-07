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
	UNTERMINATED_FUNCTION: {
		message: markup`Unterminated function. Please add a right parenthesis.`,
	},
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
		message: markup`The function <emphasis>calc()</emphasis> doesn't support this token`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`<emphasis>calc()</emphasis> supports only numbers, percentages, dimensions or other sums.`,
			},
		],
	},

	CALC_MISSING_SPACES: {
		message: markup`The operators <emphasis>+</emphasis> and <emphasis>-</emphasis> need to have a space left and right`,
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

	FIT_CONTENT_UNKOWN_FUNCTION: {
		message: markup`Unknown unit on the <emphasis>fit-content()</emphasis>.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`The only valid values are dimensions, percentages or numbers.`,
			},
		],
	},

	CALC_INCORRECT_NUMBER_VALUE: {
		message: markup`Incorrect character, expected a number or a parenthesis.`,
	},
	EXPECTED_ID_HASH: {
		message: markup`Expected the use of an identifier after <emphasis>#</emphasis>.`,
	},

	MEDIA_QUERY_UNKNOWN_MEDIA_TYPES: (
		wrongValue: string,
		supportedFeatures: string[],
	) => ({
		message: markup`Unknown media type provided to the media query.`,
		advice: buildSuggestionAdvice(wrongValue, supportedFeatures, {minRating: 0}),
	}),

	MEDIA_QUERY_DEPRECATED_MEDIA_TYPE: (wrongValue: string) => ({
		message: markup`The media type <emphasis>${wrongValue}</emphasis> is deprecated.`,
	}),
	MEDIA_QUERY_MALFORMED_RANGE: {
		message: markup`The range value provided to the media query is not correct`,
	},
	MEDIA_QUERY_EXPECTED_PARENTHESIS: {
		message: markup`A left parenthesis is expected in this position.`,
	},
	MEDIA_QUERY_EXPECTED_COMPARISON: {
		message: markup`The comparison is not correct, only <emphasis> \<, \> and = </emphasis> are valid.`,
	},
	MEDIA_QUERY_EXPECTED_NOT_OR_PARENTHESIS: {
		message: markup`A left parenthesis or the keyword <emphasis>not</emphasis> are expected in this position.`,
	},

	MEDIA_QUERY_FEATURE_UNEXPECTED_VALUE: {
		message: markup`The value provided inside the media feature is not correct.`,
	},

	MEDIA_QUERY_FEATURE_EXPECTED_KEYWORD: (keyword: string) => ({
		message: markup`The keyword <emphasis>${keyword}</emphasis> is expected in this position.`,
	}),

	MEDIA_QUERY_FEATURE_MALFORMED_PLAN: {
		message: markup`The media feature is not grammatically correct`,
	},

	MEDIA_QUERY_FEATURE_INCORRENT_RATIO: {
		message: markup`The ratio value is not syntactically correct; you always have to provide a denominator after the slash.`,
	},

	AT_SUPPORTS_MALFORMED: {
		message: markup`The rule @supports is not syntactically correct`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`At this position, it's expected a <emphasis>parenthesis</emphasis> or a <emphasis>condition ("or", "and" or "not")</emphasis>`,
			},
		],
	},

	AT_PAGE_MALFORMED: {
		message: markup`No whitespace is allowed between selector and pseudo page.`,
	},
	AT_PAGE_INVALID_PSEUDO_PAGE: {
		message: markup`Pseudo page can accept only <emphasis>left, right, first or blank</emphasis> and there can't be spaces in between`,
	},

	AT_PAGE_INVALID_DECLARATION: (wrongIdent, validProperties) => ({
		message: markup`The at-rule <emphasis>@page</emphasis> accepts only a certain number of properties and <emphasis>${wrongIdent}</emphasis> is not one of them.`,
		advice: buildSuggestionAdvice(
			wrongIdent,
			validProperties,
			{minRating: 0, ignoreCase: false},
		),
	}),

	AT_PAGE_AT_RULE_INVALID_DECLARATION: (
		currentAtRule,
		wrongIdent,
		validProperties,
	) => ({
		message: markup`The at-rule <emphasis>@${currentAtRule}</emphasis> accepts only a certain number of properties and <emphasis>${wrongIdent}</emphasis> is not one of them.`,
		advice: buildSuggestionAdvice(
			wrongIdent,
			validProperties,
			{minRating: 0, ignoreCase: false},
		),
	}),

	AT_FONT_FACE_MISSING_SRC: {
		message: markup`The rule <emphasis>@font-face</emphasis> needs the property <emphasis>src</emphasis> in order to be valid.`,
	},

	AT_IMPORT_INVALID_ARGUMENT: {
		message: markup`Unexpected argument in at-import`,
	},

	MIN_MAX_INVALID_ARGUMENTS: {
		message: markup`The function <emphasis>minmax()</emphasis> requires two arguments or the arguments provided are not supported.`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use <emphasis>min-content</emphasis>, <emphasis>max-content</emphasis> or <emphasis>auto</emphasis>`,
			},

			{
				type: "log",
				category: "info",
				text: markup`Use use lengths or dimensions. e.g. em, fr, px, %, etc.`,
			},
		],
	},

	MIN_MAX_INVALID_FLEX_ARGUMENT: {
		message: markup`A flex argument is permitted only as <emphasis>second argument</emphasis> of the function <emphasis>minmax()</emphasis>`,
	},

	GRID_REPEAT_POSITIVE_INTEGER: {
		message: markup`Only positive numbers are accepted`,
	},

	GRID_REPEAT_TRACK_REPEAT_MANDATORY: {
		message: markup`The positive number must be followed by a comma`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`A track-repeat must specify what to repeat`,
			},
		],
	},
	GRID_REPEAT_INCORRECT_IDENT: {
		message: markup`The only accepted identifiers are <emphasis>auto-fill</emphasis> and <emphasis>auto-fit</emphasis>`,
	},

	GRID_REPEAT_UNTERMINATED: {
		message: markup`The <emphasis>repeat()</emphasis> doesn't have any value to repeat.`,
	},

	GRID_REPEAT_EXPECTED_IDENTIFIER: {
		message: markup`Only identifiers are expected at this position.`,
	},

	GRID_REPEAT_UNCLOSED_LINE_NAME: (ident: string) => ({
		message: markup`The line name doesn't have a closing square bracket`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Add a <emphasis>]</emphasis> after <emphasis>${ident}</emphasis>.`,
			},
		],
	}),

	GRID_REPEAT_WRONG_FUNCTION: {
		message: markup`Unsupported function at this position`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use <emphasis>minmax()</emphasis> or <emphasis>fit-content()</emphasis>.`,
			},
		],
	},

	GRID_REPEAT_WRONG_IDENTIFIER: {
		message: markup`Unsupported identifier`,
		advice: [
			{
				type: "log",
				category: "info",
				text: markup`Use <emphasis>min-content</emphasis> or <emphasis>max-content</emphasis>.`,
			},
		],
	},
});
