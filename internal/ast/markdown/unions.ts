import * as n from "@internal/ast";

export type MarkdownListChildren = n.MarkdownQuoteBlock | n.MarkdownParagraph;

export type MarkdownQuoteChildren =
	| n.MarkdownQuoteBlock
	| n.MarkdownParagraph
	| n.MarkdownHeadingBlock;

export type MarkdownReference = Array<
	| n.MarkdownText
	| n.MarkdownEmphasisInline
	| n.MarkdownBoldInline
	| n.MarkdownDefinitionInline
>;

export type AnyMarkdownInlineNode =
	| n.MarkdownText
	| n.MarkdownBoldInline
	| n.MarkdownCodeInline
	| n.MarkdownAutomaticLinkInline
	| n.MarkdownReferenceInline
	| n.MarkdownImageInline
	| n.MarkdownEmphasisInline;

export type AnyMarkdownNode =
	| n.MarkdownQuoteBlock
	| n.MarkdownCodeBlock
	| n.MarkdownDividerBlock
	| n.MarkdownHeadingBlock
	| n.MarkdownParagraph
	| n.MarkdownListBlock
	| n.MarkdownTable
	| n.MarkdownTableRow
	| n.MarkdownTableCell
	| n.MarkdownDefinitionInline
	| n.MarkdownText
	| n.MarkdownBoldInline
	| n.MarkdownCodeInline
	| n.MarkdownAutomaticLinkInline
	| n.MarkdownReferenceInline
	| n.MarkdownImageInline
	| n.MarkdownEmphasisInline;
