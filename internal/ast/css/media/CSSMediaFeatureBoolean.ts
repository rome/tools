import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureBoolean extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureBoolean";
	readonly value: string;
}

export const cssMediaFeatureBoolean = createBuilder<CSSMediaFeatureBoolean>(
	"CSSMediaFeatureBoolean",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
