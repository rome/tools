import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcFunction extends NodeBaseWithComments {
	readonly type: "CSSCalcFunction";
}

export const cssCalcFunction = createBuilder<CSSCalcFunction>(
	"CSSCalcFunction",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
