import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTimePercentageType = JSNodeBase & {
	type: "CSSTimePercentageType";
};

export const cssTimePercentageType = createBuilder<CSSTimePercentageType>(
	"CSSTimePercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
