import {MarkdownTableRow} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function MarkdownTableRow(
	builder: Builder,
	node: MarkdownTableRow,
): Token {
	throw Error("Must be rendered through MarkdownTable.");
}
