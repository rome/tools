import {HTMLRoot} from "@internal/ast";
import {Builder, Token, concat, hardline} from "@internal/formatter";

export default function HTMLRoot(builder: Builder, node: HTMLRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
