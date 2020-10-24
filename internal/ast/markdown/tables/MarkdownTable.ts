import {MarkdownTableRow, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownTable extends NodeBaseWithComments {
	type: "MarkdownTable";
	align: AlignType;
	children: Array<MarkdownTableRow>;
}

export type AlignType = "left" | "right" | "center" | null;

export const markdownTable = createBuilder<MarkdownTable>(
	"MarkdownTable",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
