import {CSSRule} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	join,
	softline,
	space,
} from "@internal/formatter";

export default function CSSRule(builder: Builder, node: CSSRule): Token {
	return concat([
		join(
			concat([",", softline]),
			node.prelude.map((token) => builder.tokenize(token, node)),
		),
		space,
		builder.tokenize(node.block, node),
	]);
}
