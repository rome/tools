import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSAngleType = NodeBaseWithComments & {
	type: "CSSAngleType";
	// TODO
};

export const cssAngleType = createBuilder<CSSAngleType>(
	"CSSAngleType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
