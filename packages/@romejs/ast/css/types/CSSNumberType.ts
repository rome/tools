import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSNumberType = NodeBaseWithComments & {
	type: "CSSNumberType";
};

export const cssNumberType = createBuilder<CSSNumberType>(
	"CSSNumberType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
