import {CSSParser} from "@internal/css-parser/types";
import {CSSCalcFunction} from "@internal/ast";
import {parseCalcSum} from "@internal/css-parser/parser/calculations";
import {nextToken} from "@internal/css-parser/tokenizer";

export function parseCalcFunction(
	parser: CSSParser,
): CSSCalcFunction | undefined {
	const start = parser.getPosition();
	const value = parseCalcSum(parser);

	if (value) {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSCalcFunction",
				name: "calc",
				params: [value],
			},
		);
	}

	return undefined;
}
