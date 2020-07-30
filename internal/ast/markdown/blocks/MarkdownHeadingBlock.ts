import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// # Something
// #### Some other thing
export interface MarkdownHeadingBlock extends NodeBaseWithComments {
	type: "MarkdownHeadingBlock";
	level: number;
	value: string;
}

export const markdownHeadingBlock = createBuilder<MarkdownHeadingBlock>(
	"MarkdownHeadingBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			level: true,
		},
	},
);
