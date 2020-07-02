import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRoot = JSNodeBase & {
	type: "CSSRoot";
};

export const cssRoot = createBuilder<CSSRoot>(
	"CSSRoot",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
