import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSFrequencyPercentageType = NodeBaseWithComments & {
	type: "CSSFrequencyPercentageType";
	// TODO
};

export const cssFrequencyPercentageType = createBuilder<CSSFrequencyPercentageType>(
	"CSSFrequencyPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
