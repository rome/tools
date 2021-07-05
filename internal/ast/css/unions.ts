import * as n from "@internal/ast";

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
	| n.CSSFunction
	| n.CSSVarFunction
	| n.CSSBlock
	| n.CSSDimension
	| n.CSSPercentage
	| n.CSSIdentifier
	| n.CSSNumber
	| n.CSSHash
	| n.CSSWhitespace
	| n.CSSString
	| n.CSSComma
	| n.CSSCustomProperty
	| n.CSSUrlFunction
	| n.CSSCalcFunction
	| n.CSSCalcSum
	| n.CSSMaxFunction
	| n.CSSMinFunction
	| n.CSSMinmaxFunction
	| n.CSSMediaQueryList
	| n.CSSSupportsCondition
	| n.CSSAtImport
	| n.CSSFitContent
	| n.CSSRaw;

export type AnyFunction =
	| n.CSSFunction
	| n.CSSVarFunction
	| n.CSSUrlFunction
	| n.CSSAtImport
	| n.CSSRaw;

export type RangeNameAndValue = [
	n.CSSMediaFeatureName,
	n.CSSMediaFeatureComparison,
	n.CSSMediaFeatureValue
];
export type RangeValueAndName = [
	n.CSSMediaFeatureValue,
	n.CSSMediaFeatureComparison,
	n.CSSMediaFeatureName
];
export type RangeValueGTValue = [
	n.CSSMediaFeatureValue,
	n.CSSMediaFeatureLT,
	n.CSSMediaFeatureName,
	n.CSSMediaFeatureLT,
	n.CSSMediaFeatureName
];
export type RangeValueLTValue = [
	n.CSSMediaFeatureValue,
	n.CSSMediaFeatureGT,
	n.CSSMediaFeatureName,
	n.CSSMediaFeatureGT,
	n.CSSMediaFeatureName
];

export type CSSAtRuleValue =
	| n.CSSBlock
	| n.CSSKeyframe
	| n.CSSMediaQueryList
	| n.CSSAtPage
	| n.CSSFontFace
	| n.CSSAtImport;

export type CSSFitContentValue = n.CSSDimension | n.CSSPercentage | n.CSSNumber;

export type CSSAtImportValue = n.CSSString | n.CSSUrlFunction;

export type CSSBlockValue = Array<
	AnyCSSValue | CSSRule | n.CSSAtRule | n.CSSDeclaration
>;

export type CSSPseudoSelector =
	| n.CSSPseudoClassSelector
	| n.CSSPseudoElementSelector;

export type CSSGridRepeatParams = [CSSGridRepeatTracker, CSSGridRepeatValues];

export type CSSGridRepeatTracker = n.CSSRaw | n.CSSNumber;

export type CSSGridRepeatValues =
	| n.CSSPercentage
	| n.CSSDimension
	| n.CSSMinmaxFunction
	| n.CSSFlex
	| n.CSSRaw;
