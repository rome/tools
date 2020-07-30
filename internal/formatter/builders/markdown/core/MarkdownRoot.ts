import {MarkdownRoot} from "@internal/ast";
import {Builder, Token, concat, hardline} from "@internal/formatter";

export default function MarkdownRoot(
	builder: Builder,
	node: MarkdownRoot,
): Token {
	return concat([builder.tokenizeStatementList(node.body, node), hardline]);
}
