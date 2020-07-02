import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSAnglePercentageType = JSNodeBase & {
	type: "CSSAnglePercentageType";
};

export const cssAnglePercentageType = createBuilder<CSSAnglePercentageType>(
	"CSSAnglePercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
