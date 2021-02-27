import {CSSQualifiedName} from "@internal/ast";
import {Builder, Token, concat} from "@internal/formatter";

export default function CSSQualifiedName(
	builder: Builder,
	node: CSSQualifiedName,
): Token {
	const tokens: Token[] = [];
	if (node.namespace !== undefined) {
		tokens.push(node.namespace);
		tokens.push("|");
	}
	tokens.push(node.localName);
	return concat(tokens);
}
