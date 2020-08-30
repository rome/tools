import {MarkdownText, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownCodeBlock extends NodeBaseWithComments {
	type: "MarkdownCodeBlock";
	language: string;
	value?: MarkdownText;
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
