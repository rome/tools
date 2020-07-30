import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownDividerBlock extends NodeBaseWithComments {
	type: "MarkdownDividerBlock";
	value: string;
}

export const markdownDividerBlock = createBuilder<MarkdownDividerBlock>(
	"MarkdownDividerBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
