import {TSTupleElement} from "@internal/ast";
import {Builder, Token, Tokens, concat, space} from "@internal/formatter";

export default function TSTupleElement(
	builder: Builder,
	node: TSTupleElement,
): Token {
	let tokens: Tokens = [];

	if (node.name) {
		tokens.push(builder.tokenize(node.name, node));
	}

	if (node.optional) {
		tokens.push("?");
	}

	if (node.name) {
		tokens.push(":");
		tokens.push(space);
	}

	tokens.push(builder.tokenize(node.typeAnnotation, node));

	return concat(tokens);
}
