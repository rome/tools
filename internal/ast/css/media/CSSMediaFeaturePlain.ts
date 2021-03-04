import {CSSMediaFeatureName, CSSMediaFeatureValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeaturePlain extends NodeBaseWithComments {
	readonly type: "CSSMediaFeaturePlain";
	readonly name?: CSSMediaFeatureName;
	readonly value: CSSMediaFeatureValue;
}

export const cssMediaFeaturePlain = createBuilder<CSSMediaFeaturePlain>(
	"CSSMediaFeaturePlain",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
			name: true
		},
	},
);
