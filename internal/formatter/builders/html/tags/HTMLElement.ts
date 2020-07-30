import {HTMLElement} from "@internal/ast";
import {
	Builder,
	Token,
	Tokens,
	concat,
	indent,
	join,
	softline,
	space,
} from "@internal/formatter";
import {hasInnerComments} from "../../comments";

export default function HTMLElement(builder: Builder, node: HTMLElement): Token {
	const name = builder.tokenize(node.name, node);
	const tokens: Tokens = ["<", name];

	for (const attr of node.attributes) {
		tokens.push(space);
		tokens.push(builder.tokenize(attr.name, attr));
		tokens.push("=");
		tokens.push(builder.tokenize(attr.value, attr));
	}

	if (
		node.children.length === 0 &&
		node.selfClosing !== false &&
		!hasInnerComments(node)
	) {
		tokens.push(space, "/>");
	} else {
		tokens.push(">");

		const children: Tokens = [];

		children.push(builder.tokenizeInnerComments(node, true));

		for (let i = 0; i < node.children.length; i++) {
			const child = node.children[i];
			const printed = builder.tokenize(child, node);

			if (i > 0 && builder.getLinesBetween(node.children[i - 1], child) > 1) {
				children.push(concat([softline, printed]));
			} else {
				children.push(printed);
			}
		}

		tokens.push(concat([indent(join(softline, children)), softline]));

		tokens.push("</");
		tokens.push(name);
		tokens.push(">");
	}

	return concat(tokens);
}
