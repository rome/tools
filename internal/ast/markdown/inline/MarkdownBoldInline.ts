import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// **something**
export interface MarkdownBoldInline extends NodeBaseWithComments {
	type: "MarkdownBoldInline";
	value: string;
}

export const markdownBoldInline = createBuilder<MarkdownBoldInline>(
	"MarkdownBoldInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
