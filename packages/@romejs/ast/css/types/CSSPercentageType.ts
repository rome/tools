import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSPercentageType = NodeBaseWithComments & {
	type: "CSSPercentageType";
};

export const cssPercentageType = createBuilder<CSSPercentageType>(
	"CSSPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
