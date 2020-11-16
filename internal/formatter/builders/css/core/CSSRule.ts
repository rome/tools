import {CSSRule} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSRule(builder: Builder, node: CSSRule): Token {
	return concat([
		...node.prelude.map((token) => builder.tokenize(token, node)),
		builder.tokenize(node.block, node),
	]);
}
