import {CSSAtRule} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSAtRule(builder: Builder, node: CSSAtRule): Token {
	return concat([
		...node.prelude.map((token) => builder.tokenize(token, node)),

		space,
		builder.tokenize(node.block, node),
	]);
}
