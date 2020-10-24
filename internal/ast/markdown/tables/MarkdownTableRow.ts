import {MarkdownTableCell, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownTableRow extends NodeBaseWithComments {
	type: "MarkdownTableRow";
	children: Array<MarkdownTableCell>;
}

export const markdownTableRow = createBuilder<MarkdownTableRow>(
	"MarkdownTableRow",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
