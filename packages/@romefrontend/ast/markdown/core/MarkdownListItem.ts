import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {MarkdownListChildren} from "@romefrontend/ast/markdown/types";

export type MarkdownListItem = NodeBaseWithComments & {
	type: "MarkdownListItem";
	checked: boolean | null;
	children: Array<MarkdownListChildren>;
};

export const markdownListItem = createBuilder<MarkdownListItem>(
	"MarkdownListItem",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
