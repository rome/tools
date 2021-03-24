import {CSSMediaFeatureComparison} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSMediaFeatureComparison(
	builder: Builder,
	node: CSSMediaFeatureComparison,
): Token {
	return builder.tokenize(node.value, node);
}
