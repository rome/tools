import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {MarkdownQuoteChildren} from "@romefrontend/ast/markdown/unions";

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
