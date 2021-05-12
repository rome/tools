import {CSSMediaQueryList} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaQueryList(
	builder: Builder,
	node: CSSMediaQueryList,
): Token {
	const tokens: Token[] = [];

	tokens.push(
		...node.prelude.map((child, index) => {
			const tokens: Token[] = [];
			if (index > 0) {
				tokens.push(",", space);
			}
			tokens.push(builder.tokenize(child, node));
			return concat(tokens);
		}),
	);
	tokens.push(builder.tokenize(node.block, node));
	return concat(tokens);
}
