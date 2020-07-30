import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// *emphasis*
export interface MarkdownEmphasisInline extends NodeBaseWithComments {
	type: "MarkdownEmphasisInline";
	value: string;
}

export const markdownEmphasisInline = createBuilder<MarkdownEmphasisInline>(
	"MarkdownEmphasisInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
