import {AnyMarkdownInlineNode, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// **something**
export interface MarkdownBoldInline extends NodeBaseWithComments {
	type: "MarkdownBoldInline";
	value: Array<AnyMarkdownInlineNode>;
}

export const markdownBoldInline = createBuilder<MarkdownBoldInline>(
	"MarkdownBoldInline",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
