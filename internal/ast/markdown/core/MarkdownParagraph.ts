import {AnyMarkdownInlineNode, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownParagraph extends NodeBaseWithComments {
	type: "MarkdownParagraph";
	children: Array<AnyMarkdownInlineNode>;
}

export const markdownParagraph = createBuilder<MarkdownParagraph>(
	"MarkdownParagraph",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
