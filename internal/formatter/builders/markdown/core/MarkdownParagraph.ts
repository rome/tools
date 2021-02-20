import {MarkdownParagraph} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function MarkdownParagraph(
	builder: Builder,
	node: MarkdownParagraph,
): Token {
	return concat(
		node.children.map((child) => {
			return builder.tokenize(child, node);
		}),
	);
}
