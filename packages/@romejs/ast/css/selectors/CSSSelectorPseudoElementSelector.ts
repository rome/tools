import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorPseudoElementSelector = JSNodeBase & {
	type: "CSSSelectorPseudoElementSelector";
};

export const cssSelectorPseudoElementSelector = createBuilder<CSSSelectorPseudoElementSelector>(
	"CSSSelectorPseudoElementSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
