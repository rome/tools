import {CSSMinFunction} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSMinFunction(
	builder: Builder,
	node: CSSMinFunction,
): Token {
	return concat([
		node.name,
		"(",
		concat(node.params.map((p) => builder.tokenize(p, node))),
		")",
	]);
}
