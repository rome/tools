import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSKeyframesRuleDeclaration = NodeBaseWithComments & {
	type: "CSSKeyframesRuleDeclaration";
};

export const cssKeyframesRuleDeclaration = createBuilder<CSSKeyframesRuleDeclaration>(
	"CSSKeyframesRuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
