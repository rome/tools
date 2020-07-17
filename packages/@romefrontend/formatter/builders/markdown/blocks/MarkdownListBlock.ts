import {MarkdownListBlock} from "@romefrontend/ast";
import {
	Builder,
	Token,
	Tokens,
	concat,
	hardline,
} from "@romefrontend/formatter";

export default function MarkdownListBlock(
	builder: Builder,
	node: MarkdownListBlock,
): Token {
	const tokens: Tokens = node.children.reduce(
		(tokens, child, index) => {
			if (node.ordered) {
				tokens.push(`${index + 1}. `);
			} else {
				if (child.value) {
					tokens.push(`${child.value} `);
				}
			}
			tokens.push(builder.tokenize(child, node));
			if (index + 1 < node.children.length) {
				tokens.push(hardline);
			}

			return tokens;
		},
		([] as Tokens),
	);
	return concat(tokens);
}
