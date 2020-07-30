import {MarkdownListChildren, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface MarkdownListItem extends NodeBaseWithComments {
	type: "MarkdownListItem";
	checked: boolean | undefined;
	children: Array<MarkdownListChildren>;
	// the value of ordered list: 1./-/*
	value: string | undefined;
}

export const markdownListItem = createBuilder<MarkdownListItem>(
	"MarkdownListItem",
	{
		bindingKeys: {},
		visitorKeys: {
			children: true,
		},
	},
);
