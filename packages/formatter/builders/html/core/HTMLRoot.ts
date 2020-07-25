import {HTMLRoot} from "@romefrontend/ast";
import {Builder, Token, concat, hardline} from "@romefrontend/formatter";

export default function HTMLRoot(builder: Builder, node: HTMLRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
