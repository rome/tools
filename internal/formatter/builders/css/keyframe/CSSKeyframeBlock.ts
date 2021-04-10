import {CSSKeyframeBlock} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
	join,
	space,
} from "@internal/formatter";

export default function CSSKeyframeBlock(
	builder: Builder,
	node: CSSKeyframeBlock,
): Token {
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
		builder.tokenize(node.name, node),
		space,
		"{",
		group(indent(list, true)),
		hardline,
		"}",
	]);
}
