import {CSSAtPage} from "@internal/ast";
import {Builder, Token, concat, space} from "@internal/formatter";

export default function CSSAtPage(builder: Builder, node: CSSAtPage): Token {
	const tokens: Token[] = [];
	tokens.push(space);
	if (node.prelude) {
		tokens.push(builder.tokenize(node.prelude, node));
	}
	tokens.push(builder.tokenize(node.block, node));
	return concat(tokens);
}
