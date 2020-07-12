import {MarkdownText} from "@romefrontend/ast";
import {Builder, Token} from "@romefrontend/formatter";

export default function MarkdownText(
	builder: Builder,
	node: MarkdownText,
): Token {
	return node.value;
}
