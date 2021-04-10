import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureGT extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureGT";
	readonly hasEqual?: boolean;
}

export const cssMediaFeatureGT = createBuilder<CSSMediaFeatureGT>(
	"CSSMediaFeatureGT",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
