import {CSSPageSelector} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSPageSelector(
	builder: Builder,
	node: CSSPageSelector,
): Token {
	const tokens: Token[] = [];
	if (node.ident) {
		tokens.push(node.ident);
	}
	if (node.pseudo) {
		tokens.push(":");
		tokens.push(builder.tokenize(node.pseudo, node));
	}
	return concat(tokens);
}
