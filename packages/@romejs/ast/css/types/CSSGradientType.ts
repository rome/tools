import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSGradientType = NodeBaseWithComments & {
	type: "CSSGradientType";
};

export const cssGradientType = createBuilder<CSSGradientType>(
	"CSSGradientType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
