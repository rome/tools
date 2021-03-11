import {CSSMediaInParens} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSMediaInParens(
	builder: Builder,
	node: CSSMediaInParens,
): Token {
	return concat(["(", builder.tokenize(node.value, node), ")"]);
}
