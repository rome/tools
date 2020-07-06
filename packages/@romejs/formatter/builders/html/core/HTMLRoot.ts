import {HTMLRoot} from "@romejs/ast";
import {Builder, Token, concat, hardline} from "@romejs/formatter";

export default function HTMLRoot(builder: Builder, node: HTMLRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
