import {CSSRoot} from "@romefrontend/ast";
import {Builder, Token, concat, hardline} from "@romefrontend/formatter";

export default function CSSRoot(builder: Builder, node: CSSRoot): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
