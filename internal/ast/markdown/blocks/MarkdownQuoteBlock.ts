import {MarkdownQuoteChildren, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// > this quote
export interface MarkdownQuoteBlock extends NodeBaseWithComments {
	type: "MarkdownQuoteBlock";
	children: Array<MarkdownQuoteChildren>;
}

export const markdownQuoteBlock = createBuilder<MarkdownQuoteBlock>(
	"MarkdownQuoteBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
