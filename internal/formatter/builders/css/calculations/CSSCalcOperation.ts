import {CSSCalcOperation} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSCalcOperation(
	builder: Builder,
	node: CSSCalcOperation,
): Token {
	return concat([space, node.value, space]);
}
