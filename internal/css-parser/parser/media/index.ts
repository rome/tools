import {
	CSSMediaQuery,
	CSSMediaQueryCondition,
	CSSMediaQueryList,
} from "@internal/ast";
import {CSSParser} from "@internal/css-parser/types";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaType} from "@internal/css-parser/parser/media/type";

function parseMedia(parser: CSSParser): CSSMediaQuery | undefined {
	const start = parser.getPosition();
	let condition: CSSMediaQueryCondition = undefined;
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
			return parser.finishNode(
				start,
				{
					type: "CSSMediaQuery",
					condition,
					value: mediaType,
				},
			);
		}
	}

	return undefined;
}

export function parseMediaList(parser: CSSParser): CSSMediaQueryList | undefined {
	const start = parser.getPosition();
	const list: CSSMediaQuery[] = [];
	// TODO: implement loop
	while (!(parser.matchToken("EOF") && parser.matchToken("LeftCurlyBracket"))) {
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
		parser.nextToken();
	}

	return parser.finishNode(
		start,
		{
			type: "CSSMediaQueryList",
			value: list,
		},
	);
}
