import {CSSParser} from "@internal/css-parser/types";
import {CSSCalcFunction} from "@internal/ast";
import {parseCalcSum} from "@internal/css-parser/parser/calculations";

export function parseCalcFunction(
	parser: CSSParser,
): CSSCalcFunction | undefined {
	const start = parser.getPosition();
	const value = parseCalcSum(parser);

	if (value) {
		parser.nextToken();
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
