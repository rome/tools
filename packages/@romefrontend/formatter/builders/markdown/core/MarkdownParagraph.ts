import {MarkdownParagraph} from "@romefrontend/ast";
import {Builder, Token, Tokens, concat} from "@romefrontend/formatter";

export default function MarkdownParagraph(
	builder: Builder,
	node: MarkdownParagraph,
): Token {
	const tokens: Tokens = node.children.map((child) => {
		return builder.tokenize(child, node);
	});
	return concat(tokens);
}
