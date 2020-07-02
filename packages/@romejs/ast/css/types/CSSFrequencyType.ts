import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSFrequencyType = JSNodeBase & {
	type: "CSSFrequencyType";
};

export const cssFrequencyType = createBuilder<CSSFrequencyType>(
	"CSSFrequencyType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
