import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<Test | (Test[])>;

/* GENERATED:START(hash:af0cf2e610d11d715c74f35617f49733d1b399f8,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-ignore
import htmlUseClosingNonVoid from "./html/useClosingNonVoid.test.rjson";
// @ts-ignore
import htmlUseHtmlLang from "./html/useHtmlLang.test.rjson";
// @ts-ignore
import htmlUseValidLang from "./html/useValidLang.test.rjson";
// @ts-ignore
import jsNoArguments from "./js/noArguments.test.rjson";
// @ts-ignore
import jsNoAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor.test.rjson";
// @ts-ignore
import jsNoCatchAssign from "./js/noCatchAssign.test.rjson";
// @ts-ignore
import jsNoCommaOperator from "./js/noCommaOperator.test.rjson";
// @ts-ignore
import jsNoCompareNegZero from "./js/noCompareNegZero.test.rjson";
// @ts-ignore
import jsNoCondAssign from "./js/noCondAssign.test.rjson";
// @ts-ignore
import jsNoDebugger from "./js/noDebugger.test.rjson";
// @ts-ignore
import jsNoDelete from "./js/noDelete.test.rjson";
// @ts-ignore
import jsNoDeleteVars from "./js/noDeleteVars.test.rjson";
// @ts-ignore
import jsNoDoubleEquals from "./js/noDoubleEquals.test.rjson";
// @ts-ignore
import jsNoDupeArgs from "./js/noDupeArgs.test.rjson";
// @ts-ignore
import jsNoDuplicateCase from "./js/noDuplicateCase.test.rjson";
// @ts-ignore
import jsNoDuplicateImportSource from "./js/noDuplicateImportSource.test.rjson";
// @ts-ignore
import jsNoDuplicateKeys from "./js/noDuplicateKeys.test.rjson";
// @ts-ignore
import jsNoEmptyBlocks from "./js/noEmptyBlocks.test.rjson";
// @ts-ignore
import jsNoExtraBooleanCast from "./js/noExtraBooleanCast.test.rjson";
// @ts-ignore
import jsNoFunctionAssign from "./js/noFunctionAssign.test.rjson";
// @ts-ignore
import jsNoGetterReturn from "./js/noGetterReturn.test.rjson";
// @ts-ignore
import jsNoImportAssign from "./js/noImportAssign.test.rjson";
// @ts-ignore
import jsNoLabelVar from "./js/noLabelVar.test.rjson";
// @ts-ignore
import jsNoNegationElse from "./js/noNegationElse.test.rjson";
// @ts-ignore
import jsNoNestedTernary from "./js/noNestedTernary.test.rjson";
// @ts-ignore
import jsNoRestrictedGlobals from "./js/noRestrictedGlobals.test.rjson";
// @ts-ignore
import jsNoSetterReturn from "./js/noSetterReturn.test.rjson";
// @ts-ignore
import jsNoShadowRestrictedNames from "./js/noShadowRestrictedNames.test.rjson";
// @ts-ignore
import jsNoShoutyConstants from "./js/noShoutyConstants.test.rjson";
// @ts-ignore
import jsNoSingleCharRegexAlternatives from "./js/noSingleCharRegexAlternatives.test.rjson";
// @ts-ignore
import jsNoSparseArray from "./js/noSparseArray.test.rjson";
// @ts-ignore
import jsNoTemplateCurlyInString from "./js/noTemplateCurlyInString.test.rjson";
// @ts-ignore
import jsNoUndeclaredVariables from "./js/noUndeclaredVariables.test.rjson";
// @ts-ignore
import jsNoUnnecessaryContinue from "./js/noUnnecessaryContinue.test.rjson";
// @ts-ignore
import jsNoUnsafeFinally from "./js/noUnsafeFinally.test.rjson";
// @ts-ignore
import jsNoUnsafeNegation from "./js/noUnsafeNegation.test.rjson";
// @ts-ignore
import jsNoUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.rjson";
// @ts-ignore
import jsNoUnusedVariables from "./js/noUnusedVariables.test.rjson";
// @ts-ignore
import jsNoVar from "./js/noVar.test.rjson";
// @ts-ignore
import jsPreferOptionalChaining from "./js/preferOptionalChaining.test.rjson";
// @ts-ignore
import jsUseBlockStatements from "./js/useBlockStatements.test.rjson";
// @ts-ignore
import jsUseDefaultExportBasename from "./js/useDefaultExportBasename.test.rjson";
// @ts-ignore
import jsUseDefaultImportBasename from "./js/useDefaultImportBasename.test.rjson";
// @ts-ignore
import jsUseFunctionDeclarations from "./js/useFunctionDeclarations.test.rjson";
// @ts-ignore
import jsUseSingleCaseStatement from "./js/useSingleCaseStatement.test.rjson";
// @ts-ignore
import jsUseSingleVarDeclarator from "./js/useSingleVarDeclarator.test.rjson";
// @ts-ignore
import jsUseSortedSpecifiers from "./js/useSortedSpecifiers.test.rjson";
// @ts-ignore
import jsUseTemplate from "./js/useTemplate.test.rjson";
// @ts-ignore
import jsUseWhile from "./js/useWhile.test.rjson";
// @ts-ignore
import jsxA11YNoAccessKey from "./jsx-a11y/noAccessKey.test.rjson";
// @ts-ignore
import jsxA11YNoAriaUnsupportedElements from "./jsx-a11y/noAriaUnsupportedElements.test.rjson";
// @ts-ignore
import jsxA11YNoAutofocus from "./jsx-a11y/noAutofocus.test.rjson";
// @ts-ignore
import jsxA11YNoDistractingElements from "./jsx-a11y/noDistractingElements.test.rjson";
// @ts-ignore
import jsxA11YNoHeaderScope from "./jsx-a11y/noHeaderScope.test.rjson";
// @ts-ignore
import jsxA11YNoNoninteractiveElementToInteractiveRole from "./jsx-a11y/noNoninteractiveElementToInteractiveRole.test.rjson";
// @ts-ignore
import jsxA11YNoNoninteractiveTabindex from "./jsx-a11y/noNoninteractiveTabindex.test.rjson";
// @ts-ignore
import jsxA11YNoOnChange from "./jsx-a11y/noOnChange.test.rjson";
// @ts-ignore
import jsxA11YNoPositiveTabindex from "./jsx-a11y/noPositiveTabindex.test.rjson";
// @ts-ignore
import jsxA11YNoRedundantAlt from "./jsx-a11y/noRedundantAlt.test.rjson";
// @ts-ignore
import jsxA11YNoRedundantRoles from "./jsx-a11y/noRedundantRoles.test.rjson";
// @ts-ignore
import jsxA11YNoTargetBlank from "./jsx-a11y/noTargetBlank.test.rjson";
// @ts-ignore
import jsxA11YUseAltText from "./jsx-a11y/useAltText.test.rjson";
// @ts-ignore
import jsxA11YUseAnchorContent from "./jsx-a11y/useAnchorContent.test.rjson";
// @ts-ignore
import jsxA11YUseAriaProps from "./jsx-a11y/useAriaProps.test.rjson";
// @ts-ignore
import jsxA11YUseAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole.test.rjson";
// @ts-ignore
import jsxA11YUseAriaProptypes from "./jsx-a11y/useAriaProptypes.test.rjson";
// @ts-ignore
import jsxA11YUseHeadingContent from "./jsx-a11y/useHeadingContent.test.rjson";
// @ts-ignore
import jsxA11YUseHtmlLang from "./jsx-a11y/useHtmlLang.test.rjson";
// @ts-ignore
import jsxA11YUseIframeTitle from "./jsx-a11y/useIframeTitle.test.rjson";
// @ts-ignore
import jsxA11YUseKeyWithClickEvents from "./jsx-a11y/useKeyWithClickEvents.test.rjson";
// @ts-ignore
import jsxA11YUseKeyWithMouseEvents from "./jsx-a11y/useKeyWithMouseEvents.test.rjson";
// @ts-ignore
import jsxA11YUseMediaCaption from "./jsx-a11y/useMediaCaption.test.rjson";
// @ts-ignore
import jsxA11YUseValidAnchor from "./jsx-a11y/useValidAnchor.test.rjson";
// @ts-ignore
import jsxA11YUseValidLang from "./jsx-a11y/useValidLang.test.rjson";
// @ts-ignore
import jsxNoCommentText from "./jsx/noCommentText.test.rjson";
// @ts-ignore
import jsxNoDuplicateProps from "./jsx/noDuplicateProps.test.rjson";
// @ts-ignore
import jsxNoImplicitBoolean from "./jsx/noImplicitBoolean.test.rjson";
// @ts-ignore
import jsxNoPropSpreading from "./jsx/noPropSpreading.test.rjson";
// @ts-ignore
import jsxUseJSXFileExtension from "./jsx/useJSXFileExtension.test.rjson";
// @ts-ignore
import jsxUsePascalCase from "./jsx/usePascalCase.test.rjson";
// @ts-ignore
import jsxUseSelfClosingElements from "./jsx/useSelfClosingElements.test.rjson";
// @ts-ignore
import reactNoAccessStateInSetState from "./react/noAccessStateInSetState.test.rjson";
// @ts-ignore
import reactNoArrayIndexKey from "./react/noArrayIndexKey.test.rjson";
// @ts-ignore
import reactNoChildrenProp from "./react/noChildrenProp.test.rjson";
// @ts-ignore
import reactNoDanger from "./react/noDanger.test.rjson";
// @ts-ignore
import reactNoDangerWithChildren from "./react/noDangerWithChildren.test.rjson";
// @ts-ignore
import reactNoDidMountSetState from "./react/noDidMountSetState.test.rjson";
// @ts-ignore
import reactNoDidUpdateSetState from "./react/noDidUpdateSetState.test.rjson";
// @ts-ignore
import reactNoDirectMutationState from "./react/noDirectMutationState.test.rjson";
// @ts-ignore
import reactNoFindDOMNode from "./react/noFindDOMNode.test.rjson";
// @ts-ignore
import reactNoRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate.test.rjson";
// @ts-ignore
import reactNoRenderReturnValue from "./react/noRenderReturnValue.test.rjson";
// @ts-ignore
import reactNoStringRefs from "./react/noStringRefs.test.rjson";
// @ts-ignore
import reactNoThisInSFC from "./react/noThisInSFC.test.rjson";
// @ts-ignore
import reactNoUnsafe from "./react/noUnsafe.test.rjson";
// @ts-ignore
import reactNoUselessFragment from "./react/noUselessFragment.test.rjson";
// @ts-ignore
import reactNoVoidElementsWithChildren from "./react/noVoidElementsWithChildren.test.rjson";
// @ts-ignore
import reactNoWillUpdateSetState from "./react/noWillUpdateSetState.test.rjson";
// @ts-ignore
import reactUseButtonType from "./react/useButtonType.test.rjson";
// @ts-ignore
import reactUseFragmentSyntax from "./react/useFragmentSyntax.test.rjson";
// @ts-ignore
import reactUseKey from "./react/useKey.test.rjson";
// @ts-ignore
import reactUseRenderReturn from "./react/useRenderReturn.test.rjson";
// @ts-ignore
import reactUseSortComp from "./react/useSortComp.test.rjson";
// @ts-ignore
import reactUseStylePropObject from "./react/useStylePropObject.test.rjson";
// @ts-ignore
import regexNoDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions.test.rjson";
// @ts-ignore
import regexNoEmptyCharacterClass from "./regex/noEmptyCharacterClass.test.rjson";
// @ts-ignore
import regexNoEmptyMatches from "./regex/noEmptyMatches.test.rjson";
// @ts-ignore
import regexNoMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals.test.rjson";
// @ts-ignore
import regexNoPosixInRegularExpression from "./regex/noPosixInRegularExpression.test.rjson";
// @ts-ignore
import tsPreferShorthandArrayType from "./ts/preferShorthandArrayType.test.rjson";

export const tests: Tests = {
	"html/useClosingNonVoid": htmlUseClosingNonVoid,
	"html/useHtmlLang": htmlUseHtmlLang,
	"html/useValidLang": htmlUseValidLang,
	"js/noArguments": jsNoArguments,
	"js/noAsyncPromiseExecutor": jsNoAsyncPromiseExecutor,
	"js/noCatchAssign": jsNoCatchAssign,
	"js/noCommaOperator": jsNoCommaOperator,
	"js/noCompareNegZero": jsNoCompareNegZero,
	"js/noCondAssign": jsNoCondAssign,
	"js/noDebugger": jsNoDebugger,
	"js/noDelete": jsNoDelete,
	"js/noDeleteVars": jsNoDeleteVars,
	"js/noDoubleEquals": jsNoDoubleEquals,
	"js/noDupeArgs": jsNoDupeArgs,
	"js/noDuplicateCase": jsNoDuplicateCase,
	"js/noDuplicateImportSource": jsNoDuplicateImportSource,
	"js/noDuplicateKeys": jsNoDuplicateKeys,
	"js/noEmptyBlocks": jsNoEmptyBlocks,
	"js/noExtraBooleanCast": jsNoExtraBooleanCast,
	"js/noFunctionAssign": jsNoFunctionAssign,
	"js/noGetterReturn": jsNoGetterReturn,
	"js/noImportAssign": jsNoImportAssign,
	"js/noLabelVar": jsNoLabelVar,
	"js/noNegationElse": jsNoNegationElse,
	"js/noNestedTernary": jsNoNestedTernary,
	"js/noRestrictedGlobals": jsNoRestrictedGlobals,
	"js/noSetterReturn": jsNoSetterReturn,
	"js/noShadowRestrictedNames": jsNoShadowRestrictedNames,
	"js/noShoutyConstants": jsNoShoutyConstants,
	"js/noSingleCharRegexAlternatives": jsNoSingleCharRegexAlternatives,
	"js/noSparseArray": jsNoSparseArray,
	"js/noTemplateCurlyInString": jsNoTemplateCurlyInString,
	"js/noUndeclaredVariables": jsNoUndeclaredVariables,
	"js/noUnnecessaryContinue": jsNoUnnecessaryContinue,
	"js/noUnsafeFinally": jsNoUnsafeFinally,
	"js/noUnsafeNegation": jsNoUnsafeNegation,
	"js/noUnusedTemplateLiteral": jsNoUnusedTemplateLiteral,
	"js/noUnusedVariables": jsNoUnusedVariables,
	"js/noVar": jsNoVar,
	"js/preferOptionalChaining": jsPreferOptionalChaining,
	"js/useBlockStatements": jsUseBlockStatements,
	"js/useDefaultExportBasename": jsUseDefaultExportBasename,
	"js/useDefaultImportBasename": jsUseDefaultImportBasename,
	"js/useFunctionDeclarations": jsUseFunctionDeclarations,
	"js/useSingleCaseStatement": jsUseSingleCaseStatement,
	"js/useSingleVarDeclarator": jsUseSingleVarDeclarator,
	"js/useSortedSpecifiers": jsUseSortedSpecifiers,
	"js/useTemplate": jsUseTemplate,
	"js/useWhile": jsUseWhile,
	"jsx-a11y/noAccessKey": jsxA11YNoAccessKey,
	"jsx-a11y/noAriaUnsupportedElements": jsxA11YNoAriaUnsupportedElements,
	"jsx-a11y/noAutofocus": jsxA11YNoAutofocus,
	"jsx-a11y/noDistractingElements": jsxA11YNoDistractingElements,
	"jsx-a11y/noHeaderScope": jsxA11YNoHeaderScope,
	"jsx-a11y/noNoninteractiveElementToInteractiveRole": jsxA11YNoNoninteractiveElementToInteractiveRole,
	"jsx-a11y/noNoninteractiveTabindex": jsxA11YNoNoninteractiveTabindex,
	"jsx-a11y/noOnChange": jsxA11YNoOnChange,
	"jsx-a11y/noPositiveTabindex": jsxA11YNoPositiveTabindex,
	"jsx-a11y/noRedundantAlt": jsxA11YNoRedundantAlt,
	"jsx-a11y/noRedundantRoles": jsxA11YNoRedundantRoles,
	"jsx-a11y/noTargetBlank": jsxA11YNoTargetBlank,
	"jsx-a11y/useAltText": jsxA11YUseAltText,
	"jsx-a11y/useAnchorContent": jsxA11YUseAnchorContent,
	"jsx-a11y/useAriaProps": jsxA11YUseAriaProps,
	"jsx-a11y/useAriaPropsForRole": jsxA11YUseAriaPropsForRole,
	"jsx-a11y/useAriaProptypes": jsxA11YUseAriaProptypes,
	"jsx-a11y/useHeadingContent": jsxA11YUseHeadingContent,
	"jsx-a11y/useHtmlLang": jsxA11YUseHtmlLang,
	"jsx-a11y/useIframeTitle": jsxA11YUseIframeTitle,
	"jsx-a11y/useKeyWithClickEvents": jsxA11YUseKeyWithClickEvents,
	"jsx-a11y/useKeyWithMouseEvents": jsxA11YUseKeyWithMouseEvents,
	"jsx-a11y/useMediaCaption": jsxA11YUseMediaCaption,
	"jsx-a11y/useValidAnchor": jsxA11YUseValidAnchor,
	"jsx-a11y/useValidLang": jsxA11YUseValidLang,
	"jsx/noCommentText": jsxNoCommentText,
	"jsx/noDuplicateProps": jsxNoDuplicateProps,
	"jsx/noImplicitBoolean": jsxNoImplicitBoolean,
	"jsx/noPropSpreading": jsxNoPropSpreading,
	"jsx/useJSXFileExtension": jsxUseJSXFileExtension,
	"jsx/usePascalCase": jsxUsePascalCase,
	"jsx/useSelfClosingElements": jsxUseSelfClosingElements,
	"react/noAccessStateInSetState": reactNoAccessStateInSetState,
	"react/noArrayIndexKey": reactNoArrayIndexKey,
	"react/noChildrenProp": reactNoChildrenProp,
	"react/noDanger": reactNoDanger,
	"react/noDangerWithChildren": reactNoDangerWithChildren,
	"react/noDidMountSetState": reactNoDidMountSetState,
	"react/noDidUpdateSetState": reactNoDidUpdateSetState,
	"react/noDirectMutationState": reactNoDirectMutationState,
	"react/noFindDOMNode": reactNoFindDOMNode,
	"react/noRedundantShouldComponentUpdate": reactNoRedundantShouldComponentUpdate,
	"react/noRenderReturnValue": reactNoRenderReturnValue,
	"react/noStringRefs": reactNoStringRefs,
	"react/noThisInSFC": reactNoThisInSFC,
	"react/noUnsafe": reactNoUnsafe,
	"react/noUselessFragment": reactNoUselessFragment,
	"react/noVoidElementsWithChildren": reactNoVoidElementsWithChildren,
	"react/noWillUpdateSetState": reactNoWillUpdateSetState,
	"react/useButtonType": reactUseButtonType,
	"react/useFragmentSyntax": reactUseFragmentSyntax,
	"react/useKey": reactUseKey,
	"react/useRenderReturn": reactUseRenderReturn,
	"react/useSortComp": reactUseSortComp,
	"react/useStylePropObject": reactUseStylePropObject,
	"regex/noDuplicateGroupNamesInRegularExpressions": regexNoDuplicateGroupNamesInRegularExpressions,
	"regex/noEmptyCharacterClass": regexNoEmptyCharacterClass,
	"regex/noEmptyMatches": regexNoEmptyMatches,
	"regex/noMultipleSpacesInRegularExpressionLiterals": regexNoMultipleSpacesInRegularExpressionLiterals,
	"regex/noPosixInRegularExpression": regexNoPosixInRegularExpression,
	"ts/preferShorthandArrayType": tsPreferShorthandArrayType,
};
/* GENERATED:END(id:main) */
