import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {MarkdownReference} from "@romefrontend/ast/markdown/unions";

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
