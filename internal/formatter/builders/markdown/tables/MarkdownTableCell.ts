import {MarkdownTableCell} from "@internal/ast";
import {Builder, Token} from "@internal/formatter";

export default function MarkdownTableCell(
	builder: Builder,
	node: MarkdownTableCell,
): Token {
	throw Error("Must be rendered through MarkdownTable.");
}
