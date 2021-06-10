import {CSSMinmaxFunction} from "@internal/ast";
import {Builder, concat, join, space, Token} from "@internal/formatter";

export default function CSSMinmaxFunction(
	builder: Builder,
	node: CSSMinmaxFunction,
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

