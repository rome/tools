import {CSSMinFunction} from "@internal/ast";
import {Builder, Token, concat, join, space} from "@internal/formatter";

export default function CSSMinFunction(
	builder: Builder,
	node: CSSMinFunction,
): Token {
	return concat([
		node.name,
		"(",
		join(
			concat([",", space]),
			node.params.map((p) => builder.tokenize(p, node)),
		),
		")",
	]);
}
