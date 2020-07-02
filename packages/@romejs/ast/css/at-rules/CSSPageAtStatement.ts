import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSPageAtStatement = JSNodeBase & {
	type: "CSSPageAtStatement";
};

export const cssPageAtStatement = createBuilder<CSSPageAtStatement>(
	"CSSPageAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
