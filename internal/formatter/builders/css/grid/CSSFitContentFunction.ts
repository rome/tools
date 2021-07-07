import {CSSFitContentFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSFitContentFunction(
	builder: Builder,
	node: CSSFitContentFunction,
): Token {
	return concat([
		node.name,
		"(",
		concat(node.params.map((p) => builder.tokenize(p, node))),
		")",
	]);
}
