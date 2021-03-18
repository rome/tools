import {CSSMediaFeatureBoolean} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSMediaFeatureBoolean(
	builder: Builder,
	node: CSSMediaFeatureBoolean,
): Token {
	return node.value;
}
