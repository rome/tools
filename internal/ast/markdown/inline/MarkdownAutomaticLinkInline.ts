import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// <something.com/>
// TODO here we have to obscure the email
export interface MarkdownAutomaticLinkInline extends NodeBaseWithComments {
	type: "MarkdownAutomaticLinkInline";
}

export const markdownAutomaticLinkInline = createBuilder<MarkdownAutomaticLinkInline>(
	"MarkdownAutomaticLinkInline",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
