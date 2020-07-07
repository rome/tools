import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {MarkdownListChildren} from "@romejs/ast/markdown/types";

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
