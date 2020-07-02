import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesRuleDeclaration = JSNodeBase & {
	type: "CSSKeyframesRuleDeclaration";
};

export const cssKeyframesRuleDeclaration = createBuilder<CSSKeyframesRuleDeclaration>(
	"CSSKeyframesRuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
