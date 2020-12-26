import {CSSBlock} from "@internal/ast";
import {
	Builder,
	Token,
	concat,
	group,
	hardline,
	indent,
} from "@internal/formatter";

export default function CSSBlock(builder: Builder, node: CSSBlock): Token {
	const tokens: Token[] = [];
	if (node.value) {
		if (node.startingTokenValue) {
			return concat([
				node.startingTokenValue,
				group(indent(builder.tokenizeStatementList(node.value, node), true)),
				hardline,
				"}",
			]);
		}
	}
	return concat(tokens);
}
