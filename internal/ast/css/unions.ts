import * as n from "@internal/ast";
import {CSSGridRepeatValue} from "@internal/ast/css/grid/CSSGridRepeatValue";

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
	| AnyFunction
	| n.CSSFunction
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
	| n.CSSCalcSum
	| n.CSSMediaQueryList
	| n.CSSSupportsCondition
	| n.CSSAtImport
	| n.CSSFitContentFunction
	| n.CSSFlex
	| n.CSSGridRepeatValue
	| n.CSSRaw;

export type AnyFunction =
	| n.CSSFunction
	| n.CSSVarFunction
	| n.CSSUrlFunction
	| n.CSSCalcFunction
	| n.CSSFitContentFunction
	| n.CSSMinFunction
	| n.CSSMaxFunction
	| n.CSSMinmaxFunction
	| n.CSSRepeatFunction;

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
	AnyCSSValue | n.CSSRule | n.CSSAtRule | n.CSSDeclaration
>;

export type CSSPseudoSelector =
	| n.CSSPseudoClassSelector
	| n.CSSPseudoElementSelector;

export type CSSMinmaxParam = n.CSSRaw | n.CSSDimension | n.CSSPercentage;

export type CSSGridRepeatParams = [
	tracker: CSSGridRepeatTracker,
	values: CSSGridRepeatValue
];

export type CSSGridRepeatTracker = n.CSSRaw | n.CSSNumber;

export type CSSGridRepeatValues = Array<
	| n.CSSPercentage
	| n.CSSDimension
	| n.CSSMinmaxFunction
	| n.CSSFitContentFunction
	| n.CSSFlex
	| n.CSSRaw
	| n.CSSLineName
>;
