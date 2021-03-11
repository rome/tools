import {
	CSSMediaFeatureComparison,
	CSSMediaFeatureGT,
	CSSMediaFeatureLT,
	CSSMediaFeatureName,
	CSSMediaFeatureValue,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export type RangeNameAndValue = [
	CSSMediaFeatureName,
	CSSMediaFeatureComparison,
	CSSMediaFeatureValue
];
export type RangeValueAndName = [
	CSSMediaFeatureValue,
	CSSMediaFeatureComparison,
	CSSMediaFeatureName
];
export type RangeValueGTValue = [
	CSSMediaFeatureValue,
	CSSMediaFeatureLT,
	CSSMediaFeatureName,
	CSSMediaFeatureLT,
	CSSMediaFeatureName
];
export type RangeValueLTValue = [
	CSSMediaFeatureValue,
	CSSMediaFeatureGT,
	CSSMediaFeatureName,
	CSSMediaFeatureGT,
	CSSMediaFeatureName
];

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
