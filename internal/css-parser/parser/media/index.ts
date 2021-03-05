import {
	CSSMediaConditionWithoutOr,
	CSSMediaQuery,
	CSSMediaQueryCondition,
	CSSMediaQueryList,
} from "@internal/ast";
import {CSSParser} from "@internal/css-parser/types";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaType} from "@internal/css-parser/parser/media/type";
import {parseMediaFeature} from "@internal/css-parser/parser/media/feature";

function tryParseConditionWithoutOr(
	parser: CSSParser,
): CSSMediaConditionWithoutOr | undefined {
	return undefined;
}

function parseMedia(parser: CSSParser): CSSMediaQuery | undefined {
	const start = parser.getPosition();
	let condition: CSSMediaQueryCondition = undefined;
	let conditionWithoutOr: CSSMediaConditionWithoutOr | undefined = undefined;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();

	if (token.type === "Ident") {
		if (token.value === "not") {
			condition = "not";
			parser.nextToken();
		} else if (token.value === "only") {
			condition = "only";
			parser.nextToken();
		}

		const mediaType = parseMediaType(parser);

		if (mediaType) {
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
	} else if (token.type === "LeftParen") {
		const mediaFeature = parseMediaFeature(parser);

		if (mediaFeature) {
			// return parser.finishNode(
			// 	start,
			// 	{
			// 		type: "CSSMediaQuery",
			// 		value: mediaFeature,
			// 	},
			// );
		}
	}

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
