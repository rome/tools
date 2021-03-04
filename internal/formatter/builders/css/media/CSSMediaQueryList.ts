import {CSSMediaQueryList} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSMediaQueryList(
	builder: Builder,
	node: CSSMediaQueryList,
): Token {
	return concat([...node.value.map((child) => builder.tokenize(child, node))]);
}
