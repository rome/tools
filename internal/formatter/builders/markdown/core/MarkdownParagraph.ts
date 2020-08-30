import {MarkdownParagraph} from "@internal/ast";
import {Builder, Token, Tokens, concat, hardline} from "@internal/formatter";

export default function MarkdownParagraph(
	builder: Builder,
	node: MarkdownParagraph,
): Token {
	const tokens: Tokens = node.children.map((child) => {
		return builder.tokenize(child, node);
	});
	tokens.push(hardline);
	return concat(tokens);
}
