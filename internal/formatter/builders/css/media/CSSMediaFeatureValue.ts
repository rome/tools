import {CSSMediaFeatureValue} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSMediaFeatureValue(
	builder: Builder,
	node: CSSMediaFeatureValue,
): Token {
	return builder.tokenize(node.value, node);
}
