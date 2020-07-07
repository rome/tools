import * as n from "@romejs/ast";

export type MarkdownListChildren = n.MarkdownQuoteBlock | n.MarkdownParagraph;

export type MarkdownQuoteChildren =
	| n.MarkdownQuoteBlock
	| n.MarkdownParagraph
	| n.MarkdownHeadingBlock;

export type MarkdownReference = n.MarkdownParagraph | n.MarkdownDefinitionInline;
