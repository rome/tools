import {
	CSSCalcOperation,
	CSSCalcProduct,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSCalcSumValue = Array<CSSCalcProduct | CSSCalcOperation>;

export interface CSSCalcSum extends NodeBaseWithComments {
	readonly type: "CSSCalcSum";
	readonly value: CSSCalcSumValue;
}

export const cssCalcSum = createBuilder<CSSCalcSum>(
	"CSSCalcSum",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
