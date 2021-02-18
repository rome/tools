import {CSSCalcSum, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcFunction extends NodeBaseWithComments {
	readonly type: "CSSCalcFunction";
	readonly name: string;
	readonly params: [CSSCalcSum];
}

export const cssCalcFunction = createBuilder<CSSCalcFunction>(
	"CSSCalcFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
