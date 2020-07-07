import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {MarkdownQuoteChildren} from "@romejs/ast/markdown/types";

// > this quote
export type MarkdownQuoteBlock = NodeBaseWithComments & {
	type: "MarkdownQuoteBlock";
	children: Array<MarkdownQuoteChildren>;
};

export const markdownQuoteBlock = createBuilder<MarkdownQuoteBlock>(
	"MarkdownQuoteBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
