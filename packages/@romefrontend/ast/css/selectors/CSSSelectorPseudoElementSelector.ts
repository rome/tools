import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// ::after
export interface CSSSelectorPseudoElementSelector extends NodeBaseWithComments {
	type: "CSSSelectorPseudoElementSelector";
	name: CSSIdentifierType;
}

export const cssSelectorPseudoElementSelector = createBuilder<CSSSelectorPseudoElementSelector>(
	"CSSSelectorPseudoElementSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
		},
	},
);
