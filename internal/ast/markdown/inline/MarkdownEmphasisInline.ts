import {AnyMarkdownInlineNode, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// *emphasis*
export interface MarkdownEmphasisInline extends NodeBaseWithComments {
	type: "MarkdownEmphasisInline";
	value: Array<AnyMarkdownInlineNode>;
}

export const markdownEmphasisInline = createBuilder<MarkdownEmphasisInline>(
	"MarkdownEmphasisInline",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
