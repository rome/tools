import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type MarkdownCodeInline = NodeBaseWithComments & {
	type: "MarkdownCodeInline";
	value: string;
};

export const markdownCodeInline = createBuilder<MarkdownCodeInline>(
	"MarkdownCodeInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
