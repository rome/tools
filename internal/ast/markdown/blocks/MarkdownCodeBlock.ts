import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownCodeBlock extends NodeBaseWithComments {
	type: "MarkdownCodeBlock";
	language: string;
	value?: string;
}

export const markdownCodeBlock = createBuilder<MarkdownCodeBlock>(
	"MarkdownCodeBlock",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
