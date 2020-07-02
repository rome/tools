import {CSSIdentifierType, NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// :hover
export type CSSSelectorPseudoClass = NodeBaseWithComments & {
	type: "CSSSelectorPseudoClass";
	name: CSSIdentifierType;
};

export const cssSelectorPseudoClass = createBuilder<CSSSelectorPseudoClass>(
	"CSSSelectorPseudoClass",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
		},
	},
);
