import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSStringType = JSNodeBase & {
	type: "CSSStringType";
};

export const cssStringType = createBuilder<CSSStringType>(
	"CSSStringType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
