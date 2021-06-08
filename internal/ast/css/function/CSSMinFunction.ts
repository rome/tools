import {CSSCalcSum, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMinFunction extends NodeBaseWithComments {
	readonly type: "CSSMinFunction";
	readonly params: CSSCalcSum[];
	readonly name: string;
}

export const cssMinFunction = createBuilder<CSSMinFunction>(
	"CSSMinFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true
		},
	},
);
