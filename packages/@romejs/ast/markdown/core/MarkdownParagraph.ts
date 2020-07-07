import {MarkdownText, NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type MarkdownParagraph = NodeBaseWithComments & {
	type: "MarkdownParagraph";
	value: Array<MarkdownText>;
};

export const markdownParagraph = createBuilder<MarkdownParagraph>(
	"MarkdownParagraph",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
