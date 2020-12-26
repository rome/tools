import {CSSDimension} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSDimension(
	builder: Builder,
	node: CSSDimension,
): Token {
	const tokens: Token[] = [];
	tokens.push(node.value.toString());
	if (node.unit) {
		tokens.push(node.unit);
	}
	return concat(tokens);
}
