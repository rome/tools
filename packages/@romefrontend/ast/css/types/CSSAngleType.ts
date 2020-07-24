import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSAngleType extends NodeBaseWithComments {
	type: "CSSAngleType";
	// TODO
}

export const cssAngleType = createBuilder<CSSAngleType>(
	"CSSAngleType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
