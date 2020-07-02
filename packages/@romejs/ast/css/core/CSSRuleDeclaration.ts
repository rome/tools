import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRuleDeclaration = NodeBaseWithComments & {
	type: "CSSRuleDeclaration";
};

export const cssRuleDeclaration = createBuilder<CSSRuleDeclaration>(
	"CSSRuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
