import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSIntegerType = JSNodeBase & {
	type: "CSSIntegerType";
};

export const cssIntegerType = createBuilder<CSSIntegerType>(
	"CSSIntegerType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
