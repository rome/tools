import {CSSMediaFeature} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSMediaFeature(
	builder: Builder,
	node: CSSMediaFeature,
): Token {
	return builder.tokenize(node.value, node);
}
