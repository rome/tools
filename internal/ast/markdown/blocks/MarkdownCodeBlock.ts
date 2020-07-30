import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface MarkdownCodeBlock extends NodeBaseWithComments {
	type: "MarkdownCodeBlock";
}

export const markdownCodeBlock = createBuilder<MarkdownCodeBlock>(
	"MarkdownCodeBlock",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
