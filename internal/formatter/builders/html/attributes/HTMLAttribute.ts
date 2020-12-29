import {HTMLAttribute} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function HTMLAttribute(
	builder: Builder,
	node: HTMLAttribute,
): Token {
	const tokens: Token[] = [builder.tokenize(node.name, node)];

	if (node.value) {
		tokens.push("=");
		tokens.push(builder.tokenize(node.value, node));
	}

	return concat(tokens);
}
