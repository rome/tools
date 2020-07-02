import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSupportsAtStatement = JSNodeBase & {
	type: "CSSSupportsAtStatement";
};

export const cssSupportsAtStatement = createBuilder<CSSSupportsAtStatement>(
	"CSSSupportsAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
