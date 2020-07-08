import {MarkdownListItem, NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type MarkdownListBlock = NodeBaseWithComments & {
	type: "MarkdownListBlock";
	kind: "dot-list" | "numeric-list";
	children: Array<MarkdownListItem>;
};

export const markdownListBlock = createBuilder<MarkdownListBlock>(
	"MarkdownListBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
