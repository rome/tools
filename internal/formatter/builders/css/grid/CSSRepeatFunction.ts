import {CSSRepeatFunction} from "@internal/ast";
import {Builder, concat, space, Token} from "@internal/formatter";

export default function CSSRepeatFunction(
	builder: Builder,
	node: CSSRepeatFunction,
): Token {
	const [tracker, values] = node.params;
	return concat([
		node.name,
		"(",
		concat([
			builder.tokenize(tracker, node),
			",",
			concat(
				values.map(value => {
					return concat([space, builder.tokenize(value, node)]);
				}
			))
		]),
		")",
	]);
}
