import {MarkdownListItem} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function MarkdownListItem(
	builder: Builder,
	node: MarkdownListItem,
): Token {
	const tokens: Token[] = [];
	if (node.checked !== undefined) {
		tokens.push(`[${node.checked ? "x" : " "}]`);
		tokens.push(space);
	}

	for (const child of node.children) {
		tokens.push(builder.tokenize(child, node));
	}

	return concat(tokens);
}
