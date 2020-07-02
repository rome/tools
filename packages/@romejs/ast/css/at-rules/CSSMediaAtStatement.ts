import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSMediaAtStatement = JSNodeBase & {
	type: "CSSMediaAtStatement";
};

export const cssMediaAtStatement = createBuilder<CSSMediaAtStatement>(
	"CSSMediaAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
