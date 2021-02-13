import {CSSCalcSum} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcSum(builder: Builder, node: CSSCalcSum): Token {
	return concat(node.value.map((n) => builder.tokenize(n, node)));
}
