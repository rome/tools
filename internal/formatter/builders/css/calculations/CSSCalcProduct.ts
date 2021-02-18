import {CSSCalcProduct} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcProduct(
	builder: Builder,
	node: CSSCalcProduct,
): Token {
	return concat(node.value.map((n) => builder.tokenize(n, node)));
}
