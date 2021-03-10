import {CSSMediaInParens} from "@internal/ast";
import {Builder, concat, Token} from "@internal/formatter";

export default function CSSMediaInParens(
	builder: Builder,
	node: CSSMediaInParens,
): Token {
	return concat([
		"(",
		builder.tokenize(node.value, node),
		")"
	])

}
