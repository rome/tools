import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSFrequencyPercentageType extends NodeBaseWithComments {
	type: "CSSFrequencyPercentageType";
	// TODO
}

export const cssFrequencyPercentageType = createBuilder<CSSFrequencyPercentageType>(
	"CSSFrequencyPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
