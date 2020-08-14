/* GENERATED:START(hash:7eba8757b5c471ad06578872a5618edd56a74472,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
import useClosingNonVoid from "./html/useClosingNonVoid";
import noArguments from "./js/noArguments";
import noAsyncPromiseExecutor from "./js/noAsyncPromiseExecutor";
import noCatchAssign from "./js/noCatchAssign";
import noCommaOperator from "./js/noCommaOperator";
import noCompareNegZero from "./js/noCompareNegZero";
import noCondAssign from "./js/noCondAssign";
import noDebugger from "./js/noDebugger";
import noDelete from "./js/noDelete";
import noDeleteVars from "./js/noDeleteVars";
import noDoubleEquals from "./js/noDoubleEquals";
import noDupeArgs from "./js/noDupeArgs";
import noDuplicateCase from "./js/noDuplicateCase";
import noDuplicateImportSource from "./js/noDuplicateImportSource";
import noDuplicateKeys from "./js/noDuplicateKeys";
import noEmptyBlocks from "./js/noEmptyBlocks";
import noExtraBooleanCast from "./js/noExtraBooleanCast";
import noFunctionAssign from "./js/noFunctionAssign";
import noGetterReturn from "./js/noGetterReturn";
import noImportAssign from "./js/noImportAssign";
import noLabelVar from "./js/noLabelVar";
import noNegationElse from "./js/noNegationElse";
import noNestedTernary from "./js/noNestedTernary";
import noRestrictedGlobals from "./js/noRestrictedGlobals";
import noSetterReturn from "./js/noSetterReturn";
import noShadowRestrictedNames from "./js/noShadowRestrictedNames";
import noShorthandArrayType from "./js/noShorthandArrayType";
import noShoutyConstants from "./js/noShoutyConstants";
import noSparseArray from "./js/noSparseArray";
import noTemplateCurlyInString from "./js/noTemplateCurlyInString";
import noUndeclaredVariables from "./js/noUndeclaredVariables";
import noUnsafeFinally from "./js/noUnsafeFinally";
import noUnsafeNegation from "./js/noUnsafeNegation";
import noUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral";
import noUnusedVariables from "./js/noUnusedVariables";
import noVar from "./js/noVar";
import useBlockStatements from "./js/useBlockStatements";
import useCamelCase from "./js/useCamelCase";
import useDefaultExportBasename from "./js/useDefaultExportBasename";
import useDefaultImportBasename from "./js/useDefaultImportBasename";
import useFunctionDeclarations from "./js/useFunctionDeclarations";
import useSingleCaseStatement from "./js/useSingleCaseStatement";
import useSingleVarDeclarator from "./js/useSingleVarDeclarator";
import useSortedSpecifiers from "./js/useSortedSpecifiers";
import useTemplate from "./js/useTemplate";
import useWhile from "./js/useWhile";
import noAccessKey from "./jsx-a11y/noAccessKey";
import noAriaUnsupportedElements from "./jsx-a11y/noAriaUnsupportedElements";
import noAutofocus from "./jsx-a11y/noAutofocus";
import noDistractingElements from "./jsx-a11y/noDistractingElements";
import noHeaderScope from "./jsx-a11y/noHeaderScope";
import noNoninteractiveElementToInteractiveRole from "./jsx-a11y/noNoninteractiveElementToInteractiveRole";
import noNoninteractiveTabindex from "./jsx-a11y/noNoninteractiveTabindex";
import noOnChange from "./jsx-a11y/noOnChange";
import noPositiveTabindex from "./jsx-a11y/noPositiveTabindex";
import noRedundantAlt from "./jsx-a11y/noRedundantAlt";
import noRedundantRoles from "./jsx-a11y/noRedundantRoles";
import noTargetBlank from "./jsx-a11y/noTargetBlank";
import useAltText from "./jsx-a11y/useAltText";
import useAnchorContent from "./jsx-a11y/useAnchorContent";
import useAriaProps from "./jsx-a11y/useAriaProps";
import useAriaPropsForRole from "./jsx-a11y/useAriaPropsForRole";
import useAriaProptypes from "./jsx-a11y/useAriaProptypes";
import useHeadingContent from "./jsx-a11y/useHeadingContent";
import useHtmlLang from "./jsx-a11y/useHtmlLang";
import useIframeTitle from "./jsx-a11y/useIframeTitle";
import useKeyWithClickEvents from "./jsx-a11y/useKeyWithClickEvents";
import useKeyWithMouseEvents from "./jsx-a11y/useKeyWithMouseEvents";
import useMediaCaption from "./jsx-a11y/useMediaCaption";
import useValidAnchor from "./jsx-a11y/useValidAnchor";
import useValidLang from "./jsx-a11y/useValidLang";
import noCommentText from "./jsx/noCommentText";
import noDuplicateProps from "./jsx/noDuplicateProps";
import noImplicitBoolean from "./jsx/noImplicitBoolean";
import noPropSpreading from "./jsx/noPropSpreading";
import useJSXFileExtension from "./jsx/useJSXFileExtension";
import usePascalCase from "./jsx/usePascalCase";
import useSelfClosingElements from "./jsx/useSelfClosingElements";
import noAccessStateInSetState from "./react/noAccessStateInSetState";
import noArrayIndexKey from "./react/noArrayIndexKey";
import noChildrenProp from "./react/noChildrenProp";
import noDanger from "./react/noDanger";
import noDangerWithChildren from "./react/noDangerWithChildren";
import noDidMountSetState from "./react/noDidMountSetState";
import noDidUpdateSetState from "./react/noDidUpdateSetState";
import noDirectMutationState from "./react/noDirectMutationState";
import noFindDOMNode from "./react/noFindDOMNode";
import noRedundantShouldComponentUpdate from "./react/noRedundantShouldComponentUpdate";
import noRenderReturnValue from "./react/noRenderReturnValue";
import noStringRefs from "./react/noStringRefs";
import noThisInSFC from "./react/noThisInSFC";
import noUnsafe from "./react/noUnsafe";
import noUselessFragment from "./react/noUselessFragment";
import noVoidElementsWithChildren from "./react/noVoidElementsWithChildren";
import noWillUpdateSetState from "./react/noWillUpdateSetState";
import useButtonType from "./react/useButtonType";
import useFragmentSyntax from "./react/useFragmentSyntax";
import useKey from "./react/useKey";
import useRenderReturn from "./react/useRenderReturn";
import useSortComp from "./react/useSortComp";
import useStylePropObject from "./react/useStylePropObject";
import noDuplicateGroupNamesInRegularExpressions from "./regex/noDuplicateGroupNamesInRegularExpressions";
import noEmptyCharacterClass from "./regex/noEmptyCharacterClass";
import noEmptyMatches from "./regex/noEmptyMatches";
import noMultipleSpacesInRegularExpressionLiterals from "./regex/noMultipleSpacesInRegularExpressionLiterals";
import noPosixInRegularExpression from "./regex/noPosixInRegularExpression";
import noReferenceToNonExistingGroup from "./regex/noReferenceToNonExistingGroup";
import noExplicitAny from "./ts/noExplicitAny";
import useInterfaces from "./ts/useInterfaces";
import {AnyVisitors} from "@internal/compiler";

export const lintTransforms: AnyVisitors = [
	useClosingNonVoid,
	noArguments,
	noAsyncPromiseExecutor,
	noCatchAssign,
	noCommaOperator,
	noCompareNegZero,
	noCondAssign,
	noDebugger,
	noDelete,
	noDeleteVars,
	noDoubleEquals,
	noDupeArgs,
	noDuplicateCase,
	noDuplicateImportSource,
	noDuplicateKeys,
	noEmptyBlocks,
	noExtraBooleanCast,
	noFunctionAssign,
	noGetterReturn,
	noImportAssign,
	noLabelVar,
	noNegationElse,
	noNestedTernary,
	noRestrictedGlobals,
	noSetterReturn,
	noShadowRestrictedNames,
	noShorthandArrayType,
	noShoutyConstants,
	noSparseArray,
	noTemplateCurlyInString,
	noUndeclaredVariables,
	noUnsafeFinally,
	noUnsafeNegation,
	noUnusedTemplateLiteral,
	noUnusedVariables,
	noVar,
	useBlockStatements,
	useCamelCase,
	useDefaultExportBasename,
	useDefaultImportBasename,
	useFunctionDeclarations,
	useSingleCaseStatement,
	useSingleVarDeclarator,
	useSortedSpecifiers,
	useTemplate,
	useWhile,
	noAccessKey,
	noAriaUnsupportedElements,
	noAutofocus,
	noDistractingElements,
	noHeaderScope,
	noNoninteractiveElementToInteractiveRole,
	noNoninteractiveTabindex,
	noOnChange,
	noPositiveTabindex,
	noRedundantAlt,
	noRedundantRoles,
	noTargetBlank,
	useAltText,
	useAnchorContent,
	useAriaProps,
	useAriaPropsForRole,
	useAriaProptypes,
	useHeadingContent,
	useHtmlLang,
	useIframeTitle,
	useKeyWithClickEvents,
	useKeyWithMouseEvents,
	useMediaCaption,
	useValidAnchor,
	useValidLang,
	noCommentText,
	noDuplicateProps,
	noImplicitBoolean,
	noPropSpreading,
	useJSXFileExtension,
	usePascalCase,
	useSelfClosingElements,
	noAccessStateInSetState,
	noArrayIndexKey,
	noChildrenProp,
	noDanger,
	noDangerWithChildren,
	noDidMountSetState,
	noDidUpdateSetState,
	noDirectMutationState,
	noFindDOMNode,
	noRedundantShouldComponentUpdate,
	noRenderReturnValue,
	noStringRefs,
	noThisInSFC,
	noUnsafe,
	noUselessFragment,
	noVoidElementsWithChildren,
	noWillUpdateSetState,
	useButtonType,
	useFragmentSyntax,
	useKey,
	useRenderReturn,
	useSortComp,
	useStylePropObject,
	noDuplicateGroupNamesInRegularExpressions,
	noEmptyCharacterClass,
	noEmptyMatches,
	noMultipleSpacesInRegularExpressionLiterals,
	noPosixInRegularExpression,
	noReferenceToNonExistingGroup,
	noExplicitAny,
	useInterfaces,
];
/* GENERATED:END(id:main) */
