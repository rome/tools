import {
	CSSKeyframesFromKeyword,
	CSSKeyframesToKeyword,
	CSSPercentageType,
	CSSRuleDeclaration,
	NodeBaseWithComments,
} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSKeyframesRuleDeclaration extends NodeBaseWithComments {
	type: "CSSKeyframesRuleDeclaration";
	selector: CSSPercentageType | CSSKeyframesFromKeyword | CSSKeyframesToKeyword;
	declarations: Array<CSSRuleDeclaration>;
}

export const cssKeyframesRuleDeclaration = createBuilder<CSSKeyframesRuleDeclaration>(
	"CSSKeyframesRuleDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			selector: true,
			declarations: true,
		},
	},
);
