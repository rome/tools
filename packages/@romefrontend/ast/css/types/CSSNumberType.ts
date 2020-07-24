import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSNumberType extends NodeBaseWithComments {
	type: "CSSNumberType";
	// TODO
}

export const cssNumberType = createBuilder<CSSNumberType>(
	"CSSNumberType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
