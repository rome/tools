import {CSSSupportsFeature} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSSupportsFeature(
	builder: Builder,
	node: CSSSupportsFeature,
): Token {
	return builder.tokenize(node.value, node);
}
