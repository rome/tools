import {CSSMediaQuery} from "@internal/ast";
import {Builder, concat, space, Token} from "@internal/formatter";

export default function CSSMediaQuery(
	builder: Builder,
	node: CSSMediaQuery,
): Token {
	const tokens: Token[] = [];
	if (node.condition) {
		tokens.push(node.condition);
		tokens.push(space)
	}
	tokens.push(

	builder.tokenize(node.value, node)
	)
	return concat(tokens);
}
