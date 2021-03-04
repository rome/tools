import {CSSMediaFeatureEQ, CSSMediaFeatureGT, CSSMediaFeatureLT, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureComparison extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureComparison";
	readonly value: CSSMediaFeatureLT | CSSMediaFeatureEQ | CSSMediaFeatureGT
}

export const cssMediaFeatureComparison = createBuilder<CSSMediaFeatureComparison>(
	"CSSMediaFeatureComparison",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
