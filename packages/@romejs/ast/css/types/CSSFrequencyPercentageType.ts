import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSFrequencyPercentageType = JSNodeBase & {
	type: "CSSFrequencyPercentageType";
};

export const cssFrequencyPercentageType = createBuilder<CSSFrequencyPercentageType>(
	"CSSFrequencyPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
