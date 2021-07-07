import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSFlex extends NodeBaseWithComments {
	readonly type: "CSSFlex";
	readonly value: number;
}

export const cssFlex = createBuilder<CSSFlex>(
	"CSSFlex",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
