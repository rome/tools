import {CSSParser} from "@internal/css-parser/types";
import {
	CSSMediaAnd,
	CSSMediaInParens,
	CSSMediaNot,
	CSSMediaOr,
} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaInParens} from "@internal/css-parser/parser/media/inParens";
import {descriptions} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";
import {AND, CONDITIONS, NOT, OR} from "@internal/css-parser/utils";

export function isCondition(value: string) {
	return CONDITIONS.includes(value);
}

function parseCondition(
	parser: CSSParser,
	keyword: NOT | AND | OR,
	keywordIsPast?: boolean,
): CSSMediaInParens | undefined {
	const token = parser.getToken();
	// there are cases where sometime we already past the check of the keyword
	// in these cases, we go straight parsing the the media
	if (keywordIsPast) {
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		return parseMediaInParens(parser);
	} else if (token.type === "Ident" && token.value === keyword) {
		// move forward
		nextToken(parser);
		// remove white spaces between keyword and next important token
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		return parseMediaInParens(parser);
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.MEDIA_QUERY_FEATURE_EXPECTED_KEYWORD(
			keyword,
		),
		token,
	});
	nextToken(parser);

	return undefined;
}

export function parseMediaNot(
	parser: CSSParser,
	startOfNotToken?: Position,
): CSSMediaNot | undefined {
	const start = startOfNotToken ?? parser.getPosition();

	const value = parseCondition(parser, NOT, !!startOfNotToken);

	if (value) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaNot",
				value,
			},
		);
	}
	return undefined;
}

export function parseMediaAnd(parser: CSSParser): CSSMediaAnd | undefined {
	const start = parser.getPosition();

	const value = parseCondition(parser, AND);

	if (value) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaAnd",
				value,
			},
		);
	}
	return undefined;
}

export function parseMediaOr(parser: CSSParser): CSSMediaOr | undefined {
	const start = parser.getPosition();

	const value = parseCondition(parser, OR);

	if (value) {
		return parser.finishNode(
			start,
			{
				type: "CSSMediaOr",
				value,
			},
		);
	}
	return undefined;
}
