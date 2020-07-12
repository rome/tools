import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {AnyMarkdownInlineNode} from "@romefrontend/ast/markdown/unions";

export type MarkdownParagraph = NodeBaseWithComments & {
	type: "MarkdownParagraph";
	children: Array<AnyMarkdownInlineNode>;
};

export const markdownParagraph = createBuilder<MarkdownParagraph>(
	"MarkdownParagraph",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
