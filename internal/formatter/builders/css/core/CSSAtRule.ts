import {CSSAtRule} from "@internal/ast";
import {Builder, Token, concat, hardline, space} from "@internal/formatter";

export default function CSSAtRule(builder: Builder, node: CSSAtRule): Token {
	const tokens: Token[] = [];
	tokens.push(`@${node.name}`);
	tokens.push(space);
	tokens.push(...node.prelude.map((token) => builder.tokenize(token, node)));
	if (node.prelude.length > 0) {
		tokens.push(space);
	}
	tokens.push(builder.tokenize(node.block, node));
	tokens.push(hardline);
	return concat(tokens);
}
