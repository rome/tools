import {CSSCalcFunction} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSCalcFunction(
	builder: Builder,
	node: CSSCalcFunction,
): Token {
	return concat([node.name, "(", concat(
		node.params.map(p => builder.tokenize(p, node))
	) , ")"]);
}
