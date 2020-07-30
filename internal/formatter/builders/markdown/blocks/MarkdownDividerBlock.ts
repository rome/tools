import {MarkdownDividerBlock} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function MarkdownDividerBlock(
	builder: Builder,
	node: MarkdownDividerBlock,
): Token {
	return node.value;
}
