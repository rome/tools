import {CSSMediaFeatureEQ} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSMediaFeatureEQ(
	builder: Builder,
	node: CSSMediaFeatureEQ,
): Token {
	return "=";
}
