import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

// <something.com/>
// TODO here we have to obscure the email
export type MarkdownAutomaticLinkInline = NodeBaseWithComments & {
	type: "MarkdownAutomaticLinkInline";
};

export const markdownAutomaticLinkInline = createBuilder<MarkdownAutomaticLinkInline>(
	"MarkdownAutomaticLinkInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
