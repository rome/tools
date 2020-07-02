import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorPseudoElementSelector = NodeBaseWithComments & {
	type: "CSSSelectorPseudoElementSelector";
};

export const cssSelectorPseudoElementSelector = createBuilder<CSSSelectorPseudoElementSelector>(
	"CSSSelectorPseudoElementSelector",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
