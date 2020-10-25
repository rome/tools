import {MarkdownTableRow, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownTable extends NodeBaseWithComments {
	type: "MarkdownTable";
	children: Array<MarkdownTableRow>;
}

export const markdownTable = createBuilder<MarkdownTable>(
	"MarkdownTable",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
