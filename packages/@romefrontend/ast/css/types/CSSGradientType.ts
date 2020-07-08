import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSGradientType = NodeBaseWithComments & {
	type: "CSSGradientType";
	// TODO
};

export const cssGradientType = createBuilder<CSSGradientType>(
	"CSSGradientType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
