import {CSSCalcNumberProduct} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcNumberProduct(
	builder: Builder,
	node: CSSCalcNumberProduct,
): Token {
	return concat(node.value.map((n) => builder.tokenize(n, node)));
}
