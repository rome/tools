import {CSSAttributeSelector} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSAttributeSelector(
	builder: Builder,
	node: CSSAttributeSelector,
): Token {
	const tokens: Token[] = ["[", builder.tokenize(node.attribute, node)];

	if (node.matcher) {
		tokens.push(node.matcher);
	}

	if (node.value) {
		tokens.push(builder.tokenize(node.value, node));
	}

	if (node.modifier) {
		tokens.push(space);
		tokens.push(node.modifier);
	}

	tokens.push("]");

	return concat(tokens);
}
