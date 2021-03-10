import {DiagnosticCategory} from "@internal/diagnostics";
import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<{
	category: DiagnosticCategory;
	cases: Test | (Test[]) | (Test[][]);
}>;

/* GENERATED:START(hash:7c338129a77fa5641ec09fc8d7de4672265d412c,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-ignore
import noAriaUnsupportedElements from "./a11y/noAriaUnsupportedElements.test.toml";
// @ts-ignore
import noDistractingElements from "./a11y/noDistractingElements.test.toml";
// @ts-ignore
import noNoninteractiveElementToInteractiveRole from "./a11y/noNoninteractiveElementToInteractiveRole.test.toml";
// @ts-ignore
import noNoninteractiveTabindex from "./a11y/noNoninteractiveTabindex.test.toml";
// @ts-ignore
import noSvgWithoutTitle from "./a11y/noSvgWithoutTitle.test.toml";
// @ts-ignore
import useAltText from "./a11y/useAltText.test.toml";
// @ts-ignore
import useAriaProptypes from "./a11y/useAriaProptypes.test.toml";
// @ts-ignore
import useHtmlLang from "./a11y/useHtmlLang.test.toml";
// @ts-ignore
import useIframeTitle from "./a11y/useIframeTitle.test.toml";
// @ts-ignore
import useMediaCaption from "./a11y/useMediaCaption.test.toml";
// @ts-ignore
import useValidLang from "./a11y/useValidLang.test.toml";
// @ts-ignore
import useClosingNonVoid from "./html/useClosingNonVoid.test.toml";
// @ts-ignore
import noArguments from "./js/noArguments.test.toml";
// @ts-ignore
import noAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor.test.toml";
// @ts-ignore
import noCatchAssign from "./js/noCatchAssign.test.toml";
// @ts-ignore
import noCommaOperator from "./js/noCommaOperator.test.toml";
// @ts-ignore
import noCompareNegZero from "./js/noCompareNegZero.test.toml";
// @ts-ignore
import noCondAssign from "./js/noCondAssign.test.toml";
// @ts-ignore
import noDebugger from "./js/noDebugger.test.toml";
// @ts-ignore
import noDelete from "./js/noDelete.test.toml";
// @ts-ignore
import noDeleteVars from "./js/noDeleteVars.test.toml";
// @ts-ignore
import noDoubleEquals from "./js/noDoubleEquals.test.toml";
// @ts-ignore
import noDupeArgs from "./js/noDupeArgs.test.toml";
// @ts-ignore
import noDuplicateCase from "./js/noDuplicateCase.test.toml";
// @ts-ignore
import noDuplicateImportSource from "./js/noDuplicateImportSource.test.toml";
// @ts-ignore
import noDuplicateKeys from "./js/noDuplicateKeys.test.toml";
// @ts-ignore
import noEmptyBlocks from "./js/noEmptyBlocks.test.toml";
// @ts-ignore
import noExtraBooleanCast from "./js/noExtraBooleanCast.test.toml";
// @ts-ignore
import noFunctionAssign from "./js/noFunctionAssign.test.toml";
// @ts-ignore
import noGetterReturn from "./js/noGetterReturn.test.toml";
// @ts-ignore
import noImportAssign from "./js/noImportAssign.test.toml";
// @ts-ignore
import noLabelVar from "./js/noLabelVar.test.toml";
// @ts-ignore
import noNegationElse from "./js/noNegationElse.test.toml";
// @ts-ignore
import noNestedTernary from "./js/noNestedTernary.test.toml";
// @ts-ignore
import noRestrictedGlobals from "./js/noRestrictedGlobals.test.toml";
// @ts-ignore
import noSetterReturn from "./js/noSetterReturn.test.toml";
// @ts-ignore
import noShadowRestrictedNames from "./js/noShadowRestrictedNames.test.toml";
// @ts-ignore
import noShoutyConstants from "./js/noShoutyConstants.test.toml";
// @ts-ignore
import noSingleCharRegexAlternatives from "./js/noSingleCharRegexAlternatives.test.toml";
// @ts-ignore
import noSparseArray from "./js/noSparseArray.test.toml";
// @ts-ignore
import noTemplateCurlyInString from "./js/noTemplateCurlyInString.test.toml";
// @ts-ignore
import noUndeclaredVariables from "./js/noUndeclaredVariables.test.toml";
// @ts-ignore
import noUnnecessaryContinue from "./js/noUnnecessaryContinue.test.toml";
// @ts-ignore
import noUnsafeFinally from "./js/noUnsafeFinally.test.toml";
// @ts-ignore
import noUnsafeNegation from "./js/noUnsafeNegation.test.toml";
// @ts-ignore
import noUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.toml";
// @ts-ignore
import noUnusedVariables from "./js/noUnusedVariables.test.toml";
// @ts-ignore
import noVar from "./js/noVar.test.toml";
// @ts-ignore
import preferOptionalChaining from "./js/preferOptionalChaining.test.toml";
// @ts-ignore
import useBlockStatements from "./js/useBlockStatements.test.toml";
// @ts-ignore
import useDefaultExportBasename from "./js/useDefaultExportBasename.test.toml";
// @ts-ignore
import useDefaultImportBasename from "./js/useDefaultImportBasename.test.toml";
// @ts-ignore
import useFunctionDeclarations from "./js/useFunctionDeclarations.test.toml";
// @ts-ignore
import useSimplifiedLogicalExpression from "./js/useSimplifiedLogicalExpression.test.toml";
// @ts-ignore
import useSingleCaseStatement from "./js/useSingleCaseStatement.test.toml";
// @ts-ignore
import useSingleVarDeclarator from "./js/useSingleVarDeclarator.test.toml";
// @ts-ignore
import useSortedSpecifiers from "./js/useSortedSpecifiers.test.toml";
// @ts-ignore
import useTemplate from "./js/useTemplate.test.toml";
// @ts-ignore
import useWhile from "./js/useWhile.test.toml";
// @ts-ignore
import noAccessKey from "./jsx-a11y/noAccessKey.test.toml";
// @ts-ignore
import noAutofocus from "./jsx-a11y/noAutofocus.test.toml";
// @ts-ignore
import noHeaderScope from "./jsx-a11y/noHeaderScope.test.toml";
// @ts-ignore
import noOnChange from "./jsx-a11y/noOnChange.test.toml";
// @ts-ignore
import noPositiveTabindex from "./jsx-a11y/noPositiveTabindex.test.toml";
// @ts-ignore
import noRedundantAlt from "./jsx-a11y/noRedundantAlt.test.toml";
// @ts-ignore
import noRedundantRoles from "./jsx-a11y/noRedundantRoles.test.toml";
// @ts-ignore
import noTargetBlank from "./jsx-a11y/noTargetBlank.test.toml";
// @ts-ignore
import useAnchorContent from "./jsx-a11y/useAnchorContent.test.toml";
// @ts-ignore
import useAriaProps from "./jsx-a11y/useAriaProps.test.toml";
// @ts-ignore
import useAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole.test.toml";
// @ts-ignore
import useHeadingContent from "./jsx-a11y/useHeadingContent.test.toml";
// @ts-ignore
import useKeyWithClickEvents from "./jsx-a11y/useKeyWithClickEvents.test.toml";
// @ts-ignore
import useKeyWithMouseEvents from "./jsx-a11y/useKeyWithMouseEvents.test.toml";
// @ts-ignore
import useValidAnchor from "./jsx-a11y/useValidAnchor.test.toml";
// @ts-ignore
import noCommentText from "./jsx/noCommentText.test.toml";
// @ts-ignore
import noDuplicateProps from "./jsx/noDuplicateProps.test.toml";
// @ts-ignore
import noImplicitBoolean from "./jsx/noImplicitBoolean.test.toml";
// @ts-ignore
import noPropSpreading from "./jsx/noPropSpreading.test.toml";
// @ts-ignore
import useJSXFileExtension from "./jsx/useJSXFileExtension.test.toml";
// @ts-ignore
import usePascalCase from "./jsx/usePascalCase.test.toml";
// @ts-ignore
import useSelfClosingElements from "./jsx/useSelfClosingElements.test.toml";
// @ts-ignore
import noAccessStateInSetState from "./react/noAccessStateInSetState.test.toml";
// @ts-ignore
import noArrayIndexKey from "./react/noArrayIndexKey.test.toml";
// @ts-ignore
import noChildrenProp from "./react/noChildrenProp.test.toml";
// @ts-ignore
import noDanger from "./react/noDanger.test.toml";
// @ts-ignore
import noDangerWithChildren from "./react/noDangerWithChildren.test.toml";
// @ts-ignore
import noDidMountSetState from "./react/noDidMountSetState.test.toml";
// @ts-ignore
import noDidUpdateSetState from "./react/noDidUpdateSetState.test.toml";
// @ts-ignore
import noDirectMutationState from "./react/noDirectMutationState.test.toml";
// @ts-ignore
import noFindDOMNode from "./react/noFindDOMNode.test.toml";
// @ts-ignore
import noRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate.test.toml";
// @ts-ignore
import noRenderReturnValue from "./react/noRenderReturnValue.test.toml";
// @ts-ignore
import noStringRefs from "./react/noStringRefs.test.toml";
// @ts-ignore
import noThisInSFC from "./react/noThisInSFC.test.toml";
// @ts-ignore
import noUnsafe from "./react/noUnsafe.test.toml";
// @ts-ignore
import noUselessFragment from "./react/noUselessFragment.test.toml";
// @ts-ignore
import noVoidElementsWithChildren from "./react/noVoidElementsWithChildren.test.toml";
// @ts-ignore
import noWillUpdateSetState from "./react/noWillUpdateSetState.test.toml";
// @ts-ignore
import useButtonType from "./react/useButtonType.test.toml";
// @ts-ignore
import useFragmentSyntax from "./react/useFragmentSyntax.test.toml";
// @ts-ignore
import useKey from "./react/useKey.test.toml";
// @ts-ignore
import useRenderReturn from "./react/useRenderReturn.test.toml";
// @ts-ignore
import useSortComp from "./react/useSortComp.test.toml";
// @ts-ignore
import useStylePropObject from "./react/useStylePropObject.test.toml";
// @ts-ignore
import noDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions.test.toml";
// @ts-ignore
import noEmptyCharacterClass from "./regex/noEmptyCharacterClass.test.toml";
// @ts-ignore
import noEmptyMatches from "./regex/noEmptyMatches.test.toml";
// @ts-ignore
import noMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals.test.toml";
// @ts-ignore
import noPosixInRegularExpression from "./regex/noPosixInRegularExpression.test.toml";
// @ts-ignore
import preferShorthandArrayType from "./ts/preferShorthandArrayType.test.toml";

export const tests: Tests = {
	"a11y/noAriaUnsupportedElements": {
		category: ["lint", "a11y", "noAriaUnsupportedElements"],
		cases: noAriaUnsupportedElements,
	},
	"a11y/noDistractingElements": {
		category: ["lint", "a11y", "noDistractingElements"],
		cases: noDistractingElements,
	},
	"a11y/noNoninteractiveElementToInteractiveRole": {
		category: ["lint", "a11y", "noNoninteractiveElementToInteractiveRole"],
		cases: noNoninteractiveElementToInteractiveRole,
	},
	"a11y/noNoninteractiveTabindex": {
		category: ["lint", "a11y", "noNoninteractiveTabindex"],
		cases: noNoninteractiveTabindex,
	},
	"a11y/noSvgWithoutTitle": {
		category: ["lint", "a11y", "noSvgWithoutTitle"],
		cases: noSvgWithoutTitle,
	},
	"a11y/useAltText": {
		category: ["lint", "a11y", "useAltText"],
		cases: useAltText,
	},
	"a11y/useAriaProptypes": {
		category: ["lint", "a11y", "useAriaProptypes"],
		cases: useAriaProptypes,
	},
	"a11y/useHtmlLang": {
		category: ["lint", "a11y", "useHtmlLang"],
		cases: useHtmlLang,
	},
	"a11y/useIframeTitle": {
		category: ["lint", "a11y", "useIframeTitle"],
		cases: useIframeTitle,
	},
	"a11y/useMediaCaption": {
		category: ["lint", "a11y", "useMediaCaption"],
		cases: useMediaCaption,
	},
	"a11y/useValidLang": {
		category: ["lint", "a11y", "useValidLang"],
		cases: useValidLang,
	},
	"html/useClosingNonVoid": {
		category: ["lint", "html", "useClosingNonVoid"],
		cases: useClosingNonVoid,
	},
	"js/noArguments": {
		category: ["lint", "js", "noArguments"],
		cases: noArguments,
	},
	"js/noAsyncPromiseExecutor": {
		category: ["lint", "js", "noAsyncPromiseExecutor"],
		cases: noAsyncPromiseExecutor,
	},
	"js/noCatchAssign": {
		category: ["lint", "js", "noCatchAssign"],
		cases: noCatchAssign,
	},
	"js/noCommaOperator": {
		category: ["lint", "js", "noCommaOperator"],
		cases: noCommaOperator,
	},
	"js/noCompareNegZero": {
		category: ["lint", "js", "noCompareNegZero"],
		cases: noCompareNegZero,
	},
	"js/noCondAssign": {
		category: ["lint", "js", "noCondAssign"],
		cases: noCondAssign,
	},
	"js/noDebugger": {
		category: ["lint", "js", "noDebugger"],
		cases: noDebugger,
	},
	"js/noDelete": {
		category: ["lint", "js", "noDelete"],
		cases: noDelete,
	},
	"js/noDeleteVars": {
		category: ["lint", "js", "noDeleteVars"],
		cases: noDeleteVars,
	},
	"js/noDoubleEquals": {
		category: ["lint", "js", "noDoubleEquals"],
		cases: noDoubleEquals,
	},
	"js/noDupeArgs": {
		category: ["lint", "js", "noDupeArgs"],
		cases: noDupeArgs,
	},
	"js/noDuplicateCase": {
		category: ["lint", "js", "noDuplicateCase"],
		cases: noDuplicateCase,
	},
	"js/noDuplicateImportSource": {
		category: ["lint", "js", "noDuplicateImportSource"],
		cases: noDuplicateImportSource,
	},
	"js/noDuplicateKeys": {
		category: ["lint", "js", "noDuplicateKeys"],
		cases: noDuplicateKeys,
	},
	"js/noEmptyBlocks": {
		category: ["lint", "js", "noEmptyBlocks"],
		cases: noEmptyBlocks,
	},
	"js/noExtraBooleanCast": {
		category: ["lint", "js", "noExtraBooleanCast"],
		cases: noExtraBooleanCast,
	},
	"js/noFunctionAssign": {
		category: ["lint", "js", "noFunctionAssign"],
		cases: noFunctionAssign,
	},
	"js/noGetterReturn": {
		category: ["lint", "js", "noGetterReturn"],
		cases: noGetterReturn,
	},
	"js/noImportAssign": {
		category: ["lint", "js", "noImportAssign"],
		cases: noImportAssign,
	},
	"js/noLabelVar": {
		category: ["lint", "js", "noLabelVar"],
		cases: noLabelVar,
	},
	"js/noNegationElse": {
		category: ["lint", "js", "noNegationElse"],
		cases: noNegationElse,
	},
	"js/noNestedTernary": {
		category: ["lint", "js", "noNestedTernary"],
		cases: noNestedTernary,
	},
	"js/noRestrictedGlobals": {
		category: ["lint", "js", "noRestrictedGlobals"],
		cases: noRestrictedGlobals,
	},
	"js/noSetterReturn": {
		category: ["lint", "js", "noSetterReturn"],
		cases: noSetterReturn,
	},
	"js/noShadowRestrictedNames": {
		category: ["lint", "js", "noShadowRestrictedNames"],
		cases: noShadowRestrictedNames,
	},
	"js/noShoutyConstants": {
		category: ["lint", "js", "noShoutyConstants"],
		cases: noShoutyConstants,
	},
	"js/noSingleCharRegexAlternatives": {
		category: ["lint", "js", "noSingleCharRegexAlternatives"],
		cases: noSingleCharRegexAlternatives,
	},
	"js/noSparseArray": {
		category: ["lint", "js", "noSparseArray"],
		cases: noSparseArray,
	},
	"js/noTemplateCurlyInString": {
		category: ["lint", "js", "noTemplateCurlyInString"],
		cases: noTemplateCurlyInString,
	},
	"js/noUndeclaredVariables": {
		category: ["lint", "js", "noUndeclaredVariables"],
		cases: noUndeclaredVariables,
	},
	"js/noUnnecessaryContinue": {
		category: ["lint", "js", "noUnnecessaryContinue"],
		cases: noUnnecessaryContinue,
	},
	"js/noUnsafeFinally": {
		category: ["lint", "js", "noUnsafeFinally"],
		cases: noUnsafeFinally,
	},
	"js/noUnsafeNegation": {
		category: ["lint", "js", "noUnsafeNegation"],
		cases: noUnsafeNegation,
	},
	"js/noUnusedTemplateLiteral": {
		category: ["lint", "js", "noUnusedTemplateLiteral"],
		cases: noUnusedTemplateLiteral,
	},
	"js/noUnusedVariables": {
		category: ["lint", "js", "noUnusedVariables"],
		cases: noUnusedVariables,
	},
	"js/noVar": {
		category: ["lint", "js", "noVar"],
		cases: noVar,
	},
	"js/preferOptionalChaining": {
		category: ["lint", "js", "preferOptionalChaining"],
		cases: preferOptionalChaining,
	},
	"js/useBlockStatements": {
		category: ["lint", "js", "useBlockStatements"],
		cases: useBlockStatements,
	},
	"js/useDefaultExportBasename": {
		category: ["lint", "js", "useDefaultExportBasename"],
		cases: useDefaultExportBasename,
	},
	"js/useDefaultImportBasename": {
		category: ["lint", "js", "useDefaultImportBasename"],
		cases: useDefaultImportBasename,
	},
	"js/useFunctionDeclarations": {
		category: ["lint", "js", "useFunctionDeclarations"],
		cases: useFunctionDeclarations,
	},
	"js/useSimplifiedLogicalExpression": {
		category: ["lint", "js", "useSimplifiedLogicalExpression"],
		cases: useSimplifiedLogicalExpression,
	},
	"js/useSingleCaseStatement": {
		category: ["lint", "js", "useSingleCaseStatement"],
		cases: useSingleCaseStatement,
	},
	"js/useSingleVarDeclarator": {
		category: ["lint", "js", "useSingleVarDeclarator"],
		cases: useSingleVarDeclarator,
	},
	"js/useSortedSpecifiers": {
		category: ["lint", "js", "useSortedSpecifiers"],
		cases: useSortedSpecifiers,
	},
	"js/useTemplate": {
		category: ["lint", "js", "useTemplate"],
		cases: useTemplate,
	},
	"js/useWhile": {
		category: ["lint", "js", "useWhile"],
		cases: useWhile,
	},
	"jsx-a11y/noAccessKey": {
		category: ["lint", "jsx-a11y", "noAccessKey"],
		cases: noAccessKey,
	},
	"jsx-a11y/noAutofocus": {
		category: ["lint", "jsx-a11y", "noAutofocus"],
		cases: noAutofocus,
	},
	"jsx-a11y/noHeaderScope": {
		category: ["lint", "jsx-a11y", "noHeaderScope"],
		cases: noHeaderScope,
	},
	"jsx-a11y/noOnChange": {
		category: ["lint", "jsx-a11y", "noOnChange"],
		cases: noOnChange,
	},
	"jsx-a11y/noPositiveTabindex": {
		category: ["lint", "jsx-a11y", "noPositiveTabindex"],
		cases: noPositiveTabindex,
	},
	"jsx-a11y/noRedundantAlt": {
		category: ["lint", "jsx-a11y", "noRedundantAlt"],
		cases: noRedundantAlt,
	},
	"jsx-a11y/noRedundantRoles": {
		category: ["lint", "jsx-a11y", "noRedundantRoles"],
		cases: noRedundantRoles,
	},
	"jsx-a11y/noTargetBlank": {
		category: ["lint", "jsx-a11y", "noTargetBlank"],
		cases: noTargetBlank,
	},
	"jsx-a11y/useAnchorContent": {
		category: ["lint", "jsx-a11y", "useAnchorContent"],
		cases: useAnchorContent,
	},
	"jsx-a11y/useAriaProps": {
		category: ["lint", "jsx-a11y", "useAriaProps"],
		cases: useAriaProps,
	},
	"jsx-a11y/useAriaPropsForRole": {
		category: ["lint", "jsx-a11y", "useAriaPropsForRole"],
		cases: useAriaPropsForRole,
	},
	"jsx-a11y/useHeadingContent": {
		category: ["lint", "jsx-a11y", "useHeadingContent"],
		cases: useHeadingContent,
	},
	"jsx-a11y/useKeyWithClickEvents": {
		category: ["lint", "jsx-a11y", "useKeyWithClickEvents"],
		cases: useKeyWithClickEvents,
	},
	"jsx-a11y/useKeyWithMouseEvents": {
		category: ["lint", "jsx-a11y", "useKeyWithMouseEvents"],
		cases: useKeyWithMouseEvents,
	},
	"jsx-a11y/useValidAnchor": {
		category: ["lint", "jsx-a11y", "useValidAnchor"],
		cases: useValidAnchor,
	},
	"jsx/noCommentText": {
		category: ["lint", "jsx", "noCommentText"],
		cases: noCommentText,
	},
	"jsx/noDuplicateProps": {
		category: ["lint", "jsx", "noDuplicateProps"],
		cases: noDuplicateProps,
	},
	"jsx/noImplicitBoolean": {
		category: ["lint", "jsx", "noImplicitBoolean"],
		cases: noImplicitBoolean,
	},
	"jsx/noPropSpreading": {
		category: ["lint", "jsx", "noPropSpreading"],
		cases: noPropSpreading,
	},
	"jsx/useJSXFileExtension": {
		category: ["lint", "jsx", "useJSXFileExtension"],
		cases: useJSXFileExtension,
	},
	"jsx/usePascalCase": {
		category: ["lint", "jsx", "usePascalCase"],
		cases: usePascalCase,
	},
	"jsx/useSelfClosingElements": {
		category: ["lint", "jsx", "useSelfClosingElements"],
		cases: useSelfClosingElements,
	},
	"react/noAccessStateInSetState": {
		category: ["lint", "react", "noAccessStateInSetState"],
		cases: noAccessStateInSetState,
	},
	"react/noArrayIndexKey": {
		category: ["lint", "react", "noArrayIndexKey"],
		cases: noArrayIndexKey,
	},
	"react/noChildrenProp": {
		category: ["lint", "react", "noChildrenProp"],
		cases: noChildrenProp,
	},
	"react/noDanger": {
		category: ["lint", "react", "noDanger"],
		cases: noDanger,
	},
	"react/noDangerWithChildren": {
		category: ["lint", "react", "noDangerWithChildren"],
		cases: noDangerWithChildren,
	},
	"react/noDidMountSetState": {
		category: ["lint", "react", "noDidMountSetState"],
		cases: noDidMountSetState,
	},
	"react/noDidUpdateSetState": {
		category: ["lint", "react", "noDidUpdateSetState"],
		cases: noDidUpdateSetState,
	},
	"react/noDirectMutationState": {
		category: ["lint", "react", "noDirectMutationState"],
		cases: noDirectMutationState,
	},
	"react/noFindDOMNode": {
		category: ["lint", "react", "noFindDOMNode"],
		cases: noFindDOMNode,
	},
	"react/noRedundantShouldComponentUpdate": {
		category: ["lint", "react", "noRedundantShouldComponentUpdate"],
		cases: noRedundantShouldComponentUpdate,
	},
	"react/noRenderReturnValue": {
		category: ["lint", "react", "noRenderReturnValue"],
		cases: noRenderReturnValue,
	},
	"react/noStringRefs": {
		category: ["lint", "react", "noStringRefs"],
		cases: noStringRefs,
	},
	"react/noThisInSFC": {
		category: ["lint", "react", "noThisInSFC"],
		cases: noThisInSFC,
	},
	"react/noUnsafe": {
		category: ["lint", "react", "noUnsafe"],
		cases: noUnsafe,
	},
	"react/noUselessFragment": {
		category: ["lint", "react", "noUselessFragment"],
		cases: noUselessFragment,
	},
	"react/noVoidElementsWithChildren": {
		category: ["lint", "react", "noVoidElementsWithChildren"],
		cases: noVoidElementsWithChildren,
	},
	"react/noWillUpdateSetState": {
		category: ["lint", "react", "noWillUpdateSetState"],
		cases: noWillUpdateSetState,
	},
	"react/useButtonType": {
		category: ["lint", "react", "useButtonType"],
		cases: useButtonType,
	},
	"react/useFragmentSyntax": {
		category: ["lint", "react", "useFragmentSyntax"],
		cases: useFragmentSyntax,
	},
	"react/useKey": {
		category: ["lint", "react", "useKey"],
		cases: useKey,
	},
	"react/useRenderReturn": {
		category: ["lint", "react", "useRenderReturn"],
		cases: useRenderReturn,
	},
	"react/useSortComp": {
		category: ["lint", "react", "useSortComp"],
		cases: useSortComp,
	},
	"react/useStylePropObject": {
		category: ["lint", "react", "useStylePropObject"],
		cases: useStylePropObject,
	},
	"regex/noDuplicateGroupNamesInRegularExpressions": {
		category: ["lint", "regex", "noDuplicateGroupNamesInRegularExpressions"],
		cases: noDuplicateGroupNamesInRegularExpressions,
	},
	"regex/noEmptyCharacterClass": {
		category: ["lint", "regex", "noEmptyCharacterClass"],
		cases: noEmptyCharacterClass,
	},
	"regex/noEmptyMatches": {
		category: ["lint", "regex", "noEmptyMatches"],
		cases: noEmptyMatches,
	},
	"regex/noMultipleSpacesInRegularExpressionLiterals": {
		category: ["lint", "regex", "noMultipleSpacesInRegularExpressionLiterals"],
		cases: noMultipleSpacesInRegularExpressionLiterals,
	},
	"regex/noPosixInRegularExpression": {
		category: ["lint", "regex", "noPosixInRegularExpression"],
		cases: noPosixInRegularExpression,
	},
	"ts/preferShorthandArrayType": {
		category: ["lint", "ts", "preferShorthandArrayType"],
		cases: preferShorthandArrayType,
	},
};
/* GENERATED:END(id:main) */
