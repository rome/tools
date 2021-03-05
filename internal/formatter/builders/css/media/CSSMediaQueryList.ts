import {CSSMediaQueryList} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaQueryList(
	builder: Builder,
	node: CSSMediaQueryList,
): Token {
	return concat(node.value.map((child, index) => {
		const tokens: Token[] = [];
		if (index > 0) {
			tokens.push(",", space)
		}
		tokens.push(builder.tokenize(child, node));
		return concat(tokens);
	}));
}
