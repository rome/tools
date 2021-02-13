import {CSSCalcFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcFunction(
	builder: Builder,
	node: CSSCalcFunction,
): Token {
	return concat([node.name, "(", builder.tokenize(node.value, node), ")"]);
}
