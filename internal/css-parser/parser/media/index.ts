import {
	CSSMediaConditionWithoutOr,
	CSSMediaConditionWithoutOrWithParens,
	CSSMediaQuery,
	CSSMediaQueryCondition,
	CSSMediaQueryList,
} from "@internal/ast";
import {CSSParser} from "@internal/css-parser/types";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaType} from "@internal/css-parser/parser/media/type";
import {parseMediaInParens} from "@internal/css-parser/parser/media/inParens";
import {
	parseMediaAnd,
	parseMediaNot,
} from "@internal/css-parser/parser/media/conditions";
import {descriptions} from "@internal/diagnostics";
import {parseMediaCondition} from "@internal/css-parser/parser/media/comparison";

function tryParseConditionWithoutOr(
	parser: CSSParser,
): CSSMediaConditionWithoutOr | undefined {
	// the start should be from "and" keyword
	const start = parser.getPosition();

	parser.nextToken();

	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	const token = parser.getToken();

	if (token.type === "Ident") {
		if (token.value === "not") {
			const mediaNot = parseMediaNot(parser);
			if (mediaNot) {
				return parser.finishNode(
					start,
					{
						type: "CSSMediaConditionWithoutOr",
						value: mediaNot,
					},
				);
			}
		}
	} else if (token.type === "LeftParen") {
		let value: CSSMediaConditionWithoutOrWithParens;
		const feature = parseMediaInParens(parser);
		if (feature) {
			value = [feature];
			while (true) {
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}

				const token = parser.getToken();

				if (token.type === "Ident" && token.value === "and") {
					const mediaAnd = parseMediaAnd(parser);
					if (mediaAnd) {
						value.push(mediaAnd);
					}
				} else {
					break;
				}
			}

			return parser.finishNode(
				start,
				{
					type: "CSSMediaConditionWithoutOr",
					value,
				},
			);
		}
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MEDIA_QUERY_EXPECTED_NOT_OR_PARENTHESIS,
			token,
		});
	}

	return undefined;
}

function parseMedia(parser: CSSParser): CSSMediaQuery | undefined {
	const start = parser.getPosition();
	let condition: CSSMediaQueryCondition | undefined = undefined;
	let conditionWithoutOr: CSSMediaConditionWithoutOr | undefined = undefined;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	let hasNot = false;
	const token = parser.getToken();

	// both AST nodes media-condition
	// and token before media-type can start
	// with the word "not"
	// this means we need to make some checks ahead
	if (token.type === "Ident") {
		if (token.value === "not") {
			condition = "not";
			parser.nextToken();
			hasNot = true;
		} else if (token.value === "only") {
			condition = "only";
			parser.nextToken();
		}
	}

	function goAndParseMediaType() {
		const mediaType = parseMediaType(parser);

		if (mediaType) {
			// moving forward and removing white spaces
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			const token = parser.getToken();
			if (token.type === "Ident" && token.value === "and") {
				conditionWithoutOr = tryParseConditionWithoutOr(parser);
			}

			return parser.finishNode(
				start,
				{
					type: "CSSMediaQuery",
					condition,
					conditionWithoutOr,
					value: mediaType,
				},
			);
		}
		return undefined
	}

	// it doesn't have the not word, so we can safely start parsing the media type
	if (!hasNot) {
		const mediaType = parseMediaType(parser);

		if (mediaType) {
			return goAndParseMediaType();
		}
	} else {
		// let's remove spaces
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		const token = parser.nextToken();

		// if current token is a parenthesis, it means we have a media condition
		// else, we go and parse everything as a media type
		if (token.type === "LeftParen") {
			const mediaCondition = parseMediaCondition(parser);

			if (mediaCondition) {
				return parser.finishNode(
					start,
					{
						type: "CSSMediaQuery",
						condition,
						conditionWithoutOr,
						value: mediaCondition,
					},
				);
			}
		} else {

			return goAndParseMediaType();
		}
	}

	// } else if (token.type === "LeftParen") {

	// }

	return undefined;
}

export function parseMediaList(parser: CSSParser): CSSMediaQueryList | undefined {
	const start = parser.getPosition();
	const list: CSSMediaQuery[] = [];
	// TODO: implement loop
	const media = parseMedia(parser);
	if (media) {
		list.push(media);
	}
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	while (!(parser.matchToken("EOF") && parser.matchToken("LeftCurlyBracket"))) {
		if (parser.matchToken("Comma")) {
			parser.nextToken();
		}
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		if (parser.matchToken("LeftCurlyBracket")) {
			break;
		}
		const media = parseMedia(parser);
		if (media) {
			list.push(media);
		}
	}

	return parser.finishNode(
		start,
		{
			type: "CSSMediaQueryList",
			value: list,
		},
	);
}
