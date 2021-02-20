import {CSSKeyframeBlock} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSKeyframeBlock(
	builder: Builder,
	node: CSSKeyframeBlock,
): Token {
	return concat([
		builder.tokenize(node.name, node),
		space,
		builder.tokenize(node.value, node),
	]);
}
