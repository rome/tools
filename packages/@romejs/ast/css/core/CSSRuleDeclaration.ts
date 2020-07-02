import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRuleDeclaration = JSNodeBase & {
	type: "CSSRuleDeclaration";
};

export const cssRuleDeclaration = createBuilder<CSSRuleDeclaration>(
	"CSSRuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
