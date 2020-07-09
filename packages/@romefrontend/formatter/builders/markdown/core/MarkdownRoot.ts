import {MarkdownRoot} from "@romefrontend/ast";
import {Builder, concat, hardline, Token} from "@romefrontend/formatter";

export default function MarkdownRoot(
	builder: Builder,
	node: MarkdownRoot,
): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
