import { MarkdownParagraph } from './../core/MarkdownParagraph';
import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownTableCell extends NodeBaseWithComments {
	type: "MarkdownTableCell";
	children: Array<MarkdownParagraph>;
}

export const markdownTableCell = createBuilder<MarkdownTableCell>(
	"MarkdownTableCell",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
