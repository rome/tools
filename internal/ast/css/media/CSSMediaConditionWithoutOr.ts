import {CSSMediaAnd, CSSMediaInParens, CSSMediaNot, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaConditionWithoutOr extends NodeBaseWithComments {
	readonly type: "CSSMediaConditionWithoutOr";
	readonly value: CSSMediaAnd | CSSMediaInParens | CSSMediaNot
}

export const cssMediaConditionWithoutOr = createBuilder<CSSMediaConditionWithoutOr>(
	"CSSMediaConditionWithoutOr",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
