import {CSSDeclaration} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSDeclaration(
	builder: Builder,
	node: CSSDeclaration,
): Token {
	const tokens: Token[] = [];

	tokens.push(node.name);
	tokens.push(":");
	tokens.push(
		...node.value.map((value) => {
			return concat([space, builder.tokenize(value, node)]);
		}),
	);
	if (node.important) {
		tokens.push(space);
		tokens.push("!important");
	}
	tokens.push(";");

	return concat(tokens);
}
