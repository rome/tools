import {CSSKeyframeName} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSKeyframeName(
	builder: Builder,
	node: CSSKeyframeName,
): Token {
	return builder.tokenize(node.value, node);
}
