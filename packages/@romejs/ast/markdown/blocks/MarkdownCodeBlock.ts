import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type MarkdownCodeBlock = NodeBaseWithComments & {
	type: "MarkdownCodeBlock";
};

export const markdownCodeBlock = createBuilder<MarkdownCodeBlock>(
	"MarkdownCodeBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
