/* GENERATED:START(hash:93eff879ca76083328aedd21b5e190ba1aa4f354,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
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

export type a11YRules =
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
export type a11YRulesWithCategory = `a11y/${a11YRules}`;

export type cssRules = "noDuplicateCustomProperties" | "noImportantInKeyframes";
export type cssRulesWithCategory = `css/${cssRules}`;

export type htmlRules = "useClosingNonVoid";
export type htmlRulesWithCategory = `html/${htmlRules}`;

export type jsRules =
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
export type jsRulesWithCategory = `js/${jsRules}`;

export type jsxRules =
	| "noCommentText"
	| "noDuplicateProps"
	| "noImplicitBoolean"
	| "noPropSpreading"
	| "useJSXFileExtension"
	| "usePascalCase"
	| "useSelfClosingElements";
export type jsxRulesWithCategory = `jsx/${jsxRules}`;

export type reactRules =
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
export type reactRulesWithCategory = `react/${reactRules}`;

export type regexRules =
	| "noDuplicateGroupNamesInRegularExpressions"
	| "noEmptyCharacterClass"
	| "noEmptyMatches"
	| "noMultipleSpacesInRegularExpressionLiterals"
	| "noPosixInRegularExpression"
	| "noReferenceToNonExistingGroup";
export type regexRulesWithCategory = `regex/${regexRules}`;

export type tsRules =
	| "noExplicitAny"
	| "preferShorthandArrayType"
	| "useInterfaces"
	| "useSimplifiedBooleanExpression"
	| "useTsExpectError";
export type tsRulesWithCategory = `ts/${tsRules}`;

export type RuleNames =
	| a11YRules
	| cssRules
	| htmlRules
	| jsRules
	| jsxRules
	| reactRules
	| regexRules
	| tsRules;

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
export type a11YRulesCategoryRules = Map<a11YRules, boolean>;
export type cssRulesCategoryRules = Map<cssRules, boolean>;
export type htmlRulesCategoryRules = Map<htmlRules, boolean>;
export type jsRulesCategoryRules = Map<jsRules, boolean>;
export type jsxRulesCategoryRules = Map<jsxRules, boolean>;
export type reactRulesCategoryRules = Map<reactRules, boolean>;
export type regexRulesCategoryRules = Map<regexRules, boolean>;
export type tsRulesCategoryRules = Map<tsRules, boolean>;

export type LintRuleName =
	| a11YRulesWithCategory
	| cssRulesWithCategory
	| htmlRulesWithCategory
	| jsRulesWithCategory
	| jsxRulesWithCategory
	| reactRulesWithCategory
	| regexRulesWithCategory
	| tsRulesWithCategory;

// These types are used for the project load
export type ProjectLintRules = {
	a11y?: a11YRulesCategoryRules;
	css?: cssRulesCategoryRules;
	html?: htmlRulesCategoryRules;
	js?: jsRulesCategoryRules;
	jsx?: jsxRulesCategoryRules;
	react?: reactRulesCategoryRules;
	regex?: regexRulesCategoryRules;
	ts?: tsRulesCategoryRules;
};
/* GENERATED:END(id:main) */
