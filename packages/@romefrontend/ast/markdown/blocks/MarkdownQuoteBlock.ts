import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {MarkdownQuoteChildren} from "@romefrontend/ast/markdown/unions";

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
