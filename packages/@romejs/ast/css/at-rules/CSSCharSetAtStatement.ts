import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSCharSetAtStatement = JSNodeBase & {
	type: "CSSCharSetAtStatement";
};

export const cssCharSetAtStatement = createBuilder<CSSCharSetAtStatement>(
	"CSSCharSetAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
