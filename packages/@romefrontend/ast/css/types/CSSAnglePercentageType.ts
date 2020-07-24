import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSAnglePercentageType extends NodeBaseWithComments {
	type: "CSSAnglePercentageType";
	// TODO
}

export const cssAnglePercentageType = createBuilder<CSSAnglePercentageType>(
	"CSSAnglePercentageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
