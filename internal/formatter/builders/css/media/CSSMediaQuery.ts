import {CSSMediaQuery} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSMediaQuery(
	builder: Builder,
	node: CSSMediaQuery,
): Token {
	const tokens: Token[] = [];
	if (node.condition) {
		tokens.push( node.condition);
		tokens.push(space);
	}
	tokens.push(builder.tokenize(node.value, node));
	if (node.conditionWithoutOr) {
		tokens.push(space);
		tokens.push(builder.tokenize(node.conditionWithoutOr, node));
	}
	return concat(tokens);
}
