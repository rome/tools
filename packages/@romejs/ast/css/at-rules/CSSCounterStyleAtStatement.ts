import {
	CSSIdentifierType,
	CSSRuleDeclaration,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

// @counter-style
export type CSSCounterStyleAtStatement = NodeBaseWithComments & {
	type: "CSSCounterStyleAtStatement";
	name: CSSIdentifierType;
	declarations: Array<CSSRuleDeclaration>;
	// TODO should we have a field for each known valid @counter-style property?
};

export const cssCounterStyleAtStatement = createBuilder<CSSCounterStyleAtStatement>(
	"CSSCounterStyleAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			declarations: true,
		},
	},
);
