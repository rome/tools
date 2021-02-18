import {
	CSSCalcSum,
	CSSDimension,
	CSSNumber,
	CSSPercentage,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcValue extends NodeBaseWithComments {
	readonly type: "CSSCalcValue";
	readonly value: CSSNumber | CSSDimension | CSSPercentage | CSSCalcSum;
}

export const cssCalcValue = createBuilder<CSSCalcValue>(
	"CSSCalcValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
