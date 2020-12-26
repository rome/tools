import {CSSRoot} from "@internal/ast";
import {Builder, Token, concat, hardline} from "@internal/formatter";

export default function CSSRoot(builder: Builder, node: CSSRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
