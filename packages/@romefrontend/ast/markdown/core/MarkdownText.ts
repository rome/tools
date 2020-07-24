import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface MarkdownText extends NodeBaseWithComments {
	type: "MarkdownText";
	value: string;
}

export const markdownText = createBuilder<MarkdownText>(
	"MarkdownText",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
