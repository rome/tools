import {MarkdownListBlock} from "@internal/ast";
import {
	Builder,
	Token,
	Tokens,
	concat,
	hardline,
	space,
} from "@internal/formatter";

export default function MarkdownListBlock(
	builder: Builder,
	node: MarkdownListBlock,
): Token {
	const tokens: Tokens = node.children.reduce(
		(tokens, child, index) => {
			if (node.ordered) {
				tokens.push(`${index + 1}.`);
				tokens.push(space);
			} else {
				if (child.value) {
					tokens.push(`${child.value}`);
					tokens.push(space);
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
