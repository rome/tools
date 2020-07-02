import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorPseudoClass = NodeBaseWithComments & {
	type: "CSSSelectorPseudoClass";
};

export const cssSelectorPseudoClass = createBuilder<CSSSelectorPseudoClass>(
	"CSSSelectorPseudoClass",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
