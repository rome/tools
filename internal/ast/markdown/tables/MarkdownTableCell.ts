import {AnyMarkdownInlineNode, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownTableCell extends NodeBaseWithComments {
	type: "MarkdownTableCell";
	children: Array<AnyMarkdownInlineNode>;
}

export const markdownTableCell = createBuilder<MarkdownTableCell>(
	"MarkdownTableCell",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
