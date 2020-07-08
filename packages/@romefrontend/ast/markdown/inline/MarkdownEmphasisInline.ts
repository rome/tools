import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

// *emphasis*
export type MarkdownEmphasisInline = NodeBaseWithComments & {
	type: "MarkdownEmphasisInline";
	value: string;
};

export const markdownEmphasisInline = createBuilder<MarkdownEmphasisInline>(
	"MarkdownEmphasisInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
