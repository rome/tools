import {CSSRatio} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSRatio(builder: Builder, node: CSSRatio): Token {
	return concat([
		builder.tokenize(node.numerator, node),
		"/",
		builder.tokenize(node.denominator, node),
	]);
}
