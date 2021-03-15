import {CSSMediaFeatureLT} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSMediaFeatureLT(
	builder: Builder,
	node: CSSMediaFeatureLT,
): Token {
	const tokens: Token[] = ["<"];
	if (node.hasEqual) {
		tokens.push("=");
	}
	return concat(tokens)
}
