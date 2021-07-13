import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaInParens} from "@internal/ast";
import {parseMediaFeature} from "@internal/css-parser/parser/media/feature";
import {
	matchToken,
	nextToken,
	readToken,
	skipWhitespaces,
} from "@internal/css-parser/tokenizer";
import {parseMediaCondition} from "@internal/css-parser/parser/media/comparison";
import {NOT} from "@internal/css-parser/utils";

export function parseMediaInParens(
	parser: CSSParser,
): CSSMediaInParens | undefined {
	// remove possible white spaces
	skipWhitespaces(parser);
	const token = parser.getToken();

	if (token.type === "LeftParen") {
		const start = parser.getPosition();
		const maybeIdent = nextToken(parser);
		if (maybeIdent.type === "Ident" && maybeIdent.value === NOT) {
			// TODO: refactor here to understand boolean or not
			const mediaCondition = parseMediaCondition(parser);
			if (mediaCondition) {
				// we now remove possible white spaces
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}
				const maybeRightParenToken = parser.getToken();
				if (maybeRightParenToken.type === "RightParen") {
					nextToken(parser);
					return parser.finishNode(
						start,
						{
							type: "CSSMediaInParens",
							value: mediaCondition,
						},
					);
				}
			}
		} else {
			const feature = parseMediaFeature(parser);
			if (feature) {
				// we now remove possible white spaces
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}
				const maybeRightParenToken = parser.getToken();
				if (maybeRightParenToken.type === "RightParen") {
					nextToken(parser);
					return parser.finishNode(
						start,
						{
							type: "CSSMediaInParens",
							value: feature,
						},
					);
				}
			}
		}
	}

	return undefined;
}
