import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSURLType = JSNodeBase & {
	type: "CSSURLType";
};

export const cssurlType = createBuilder<CSSURLType>(
	"CSSURLType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
