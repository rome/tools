import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSNumberType = JSNodeBase & {
	type: "CSSNumberType";
};

export const cssNumberType = createBuilder<CSSNumberType>(
	"CSSNumberType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
