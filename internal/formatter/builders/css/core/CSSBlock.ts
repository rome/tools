import {CSSBlock} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
	join,
} from "@internal/formatter";

export default function CSSBlock(builder: Builder, node: CSSBlock): Token {
	const tokens: Token[] = [];
	if (node.value) {
		if (node.startingTokenValue) {
			const list = join(
				hardline,
				node.value.map((child) => {
					const tokens: Token[] = [builder.tokenize(child, node)];
					if (child.type === "CSSDeclaration") {
						tokens.push(";");
					}
					return concat(tokens);
				}),
			);

			return concat([
				node.startingTokenValue,
				group(indent(list, true)),
				hardline,
				"}",
			]);
		}
	}
	return concat(tokens);
}
