import {CSSRoot} from "@romejs/ast";
import {Builder, Token, concat, hardline} from "@romejs/formatter";

export default function CSSRoot(builder: Builder, node: CSSRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
