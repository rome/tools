import {MarkdownReference, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// [link](www.example.com)
export interface MarkdownReferenceInline extends NodeBaseWithComments {
	type: "MarkdownReferenceInline";
	value: string;
	reference?: MarkdownReference;
	title?: string;
}

export const markdownReferenceInline = createBuilder<MarkdownReferenceInline>(
	"MarkdownReferenceInline",
	{
		bindingKeys: {},
		visitorKeys: {
			reference: true,
		},
	},
);
