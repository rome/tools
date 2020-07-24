import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSGradientType extends NodeBaseWithComments {
	type: "CSSGradientType";
	// TODO
}

export const cssGradientType = createBuilder<CSSGradientType>(
	"CSSGradientType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
