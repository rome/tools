import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

// [link]: www.example.com "Title"
export interface MarkdownDefinitionInline extends NodeBaseWithComments {
	type: "MarkdownDefinitionInline";
	value: string;
	url: string;
	title: string;
	// TODO make sure identifier is unique somewhere/somehow
	identifier: string;
}

export const markdownDefinitionInline = createBuilder<MarkdownDefinitionInline>(
	"MarkdownDefinitionInline",
	{
		bindingKeys: {},
		visitorKeys: {
			url: true,
			title: true,
			identifier: true,
		},
	},
);
