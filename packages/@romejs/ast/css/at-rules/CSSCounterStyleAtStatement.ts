import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSCounterStyleAtStatement = JSNodeBase & {
	type: "CSSCounterStyleAtStatement";
};

export const cssCounterStyleAtStatement = createBuilder<CSSCounterStyleAtStatement>(
	"CSSCounterStyleAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
