import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

// # Something
// #### Some other thing
export type MarkdownHeadingBlock = NodeBaseWithComments & {
	type: "MarkdownHeadingBlock";
	level: number;
	value: string;
};

export const markdownHeadingBlock = createBuilder<MarkdownHeadingBlock>(
	"MarkdownHeadingBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			level: true,
		},
	},
);
