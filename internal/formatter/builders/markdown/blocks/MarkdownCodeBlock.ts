import {MarkdownCodeBlock} from "@internal/ast";
import {Builder, Token, concat, hardline} from "@internal/formatter";

export default function MarkdownCodeBlock(
	builder: Builder,
	node: MarkdownCodeBlock,
): Token {
	return concat([
		"```",
		node.language !== "unknown" ? node.language : "",
		hardline,
		node.value ?? "",
		hardline,
		"```",
	]);
}
