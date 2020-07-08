import {CSSSelectorChain} from "@romefrontend/ast";
import {Builder, Token, concat} from "@romefrontend/formatter";

export default function CSSSelectorChain(
	builder: Builder,
	node: CSSSelectorChain,
): Token {
	const tokens = [];

	if (node.tagName !== undefined) {
		tokens.push(builder.tokenize(node.tagName, node));
	}

	for (const selector of node.selectors) {
		tokens.push(builder.tokenize(selector, node));
	}

	return concat(tokens);
}
