import {MarkdownListItem} from "@internal/ast";
import {Builder, Token, Tokens, concat} from "@internal/formatter";

export default function MarkdownListItem(
	builder: Builder,
	node: MarkdownListItem,
): Token {
	const tokens: Tokens = [];
	if (node.checked !== undefined) {
		tokens.push(`[${node.checked ? "x" : " "}]`);
	}

	tokens.push(builder.tokenizeStatementList(node.children, node));

	return concat(tokens);
}
