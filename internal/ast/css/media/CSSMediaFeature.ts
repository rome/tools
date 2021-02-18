import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeature extends NodeBaseWithComments {
	readonly type: "CSSMediaFeature";
}

export const cssMediaFeature = createBuilder<CSSMediaFeature>(
	"CSSMediaFeature",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
