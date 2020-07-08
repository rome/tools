import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type MarkdownDividerBlock = NodeBaseWithComments & {
	type: "MarkdownDividerBlock";
};

export const markdownDividerBlock = createBuilder<MarkdownDividerBlock>(
	"MarkdownDividerBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
