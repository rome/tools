import {CSSSelectorCombinator} from "@romefrontend/ast";
import {Builder, Token, concat, space} from "@romefrontend/formatter";

export default function CSSSelectorCombinator(
	builder: Builder,
	node: CSSSelectorCombinator,
): Token {
	const left = builder.tokenize(node.left, node);
	const tokens = [left];

	const {kind} = node;
	if (kind === "descendant") {
		tokens.push(space);
	} else {
		tokens.push(space);

		if (kind === "adjacent-sibling") {
			tokens.push("+");
		} else if (kind === "child") {
			tokens.push(">");
		} else if (kind === "general-sibling") {
			tokens.push("~");
		} else if (kind === "column") {
			tokens.push("||");
		}

		tokens.push(space);
	}

	const right = builder.tokenize(node.right, node);
	tokens.push(right);
	return concat(tokens);
}
