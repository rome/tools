import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureName extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureName";
	readonly value: string;
}

export const cssMediaFeatureName = createBuilder<CSSMediaFeatureName>(
	"CSSMediaFeatureName",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
