import {CSSParser} from "@internal/css-parser/types";
import {CSSMediaInParens} from "@internal/ast";
import {parseMediaFeature} from "@internal/css-parser/parser/media/feature";

export function parseMediaInParens(
	parser: CSSParser,
): CSSMediaInParens | undefined {

	const token = parser.getToken();

	if (token.type === "LeftParen") {
		const start = parser.getPosition();
		const maybeIdent = parser.nextToken();
		if (maybeIdent.type === "Ident" && maybeIdent.value !== "not") {
			const feature = parseMediaFeature(parser);
			console.log(feature)
			if (feature) {
				console.log(parser.getToken())
				return parser.finishNode(start, {
					type: "CSSMediaInParens",
					value: feature
				})
			}
		}
	}

	return undefined;
}
