import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRulesetStatement = NodeBaseWithComments & {
	type: "CSSRulesetStatement";
};

export const cssRulesetStatement = createBuilder<CSSRulesetStatement>(
	"CSSRulesetStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
