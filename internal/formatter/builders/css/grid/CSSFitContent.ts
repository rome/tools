import {CSSFitContent} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSFitContent(
	builder: Builder,
	node: CSSFitContent,
): Token {
	return concat([
		node.name,
		"(",
		concat(node.params.map((p) => builder.tokenize(p, node))),
		")",
	]);
}
