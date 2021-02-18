import {CSSCalcNumberSum, CSSNumber, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcNumberValue extends NodeBaseWithComments {
	readonly type: "CSSCalcNumberValue";
	readonly value: CSSNumber | CSSCalcNumberSum;
}

export const cssCalcNumberValue = createBuilder<CSSCalcNumberValue>(
	"CSSCalcNumberValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
