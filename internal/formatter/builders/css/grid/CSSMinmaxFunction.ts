import {CSSMinmaxFunction} from "@internal/ast";
import {Builder, Token, concat, join, space} from "@internal/formatter";

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
