import * as n from "@romefrontend/ast";

export type AnyCSSRuleStatement =
	| n.CSSRulesetStatement
	| n.CSSCharSetAtStatement
	| n.CSSImportAtStatement
	| n.CSSNamespaceAtStatement
	| n.CSSMediaAtStatement
	| n.CSSSupportsAtStatement
	| n.CSSDocumentAtStatement
	| n.CSSPageAtStatement
	| n.CSSFontFaceAtStatement
	| n.CSSKeyframesAtStatement
	| n.CSSViewportAtStatement
	| n.CSSCounterStyleAtStatement;

export type AnyCSSSelector =
	| n.CSSSelectorClass
	| n.CSSSelectorId
	| n.CSSSelectorTag
	| n.CSSSelectorAttribute
	| n.CSSSelectorUniversal
	| n.CSSSelectorPseudoClass
	| n.CSSSelectorPseudoElementSelector
	| n.CSSSelectorCombinator
	| n.CSSSelectorChain;
