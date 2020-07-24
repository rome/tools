import {MarkdownListItem, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface MarkdownListBlock extends NodeBaseWithComments {
	type: "MarkdownListBlock";
	ordered: boolean;
	children: Array<MarkdownListItem>;
}

export const markdownListBlock = createBuilder<MarkdownListBlock>(
	"MarkdownListBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
