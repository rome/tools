import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTimePercentageType = NodeBaseWithComments & {
	type: "CSSTimePercentageType";
};

export const cssTimePercentageType = createBuilder<CSSTimePercentageType>(
	"CSSTimePercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
