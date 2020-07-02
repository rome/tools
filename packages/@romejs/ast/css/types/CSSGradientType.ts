import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSGradientType = JSNodeBase & {
	type: "CSSGradientType";
};

export const cssGradientType = createBuilder<CSSGradientType>(
	"CSSGradientType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
