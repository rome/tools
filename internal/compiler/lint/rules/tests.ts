import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: Array<string>;
	valid?: Array<string>;
	filename: string;
};

type Tests = Dict<Test | Array<Test>>;

/* GENERATED:START(hash:91ad1102a06130294ccebd4b443466e7ece68e55,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-ignore
import useClosingNonVoid from "./html/useClosingNonVoid.test.rjson";
// @ts-ignore
import noArguments from "./js/noArguments.test.rjson";
// @ts-ignore
import noAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor.test.rjson";
// @ts-ignore
import noCatchAssign from "./js/noCatchAssign.test.rjson";
// @ts-ignore
import noCommaOperator from "./js/noCommaOperator.test.rjson";
// @ts-ignore
import noCompareNegZero from "./js/noCompareNegZero.test.rjson";
// @ts-ignore
import noCondAssign from "./js/noCondAssign.test.rjson";
// @ts-ignore
import noDebugger from "./js/noDebugger.test.rjson";
// @ts-ignore
import noDelete from "./js/noDelete.test.rjson";
// @ts-ignore
import noDeleteVars from "./js/noDeleteVars.test.rjson";
// @ts-ignore
import noDoubleEquals from "./js/noDoubleEquals.test.rjson";
// @ts-ignore
import noDupeArgs from "./js/noDupeArgs.test.rjson";
// @ts-ignore
import noDuplicateCase from "./js/noDuplicateCase.test.rjson";
// @ts-ignore
import noDuplicateImportSource from "./js/noDuplicateImportSource.test.rjson";
// @ts-ignore
import noDuplicateKeys from "./js/noDuplicateKeys.test.rjson";
// @ts-ignore
import noEmptyBlocks from "./js/noEmptyBlocks.test.rjson";
// @ts-ignore
import noExtraBooleanCast from "./js/noExtraBooleanCast.test.rjson";
// @ts-ignore
import noFunctionAssign from "./js/noFunctionAssign.test.rjson";
// @ts-ignore
import noGetterReturn from "./js/noGetterReturn.test.rjson";
// @ts-ignore
import noImportAssign from "./js/noImportAssign.test.rjson";
// @ts-ignore
import noLabelVar from "./js/noLabelVar.test.rjson";
// @ts-ignore
import noNegationElse from "./js/noNegationElse.test.rjson";
// @ts-ignore
import noNestedTernary from "./js/noNestedTernary.test.rjson";
// @ts-ignore
import noRestrictedGlobals from "./js/noRestrictedGlobals.test.rjson";
// @ts-ignore
import noSetterReturn from "./js/noSetterReturn.test.rjson";
// @ts-ignore
import noShadowRestrictedNames from "./js/noShadowRestrictedNames.test.rjson";
// @ts-ignore
import noShorthandArrayType from "./js/noShorthandArrayType.test.rjson";
// @ts-ignore
import noShoutyConstants from "./js/noShoutyConstants.test.rjson";
// @ts-ignore
import noSparseArray from "./js/noSparseArray.test.rjson";
// @ts-ignore
import noTemplateCurlyInString from "./js/noTemplateCurlyInString.test.rjson";
// @ts-ignore
import noUndeclaredVariables from "./js/noUndeclaredVariables.test.rjson";
// @ts-ignore
import noUnsafeFinally from "./js/noUnsafeFinally.test.rjson";
// @ts-ignore
import noUnsafeNegation from "./js/noUnsafeNegation.test.rjson";
// @ts-ignore
import noUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.rjson";
// @ts-ignore
import noUnusedVariables from "./js/noUnusedVariables.test.rjson";
// @ts-ignore
import noVar from "./js/noVar.test.rjson";
// @ts-ignore
import useBlockStatements from "./js/useBlockStatements.test.rjson";
// @ts-ignore
import useDefaultExportBasename from "./js/useDefaultExportBasename.test.rjson";
// @ts-ignore
import useDefaultImportBasename from "./js/useDefaultImportBasename.test.rjson";
// @ts-ignore
import useFunctionDeclarations from "./js/useFunctionDeclarations.test.rjson";
// @ts-ignore
import useSingleCaseStatement from "./js/useSingleCaseStatement.test.rjson";
// @ts-ignore
import useSingleVarDeclarator from "./js/useSingleVarDeclarator.test.rjson";
// @ts-ignore
import useSortedSpecifiers from "./js/useSortedSpecifiers.test.rjson";
// @ts-ignore
import useTemplate from "./js/useTemplate.test.rjson";
// @ts-ignore
import useWhile from "./js/useWhile.test.rjson";
// @ts-ignore
import noAccessKey from "./jsx-a11y/noAccessKey.test.rjson";
// @ts-ignore
import noAriaUnsupportedElements from "./jsx-a11y/noAriaUnsupportedElements.test.rjson";
// @ts-ignore
import noAutofocus from "./jsx-a11y/noAutofocus.test.rjson";
// @ts-ignore
import noDistractingElements from "./jsx-a11y/noDistractingElements.test.rjson";
// @ts-ignore
import noHeaderScope from "./jsx-a11y/noHeaderScope.test.rjson";
// @ts-ignore
import noNoninteractiveElementToInteractiveRole from "./jsx-a11y/noNoninteractiveElementToInteractiveRole.test.rjson";
// @ts-ignore
import noNoninteractiveTabindex from "./jsx-a11y/noNoninteractiveTabindex.test.rjson";
// @ts-ignore
import noOnChange from "./jsx-a11y/noOnChange.test.rjson";
// @ts-ignore
import noPositiveTabindex from "./jsx-a11y/noPositiveTabindex.test.rjson";
// @ts-ignore
import noRedundantAlt from "./jsx-a11y/noRedundantAlt.test.rjson";
// @ts-ignore
import noRedundantRoles from "./jsx-a11y/noRedundantRoles.test.rjson";
// @ts-ignore
import noTargetBlank from "./jsx-a11y/noTargetBlank.test.rjson";
// @ts-ignore
import useAltText from "./jsx-a11y/useAltText.test.rjson";
// @ts-ignore
import useAnchorContent from "./jsx-a11y/useAnchorContent.test.rjson";
// @ts-ignore
import useAriaProps from "./jsx-a11y/useAriaProps.test.rjson";
// @ts-ignore
import useAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole.test.rjson";
// @ts-ignore
import useAriaProptypes from "./jsx-a11y/useAriaProptypes.test.rjson";
// @ts-ignore
import useHeadingContent from "./jsx-a11y/useHeadingContent.test.rjson";
// @ts-ignore
import useHtmlLang from "./jsx-a11y/useHtmlLang.test.rjson";
// @ts-ignore
import useIframeTitle from "./jsx-a11y/useIframeTitle.test.rjson";
// @ts-ignore
import useKeyWithClickEvents from "./jsx-a11y/useKeyWithClickEvents.test.rjson";
// @ts-ignore
import useKeyWithMouseEvents from "./jsx-a11y/useKeyWithMouseEvents.test.rjson";
// @ts-ignore
import useMediaCaption from "./jsx-a11y/useMediaCaption.test.rjson";
// @ts-ignore
import useValidAnchor from "./jsx-a11y/useValidAnchor.test.rjson";
// @ts-ignore
import useValidLang from "./jsx-a11y/useValidLang.test.rjson";
// @ts-ignore
import noCommentText from "./jsx/noCommentText.test.rjson";
// @ts-ignore
import noDuplicateProps from "./jsx/noDuplicateProps.test.rjson";
// @ts-ignore
import noImplicitBoolean from "./jsx/noImplicitBoolean.test.rjson";
// @ts-ignore
import noPropSpreading from "./jsx/noPropSpreading.test.rjson";
// @ts-ignore
import useJSXFileExtension from "./jsx/useJSXFileExtension.test.rjson";
// @ts-ignore
import usePascalCase from "./jsx/usePascalCase.test.rjson";
// @ts-ignore
import useSelfClosingElements from "./jsx/useSelfClosingElements.test.rjson";
// @ts-ignore
import noAccessStateInSetState from "./react/noAccessStateInSetState.test.rjson";
// @ts-ignore
import noArrayIndexKey from "./react/noArrayIndexKey.test.rjson";
// @ts-ignore
import noChildrenProp from "./react/noChildrenProp.test.rjson";
// @ts-ignore
import noDanger from "./react/noDanger.test.rjson";
// @ts-ignore
import noDangerWithChildren from "./react/noDangerWithChildren.test.rjson";
// @ts-ignore
import noDidMountSetState from "./react/noDidMountSetState.test.rjson";
// @ts-ignore
import noDidUpdateSetState from "./react/noDidUpdateSetState.test.rjson";
// @ts-ignore
import noDirectMutationState from "./react/noDirectMutationState.test.rjson";
// @ts-ignore
import noFindDOMNode from "./react/noFindDOMNode.test.rjson";
// @ts-ignore
import noRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate.test.rjson";
// @ts-ignore
import noRenderReturnValue from "./react/noRenderReturnValue.test.rjson";
// @ts-ignore
import noStringRefs from "./react/noStringRefs.test.rjson";
// @ts-ignore
import noThisInSFC from "./react/noThisInSFC.test.rjson";
// @ts-ignore
import noUnsafe from "./react/noUnsafe.test.rjson";
// @ts-ignore
import noUselessFragment from "./react/noUselessFragment.test.rjson";
// @ts-ignore
import noVoidElementsWithChildren from "./react/noVoidElementsWithChildren.test.rjson";
// @ts-ignore
import noWillUpdateSetState from "./react/noWillUpdateSetState.test.rjson";
// @ts-ignore
import useButtonType from "./react/useButtonType.test.rjson";
// @ts-ignore
import useFragmentSyntax from "./react/useFragmentSyntax.test.rjson";
// @ts-ignore
import useKey from "./react/useKey.test.rjson";
// @ts-ignore
import useRenderReturn from "./react/useRenderReturn.test.rjson";
// @ts-ignore
import useSortComp from "./react/useSortComp.test.rjson";
// @ts-ignore
import useStylePropObject from "./react/useStylePropObject.test.rjson";
// @ts-ignore
import noDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions.test.rjson";
// @ts-ignore
import noEmptyCharacterClass from "./regex/noEmptyCharacterClass.test.rjson";
// @ts-ignore
import noEmptyMatches from "./regex/noEmptyMatches.test.rjson";
// @ts-ignore
import noMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals.test.rjson";
// @ts-ignore
import noPosixInRegularExpression from "./regex/noPosixInRegularExpression.test.rjson";

export const tests: Tests = {
	"html/useClosingNonVoid": useClosingNonVoid,
	"js/noArguments": noArguments,
	"js/noAsyncPromiseExecutor": noAsyncPromiseExecutor,
	"js/noCatchAssign": noCatchAssign,
	"js/noCommaOperator": noCommaOperator,
	"js/noCompareNegZero": noCompareNegZero,
	"js/noCondAssign": noCondAssign,
	"js/noDebugger": noDebugger,
	"js/noDelete": noDelete,
	"js/noDeleteVars": noDeleteVars,
	"js/noDoubleEquals": noDoubleEquals,
	"js/noDupeArgs": noDupeArgs,
	"js/noDuplicateCase": noDuplicateCase,
	"js/noDuplicateImportSource": noDuplicateImportSource,
	"js/noDuplicateKeys": noDuplicateKeys,
	"js/noEmptyBlocks": noEmptyBlocks,
	"js/noExtraBooleanCast": noExtraBooleanCast,
	"js/noFunctionAssign": noFunctionAssign,
	"js/noGetterReturn": noGetterReturn,
	"js/noImportAssign": noImportAssign,
	"js/noLabelVar": noLabelVar,
	"js/noNegationElse": noNegationElse,
	"js/noNestedTernary": noNestedTernary,
	"js/noRestrictedGlobals": noRestrictedGlobals,
	"js/noSetterReturn": noSetterReturn,
	"js/noShadowRestrictedNames": noShadowRestrictedNames,
	"js/noShorthandArrayType": noShorthandArrayType,
	"js/noShoutyConstants": noShoutyConstants,
	"js/noSparseArray": noSparseArray,
	"js/noTemplateCurlyInString": noTemplateCurlyInString,
	"js/noUndeclaredVariables": noUndeclaredVariables,
	"js/noUnsafeFinally": noUnsafeFinally,
	"js/noUnsafeNegation": noUnsafeNegation,
	"js/noUnusedTemplateLiteral": noUnusedTemplateLiteral,
	"js/noUnusedVariables": noUnusedVariables,
	"js/noVar": noVar,
	"js/useBlockStatements": useBlockStatements,
	"js/useDefaultExportBasename": useDefaultExportBasename,
	"js/useDefaultImportBasename": useDefaultImportBasename,
	"js/useFunctionDeclarations": useFunctionDeclarations,
	"js/useSingleCaseStatement": useSingleCaseStatement,
	"js/useSingleVarDeclarator": useSingleVarDeclarator,
	"js/useSortedSpecifiers": useSortedSpecifiers,
	"js/useTemplate": useTemplate,
	"js/useWhile": useWhile,
	"jsx-a11y/noAccessKey": noAccessKey,
	"jsx-a11y/noAriaUnsupportedElements": noAriaUnsupportedElements,
	"jsx-a11y/noAutofocus": noAutofocus,
	"jsx-a11y/noDistractingElements": noDistractingElements,
	"jsx-a11y/noHeaderScope": noHeaderScope,
	"jsx-a11y/noNoninteractiveElementToInteractiveRole": noNoninteractiveElementToInteractiveRole,
	"jsx-a11y/noNoninteractiveTabindex": noNoninteractiveTabindex,
	"jsx-a11y/noOnChange": noOnChange,
	"jsx-a11y/noPositiveTabindex": noPositiveTabindex,
	"jsx-a11y/noRedundantAlt": noRedundantAlt,
	"jsx-a11y/noRedundantRoles": noRedundantRoles,
	"jsx-a11y/noTargetBlank": noTargetBlank,
	"jsx-a11y/useAltText": useAltText,
	"jsx-a11y/useAnchorContent": useAnchorContent,
	"jsx-a11y/useAriaProps": useAriaProps,
	"jsx-a11y/useAriaPropsForRole": useAriaPropsForRole,
	"jsx-a11y/useAriaProptypes": useAriaProptypes,
	"jsx-a11y/useHeadingContent": useHeadingContent,
	"jsx-a11y/useHtmlLang": useHtmlLang,
	"jsx-a11y/useIframeTitle": useIframeTitle,
	"jsx-a11y/useKeyWithClickEvents": useKeyWithClickEvents,
	"jsx-a11y/useKeyWithMouseEvents": useKeyWithMouseEvents,
	"jsx-a11y/useMediaCaption": useMediaCaption,
	"jsx-a11y/useValidAnchor": useValidAnchor,
	"jsx-a11y/useValidLang": useValidLang,
	"jsx/noCommentText": noCommentText,
	"jsx/noDuplicateProps": noDuplicateProps,
	"jsx/noImplicitBoolean": noImplicitBoolean,
	"jsx/noPropSpreading": noPropSpreading,
	"jsx/useJSXFileExtension": useJSXFileExtension,
	"jsx/usePascalCase": usePascalCase,
	"jsx/useSelfClosingElements": useSelfClosingElements,
	"react/noAccessStateInSetState": noAccessStateInSetState,
	"react/noArrayIndexKey": noArrayIndexKey,
	"react/noChildrenProp": noChildrenProp,
	"react/noDanger": noDanger,
	"react/noDangerWithChildren": noDangerWithChildren,
	"react/noDidMountSetState": noDidMountSetState,
	"react/noDidUpdateSetState": noDidUpdateSetState,
	"react/noDirectMutationState": noDirectMutationState,
	"react/noFindDOMNode": noFindDOMNode,
	"react/noRedundantShouldComponentUpdate": noRedundantShouldComponentUpdate,
	"react/noRenderReturnValue": noRenderReturnValue,
	"react/noStringRefs": noStringRefs,
	"react/noThisInSFC": noThisInSFC,
	"react/noUnsafe": noUnsafe,
	"react/noUselessFragment": noUselessFragment,
	"react/noVoidElementsWithChildren": noVoidElementsWithChildren,
	"react/noWillUpdateSetState": noWillUpdateSetState,
	"react/useButtonType": useButtonType,
	"react/useFragmentSyntax": useFragmentSyntax,
	"react/useKey": useKey,
	"react/useRenderReturn": useRenderReturn,
	"react/useSortComp": useSortComp,
	"react/useStylePropObject": useStylePropObject,
	"regex/noDuplicateGroupNamesInRegularExpressions": noDuplicateGroupNamesInRegularExpressions,
	"regex/noEmptyCharacterClass": noEmptyCharacterClass,
	"regex/noEmptyMatches": noEmptyMatches,
	"regex/noMultipleSpacesInRegularExpressionLiterals": noMultipleSpacesInRegularExpressionLiterals,
	"regex/noPosixInRegularExpression": noPosixInRegularExpression,
};
/* GENERATED:END(id:main) */
