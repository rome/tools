import {CSSCalcNumberValue} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcNumberValue(
	builder: Builder,
	node: CSSCalcNumberValue,
): Token {
	if (node.value.type === "CSSCalcNumberSum") {
		return concat(["(", builder.tokenize(node.value, node), ")"]);
	}
	return builder.tokenize(node.value, node);
}
