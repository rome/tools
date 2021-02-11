import {CSSKeyframeSelector} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSKeyframeSelector(
	builder: Builder,
	node: CSSKeyframeSelector,
): Token {
	return builder.tokenize(node.value, node);
}
