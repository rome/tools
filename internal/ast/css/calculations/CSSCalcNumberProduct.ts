import {
	CSSCalcNumberValue,
	CSSCalcOperation,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcNumberProduct extends NodeBaseWithComments {
	readonly type: "CSSCalcNumberProduct";
	readonly value: Array<CSSCalcNumberValue | CSSCalcOperation>;
}

export const cssCalcNumberProduct = createBuilder<CSSCalcNumberProduct>(
	"CSSCalcNumberProduct",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
