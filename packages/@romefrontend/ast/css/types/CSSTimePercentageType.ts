import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSTimePercentageType extends NodeBaseWithComments {
	type: "CSSTimePercentageType";
	// TODO
}

export const cssTimePercentageType = createBuilder<CSSTimePercentageType>(
	"CSSTimePercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
