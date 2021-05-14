import {
	CSSDimension,
	CSSIdentifier,
	CSSNumber,
	CSSRatio,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureValue extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureValue";
	readonly value: CSSNumber | CSSDimension | CSSIdentifier | CSSRatio;
}

export const cssMediaFeatureValue = createBuilder<CSSMediaFeatureValue>(
	"CSSMediaFeatureValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
