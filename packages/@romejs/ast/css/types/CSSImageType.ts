import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSImageType = JSNodeBase & {
	type: "CSSImageType";
};

export const cssImageType = createBuilder<CSSImageType>(
	"CSSImageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
