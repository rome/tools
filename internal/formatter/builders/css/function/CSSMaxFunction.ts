import {CSSMaxFunction} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSMaxFunction(
	builder: Builder,
	node: CSSMaxFunction,
): Token {
	return concat([
		node.name,
		"(",
		concat(node.params.map((p) => builder.tokenize(p, node))),
		")",
	]);
}
