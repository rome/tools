import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSPercentageType extends NodeBaseWithComments {
	type: "CSSPercentageType";
	value: number;
}

export const cssPercentageType = createBuilder<CSSPercentageType>(
	"CSSPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
