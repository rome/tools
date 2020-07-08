import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSPercentageType = NodeBaseWithComments & {
	type: "CSSPercentageType";
	value: number;
};

export const cssPercentageType = createBuilder<CSSPercentageType>(
	"CSSPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
