import {CSSSupportsInParens} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSSupportsInParens(
	builder: Builder,
	node: CSSSupportsInParens,
): Token {
	const tokens: Token[] = [];
	if (node.prefix) {
		tokens.push(space);
		tokens.push(node.prefix);
		tokens.push(space);
	}
	if (node.value.type === "CSSSupportsCondition") {
		return concat([...tokens, "(", builder.tokenize(node.value, node), ")"]);
	}
	return concat([...tokens, builder.tokenize(node.value, node)]);
}
