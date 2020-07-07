import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {MarkdownListChildren} from "@romejs/ast/markdown/types";

export type MarkdownListBlock = NodeBaseWithComments & {
	type: "MarkdownListBlock";
	kind: "dot-list" | "numeric-list";
	children: Array<MarkdownListChildren>;
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
