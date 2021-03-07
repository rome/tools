import {TSTupleElement} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function TSTupleElement(
	builder: Builder,
	node: TSTupleElement,
): Token {
	let tokens: Token[] = [];

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
