import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";
import {MarkdownReference} from "@romejs/ast/markdown/types";

// ![Atl text](//url)
// ![Atl text] [1]
export type MarkdownImageInline = NodeBaseWithComments & {
	type: "MarkdownImageInline";
	url: MarkdownReference;
	altText: string;
};

export const markdownImageInline = createBuilder<MarkdownImageInline>(
	"MarkdownImageInline",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
