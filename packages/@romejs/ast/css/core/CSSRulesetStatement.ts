import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRulesetStatement = JSNodeBase & {
	type: "CSSRulesetStatement";
};

export const cssRulesetStatement = createBuilder<CSSRulesetStatement>(
	"CSSRulesetStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
