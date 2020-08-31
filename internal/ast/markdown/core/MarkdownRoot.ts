import {AnyMarkdownNode, NodeBaseWithComments, RootBase} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownRoot extends NodeBaseWithComments,
RootBase {
	type: "MarkdownRoot";
	body: AnyMarkdownNode[];
}

export const markdownRoot = createBuilder<MarkdownRoot>(
	"MarkdownRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
