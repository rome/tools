import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type MarkdownDividerBlock = NodeBaseWithComments & {
	type: "MarkdownDividerBlock";
	value: string;
};

export const markdownDividerBlock = createBuilder<MarkdownDividerBlock>(
	"MarkdownDividerBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
