import {
	CSSDimension,
	CSSNumber,
	CSSString,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureValue extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureValue";
	readonly value: CSSNumber | CSSDimension | CSSString;
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
