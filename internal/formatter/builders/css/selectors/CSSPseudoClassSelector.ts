import {CSSPseudoClassSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";
import {printCommaList} from "../utils";

export default function CSSPseudoClassSelector(
	builder: Builder,
	node: CSSPseudoClassSelector,
): Token {
	const tokens: Token[] = [":", node.value];
	if (node.params) {
		tokens.push("(");
		tokens.push(printCommaList(builder, node.params, node));
		tokens.push(")");
	}
	return concat(tokens);
}
