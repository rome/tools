import {MarkdownText} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function MarkdownText(
	builder: Builder,
	node: MarkdownText,
): Token {
	return node.value;
}
