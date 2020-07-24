import {
	CSSIdentifierType,
	CSSKeyframesRuleDeclaration,
	CSSStringType,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

// @keyframes
export interface CSSKeyframesAtStatement extends NodeBaseWithComments {
	type: "CSSKeyframesAtStatement";
	name: CSSIdentifierType | CSSStringType;
	body: Array<CSSKeyframesRuleDeclaration>;
}

export const cssKeyframesAtStatement = createBuilder<CSSKeyframesAtStatement>(
	"CSSKeyframesAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {
			name: true,
			body: true,
		},
	},
);
