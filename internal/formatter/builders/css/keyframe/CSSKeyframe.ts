import {CSSKeyframe} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
	space,
} from "@internal/formatter";

export default function CSSKeyframe(builder: Builder, node: CSSKeyframe): Token {
	return concat([
		"@keyframes",
		space,
		builder.tokenize(node.name, node),
		space,
		"{",
		group(indent(builder.tokenizeStatementList(node.value, node), true)),
		hardline,
		"}",
	]);
}
