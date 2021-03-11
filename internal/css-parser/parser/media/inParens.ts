import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaInParens} from "@internal/ast";
import {parseMediaFeature} from "@internal/css-parser/parser/media/feature";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseMediaCondition} from "@internal/css-parser/parser/media/comparison";

export function parseMediaInParens(
	parser: CSSParser,
): CSSMediaInParens | undefined {
	// remove possible white spaces
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();

	if (token.type === "LeftParen") {
		const start = parser.getPosition();
		const maybeIdent = parser.nextToken();
		if (maybeIdent.type === "Ident" && maybeIdent.value === "not") {
			// TODO: refactor here to understand boolean or not
			const mediaCondition = parseMediaCondition(parser);
			if (mediaCondition) {
				// we now remove possible white spaces
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}
				const maybeRightParenToken = parser.getToken();
				// TODO: to handle when we don't have right parenthesis
				if (maybeRightParenToken.type === "RightParen") {
					parser.nextToken();
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
			// TODO: refactor here to understand boolean or not
			const feature = parseMediaFeature(parser);
			if (feature) {
				// we now remove possible white spaces
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}
				const maybeRightParenToken = parser.getToken();
				// TODO: to handle when we don't have right parenthesis
				if (maybeRightParenToken.type === "RightParen") {
					parser.nextToken();
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
