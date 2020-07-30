import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {MarkdownQuoteChildren} from "@internal/ast/markdown/unions";

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
