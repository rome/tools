import * as n from "@internal/ast";
import {
	CSSBlock,
	CSSCalcFunction,
	CSSCalcSum,
	CSSComma,
	CSSCustomProperty,
	CSSDimension,
	CSSFunction,
	CSSHash,
	CSSIdentifier,
	CSSMediaQueryList,
	CSSNumber,
	CSSPercentage,
	CSSRaw,
	CSSString,
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
	| CSSMediaQueryList
	| CSSRaw;

export type AnyFunction = CSSFunction | CSSVarFunction | CSSUrlFunction;
