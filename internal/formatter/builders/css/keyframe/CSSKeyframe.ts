import {CSSKeyframe} from "@internal/ast";
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

export default function CSSKeyframe(builder: Builder, node: CSSKeyframe): Token {
	const list = join(
		hardline,
		node.value.map((child) => {
			return builder.tokenize(child, node);
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
