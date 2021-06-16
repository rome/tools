/* GENERATED:START(hash:a3ec06e9d14504f2e66cf6ea27c28c340770c30a,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
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
export type A11YRulesWithCategory = `a11y/${A11YRules}`;

export type CssRules = "noDuplicateCustomProperties" | "noImportantInKeyframes";
export type CssRulesWithCategory = `css/${CssRules}`;

export type HtmlRules = "useClosingNonVoid";
export type HtmlRulesWithCategory = `html/${HtmlRules}`;

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
export type JsRulesWithCategory = `js/${JsRules}`;

export type JsxRules =
	| "noCommentText"
	| "noDuplicateProps"
	| "noImplicitBoolean"
	| "noPropSpreading"
	| "useJSXFileExtension"
	| "usePascalCase"
	| "useSelfClosingElements";
export type JsxRulesWithCategory = `jsx/${JsxRules}`;

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
export type ReactRulesWithCategory = `react/${ReactRules}`;

export type RegexRules =
	| "noDuplicateGroupNamesInRegularExpressions"
	| "noEmptyCharacterClass"
	| "noEmptyMatches"
	| "noMultipleSpacesInRegularExpressionLiterals"
	| "noPosixInRegularExpression"
	| "noReferenceToNonExistingGroup";
export type RegexRulesWithCategory = `regex/${RegexRules}`;

export type TsRules =
	| "noExplicitAny"
	| "preferShorthandArrayType"
	| "useInterfaces"
	| "useSimplifiedBooleanExpression"
	| "useTsExpectError";
export type TsRulesWithCategory = `ts/${TsRules}`;

export type RuleNames =
	| A11YRules
	| CssRules
	| HtmlRules
	| JsRules
	| JsxRules
	| ReactRules
	| RegexRules
	| TsRules;

export const ruleNames: Set<RuleNames> = new Set();
ruleNames.add("noAccessKey");
ruleNames.add("noAriaUnsupportedElements");
ruleNames.add("noAutofocus");
ruleNames.add("noDistractingElements");
ruleNames.add("noHeaderScope");
ruleNames.add("noNoninteractiveElementToInteractiveRole");
ruleNames.add("noNoninteractiveTabindex");
ruleNames.add("noOnChange");
ruleNames.add("noPositiveTabindex");
ruleNames.add("noRedundantAlt");
ruleNames.add("noRedundantRoles");
ruleNames.add("noSvgWithoutTitle");
ruleNames.add("noTargetBlank");
ruleNames.add("useAltText");
ruleNames.add("useAnchorContent");
ruleNames.add("useAriaProps");
ruleNames.add("useAriaPropsForRole");
ruleNames.add("useAriaProptypes");
ruleNames.add("useHeadingContent");
ruleNames.add("useHtmlLang");
ruleNames.add("useIframeTitle");
ruleNames.add("useKeyWithClickEvents");
ruleNames.add("useKeyWithMouseEvents");
ruleNames.add("useMediaCaption");
ruleNames.add("useValidAnchor");
ruleNames.add("useValidLang");
ruleNames.add("noDuplicateCustomProperties");
ruleNames.add("noImportantInKeyframes");
ruleNames.add("useClosingNonVoid");
ruleNames.add("noArguments");
ruleNames.add("noAsyncPromiseExecutor");
ruleNames.add("noCatchAssign");
ruleNames.add("noCommaOperator");
ruleNames.add("noCompareNegZero");
ruleNames.add("noCondAssign");
ruleNames.add("noDebugger");
ruleNames.add("noDelete");
ruleNames.add("noDeleteVars");
ruleNames.add("noDoubleEquals");
ruleNames.add("noDupeArgs");
ruleNames.add("noDuplicateCase");
ruleNames.add("noDuplicateImportSource");
ruleNames.add("noDuplicateKeys");
ruleNames.add("noEmptyBlocks");
ruleNames.add("noExtraBooleanCast");
ruleNames.add("noFunctionAssign");
ruleNames.add("noGetterReturn");
ruleNames.add("noImportAssign");
ruleNames.add("noLabelVar");
ruleNames.add("noNegationElse");
ruleNames.add("noNestedTernary");
ruleNames.add("noRestrictedGlobals");
ruleNames.add("noSetterReturn");
ruleNames.add("noShadowRestrictedNames");
ruleNames.add("noShoutyConstants");
ruleNames.add("noSingleCharRegexAlternatives");
ruleNames.add("noSparseArray");
ruleNames.add("noTemplateCurlyInString");
ruleNames.add("noUndeclaredVariables");
ruleNames.add("noUnnecessaryContinue");
ruleNames.add("noUnsafeFinally");
ruleNames.add("noUnsafeNegation");
ruleNames.add("noUnusedTemplateLiteral");
ruleNames.add("noUnusedVariables");
ruleNames.add("noVar");
ruleNames.add("preferOptionalChaining");
ruleNames.add("useBlockStatements");
ruleNames.add("useCamelCase");
ruleNames.add("useDefaultExportBasename");
ruleNames.add("useDefaultImportBasename");
ruleNames.add("useFunctionDeclarations");
ruleNames.add("useSimplifiedLogicalExpression");
ruleNames.add("useSingleCaseStatement");
ruleNames.add("useSingleVarDeclarator");
ruleNames.add("useSortedSpecifiers");
ruleNames.add("useTemplate");
ruleNames.add("useWhile");
ruleNames.add("noCommentText");
ruleNames.add("noDuplicateProps");
ruleNames.add("noImplicitBoolean");
ruleNames.add("noPropSpreading");
ruleNames.add("useJSXFileExtension");
ruleNames.add("usePascalCase");
ruleNames.add("useSelfClosingElements");
ruleNames.add("noAccessStateInSetState");
ruleNames.add("noArrayIndexKey");
ruleNames.add("noChildrenProp");
ruleNames.add("noDanger");
ruleNames.add("noDangerWithChildren");
ruleNames.add("noDidMountSetState");
ruleNames.add("noDidUpdateSetState");
ruleNames.add("noDirectMutationState");
ruleNames.add("noFindDOMNode");
ruleNames.add("noRedundantShouldComponentUpdate");
ruleNames.add("noRenderReturnValue");
ruleNames.add("noStringRefs");
ruleNames.add("noThisInSFC");
ruleNames.add("noUnsafe");
ruleNames.add("noUselessFragment");
ruleNames.add("noVoidElementsWithChildren");
ruleNames.add("noWillUpdateSetState");
ruleNames.add("useButtonType");
ruleNames.add("useFragmentSyntax");
ruleNames.add("useKey");
ruleNames.add("useRenderReturn");
ruleNames.add("useSortComp");
ruleNames.add("useStylePropObject");
ruleNames.add("noDuplicateGroupNamesInRegularExpressions");
ruleNames.add("noEmptyCharacterClass");
ruleNames.add("noEmptyMatches");
ruleNames.add("noMultipleSpacesInRegularExpressionLiterals");
ruleNames.add("noPosixInRegularExpression");
ruleNames.add("noReferenceToNonExistingGroup");
ruleNames.add("noExplicitAny");
ruleNames.add("preferShorthandArrayType");
ruleNames.add("useInterfaces");
ruleNames.add("useSimplifiedBooleanExpression");
ruleNames.add("useTsExpectError");

// These types are used for the project load
export type A11YRulesCategoryRules = Map<A11YRules, boolean>;
export type CssRulesCategoryRules = Map<CssRules, boolean>;
export type HtmlRulesCategoryRules = Map<HtmlRules, boolean>;
export type JsRulesCategoryRules = Map<JsRules, boolean>;
export type JsxRulesCategoryRules = Map<JsxRules, boolean>;
export type ReactRulesCategoryRules = Map<ReactRules, boolean>;
export type RegexRulesCategoryRules = Map<RegexRules, boolean>;
export type TsRulesCategoryRules = Map<TsRules, boolean>;

export type LintRuleName =
	| A11YRulesWithCategory
	| CssRulesWithCategory
	| HtmlRulesWithCategory
	| JsRulesWithCategory
	| JsxRulesWithCategory
	| ReactRulesWithCategory
	| RegexRulesWithCategory
	| TsRulesWithCategory;

// These types are used for the project load
export type ProjectLintRules = {
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
