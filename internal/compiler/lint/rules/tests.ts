import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: Array<string>;
	valid?: Array<string>;
	filename: string;
};

type Tests = Dict<Test | Array<Test>>;

/* GENERATED:START(hash:5d20bc54b36105f96b3ca80f5befae6a982ecb45,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/ast` to update. */
// @ts-ignore
import preferClosingNonVoid from "./html/preferClosingNonVoid.test.rjson";
// @ts-ignore
import caseSingleStatement from "./js/caseSingleStatement.test.rjson";
// @ts-ignore
import confusingLanguage from "./js/confusingLanguage.test.rjson";
// @ts-ignore
import defaultExportSameBasename from "./js/defaultExportSameBasename.test.rjson";
// @ts-ignore
import doubleEquals from "./js/doubleEquals.test.rjson";
// @ts-ignore
import duplicateImportSource from "./js/duplicateImportSource.test.rjson";
// @ts-ignore
import emptyBlocks from "./js/emptyBlocks.test.rjson";
// @ts-ignore
import emptyMatches from "./js/emptyMatches.test.rjson";
// @ts-ignore
import getterReturn from "./js/getterReturn.test.rjson";
// @ts-ignore
import importDefaultBasename from "./js/importDefaultBasename.test.rjson";
// @ts-ignore
import negationElse from "./js/negationElse.test.rjson";
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
import noDupeArgs from "./js/noDupeArgs.test.rjson";
// @ts-ignore
import noDuplicateCase from "./js/noDuplicateCase.test.rjson";
// @ts-ignore
import noDuplicateGroupNamesInRegularExpressions from "./js/noDuplicateGroupNamesInRegularExpressions.test.rjson";
// @ts-ignore
import noDuplicateKeys from "./js/noDuplicateKeys.test.rjson";
// @ts-ignore
import noEmptyCharacterClass from "./js/noEmptyCharacterClass.test.rjson";
// @ts-ignore
import noExtraBooleanCast from "./js/noExtraBooleanCast.test.rjson";
// @ts-ignore
import noFunctionAssign from "./js/noFunctionAssign.test.rjson";
// @ts-ignore
import noImportAssign from "./js/noImportAssign.test.rjson";
// @ts-ignore
import noLabelVar from "./js/noLabelVar.test.rjson";
// @ts-ignore
import noMultipleSpacesInRegularExpressionLiterals from "./js/noMultipleSpacesInRegularExpressionLiterals.test.rjson";
// @ts-ignore
import noNestedTernary from "./js/noNestedTernary.test.rjson";
// @ts-ignore
import noPosixInRegularExpression from "./js/noPosixInRegularExpression.test.rjson";
// @ts-ignore
import noSetterReturn from "./js/noSetterReturn.test.rjson";
// @ts-ignore
import noShadowRestrictedNames from "./js/noShadowRestrictedNames.test.rjson";
// @ts-ignore
import noShorthandArrayType from "./js/noShorthandArrayType.test.rjson";
// @ts-ignore
import noTemplateCurlyInString from "./js/noTemplateCurlyInString.test.rjson";
// @ts-ignore
import noUnsafeFinally from "./js/noUnsafeFinally.test.rjson";
// @ts-ignore
import noUnusedTemplateLiteral from "./js/noUnusedTemplateLiteral.test.rjson";
// @ts-ignore
import noUnusedVariables from "./js/noUnusedVariables.test.rjson";
// @ts-ignore
import noVar from "./js/noVar.test.rjson";
// @ts-ignore
import preferBlockStatements from "./js/preferBlockStatements.test.rjson";
// @ts-ignore
import preferFunctionDeclarations from "./js/preferFunctionDeclarations.test.rjson";
// @ts-ignore
import preferTemplate from "./js/preferTemplate.test.rjson";
// @ts-ignore
import preferWhile from "./js/preferWhile.test.rjson";
// @ts-ignore
import restrictedGlobals from "./js/restrictedGlobals.test.rjson";
// @ts-ignore
import shoutyConstants from "./js/shoutyConstants.test.rjson";
// @ts-ignore
import singleVarDeclarator from "./js/singleVarDeclarator.test.rjson";
// @ts-ignore
import sortImportExportSpecifiers from "./js/sortImportExportSpecifiers.test.rjson";
// @ts-ignore
import sparseArray from "./js/sparseArray.test.rjson";
// @ts-ignore
import unsafeNegation from "./js/unsafeNegation.test.rjson";
// @ts-ignore
import altText from "./jsx-a11y/altText.test.rjson";
// @ts-ignore
import anchorHasContent from "./jsx-a11y/anchorHasContent.test.rjson";
// @ts-ignore
import anchorIsValid from "./jsx-a11y/anchorIsValid.test.rjson";
// @ts-ignore
import ariaProps from "./jsx-a11y/ariaProps.test.rjson";
// @ts-ignore
import ariaProptypes from "./jsx-a11y/ariaProptypes.test.rjson";
// @ts-ignore
import ariaUnsupportedElements from "./jsx-a11y/ariaUnsupportedElements.test.rjson";
// @ts-ignore
import clickEventsHaveKeyEvents from "./jsx-a11y/clickEventsHaveKeyEvents.test.rjson";
// @ts-ignore
import headingHasContent from "./jsx-a11y/headingHasContent.test.rjson";
// @ts-ignore
import htmlHasLang from "./jsx-a11y/htmlHasLang.test.rjson";
// @ts-ignore
import iframeHasTitle from "./jsx-a11y/iframeHasTitle.test.rjson";
// @ts-ignore
import imgRedundantAlt from "./jsx-a11y/imgRedundantAlt.test.rjson";
// @ts-ignore
import lang from "./jsx-a11y/lang.test.rjson";
// @ts-ignore
import mediaHasCaption from "./jsx-a11y/mediaHasCaption.test.rjson";
// @ts-ignore
import mouseEventsHaveKeyEvents from "./jsx-a11y/mouseEventsHaveKeyEvents.test.rjson";
// @ts-ignore
import noAccessKey from "./jsx-a11y/noAccessKey.test.rjson";
// @ts-ignore
import noAutofocus from "./jsx-a11y/noAutofocus.test.rjson";
// @ts-ignore
import noDistractingElements from "./jsx-a11y/noDistractingElements.test.rjson";
// @ts-ignore
import noNoninteractiveElementToInteractiveRole from "./jsx-a11y/noNoninteractiveElementToInteractiveRole.test.rjson";
// @ts-ignore
import noNoninteractiveTabindex from "./jsx-a11y/noNoninteractiveTabindex.test.rjson";
// @ts-ignore
import noOnChange from "./jsx-a11y/noOnChange.test.rjson";
// @ts-ignore
import noRedundantRoles from "./jsx-a11y/noRedundantRoles.test.rjson";
// @ts-ignore
import noTargetBlank from "./jsx-a11y/noTargetBlank.test.rjson";
// @ts-ignore
import roleHasRequiredAriaProps from "./jsx-a11y/roleHasRequiredAriaProps.test.rjson";
// @ts-ignore
import scope from "./jsx-a11y/scope.test.rjson";
// @ts-ignore
import tabindexNoPositive from "./jsx-a11y/tabindexNoPositive.test.rjson";
// @ts-ignore
import noCommentText from "./jsx/noCommentText.test.rjson";
// @ts-ignore
import noDuplicateProps from "./jsx/noDuplicateProps.test.rjson";
// @ts-ignore
import noImplicitBoolean from "./jsx/noImplicitBoolean.test.rjson";
// @ts-ignore
import pascalCase from "./jsx/pascalCase.test.rjson";
// @ts-ignore
import preferSelfClosingElements from "./jsx/preferSelfClosingElements.test.rjson";
// @ts-ignore
import propsNoSpreading from "./jsx/propsNoSpreading.test.rjson";
// @ts-ignore
import buttonHasType from "./react/buttonHasType.test.rjson";
// @ts-ignore
import jsxFragments from "./react/jsxFragments.test.rjson";
// @ts-ignore
import jsxKey from "./react/jsxKey.test.rjson";
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
import noWillUpdateSetState from "./react/noWillUpdateSetState.test.rjson";
// @ts-ignore
import requireRenderReturn from "./react/requireRenderReturn.test.rjson";
// @ts-ignore
import sortComp from "./react/sortComp.test.rjson";
// @ts-ignore
import stylePropObject from "./react/stylePropObject.test.rjson";
// @ts-ignore
import voidDomElementsNoChildren from "./react/voidDomElementsNoChildren.test.rjson";

export const tests: Tests = {
	"html/preferClosingNonVoid": preferClosingNonVoid,
	"js/caseSingleStatement": caseSingleStatement,
	"js/confusingLanguage": confusingLanguage,
	"js/defaultExportSameBasename": defaultExportSameBasename,
	"js/doubleEquals": doubleEquals,
	"js/duplicateImportSource": duplicateImportSource,
	"js/emptyBlocks": emptyBlocks,
	"js/emptyMatches": emptyMatches,
	"js/getterReturn": getterReturn,
	"js/importDefaultBasename": importDefaultBasename,
	"js/negationElse": negationElse,
	"js/noArguments": noArguments,
	"js/noAsyncPromiseExecutor": noAsyncPromiseExecutor,
	"js/noCatchAssign": noCatchAssign,
	"js/noCommaOperator": noCommaOperator,
	"js/noCompareNegZero": noCompareNegZero,
	"js/noCondAssign": noCondAssign,
	"js/noDebugger": noDebugger,
	"js/noDelete": noDelete,
	"js/noDeleteVars": noDeleteVars,
	"js/noDupeArgs": noDupeArgs,
	"js/noDuplicateCase": noDuplicateCase,
	"js/noDuplicateGroupNamesInRegularExpressions": noDuplicateGroupNamesInRegularExpressions,
	"js/noDuplicateKeys": noDuplicateKeys,
	"js/noEmptyCharacterClass": noEmptyCharacterClass,
	"js/noExtraBooleanCast": noExtraBooleanCast,
	"js/noFunctionAssign": noFunctionAssign,
	"js/noImportAssign": noImportAssign,
	"js/noLabelVar": noLabelVar,
	"js/noMultipleSpacesInRegularExpressionLiterals": noMultipleSpacesInRegularExpressionLiterals,
	"js/noNestedTernary": noNestedTernary,
	"js/noPosixInRegularExpression": noPosixInRegularExpression,
	"js/noSetterReturn": noSetterReturn,
	"js/noShadowRestrictedNames": noShadowRestrictedNames,
	"js/noShorthandArrayType": noShorthandArrayType,
	"js/noTemplateCurlyInString": noTemplateCurlyInString,
	"js/noUnsafeFinally": noUnsafeFinally,
	"js/noUnusedTemplateLiteral": noUnusedTemplateLiteral,
	"js/noUnusedVariables": noUnusedVariables,
	"js/noVar": noVar,
	"js/preferBlockStatements": preferBlockStatements,
	"js/preferFunctionDeclarations": preferFunctionDeclarations,
	"js/preferTemplate": preferTemplate,
	"js/preferWhile": preferWhile,
	"js/restrictedGlobals": restrictedGlobals,
	"js/shoutyConstants": shoutyConstants,
	"js/singleVarDeclarator": singleVarDeclarator,
	"js/sortImportExportSpecifiers": sortImportExportSpecifiers,
	"js/sparseArray": sparseArray,
	"js/unsafeNegation": unsafeNegation,
	"jsx-a11y/altText": altText,
	"jsx-a11y/anchorHasContent": anchorHasContent,
	"jsx-a11y/anchorIsValid": anchorIsValid,
	"jsx-a11y/ariaProps": ariaProps,
	"jsx-a11y/ariaProptypes": ariaProptypes,
	"jsx-a11y/ariaUnsupportedElements": ariaUnsupportedElements,
	"jsx-a11y/clickEventsHaveKeyEvents": clickEventsHaveKeyEvents,
	"jsx-a11y/headingHasContent": headingHasContent,
	"jsx-a11y/htmlHasLang": htmlHasLang,
	"jsx-a11y/iframeHasTitle": iframeHasTitle,
	"jsx-a11y/imgRedundantAlt": imgRedundantAlt,
	"jsx-a11y/lang": lang,
	"jsx-a11y/mediaHasCaption": mediaHasCaption,
	"jsx-a11y/mouseEventsHaveKeyEvents": mouseEventsHaveKeyEvents,
	"jsx-a11y/noAccessKey": noAccessKey,
	"jsx-a11y/noAutofocus": noAutofocus,
	"jsx-a11y/noDistractingElements": noDistractingElements,
	"jsx-a11y/noNoninteractiveElementToInteractiveRole": noNoninteractiveElementToInteractiveRole,
	"jsx-a11y/noNoninteractiveTabindex": noNoninteractiveTabindex,
	"jsx-a11y/noOnChange": noOnChange,
	"jsx-a11y/noRedundantRoles": noRedundantRoles,
	"jsx-a11y/noTargetBlank": noTargetBlank,
	"jsx-a11y/roleHasRequiredAriaProps": roleHasRequiredAriaProps,
	"jsx-a11y/scope": scope,
	"jsx-a11y/tabindexNoPositive": tabindexNoPositive,
	"jsx/noCommentText": noCommentText,
	"jsx/noDuplicateProps": noDuplicateProps,
	"jsx/noImplicitBoolean": noImplicitBoolean,
	"jsx/pascalCase": pascalCase,
	"jsx/preferSelfClosingElements": preferSelfClosingElements,
	"jsx/propsNoSpreading": propsNoSpreading,
	"react/buttonHasType": buttonHasType,
	"react/jsxFragments": jsxFragments,
	"react/jsxKey": jsxKey,
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
	"react/noWillUpdateSetState": noWillUpdateSetState,
	"react/requireRenderReturn": requireRenderReturn,
	"react/sortComp": sortComp,
	"react/stylePropObject": stylePropObject,
	"react/voidDomElementsNoChildren": voidDomElementsNoChildren,
};
/* GENERATED:END(id:main) */
