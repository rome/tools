import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorPseudoClass = JSNodeBase & {
	type: "CSSSelectorPseudoClass";
};

export const cssSelectorPseudoClass = createBuilder<CSSSelectorPseudoClass>(
	"CSSSelectorPseudoClass",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
