import {
	NodeBaseWithComments, RangeNameAndValue, RangeValueAndName, RangeValueGTValue, RangeValueLTValue,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureRange extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureRange";
	readonly value:
		| RangeNameAndValue
		| RangeValueAndName
		| RangeValueGTValue
		| RangeValueLTValue;
}

export const cssMediaFeatureRange = createBuilder<CSSMediaFeatureRange>(
	"CSSMediaFeatureRange",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
