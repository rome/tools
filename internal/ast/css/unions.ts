import * as n from "@internal/ast";
import {
	CSSAtImport,
	CSSAtPage,
	CSSBlock,
	CSSCalcFunction,
	CSSCalcSum,
	CSSComma,
	CSSCustomProperty,
	CSSDimension,
	CSSFontFace,
	CSSFunction,
	CSSHash,
	CSSIdentifier,
	CSSKeyframe,
	CSSMaxFunction,
	CSSMediaFeatureComparison,
	CSSMediaFeatureGT,
	CSSMediaFeatureLT,
	CSSMediaFeatureName,
	CSSMediaFeatureValue,
	CSSMediaQueryList,
	CSSMinFunction,
	CSSNumber,
	CSSPercentage,
	CSSRaw,
	CSSString,
	CSSSupportsCondition,
	CSSUrlFunction,
	CSSVarFunction,
	CSSWhitespace,
} from "@internal/ast";

export type AnyCSSPattern =
	| n.CSSIdSelector
	| n.CSSTypeSelector
	| n.CSSClassSelector
	| n.CSSPseudoClassSelector
	| n.CSSPseudoElementSelector
	| n.CSSAttributeSelector
	| n.CSSUniversalSelector
	| n.CSSCombinator;

export type AnyCSSValue =
	| CSSFunction
	| CSSVarFunction
	| CSSBlock
	| CSSDimension
	| CSSPercentage
	| CSSIdentifier
	| CSSNumber
	| CSSHash
	| CSSWhitespace
	| CSSString
	| CSSComma
	| CSSCustomProperty
	| CSSUrlFunction
	| CSSCalcFunction
	| CSSCalcSum
	| CSSMaxFunction
	| CSSMinFunction
	| CSSMediaQueryList
	| CSSSupportsCondition
	| CSSAtImport
	| CSSRaw;

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

export type CSSAtRuleValue =
	| CSSBlock
	| CSSKeyframe
	| CSSMediaQueryList
	| CSSAtPage
	| CSSFontFace
	| CSSAtImport;

export type CSSAtImportValue = CSSString | CSSUrlFunction;
