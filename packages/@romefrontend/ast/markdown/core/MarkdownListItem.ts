import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {MarkdownListChildren} from "@romefrontend/ast/markdown/unions";

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
