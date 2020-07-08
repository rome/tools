import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

// **something**
export type MarkdownBoldInline = NodeBaseWithComments & {
	type: "MarkdownBoldInline";
	value: string;
};

export const markdownBoldInline = createBuilder<MarkdownBoldInline>(
	"MarkdownBoldInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
