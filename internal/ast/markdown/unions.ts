import * as n from "@internal/ast";

export type MarkdownListChildren = n.MarkdownQuoteBlock | n.MarkdownParagraph;

export type MarkdownQuoteChildren =
	| n.MarkdownQuoteBlock
	| n.MarkdownParagraph
	| n.MarkdownHeadingBlock;

export type MarkdownReference = n.MarkdownParagraph | n.MarkdownDefinitionInline;

export type AnyMarkdownInlineNode =
	| n.MarkdownText
	| n.MarkdownBoldInline
	| n.MarkdownCodeInline
	| n.MarkdownAutomaticLinkInline
	| n.MarkdownImageInline
	| n.MarkdownEmphasisInline;

export type AnyMarkdownNode =
	| n.MarkdownQuoteBlock
	| n.MarkdownDividerBlock
	| n.MarkdownHeadingBlock
	| n.MarkdownParagraph
	| n.MarkdownListBlock
	| n.MarkdownDefinitionInline;
