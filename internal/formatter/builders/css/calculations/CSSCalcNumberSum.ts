import {CSSCalcNumberSum} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcNumberSum(
	builder: Builder,
	node: CSSCalcNumberSum,
): Token {
	return concat(node.value.map((n) => builder.tokenize(n, node)));
}
