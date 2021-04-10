import {MarkdownLink} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function MarkdownLink(
	builder: Builder,
	node: MarkdownLink,
): Token {
	const tokens: Token[] = ["["];
	if (node.text && node.text.length > 0) {
		tokens.push(
			concat(
				node.text.map((ref) => {
					return builder.tokenize(ref, node);
				}),
			),
		);
	}
	tokens.push("]");

	tokens.push("(");
	tokens.push(node.link);
	if (node.title) {
		tokens.push(space);
		tokens.push(node.title);
	}
	tokens.push(")");

	return concat(tokens);
}
