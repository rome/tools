import {CSSCalcValue} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function CSSCalcValue(
	builder: Builder,
	node: CSSCalcValue,
): Token {
	return builder.tokenize(node.value, node);
}
