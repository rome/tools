import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {MarkdownReference} from "@internal/ast/markdown/unions";

// ![Atl text](//url)
// ![Atl text] [1]
export interface MarkdownImageInline extends NodeBaseWithComments {
	type: "MarkdownImageInline";
	url: MarkdownReference;
	altText: string;
}

export const markdownImageInline = createBuilder<MarkdownImageInline>(
	"MarkdownImageInline",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
		},
	},
);
