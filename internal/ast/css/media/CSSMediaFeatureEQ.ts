import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureEQ extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureEQ";
}

export const cssMediaFeatureEQ = createBuilder<CSSMediaFeatureEQ>(
	"CSSMediaFeatureEQ",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
