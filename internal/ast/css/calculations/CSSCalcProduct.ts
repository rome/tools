import {
	CSSCalcNumberValue,
	CSSCalcOperation,
	CSSCalcValue,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSCalcProductValue = Array<
	CSSCalcValue | CSSCalcOperation | CSSCalcNumberValue
>;

export interface CSSCalcProduct extends NodeBaseWithComments {
	readonly type: "CSSCalcProduct";
	readonly value: CSSCalcProductValue;
}

export const cssCalcProduct = createBuilder<CSSCalcProduct>(
	"CSSCalcProduct",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
