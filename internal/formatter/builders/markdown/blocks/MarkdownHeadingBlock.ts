import {MarkdownHeadingBlock} from "@internal/ast";
import {Builder, Token, concat, hardline, space} from "@internal/formatter";

export default function MarkdownHeadingBlock(
	builder: Builder,
	node: MarkdownHeadingBlock,
): Token {
	return concat(["#".repeat(node.level), space, node.value, hardline]);
}
