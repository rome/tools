import {CSSGridAreaValue} from "@internal/ast";
import {Builder, Token, concat, join, space} from "@internal/formatter";

export default function CSSGridAreaValue(
	builder: Builder,
	node: CSSGridAreaValue,
): Token {
	return join(
		concat([space, "/", space]),
		node.value.map((gridLine) => {
			if (gridLine.length === 1) {
				return builder.tokenize(gridLine[0], node);
			}

			return concat([
				builder.tokenize(gridLine[0], node),
				space,
				builder.tokenize(gridLine[1], node),
			]);
		}),
	);
}
