import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSPercentageType = JSNodeBase & {
	type: "CSSPercentageType";
};

export const cssPercentageType = createBuilder<CSSPercentageType>(
	"CSSPercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
