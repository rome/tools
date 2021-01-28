import {MarkdownListItem} from "@internal/ast";
import {Builder, Token, Tokens, concat, space} from "@internal/formatter";

export default function MarkdownListItem(
	builder: Builder,
	node: MarkdownListItem,
): Token {
	const tokens: Tokens = [];
	if (node.checked !== undefined) {
		tokens.push(`[${node.checked ? "x" : " "}]`);
		tokens.push(space);
	}

	node.children.forEach((child) => tokens.push(builder.tokenize(child, node)));
	return concat(tokens);
}
