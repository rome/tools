import {CSSCalcSum, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMaxFunction extends NodeBaseWithComments {
	readonly type: "CSSMaxFunction";
	readonly params: CSSCalcSum[];
	readonly name: string;
}

export const cssMaxFunction = createBuilder<CSSMaxFunction>(
	"CSSMaxFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true
		},
	},
);
