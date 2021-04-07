import {MarkdownReference, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// [link](www.example.com)
export interface MarkdownLink extends NodeBaseWithComments {
	type: "MarkdownLink";
	text?: MarkdownReference;
	title?: string;
	link: string;
}

export const markdownLink = createBuilder<MarkdownLink>(
	"MarkdownLink",
	{
		bindingKeys: {},
		visitorKeys: {
			text: true,
		},
	},
);
