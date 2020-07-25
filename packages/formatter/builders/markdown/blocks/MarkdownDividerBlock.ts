import {MarkdownDividerBlock} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function MarkdownDividerBlock(
	builder: Builder,
	node: MarkdownDividerBlock,
): Token {
	return node.value;
}
