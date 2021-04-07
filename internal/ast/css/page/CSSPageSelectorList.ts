import {CSSPageSelector, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSPageSelectorList extends NodeBaseWithComments {
	readonly type: "CSSPageSelectorList";
	readonly value: CSSPageSelector[];
}

export const cssPageSelectorList = createBuilder<CSSPageSelectorList>(
	"CSSPageSelectorList",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
