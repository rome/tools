/* GENERATED:START(hash:9f47e5e21b40939b310b24e21808e22895af2bca,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
export type LintCategories =
	| "a11y"
	| "css"
	| "html"
	| "js"
	| "jsx"
	| "react"
	| "regex"
	| "ts";

export const lintCategories: Set<LintCategories> = new Set();
lintCategories.add("a11y");
lintCategories.add("css");
lintCategories.add("html");
lintCategories.add("js");
lintCategories.add("jsx");
lintCategories.add("react");
lintCategories.add("regex");
lintCategories.add("ts");

export type A11YRules =
	| "noAccessKey"
	| "noAriaUnsupportedElements"
	| "noAutofocus"
	| "noDistractingElements"
	| "noHeaderScope"
	| "noNoninteractiveElementToInteractiveRole"
	| "noNoninteractiveTabindex"
	| "noOnChange"
	| "noPositiveTabindex"
	| "noRedundantAlt"
	| "noRedundantRoles"
	| "noSvgWithoutTitle"
	| "noTargetBlank"
	| "useAltText"
	| "useAnchorContent"
	| "useAriaProps"
	| "useAriaPropsForRole"
	| "useAriaProptypes"
	| "useHeadingContent"
	| "useHtmlLang"
	| "useIframeTitle"
	| "useKeyWithClickEvents"
	| "useKeyWithMouseEvents"
	| "useMediaCaption"
	| "useValidAnchor"
	| "useValidLang";

export type CssRules = "noDuplicateCustomProperties" | "noImportantInKeyframes";

export type HtmlRules = "useClosingNonVoid";

export type JsRules =
	| "noArguments"
	| "noAsyncPromiseExecutor"
	| "noCatchAssign"
	| "noCommaOperator"
	| "noCompareNegZero"
	| "noCondAssign"
	| "noDebugger"
	| "noDelete"
	| "noDeleteVars"
	| "noDoubleEquals"
	| "noDupeArgs"
	| "noDuplicateCase"
	| "noDuplicateImportSource"
	| "noDuplicateKeys"
	| "noEmptyBlocks"
	| "noExtraBooleanCast"
	| "noFunctionAssign"
	| "noGetterReturn"
	| "noImportAssign"
	| "noLabelVar"
	| "noNegationElse"
	| "noNestedTernary"
	| "noRestrictedGlobals"
	| "noSetterReturn"
	| "noShadowRestrictedNames"
	| "noShoutyConstants"
	| "noSingleCharRegexAlternatives"
	| "noSparseArray"
	| "noTemplateCurlyInString"
	| "noUndeclaredVariables"
	| "noUnnecessaryContinue"
	| "noUnsafeFinally"
	| "noUnsafeNegation"
	| "noUnusedTemplateLiteral"
	| "noUnusedVariables"
	| "noVar"
	| "preferOptionalChaining"
	| "useBlockStatements"
	| "useCamelCase"
	| "useDefaultExportBasename"
	| "useDefaultImportBasename"
	| "useFunctionDeclarations"
	| "useSimplifiedLogicalExpression"
	| "useSingleCaseStatement"
	| "useSingleVarDeclarator"
	| "useSortedSpecifiers"
	| "useTemplate"
	| "useWhile";

export type JsxRules =
	| "noCommentText"
	| "noDuplicateProps"
	| "noImplicitBoolean"
	| "noPropSpreading"
	| "useJSXFileExtension"
	| "usePascalCase"
	| "useSelfClosingElements";

export type ReactRules =
	| "noAccessStateInSetState"
	| "noArrayIndexKey"
	| "noChildrenProp"
	| "noDanger"
	| "noDangerWithChildren"
	| "noDidMountSetState"
	| "noDidUpdateSetState"
	| "noDirectMutationState"
	| "noFindDOMNode"
	| "noRedundantShouldComponentUpdate"
	| "noRenderReturnValue"
	| "noStringRefs"
	| "noThisInSFC"
	| "noUnsafe"
	| "noUselessFragment"
	| "noVoidElementsWithChildren"
	| "noWillUpdateSetState"
	| "useButtonType"
	| "useFragmentSyntax"
	| "useKey"
	| "useRenderReturn"
	| "useSortComp"
	| "useStylePropObject";

export type RegexRules =
	| "noDuplicateGroupNamesInRegularExpressions"
	| "noEmptyCharacterClass"
	| "noEmptyMatches"
	| "noMultipleSpacesInRegularExpressionLiterals"
	| "noPosixInRegularExpression"
	| "noReferenceToNonExistingGroup";

export type TsRules =
	| "noExplicitAny"
	| "preferShorthandArrayType"
	| "useInterfaces"
	| "useSimplifiedBooleanExpression";

export type A11YRulesCategoryRules = {[key in A11YRules]?: boolean};

export type CssRulesCategoryRules = {[key in CssRules]?: boolean};

export type HtmlRulesCategoryRules = {[key in HtmlRules]?: boolean};

export type JsRulesCategoryRules = {[key in JsRules]?: boolean};

export type JsxRulesCategoryRules = {[key in JsxRules]?: boolean};

export type ReactRulesCategoryRules = {[key in ReactRules]?: boolean};

export type RegexRulesCategoryRules = {[key in RegexRules]?: boolean};

export type TsRulesCategoryRules = {[key in TsRules]?: boolean};

export type LintRuleName = `${LintCategories}/${
	| A11YRules
	| CssRules
	| HtmlRules
	| JsRules
	| JsxRules
	| ReactRules
	| RegexRules
	| TsRules}`;

export type LintRules = {
	a11y?: A11YRulesCategoryRules;
	css?: CssRulesCategoryRules;
	html?: HtmlRulesCategoryRules;
	js?: JsRulesCategoryRules;
	jsx?: JsxRulesCategoryRules;
	react?: ReactRulesCategoryRules;
	regex?: RegexRulesCategoryRules;
	ts?: TsRulesCategoryRules;
};
/* GENERATED:END(id:main) */
