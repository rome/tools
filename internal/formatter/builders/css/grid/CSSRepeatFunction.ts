import {CSSRepeatFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

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
			builder.tokenize(values, node),
		]),
		")",
	]);
}
