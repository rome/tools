import {CSSRule} from "@internal/ast";
import {Builder, Token, concat, join, space} from "@internal/formatter";

export default function CSSRule(builder: Builder, node: CSSRule): Token {
	return concat([
		join(
			concat([",", space]),
			node.prelude.map((token) => builder.tokenize(token, node)),
		),
		space,
		builder.tokenize(node.block, node),
	]);
}
