import {CSSGridRepeatValue} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSGridRepeatValue(
	builder: Builder,
	node: CSSGridRepeatValue,
): Token {
	return concat(
		node.values.map((value) => {
			return concat([space, builder.tokenize(value, node)]);
		}),
	);
}
