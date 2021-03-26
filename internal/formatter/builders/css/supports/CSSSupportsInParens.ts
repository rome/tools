import {CSSSupportsInParens} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	hardline,
	indent,
	space,
} from "@internal/formatter";

export default function CSSSupportsInParens(
	builder: Builder,
	node: CSSSupportsInParens,
): Token {
	const tokens: Token[] = [];
	if (node.prefix) {
		if (node.prefix === "not") {
			tokens.push(space);
		} else {
			tokens.push(hardline);
		}
		tokens.push(node.prefix);
		tokens.push(space);
	}
	if (node.value.type === "CSSSupportsCondition") {
		tokens.push("(");
		tokens.push(builder.tokenize(node.value, node));
		tokens.push(")");
	} else {
		tokens.push(builder.tokenize(node.value, node));
	}
	if (node.prefix) {
		return indent(concat(tokens));
	}
	return concat(tokens);
}
