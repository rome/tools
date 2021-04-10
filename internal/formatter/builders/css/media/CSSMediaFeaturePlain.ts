import {CSSMediaFeaturePlain} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaFeaturePlain(
	builder: Builder,
	node: CSSMediaFeaturePlain,
): Token {
	return concat([
		builder.tokenize(node.name, node),
		":",
		space,
		builder.tokenize(node.value, node),
	]);
}
