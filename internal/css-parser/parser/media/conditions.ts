import {CSSParser} from "@internal/css-parser/types";
import {
	CSSMediaAnd,
	CSSMediaInParens,
	CSSMediaNot,
	CSSMediaOr,
} from "@internal/ast";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaInParens} from "@internal/css-parser/parser/media/inParens";
import {descriptions} from "@internal/diagnostics";
import {Position} from "@internal/parser-core";

const CONDITIONS = ["not", "and", "or"];

export function isCondition(value: string) {
	return CONDITIONS.includes(value);
}

function parseCondition(
	parser: CSSParser,
	keyword: "not" | "and" | "or",
	keywordIsPast?: boolean
): CSSMediaInParens | undefined {
	const token = parser.getToken();

	if (keywordIsPast) {
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		return parseMediaInParens(parser);
	} else if (token.type === "Ident" && token.value === keyword) {
		// move forward
		parser.nextToken();
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
	parser.nextToken();

	return undefined;
}

export function parseMediaNot(parser: CSSParser, 	startOfNotToken?: Position): CSSMediaNot | undefined {
	const start = startOfNotToken ?? parser.getPosition();

	const value = parseCondition(parser, "not", !!startOfNotToken);

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

	const value = parseCondition(parser, "and");

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

	const value = parseCondition(parser, "or");

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
