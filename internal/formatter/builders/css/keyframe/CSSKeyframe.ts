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
	if (node.startingTokenValue) {
		return concat([
			"@keyframes",
			space,
			builder.tokenize(node.name, node),
			space,
			node.startingTokenValue,
			group(indent(builder.tokenizeStatementList(node.value, node), true)),
			hardline,
			"}",
		]);
	}
	return concat([]);
}
