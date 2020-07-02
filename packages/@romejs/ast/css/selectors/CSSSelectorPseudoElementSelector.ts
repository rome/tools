import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// ::after
export type CSSSelectorPseudoElementSelector = NodeBaseWithComments & {
	type: "CSSSelectorPseudoElementSelector";
	name: CSSIdentifierType;
};

export const cssSelectorPseudoElementSelector = createBuilder<CSSSelectorPseudoElementSelector>(
	"CSSSelectorPseudoElementSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
		},
	},
);
