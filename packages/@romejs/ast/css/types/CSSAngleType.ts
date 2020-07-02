import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSAngleType = JSNodeBase & {
	type: "CSSAngleType";
};

export const cssAngleType = createBuilder<CSSAngleType>(
	"CSSAngleType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
