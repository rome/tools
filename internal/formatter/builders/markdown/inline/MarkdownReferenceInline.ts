import {MarkdownReferenceInline} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function MarkdownReferenceInline(
	builder: Builder,
	node: MarkdownReferenceInline,
): Token {
	const tokens: Token[] = ["["];
	if (node.title) {
		tokens.push(node.title);
	}
	tokens.push("]");

	if (node.reference && node.reference.length > 0) {
		tokens.push("(");

		tokens.push(
			concat(
				node.reference.map((ref) => {
					return builder.tokenize(ref, node);
				}),
			),
		);

		tokens.push(")");
	}

	return concat(tokens);
}
