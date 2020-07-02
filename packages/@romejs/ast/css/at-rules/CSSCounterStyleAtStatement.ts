import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSCounterStyleAtStatement = NodeBaseWithComments & {
	type: "CSSCounterStyleAtStatement";
};

export const cssCounterStyleAtStatement = createBuilder<CSSCounterStyleAtStatement>(
	"CSSCounterStyleAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
