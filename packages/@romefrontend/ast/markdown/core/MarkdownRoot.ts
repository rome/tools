import {NodeBaseWithComments, RootBase} from "@romefrontend/ast";
import {createBuilder} from "../../utils";
import {AnyMarkdownNode} from "@romefrontend/ast/markdown/unions";

export interface MarkdownRoot extends NodeBaseWithComments,
RootBase {
	type: "MarkdownRoot";
	body: Array<AnyMarkdownNode>;
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
