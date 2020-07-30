import {MarkdownReference, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// [link](www.example.com)
export interface MarkdownLinkInline extends NodeBaseWithComments {
	type: "MarkdownLinkInline";
	value: string;
	url: MarkdownReference;
	title?: string;
}

export const markdownLinkInline = createBuilder<MarkdownLinkInline>(
	"MarkdownLinkInline",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
