import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSCalcOperation extends NodeBaseWithComments {
	readonly type: "CSSCalcOperation";
	readonly value: "+" | "-" | "*" | "/";
}

export const cssCalcOperation = createBuilder<CSSCalcOperation>(
	"CSSCalcOperation",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
