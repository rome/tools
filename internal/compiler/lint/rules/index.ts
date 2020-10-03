/* GENERATED:START(hash:067905d26092d9f43a3792c9d668e05b7c1656fc,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
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
import noUnnecessaryContinue from "./js/noUnnecessaryContinue";
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
import useSimplifiedBooleanExpression from "./js/useSimplifiedBooleanExpression";
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
import useSimplifiedBooleanExpression from "./ts/useSimplifiedBooleanExpression";
import {AnyVisitor} from "@internal/compiler";

export const lintTransforms: Map<LintRuleName, AnyVisitor> = new Map();
lintTransforms.set("html/useClosingNonVoid", useClosingNonVoid);
lintTransforms.set("js/noArguments", noArguments);
lintTransforms.set("js/noAsyncPromiseExecutor", noAsyncPromiseExecutor);
lintTransforms.set("js/noCatchAssign", noCatchAssign);
lintTransforms.set("js/noCommaOperator", noCommaOperator);
lintTransforms.set("js/noCompareNegZero", noCompareNegZero);
lintTransforms.set("js/noCondAssign", noCondAssign);
lintTransforms.set("js/noDebugger", noDebugger);
lintTransforms.set("js/noDelete", noDelete);
lintTransforms.set("js/noDeleteVars", noDeleteVars);
lintTransforms.set("js/noDoubleEquals", noDoubleEquals);
lintTransforms.set("js/noDupeArgs", noDupeArgs);
lintTransforms.set("js/noDuplicateCase", noDuplicateCase);
lintTransforms.set("js/noDuplicateImportSource", noDuplicateImportSource);
lintTransforms.set("js/noDuplicateKeys", noDuplicateKeys);
lintTransforms.set("js/noEmptyBlocks", noEmptyBlocks);
lintTransforms.set("js/noExtraBooleanCast", noExtraBooleanCast);
lintTransforms.set("js/noFunctionAssign", noFunctionAssign);
lintTransforms.set("js/noGetterReturn", noGetterReturn);
lintTransforms.set("js/noImportAssign", noImportAssign);
lintTransforms.set("js/noLabelVar", noLabelVar);
lintTransforms.set("js/noNegationElse", noNegationElse);
lintTransforms.set("js/noNestedTernary", noNestedTernary);
lintTransforms.set("js/noRestrictedGlobals", noRestrictedGlobals);
lintTransforms.set("js/noSetterReturn", noSetterReturn);
lintTransforms.set("js/noShadowRestrictedNames", noShadowRestrictedNames);
lintTransforms.set("js/noShorthandArrayType", noShorthandArrayType);
lintTransforms.set("js/noShoutyConstants", noShoutyConstants);
lintTransforms.set("js/noSparseArray", noSparseArray);
lintTransforms.set("js/noTemplateCurlyInString", noTemplateCurlyInString);
lintTransforms.set("js/noUndeclaredVariables", noUndeclaredVariables);
lintTransforms.set("js/noUnnecessaryContinue", noUnnecessaryContinue);
lintTransforms.set("js/noUnsafeFinally", noUnsafeFinally);
lintTransforms.set("js/noUnsafeNegation", noUnsafeNegation);
lintTransforms.set("js/noUnusedTemplateLiteral", noUnusedTemplateLiteral);
lintTransforms.set("js/noUnusedVariables", noUnusedVariables);
lintTransforms.set("js/noVar", noVar);
lintTransforms.set("js/useBlockStatements", useBlockStatements);
lintTransforms.set("js/useCamelCase", useCamelCase);
lintTransforms.set("js/useDefaultExportBasename", useDefaultExportBasename);
lintTransforms.set("js/useDefaultImportBasename", useDefaultImportBasename);
lintTransforms.set("js/useFunctionDeclarations", useFunctionDeclarations);
lintTransforms.set(
	"js/useSimplifiedBooleanExpression",
	useSimplifiedBooleanExpression,
);
lintTransforms.set("js/useSingleCaseStatement", useSingleCaseStatement);
lintTransforms.set("js/useSingleVarDeclarator", useSingleVarDeclarator);
lintTransforms.set("js/useSortedSpecifiers", useSortedSpecifiers);
lintTransforms.set("js/useTemplate", useTemplate);
lintTransforms.set("js/useWhile", useWhile);
lintTransforms.set("jsx-a11y/noAccessKey", noAccessKey);
lintTransforms.set(
	"jsx-a11y/noAriaUnsupportedElements",
	noAriaUnsupportedElements,
);
lintTransforms.set("jsx-a11y/noAutofocus", noAutofocus);
lintTransforms.set("jsx-a11y/noDistractingElements", noDistractingElements);
lintTransforms.set("jsx-a11y/noHeaderScope", noHeaderScope);
lintTransforms.set(
	"jsx-a11y/noNoninteractiveElementToInteractiveRole",
	noNoninteractiveElementToInteractiveRole,
);
lintTransforms.set(
	"jsx-a11y/noNoninteractiveTabindex",
	noNoninteractiveTabindex,
);
lintTransforms.set("jsx-a11y/noOnChange", noOnChange);
lintTransforms.set("jsx-a11y/noPositiveTabindex", noPositiveTabindex);
lintTransforms.set("jsx-a11y/noRedundantAlt", noRedundantAlt);
lintTransforms.set("jsx-a11y/noRedundantRoles", noRedundantRoles);
lintTransforms.set("jsx-a11y/noTargetBlank", noTargetBlank);
lintTransforms.set("jsx-a11y/useAltText", useAltText);
lintTransforms.set("jsx-a11y/useAnchorContent", useAnchorContent);
lintTransforms.set("jsx-a11y/useAriaProps", useAriaProps);
lintTransforms.set("jsx-a11y/useAriaPropsForRole", useAriaPropsForRole);
lintTransforms.set("jsx-a11y/useAriaProptypes", useAriaProptypes);
lintTransforms.set("jsx-a11y/useHeadingContent", useHeadingContent);
lintTransforms.set("jsx-a11y/useHtmlLang", useHtmlLang);
lintTransforms.set("jsx-a11y/useIframeTitle", useIframeTitle);
lintTransforms.set("jsx-a11y/useKeyWithClickEvents", useKeyWithClickEvents);
lintTransforms.set("jsx-a11y/useKeyWithMouseEvents", useKeyWithMouseEvents);
lintTransforms.set("jsx-a11y/useMediaCaption", useMediaCaption);
lintTransforms.set("jsx-a11y/useValidAnchor", useValidAnchor);
lintTransforms.set("jsx-a11y/useValidLang", useValidLang);
lintTransforms.set("jsx/noCommentText", noCommentText);
lintTransforms.set("jsx/noDuplicateProps", noDuplicateProps);
lintTransforms.set("jsx/noImplicitBoolean", noImplicitBoolean);
lintTransforms.set("jsx/noPropSpreading", noPropSpreading);
lintTransforms.set("jsx/useJSXFileExtension", useJSXFileExtension);
lintTransforms.set("jsx/usePascalCase", usePascalCase);
lintTransforms.set("jsx/useSelfClosingElements", useSelfClosingElements);
lintTransforms.set("react/noAccessStateInSetState", noAccessStateInSetState);
lintTransforms.set("react/noArrayIndexKey", noArrayIndexKey);
lintTransforms.set("react/noChildrenProp", noChildrenProp);
lintTransforms.set("react/noDanger", noDanger);
lintTransforms.set("react/noDangerWithChildren", noDangerWithChildren);
lintTransforms.set("react/noDidMountSetState", noDidMountSetState);
lintTransforms.set("react/noDidUpdateSetState", noDidUpdateSetState);
lintTransforms.set("react/noDirectMutationState", noDirectMutationState);
lintTransforms.set("react/noFindDOMNode", noFindDOMNode);
lintTransforms.set(
	"react/noRedundantShouldComponentUpdate",
	noRedundantShouldComponentUpdate,
);
lintTransforms.set("react/noRenderReturnValue", noRenderReturnValue);
lintTransforms.set("react/noStringRefs", noStringRefs);
lintTransforms.set("react/noThisInSFC", noThisInSFC);
lintTransforms.set("react/noUnsafe", noUnsafe);
lintTransforms.set("react/noUselessFragment", noUselessFragment);
lintTransforms.set(
	"react/noVoidElementsWithChildren",
	noVoidElementsWithChildren,
);
lintTransforms.set("react/noWillUpdateSetState", noWillUpdateSetState);
lintTransforms.set("react/useButtonType", useButtonType);
lintTransforms.set("react/useFragmentSyntax", useFragmentSyntax);
lintTransforms.set("react/useKey", useKey);
lintTransforms.set("react/useRenderReturn", useRenderReturn);
lintTransforms.set("react/useSortComp", useSortComp);
lintTransforms.set("react/useStylePropObject", useStylePropObject);
lintTransforms.set(
	"regex/noDuplicateGroupNamesInRegularExpressions",
	noDuplicateGroupNamesInRegularExpressions,
);
lintTransforms.set("regex/noEmptyCharacterClass", noEmptyCharacterClass);
lintTransforms.set("regex/noEmptyMatches", noEmptyMatches);
lintTransforms.set(
	"regex/noMultipleSpacesInRegularExpressionLiterals",
	noMultipleSpacesInRegularExpressionLiterals,
);
lintTransforms.set(
	"regex/noPosixInRegularExpression",
	noPosixInRegularExpression,
);
lintTransforms.set(
	"regex/noReferenceToNonExistingGroup",
	noReferenceToNonExistingGroup,
);
lintTransforms.set("ts/noExplicitAny", noExplicitAny);
lintTransforms.set("ts/useInterfaces", useInterfaces);
lintTransforms.set(
	"ts/useSimplifiedBooleanExpression",
	useSimplifiedBooleanExpression,
);

export const lintRuleNames: Array<LintRuleName> = [
	"html/useClosingNonVoid",
	"js/noArguments",
	"js/noAsyncPromiseExecutor",
	"js/noCatchAssign",
	"js/noCommaOperator",
	"js/noCompareNegZero",
	"js/noCondAssign",
	"js/noDebugger",
	"js/noDelete",
	"js/noDeleteVars",
	"js/noDoubleEquals",
	"js/noDupeArgs",
	"js/noDuplicateCase",
	"js/noDuplicateImportSource",
	"js/noDuplicateKeys",
	"js/noEmptyBlocks",
	"js/noExtraBooleanCast",
	"js/noFunctionAssign",
	"js/noGetterReturn",
	"js/noImportAssign",
	"js/noLabelVar",
	"js/noNegationElse",
	"js/noNestedTernary",
	"js/noRestrictedGlobals",
	"js/noSetterReturn",
	"js/noShadowRestrictedNames",
	"js/noShorthandArrayType",
	"js/noShoutyConstants",
	"js/noSparseArray",
	"js/noTemplateCurlyInString",
	"js/noUndeclaredVariables",
	"js/noUnnecessaryContinue",
	"js/noUnsafeFinally",
	"js/noUnsafeNegation",
	"js/noUnusedTemplateLiteral",
	"js/noUnusedVariables",
	"js/noVar",
	"js/useBlockStatements",
	"js/useCamelCase",
	"js/useDefaultExportBasename",
	"js/useDefaultImportBasename",
	"js/useFunctionDeclarations",
	"js/useSimplifiedBooleanExpression",
	"js/useSingleCaseStatement",
	"js/useSingleVarDeclarator",
	"js/useSortedSpecifiers",
	"js/useTemplate",
	"js/useWhile",
	"jsx-a11y/noAccessKey",
	"jsx-a11y/noAriaUnsupportedElements",
	"jsx-a11y/noAutofocus",
	"jsx-a11y/noDistractingElements",
	"jsx-a11y/noHeaderScope",
	"jsx-a11y/noNoninteractiveElementToInteractiveRole",
	"jsx-a11y/noNoninteractiveTabindex",
	"jsx-a11y/noOnChange",
	"jsx-a11y/noPositiveTabindex",
	"jsx-a11y/noRedundantAlt",
	"jsx-a11y/noRedundantRoles",
	"jsx-a11y/noTargetBlank",
	"jsx-a11y/useAltText",
	"jsx-a11y/useAnchorContent",
	"jsx-a11y/useAriaProps",
	"jsx-a11y/useAriaPropsForRole",
	"jsx-a11y/useAriaProptypes",
	"jsx-a11y/useHeadingContent",
	"jsx-a11y/useHtmlLang",
	"jsx-a11y/useIframeTitle",
	"jsx-a11y/useKeyWithClickEvents",
	"jsx-a11y/useKeyWithMouseEvents",
	"jsx-a11y/useMediaCaption",
	"jsx-a11y/useValidAnchor",
	"jsx-a11y/useValidLang",
	"jsx/noCommentText",
	"jsx/noDuplicateProps",
	"jsx/noImplicitBoolean",
	"jsx/noPropSpreading",
	"jsx/useJSXFileExtension",
	"jsx/usePascalCase",
	"jsx/useSelfClosingElements",
	"react/noAccessStateInSetState",
	"react/noArrayIndexKey",
	"react/noChildrenProp",
	"react/noDanger",
	"react/noDangerWithChildren",
	"react/noDidMountSetState",
	"react/noDidUpdateSetState",
	"react/noDirectMutationState",
	"react/noFindDOMNode",
	"react/noRedundantShouldComponentUpdate",
	"react/noRenderReturnValue",
	"react/noStringRefs",
	"react/noThisInSFC",
	"react/noUnsafe",
	"react/noUselessFragment",
	"react/noVoidElementsWithChildren",
	"react/noWillUpdateSetState",
	"react/useButtonType",
	"react/useFragmentSyntax",
	"react/useKey",
	"react/useRenderReturn",
	"react/useSortComp",
	"react/useStylePropObject",
	"regex/noDuplicateGroupNamesInRegularExpressions",
	"regex/noEmptyCharacterClass",
	"regex/noEmptyMatches",
	"regex/noMultipleSpacesInRegularExpressionLiterals",
	"regex/noPosixInRegularExpression",
	"regex/noReferenceToNonExistingGroup",
	"ts/noExplicitAny",
	"ts/useInterfaces",
	"ts/useSimplifiedBooleanExpression",
];

export type LintRuleName =
	| "html/useClosingNonVoid"
	| "js/noArguments"
	| "js/noAsyncPromiseExecutor"
	| "js/noCatchAssign"
	| "js/noCommaOperator"
	| "js/noCompareNegZero"
	| "js/noCondAssign"
	| "js/noDebugger"
	| "js/noDelete"
	| "js/noDeleteVars"
	| "js/noDoubleEquals"
	| "js/noDupeArgs"
	| "js/noDuplicateCase"
	| "js/noDuplicateImportSource"
	| "js/noDuplicateKeys"
	| "js/noEmptyBlocks"
	| "js/noExtraBooleanCast"
	| "js/noFunctionAssign"
	| "js/noGetterReturn"
	| "js/noImportAssign"
	| "js/noLabelVar"
	| "js/noNegationElse"
	| "js/noNestedTernary"
	| "js/noRestrictedGlobals"
	| "js/noSetterReturn"
	| "js/noShadowRestrictedNames"
	| "js/noShorthandArrayType"
	| "js/noShoutyConstants"
	| "js/noSparseArray"
	| "js/noTemplateCurlyInString"
	| "js/noUndeclaredVariables"
	| "js/noUnnecessaryContinue"
	| "js/noUnsafeFinally"
	| "js/noUnsafeNegation"
	| "js/noUnusedTemplateLiteral"
	| "js/noUnusedVariables"
	| "js/noVar"
	| "js/useBlockStatements"
	| "js/useCamelCase"
	| "js/useDefaultExportBasename"
	| "js/useDefaultImportBasename"
	| "js/useFunctionDeclarations"
	| "js/useSimplifiedBooleanExpression"
	| "js/useSingleCaseStatement"
	| "js/useSingleVarDeclarator"
	| "js/useSortedSpecifiers"
	| "js/useTemplate"
	| "js/useWhile"
	| "jsx-a11y/noAccessKey"
	| "jsx-a11y/noAriaUnsupportedElements"
	| "jsx-a11y/noAutofocus"
	| "jsx-a11y/noDistractingElements"
	| "jsx-a11y/noHeaderScope"
	| "jsx-a11y/noNoninteractiveElementToInteractiveRole"
	| "jsx-a11y/noNoninteractiveTabindex"
	| "jsx-a11y/noOnChange"
	| "jsx-a11y/noPositiveTabindex"
	| "jsx-a11y/noRedundantAlt"
	| "jsx-a11y/noRedundantRoles"
	| "jsx-a11y/noTargetBlank"
	| "jsx-a11y/useAltText"
	| "jsx-a11y/useAnchorContent"
	| "jsx-a11y/useAriaProps"
	| "jsx-a11y/useAriaPropsForRole"
	| "jsx-a11y/useAriaProptypes"
	| "jsx-a11y/useHeadingContent"
	| "jsx-a11y/useHtmlLang"
	| "jsx-a11y/useIframeTitle"
	| "jsx-a11y/useKeyWithClickEvents"
	| "jsx-a11y/useKeyWithMouseEvents"
	| "jsx-a11y/useMediaCaption"
	| "jsx-a11y/useValidAnchor"
	| "jsx-a11y/useValidLang"
	| "jsx/noCommentText"
	| "jsx/noDuplicateProps"
	| "jsx/noImplicitBoolean"
	| "jsx/noPropSpreading"
	| "jsx/useJSXFileExtension"
	| "jsx/usePascalCase"
	| "jsx/useSelfClosingElements"
	| "react/noAccessStateInSetState"
	| "react/noArrayIndexKey"
	| "react/noChildrenProp"
	| "react/noDanger"
	| "react/noDangerWithChildren"
	| "react/noDidMountSetState"
	| "react/noDidUpdateSetState"
	| "react/noDirectMutationState"
	| "react/noFindDOMNode"
	| "react/noRedundantShouldComponentUpdate"
	| "react/noRenderReturnValue"
	| "react/noStringRefs"
	| "react/noThisInSFC"
	| "react/noUnsafe"
	| "react/noUselessFragment"
	| "react/noVoidElementsWithChildren"
	| "react/noWillUpdateSetState"
	| "react/useButtonType"
	| "react/useFragmentSyntax"
	| "react/useKey"
	| "react/useRenderReturn"
	| "react/useSortComp"
	| "react/useStylePropObject"
	| "regex/noDuplicateGroupNamesInRegularExpressions"
	| "regex/noEmptyCharacterClass"
	| "regex/noEmptyMatches"
	| "regex/noMultipleSpacesInRegularExpressionLiterals"
	| "regex/noPosixInRegularExpression"
	| "regex/noReferenceToNonExistingGroup"
	| "ts/noExplicitAny"
	| "ts/useInterfaces"
	| "ts/useSimplifiedBooleanExpression";
/* GENERATED:END(id:main) */
