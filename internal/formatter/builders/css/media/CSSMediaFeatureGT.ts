import {CSSMediaFeatureGT} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSMediaFeatureGT(
	builder: Builder,
	node: CSSMediaFeatureGT,
): Token {
	const tokens: Token[] = [">"];
	if (node.hasEqual) {
		tokens.push("=");
	}
	return concat(tokens)}
