import {
	CSSCalcNumberProduct,
	CSSCalcOperation,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcNumberSum extends NodeBaseWithComments {
	readonly type: "CSSCalcNumberSum";
	readonly value: Array<CSSCalcNumberProduct | CSSCalcOperation>;
}

export const cssCalcNumberSum = createBuilder<CSSCalcNumberSum>(
	"CSSCalcNumberSum",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
