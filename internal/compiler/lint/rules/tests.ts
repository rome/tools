import {DiagnosticCategory} from "@internal/diagnostics";
import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<{
	category: DiagnosticCategory;
	cases: Test | Test[] | Test[][];
}>;

/* GENERATED:START(hash:25cbc0174a887b5e1bf0e107a2953ee817a4002f,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-expect-error
import noAccessKey from "./a11y/noAccessKey.test.rjson";
// @ts-expect-error
import noAriaUnsupportedElements from "./a11y/noAriaUnsupportedElements.test.rjson";
// @ts-expect-error
import noAutofocus from "./a11y/noAutofocus.test.rjson";
// @ts-expect-error
import noDistractingElements from "./a11y/noDistractingElements.test.rjson";
// @ts-expect-error
import noHeaderScope from "./a11y/noHeaderScope.test.rjson";
// @ts-expect-error
import noNoninteractiveElementToInteractiveRole from "./a11y/noNoninteractiveElementToInteractiveRole.test.rjson";
// @ts-expect-error
import noNoninteractiveTabindex from "./a11y/noNoninteractiveTabindex.test.rjson";
// @ts-expect-error
import noOnChange from "./a11y/noOnChange.test.rjson";
// @ts-expect-error
import noPositiveTabindex from "./a11y/noPositiveTabindex.test.rjson";
// @ts-expect-error
import noRedundantAlt from "./a11y/noRedundantAlt.test.rjson";
// @ts-expect-error
import noSvgWithoutTitle from "./a11y/noSvgWithoutTitle.test.rjson";
// @ts-expect-error
import noTargetBlank from "./a11y/noTargetBlank.test.rjson";
// @ts-expect-error
import useAltText from "./a11y/useAltText.test.rjson";
// @ts-expect-error
import useAnchorContent from "./a11y/useAnchorContent.test.rjson";
// @ts-expect-error
import useAriaProps from "./a11y/useAriaProps.test.rjson";
// @ts-expect-error
import useAriaProptypes from "./a11y/useAriaProptypes.test.rjson";
// @ts-expect-error
import useHeadingContent from "./a11y/useHeadingContent.test.rjson";
// @ts-expect-error
import useHtmlLang from "./a11y/useHtmlLang.test.rjson";
// @ts-expect-error
import useIframeTitle from "./a11y/useIframeTitle.test.rjson";
// @ts-expect-error
import useKeyWithClickEvents from "./a11y/useKeyWithClickEvents.test.rjson";
// @ts-expect-error
import useKeyWithMouseEvents from "./a11y/useKeyWithMouseEvents.test.rjson";
// @ts-expect-error
import useMediaCaption from "./a11y/useMediaCaption.test.rjson";
// @ts-expect-error
import useValidLang from "./a11y/useValidLang.test.rjson";
// @ts-expect-error
import noImportantInKeyframes from "./css/noImportantInKeyframes.test.rjson";
// @ts-expect-error
import noInvalidGridTemplateAreas from "./css/noInvalidGridTemplateAreas.test.rjson";
// @ts-expect-error
import useClosingNonVoid from "./html/useClosingNonVoid.test.rjson";
// @ts-expect-error
import noArguments from "./js/noArguments.test.rjson";
// @ts-expect-error
import noAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor.test.rjson";
// @ts-expect-error
import noCatchAssign from "./js/noCatchAssign.test.rjson";
// @ts-expect-error
import noCommaOperator from "./js/noCommaOperator.test.rjson";
// @ts-expect-error
import noCompareNegZero from "./js/noCompareNegZero.test.rjson";
// @ts-expect-error
import noCondAssign from "./js/noCondAssign.test.rjson";
// @ts-expect-error
import noDebugger from "./js/noDebugger.test.rjson";
// @ts-expect-error
import noDelete from "./js/noDelete.test.rjson";
// @ts-expect-error
import noDeleteVars from "./js/noDeleteVars.test.rjson";
// @ts-expect-error
import noDoubleEquals from "./js/noDoubleEquals.test.rjson";
// @ts-expect-error
import noDupeArgs from "./js/noDupeArgs.test.rjson";
// @ts-expect-error
import noDuplicateCase from "./js/noDuplicateCase.test.rjson";
// @ts-expect-error
import noDuplicateImportSource from "./js/noDuplicateImportSource.test.rjson";
// @ts-expect-error
import noDuplicateKeys from "./js/noDuplicateKeys.test.rjson";
// @ts-expect-error
import noEmptyBlocks from "./js/noEmptyBlocks.test.rjson";
// @ts-expect-error
import noExtraBooleanCast from "./js/noExtraBooleanCast.test.rjson";
// @ts-expect-error
import noFunctionAssign from "./js/noFunctionAssign.test.rjson";
// @ts-expect-error
import noGetterReturn from "./js/noGetterReturn.test.rjson";
// @ts-expect-error
import noImportAssign from "./js/noImportAssign.test.rjson";
// @ts-expect-error
import noLabelVar from "./js/noLabelVar.test.rjson";
// @ts-expect-error
import noNegationElse from "./js/noNegationElse.test.rjson";
// @ts-expect-error
import noNestedTernary from "./js/noNestedTernary.test.rjson";
// @ts-expect-error
import noRestrictedGlobals from "./js/noRestrictedGlobals.test.rjson";
// @ts-expect-error
import noSetterReturn from "./js/noSetterReturn.test.rjson";
// @ts-expect-error
import noShadowRestrictedNames from "./js/noShadowRestrictedNames.test.rjson";
// @ts-expect-error
import noShoutyConstants from "./js/noShoutyConstants.test.rjson";
// @ts-expect-error
import noSingleCharRegexAlternatives from "./js/noSingleCharRegexAlternatives.test.rjson";
// @ts-expect-error
import noSparseArray from "./js/noSparseArray.test.rjson";
// @ts-expect-error
import noTemplateCurlyInString from "./js/noTemplateCurlyInString.test.rjson";
// @ts-expect-error
import noUndeclaredVariables from "./js/noUndeclaredVariables.test.rjson";
// @ts-expect-error
import noUnnecessaryContinue from "./js/noUnnecessaryContinue.test.rjson";
// @ts-expect-error
import noUnsafeFinally from "./js/noUnsafeFinally.test.rjson";
// @ts-expect-error
import noUnsafeNegation from "./js/noUnsafeNegation.test.rjson";
// @ts-expect-error
import noUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.rjson";
// @ts-expect-error
import noUnusedVariables from "./js/noUnusedVariables.test.rjson";
// @ts-expect-error
import noVar from "./js/noVar.test.rjson";
// @ts-expect-error
import preferOptionalChaining from "./js/preferOptionalChaining.test.rjson";
// @ts-expect-error
import useBlockStatements from "./js/useBlockStatements.test.rjson";
// @ts-expect-error
import useDefaultExportBasename from "./js/useDefaultExportBasename.test.rjson";
// @ts-expect-error
import useDefaultImportBasename from "./js/useDefaultImportBasename.test.rjson";
// @ts-expect-error
import useFunctionDeclarations from "./js/useFunctionDeclarations.test.rjson";
// @ts-expect-error
import useSimplifiedLogicalExpression from "./js/useSimplifiedLogicalExpression.test.rjson";
// @ts-expect-error
import useSingleCaseStatement from "./js/useSingleCaseStatement.test.rjson";
// @ts-expect-error
import useSingleVarDeclarator from "./js/useSingleVarDeclarator.test.rjson";
// @ts-expect-error
import useSortedSpecifiers from "./js/useSortedSpecifiers.test.rjson";
// @ts-expect-error
import useTemplate from "./js/useTemplate.test.rjson";
// @ts-expect-error
import useWhile from "./js/useWhile.test.rjson";
// @ts-expect-error
import noRedundantRoles from "./jsx-a11y/noRedundantRoles.test.rjson";
// @ts-expect-error
import useAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole.test.rjson";
// @ts-expect-error
import useValidAnchor from "./jsx-a11y/useValidAnchor.test.rjson";
// @ts-expect-error
import noCommentText from "./jsx/noCommentText.test.rjson";
// @ts-expect-error
import noDuplicateProps from "./jsx/noDuplicateProps.test.rjson";
// @ts-expect-error
import noImplicitBoolean from "./jsx/noImplicitBoolean.test.rjson";
// @ts-expect-error
import noPropSpreading from "./jsx/noPropSpreading.test.rjson";
// @ts-expect-error
import useJSXFileExtension from "./jsx/useJSXFileExtension.test.rjson";
// @ts-expect-error
import usePascalCase from "./jsx/usePascalCase.test.rjson";
// @ts-expect-error
import useSelfClosingElements from "./jsx/useSelfClosingElements.test.rjson";
// @ts-expect-error
import noAccessStateInSetState from "./react/noAccessStateInSetState.test.rjson";
// @ts-expect-error
import noArrayIndexKey from "./react/noArrayIndexKey.test.rjson";
// @ts-expect-error
import noChildrenProp from "./react/noChildrenProp.test.rjson";
// @ts-expect-error
import noDanger from "./react/noDanger.test.rjson";
// @ts-expect-error
import noDangerWithChildren from "./react/noDangerWithChildren.test.rjson";
// @ts-expect-error
import noDidMountSetState from "./react/noDidMountSetState.test.rjson";
// @ts-expect-error
import noDidUpdateSetState from "./react/noDidUpdateSetState.test.rjson";
// @ts-expect-error
import noDirectMutationState from "./react/noDirectMutationState.test.rjson";
// @ts-expect-error
import noFindDOMNode from "./react/noFindDOMNode.test.rjson";
// @ts-expect-error
import noRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate.test.rjson";
// @ts-expect-error
import noRenderReturnValue from "./react/noRenderReturnValue.test.rjson";
// @ts-expect-error
import noStringRefs from "./react/noStringRefs.test.rjson";
// @ts-expect-error
import noThisInSFC from "./react/noThisInSFC.test.rjson";
// @ts-expect-error
import noUnsafe from "./react/noUnsafe.test.rjson";
// @ts-expect-error
import noUselessFragment from "./react/noUselessFragment.test.rjson";
// @ts-expect-error
import noVoidElementsWithChildren from "./react/noVoidElementsWithChildren.test.rjson";
// @ts-expect-error
import noWillUpdateSetState from "./react/noWillUpdateSetState.test.rjson";
// @ts-expect-error
import useButtonType from "./react/useButtonType.test.rjson";
// @ts-expect-error
import useFragmentSyntax from "./react/useFragmentSyntax.test.rjson";
// @ts-expect-error
import useKey from "./react/useKey.test.rjson";
// @ts-expect-error
import useRenderReturn from "./react/useRenderReturn.test.rjson";
// @ts-expect-error
import useSortComp from "./react/useSortComp.test.rjson";
// @ts-expect-error
import useStylePropObject from "./react/useStylePropObject.test.rjson";
// @ts-expect-error
import noDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions.test.rjson";
// @ts-expect-error
import noEmptyCharacterClass from "./regex/noEmptyCharacterClass.test.rjson";
// @ts-expect-error
import noEmptyMatches from "./regex/noEmptyMatches.test.rjson";
// @ts-expect-error
import noMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals.test.rjson";
// @ts-expect-error
import noPosixInRegularExpression from "./regex/noPosixInRegularExpression.test.rjson";
// @ts-expect-error
import preferShorthandArrayType from "./ts/preferShorthandArrayType.test.rjson";
// @ts-expect-error
import preferTsExpectError from "./ts/preferTsExpectError.test.rjson";

export const tests: Tests = {
	"a11y/noAccessKey": {
		category: ["lint", "a11y", "noAccessKey"],
		cases: noAccessKey,
	},
	"a11y/noAriaUnsupportedElements": {
		category: ["lint", "a11y", "noAriaUnsupportedElements"],
		cases: noAriaUnsupportedElements,
	},
	"a11y/noAutofocus": {
		category: ["lint", "a11y", "noAutofocus"],
		cases: noAutofocus,
	},
	"a11y/noDistractingElements": {
		category: ["lint", "a11y", "noDistractingElements"],
		cases: noDistractingElements,
	},
	"a11y/noHeaderScope": {
		category: ["lint", "a11y", "noHeaderScope"],
		cases: noHeaderScope,
	},
	"a11y/noNoninteractiveElementToInteractiveRole": {
		category: ["lint", "a11y", "noNoninteractiveElementToInteractiveRole"],
		cases: noNoninteractiveElementToInteractiveRole,
	},
	"a11y/noNoninteractiveTabindex": {
		category: ["lint", "a11y", "noNoninteractiveTabindex"],
		cases: noNoninteractiveTabindex,
	},
	"a11y/noOnChange": {
		category: ["lint", "a11y", "noOnChange"],
		cases: noOnChange,
	},
	"a11y/noPositiveTabindex": {
		category: ["lint", "a11y", "noPositiveTabindex"],
		cases: noPositiveTabindex,
	},
	"a11y/noRedundantAlt": {
		category: ["lint", "a11y", "noRedundantAlt"],
		cases: noRedundantAlt,
	},
	"a11y/noSvgWithoutTitle": {
		category: ["lint", "a11y", "noSvgWithoutTitle"],
		cases: noSvgWithoutTitle,
	},
	"a11y/noTargetBlank": {
		category: ["lint", "a11y", "noTargetBlank"],
		cases: noTargetBlank,
	},
	"a11y/useAltText": {
		category: ["lint", "a11y", "useAltText"],
		cases: useAltText,
	},
	"a11y/useAnchorContent": {
		category: ["lint", "a11y", "useAnchorContent"],
		cases: useAnchorContent,
	},
	"a11y/useAriaProps": {
		category: ["lint", "a11y", "useAriaProps"],
		cases: useAriaProps,
	},
	"a11y/useAriaProptypes": {
		category: ["lint", "a11y", "useAriaProptypes"],
		cases: useAriaProptypes,
	},
	"a11y/useHeadingContent": {
		category: ["lint", "a11y", "useHeadingContent"],
		cases: useHeadingContent,
	},
	"a11y/useHtmlLang": {
		category: ["lint", "a11y", "useHtmlLang"],
		cases: useHtmlLang,
	},
	"a11y/useIframeTitle": {
		category: ["lint", "a11y", "useIframeTitle"],
		cases: useIframeTitle,
	},
	"a11y/useKeyWithClickEvents": {
		category: ["lint", "a11y", "useKeyWithClickEvents"],
		cases: useKeyWithClickEvents,
	},
	"a11y/useKeyWithMouseEvents": {
		category: ["lint", "a11y", "useKeyWithMouseEvents"],
		cases: useKeyWithMouseEvents,
	},
	"a11y/useMediaCaption": {
		category: ["lint", "a11y", "useMediaCaption"],
		cases: useMediaCaption,
	},
	"a11y/useValidLang": {
		category: ["lint", "a11y", "useValidLang"],
		cases: useValidLang,
	},
	"css/noImportantInKeyframes": {
		category: ["lint", "css", "noImportantInKeyframes"],
		cases: noImportantInKeyframes,
	},
	"css/noInvalidGridTemplateAreas": {
		category: ["lint", "css", "noInvalidGridTemplateAreas"],
		cases: noInvalidGridTemplateAreas,
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
	"jsx-a11y/noRedundantRoles": {
		category: ["lint", "jsx-a11y", "noRedundantRoles"],
		cases: noRedundantRoles,
	},
	"jsx-a11y/useAriaPropsForRole": {
		category: ["lint", "jsx-a11y", "useAriaPropsForRole"],
		cases: useAriaPropsForRole,
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
	"ts/preferTsExpectError": {
		category: ["lint", "ts", "preferTsExpectError"],
		cases: preferTsExpectError,
	},
};
/* GENERATED:END(id:main) */
