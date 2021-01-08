// selector-list: https://www.w3.org/TR/selectors-4/#grouping
import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSPattern} from "../unions";

export interface CSSSelector extends NodeBaseWithComments {
	readonly type: "CSSSelector";
	readonly patterns: AnyCSSPattern[];
}

export const cssSelector = createBuilder<CSSSelector>(
	"CSSSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			patterns: true,
		},
	},
);
