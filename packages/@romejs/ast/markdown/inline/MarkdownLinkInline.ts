import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {MarkdownReference} from "@romejs/ast/markdown/types";

// [link](www.example.com)
export type MarkdownLinkInline = NodeBaseWithComments & {
	type: "MarkdownLinkInline";
	value: string;
	url: MarkdownReference;
	title?: string;
};

export const markdownLinkInline = createBuilder<MarkdownLinkInline>(
	"MarkdownLinkInline",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
