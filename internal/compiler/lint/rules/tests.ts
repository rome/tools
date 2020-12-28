import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<Test | (Test[])>;

/* GENERATED:START(hash:a7cd9498d72550437da5594b936ef02cf2d1f589,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import htmlUseClosingNonVoid from "./html/useClosingNonVoid.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import htmlUseHtmlLang from "./html/useHtmlLang.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import htmlUseValidLang from "./html/useValidLang.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoArguments from "./js/noArguments.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoCatchAssign from "./js/noCatchAssign.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoCommaOperator from "./js/noCommaOperator.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoCompareNegZero from "./js/noCompareNegZero.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoCondAssign from "./js/noCondAssign.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDebugger from "./js/noDebugger.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDelete from "./js/noDelete.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDeleteVars from "./js/noDeleteVars.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDoubleEquals from "./js/noDoubleEquals.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDupeArgs from "./js/noDupeArgs.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDuplicateCase from "./js/noDuplicateCase.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDuplicateImportSource from "./js/noDuplicateImportSource.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoDuplicateKeys from "./js/noDuplicateKeys.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoEmptyBlocks from "./js/noEmptyBlocks.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoExtraBooleanCast from "./js/noExtraBooleanCast.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoFunctionAssign from "./js/noFunctionAssign.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoGetterReturn from "./js/noGetterReturn.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoImportAssign from "./js/noImportAssign.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoLabelVar from "./js/noLabelVar.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoNegationElse from "./js/noNegationElse.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoNestedTernary from "./js/noNestedTernary.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoRestrictedGlobals from "./js/noRestrictedGlobals.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoSetterReturn from "./js/noSetterReturn.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoShadowRestrictedNames from "./js/noShadowRestrictedNames.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoShoutyConstants from "./js/noShoutyConstants.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoSingleCharRegexAlternatives from "./js/noSingleCharRegexAlternatives.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoSparseArray from "./js/noSparseArray.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoTemplateCurlyInString from "./js/noTemplateCurlyInString.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUndeclaredVariables from "./js/noUndeclaredVariables.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUnnecessaryContinue from "./js/noUnnecessaryContinue.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUnsafeFinally from "./js/noUnsafeFinally.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUnsafeNegation from "./js/noUnsafeNegation.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoUnusedVariables from "./js/noUnusedVariables.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsNoVar from "./js/noVar.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsPreferOptionalChaining from "./js/preferOptionalChaining.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseBlockStatements from "./js/useBlockStatements.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseDefaultExportBasename from "./js/useDefaultExportBasename.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseDefaultImportBasename from "./js/useDefaultImportBasename.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseFunctionDeclarations from "./js/useFunctionDeclarations.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseSingleCaseStatement from "./js/useSingleCaseStatement.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseSingleVarDeclarator from "./js/useSingleVarDeclarator.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseSortedSpecifiers from "./js/useSortedSpecifiers.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseTemplate from "./js/useTemplate.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsUseWhile from "./js/useWhile.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoAccessKey from "./jsx-a11y/noAccessKey.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoAriaUnsupportedElements from "./jsx-a11y/noAriaUnsupportedElements.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoAutofocus from "./jsx-a11y/noAutofocus.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoDistractingElements from "./jsx-a11y/noDistractingElements.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoHeaderScope from "./jsx-a11y/noHeaderScope.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoNoninteractiveElementToInteractiveRole from "./jsx-a11y/noNoninteractiveElementToInteractiveRole.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoNoninteractiveTabindex from "./jsx-a11y/noNoninteractiveTabindex.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoOnChange from "./jsx-a11y/noOnChange.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoPositiveTabindex from "./jsx-a11y/noPositiveTabindex.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoRedundantAlt from "./jsx-a11y/noRedundantAlt.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoRedundantRoles from "./jsx-a11y/noRedundantRoles.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YNoTargetBlank from "./jsx-a11y/noTargetBlank.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseAltText from "./jsx-a11y/useAltText.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseAnchorContent from "./jsx-a11y/useAnchorContent.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseAriaProps from "./jsx-a11y/useAriaProps.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseAriaProptypes from "./jsx-a11y/useAriaProptypes.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseHeadingContent from "./jsx-a11y/useHeadingContent.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseHtmlLang from "./jsx-a11y/useHtmlLang.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseIframeTitle from "./jsx-a11y/useIframeTitle.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseKeyWithClickEvents from "./jsx-a11y/useKeyWithClickEvents.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseKeyWithMouseEvents from "./jsx-a11y/useKeyWithMouseEvents.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseMediaCaption from "./jsx-a11y/useMediaCaption.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseValidAnchor from "./jsx-a11y/useValidAnchor.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxA11YUseValidLang from "./jsx-a11y/useValidLang.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxNoCommentText from "./jsx/noCommentText.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxNoDuplicateProps from "./jsx/noDuplicateProps.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxNoImplicitBoolean from "./jsx/noImplicitBoolean.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxNoPropSpreading from "./jsx/noPropSpreading.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxUseJSXFileExtension from "./jsx/useJSXFileExtension.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxUsePascalCase from "./jsx/usePascalCase.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import jsxUseSelfClosingElements from "./jsx/useSelfClosingElements.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoAccessStateInSetState from "./react/noAccessStateInSetState.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoArrayIndexKey from "./react/noArrayIndexKey.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoChildrenProp from "./react/noChildrenProp.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoDanger from "./react/noDanger.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoDangerWithChildren from "./react/noDangerWithChildren.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoDidMountSetState from "./react/noDidMountSetState.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoDidUpdateSetState from "./react/noDidUpdateSetState.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoDirectMutationState from "./react/noDirectMutationState.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoFindDOMNode from "./react/noFindDOMNode.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoRenderReturnValue from "./react/noRenderReturnValue.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoStringRefs from "./react/noStringRefs.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoThisInSFC from "./react/noThisInSFC.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoUnsafe from "./react/noUnsafe.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoUselessFragment from "./react/noUselessFragment.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoVoidElementsWithChildren from "./react/noVoidElementsWithChildren.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactNoWillUpdateSetState from "./react/noWillUpdateSetState.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseButtonType from "./react/useButtonType.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseFragmentSyntax from "./react/useFragmentSyntax.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseKey from "./react/useKey.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseRenderReturn from "./react/useRenderReturn.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseSortComp from "./react/useSortComp.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import reactUseStylePropObject from "./react/useStylePropObject.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import regexNoDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import regexNoEmptyCharacterClass from "./regex/noEmptyCharacterClass.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import regexNoEmptyMatches from "./regex/noEmptyMatches.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import regexNoMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
import regexNoPosixInRegularExpression from "./regex/noPosixInRegularExpression.test.rjson";
// @ts-ignore
// rome-ignore lint/js/useDefaultImportBasename: avoid clashing
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
