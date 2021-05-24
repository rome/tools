import {DiagnosticLintCategory} from "@internal/diagnostics";
import {dedent} from "@internal/string-utils";
import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<{
	category: DiagnosticLintCategory;
	cases: Test[];
}>;

function normalizeCase({invalid, valid, filename}: Test): Test {
	if (valid) {
		valid = valid.map((str) => dedent(str));
	}

	if (invalid) {
		invalid = invalid.map((str) => dedent(str));
	}

	return {filename, invalid, valid};
}

function normalizeCases(
	raw:
		| Test
		| {
				cases: Test[];
			},
): Test[] {
	if ("cases" in raw) {
		return raw.cases.map((test) => normalizeCase(test));
	} else {
		return [normalizeCase(raw)];
	}
}

/* GENERATED:START(hash:45e6cd4fe20c6505a73fa55a851cea0804a5b84e,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
// @ts-ignore
import noAccessKey from "./a11y/noAccessKey.test.toml";
// @ts-ignore
import noAutofocus from "./a11y/noAutofocus.test.toml";
// @ts-ignore
import noHeaderScope from "./a11y/noHeaderScope.test.toml";
// @ts-ignore
import noOnChange from "./a11y/noOnChange.test.toml";
// @ts-ignore
import noPositiveTabindex from "./a11y/noPositiveTabindex.test.toml";
// @ts-ignore
import noRedundantAlt from "./a11y/noRedundantAlt.test.toml";
// @ts-ignore
import noRedundantRoles from "./a11y/noRedundantRoles.test.toml";
// @ts-ignore
import noTargetBlank from "./a11y/noTargetBlank.test.toml";
// @ts-ignore
import useAnchorContent from "./a11y/useAnchorContent.test.toml";
// @ts-ignore
import useAriaProps from "./a11y/useAriaProps.test.toml";
// @ts-ignore
import useAriaPropsForRole from "./a11y/useAriaPropsForRole.test.toml";
// @ts-ignore
import useHeadingContent from "./a11y/useHeadingContent.test.toml";
// @ts-ignore
import useKeyWithClickEvents from "./a11y/useKeyWithClickEvents.test.toml";
// @ts-ignore
import useKeyWithMouseEvents from "./a11y/useKeyWithMouseEvents.test.toml";
// @ts-ignore
import useValidAnchor from "./a11y/useValidAnchor.test.toml";
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
	"a11y/noAccessKey": {
		category: ["lint", "a11y", "noAccessKey"],
		cases: normalizeCases(noAccessKey),
	},
	"a11y/noAutofocus": {
		category: ["lint", "a11y", "noAutofocus"],
		cases: normalizeCases(noAutofocus),
	},
	"a11y/noHeaderScope": {
		category: ["lint", "a11y", "noHeaderScope"],
		cases: normalizeCases(noHeaderScope),
	},
	"a11y/noOnChange": {
		category: ["lint", "a11y", "noOnChange"],
		cases: normalizeCases(noOnChange),
	},
	"a11y/noPositiveTabindex": {
		category: ["lint", "a11y", "noPositiveTabindex"],
		cases: normalizeCases(noPositiveTabindex),
	},
	"a11y/noRedundantAlt": {
		category: ["lint", "a11y", "noRedundantAlt"],
		cases: normalizeCases(noRedundantAlt),
	},
	"a11y/noRedundantRoles": {
		category: ["lint", "a11y", "noRedundantRoles"],
		cases: normalizeCases(noRedundantRoles),
	},
	"a11y/noTargetBlank": {
		category: ["lint", "a11y", "noTargetBlank"],
		cases: normalizeCases(noTargetBlank),
	},
	"a11y/useAnchorContent": {
		category: ["lint", "a11y", "useAnchorContent"],
		cases: normalizeCases(useAnchorContent),
	},
	"a11y/useAriaProps": {
		category: ["lint", "a11y", "useAriaProps"],
		cases: normalizeCases(useAriaProps),
	},
	"a11y/useAriaPropsForRole": {
		category: ["lint", "a11y", "useAriaPropsForRole"],
		cases: normalizeCases(useAriaPropsForRole),
	},
	"a11y/useHeadingContent": {
		category: ["lint", "a11y", "useHeadingContent"],
		cases: normalizeCases(useHeadingContent),
	},
	"a11y/useKeyWithClickEvents": {
		category: ["lint", "a11y", "useKeyWithClickEvents"],
		cases: normalizeCases(useKeyWithClickEvents),
	},
	"a11y/useKeyWithMouseEvents": {
		category: ["lint", "a11y", "useKeyWithMouseEvents"],
		cases: normalizeCases(useKeyWithMouseEvents),
	},
	"a11y/useValidAnchor": {
		category: ["lint", "a11y", "useValidAnchor"],
		cases: normalizeCases(useValidAnchor),
	},
	"html/useClosingNonVoid": {
		category: ["lint", "html", "useClosingNonVoid"],
		cases: normalizeCases(useClosingNonVoid),
	},
	"js/noArguments": {
		category: ["lint", "js", "noArguments"],
		cases: normalizeCases(noArguments),
	},
	"js/noAsyncPromiseExecutor": {
		category: ["lint", "js", "noAsyncPromiseExecutor"],
		cases: normalizeCases(noAsyncPromiseExecutor),
	},
	"js/noCatchAssign": {
		category: ["lint", "js", "noCatchAssign"],
		cases: normalizeCases(noCatchAssign),
	},
	"js/noCommaOperator": {
		category: ["lint", "js", "noCommaOperator"],
		cases: normalizeCases(noCommaOperator),
	},
	"js/noCompareNegZero": {
		category: ["lint", "js", "noCompareNegZero"],
		cases: normalizeCases(noCompareNegZero),
	},
	"js/noCondAssign": {
		category: ["lint", "js", "noCondAssign"],
		cases: normalizeCases(noCondAssign),
	},
	"js/noDebugger": {
		category: ["lint", "js", "noDebugger"],
		cases: normalizeCases(noDebugger),
	},
	"js/noDelete": {
		category: ["lint", "js", "noDelete"],
		cases: normalizeCases(noDelete),
	},
	"js/noDeleteVars": {
		category: ["lint", "js", "noDeleteVars"],
		cases: normalizeCases(noDeleteVars),
	},
	"js/noDoubleEquals": {
		category: ["lint", "js", "noDoubleEquals"],
		cases: normalizeCases(noDoubleEquals),
	},
	"js/noDupeArgs": {
		category: ["lint", "js", "noDupeArgs"],
		cases: normalizeCases(noDupeArgs),
	},
	"js/noDuplicateCase": {
		category: ["lint", "js", "noDuplicateCase"],
		cases: normalizeCases(noDuplicateCase),
	},
	"js/noDuplicateImportSource": {
		category: ["lint", "js", "noDuplicateImportSource"],
		cases: normalizeCases(noDuplicateImportSource),
	},
	"js/noDuplicateKeys": {
		category: ["lint", "js", "noDuplicateKeys"],
		cases: normalizeCases(noDuplicateKeys),
	},
	"js/noEmptyBlocks": {
		category: ["lint", "js", "noEmptyBlocks"],
		cases: normalizeCases(noEmptyBlocks),
	},
	"js/noExtraBooleanCast": {
		category: ["lint", "js", "noExtraBooleanCast"],
		cases: normalizeCases(noExtraBooleanCast),
	},
	"js/noFunctionAssign": {
		category: ["lint", "js", "noFunctionAssign"],
		cases: normalizeCases(noFunctionAssign),
	},
	"js/noGetterReturn": {
		category: ["lint", "js", "noGetterReturn"],
		cases: normalizeCases(noGetterReturn),
	},
	"js/noImportAssign": {
		category: ["lint", "js", "noImportAssign"],
		cases: normalizeCases(noImportAssign),
	},
	"js/noLabelVar": {
		category: ["lint", "js", "noLabelVar"],
		cases: normalizeCases(noLabelVar),
	},
	"js/noNegationElse": {
		category: ["lint", "js", "noNegationElse"],
		cases: normalizeCases(noNegationElse),
	},
	"js/noNestedTernary": {
		category: ["lint", "js", "noNestedTernary"],
		cases: normalizeCases(noNestedTernary),
	},
	"js/noRestrictedGlobals": {
		category: ["lint", "js", "noRestrictedGlobals"],
		cases: normalizeCases(noRestrictedGlobals),
	},
	"js/noSetterReturn": {
		category: ["lint", "js", "noSetterReturn"],
		cases: normalizeCases(noSetterReturn),
	},
	"js/noShadowRestrictedNames": {
		category: ["lint", "js", "noShadowRestrictedNames"],
		cases: normalizeCases(noShadowRestrictedNames),
	},
	"js/noShoutyConstants": {
		category: ["lint", "js", "noShoutyConstants"],
		cases: normalizeCases(noShoutyConstants),
	},
	"js/noSingleCharRegexAlternatives": {
		category: ["lint", "js", "noSingleCharRegexAlternatives"],
		cases: normalizeCases(noSingleCharRegexAlternatives),
	},
	"js/noSparseArray": {
		category: ["lint", "js", "noSparseArray"],
		cases: normalizeCases(noSparseArray),
	},
	"js/noTemplateCurlyInString": {
		category: ["lint", "js", "noTemplateCurlyInString"],
		cases: normalizeCases(noTemplateCurlyInString),
	},
	"js/noUndeclaredVariables": {
		category: ["lint", "js", "noUndeclaredVariables"],
		cases: normalizeCases(noUndeclaredVariables),
	},
	"js/noUnnecessaryContinue": {
		category: ["lint", "js", "noUnnecessaryContinue"],
		cases: normalizeCases(noUnnecessaryContinue),
	},
	"js/noUnsafeFinally": {
		category: ["lint", "js", "noUnsafeFinally"],
		cases: normalizeCases(noUnsafeFinally),
	},
	"js/noUnsafeNegation": {
		category: ["lint", "js", "noUnsafeNegation"],
		cases: normalizeCases(noUnsafeNegation),
	},
	"js/noUnusedTemplateLiteral": {
		category: ["lint", "js", "noUnusedTemplateLiteral"],
		cases: normalizeCases(noUnusedTemplateLiteral),
	},
	"js/noUnusedVariables": {
		category: ["lint", "js", "noUnusedVariables"],
		cases: normalizeCases(noUnusedVariables),
	},
	"js/noVar": {
		category: ["lint", "js", "noVar"],
		cases: normalizeCases(noVar),
	},
	"js/preferOptionalChaining": {
		category: ["lint", "js", "preferOptionalChaining"],
		cases: normalizeCases(preferOptionalChaining),
	},
	"js/useBlockStatements": {
		category: ["lint", "js", "useBlockStatements"],
		cases: normalizeCases(useBlockStatements),
	},
	"js/useDefaultExportBasename": {
		category: ["lint", "js", "useDefaultExportBasename"],
		cases: normalizeCases(useDefaultExportBasename),
	},
	"js/useDefaultImportBasename": {
		category: ["lint", "js", "useDefaultImportBasename"],
		cases: normalizeCases(useDefaultImportBasename),
	},
	"js/useFunctionDeclarations": {
		category: ["lint", "js", "useFunctionDeclarations"],
		cases: normalizeCases(useFunctionDeclarations),
	},
	"js/useSimplifiedLogicalExpression": {
		category: ["lint", "js", "useSimplifiedLogicalExpression"],
		cases: normalizeCases(useSimplifiedLogicalExpression),
	},
	"js/useSingleCaseStatement": {
		category: ["lint", "js", "useSingleCaseStatement"],
		cases: normalizeCases(useSingleCaseStatement),
	},
	"js/useSingleVarDeclarator": {
		category: ["lint", "js", "useSingleVarDeclarator"],
		cases: normalizeCases(useSingleVarDeclarator),
	},
	"js/useSortedSpecifiers": {
		category: ["lint", "js", "useSortedSpecifiers"],
		cases: normalizeCases(useSortedSpecifiers),
	},
	"js/useTemplate": {
		category: ["lint", "js", "useTemplate"],
		cases: normalizeCases(useTemplate),
	},
	"js/useWhile": {
		category: ["lint", "js", "useWhile"],
		cases: normalizeCases(useWhile),
	},
	"jsx/noCommentText": {
		category: ["lint", "jsx", "noCommentText"],
		cases: normalizeCases(noCommentText),
	},
	"jsx/noDuplicateProps": {
		category: ["lint", "jsx", "noDuplicateProps"],
		cases: normalizeCases(noDuplicateProps),
	},
	"jsx/noImplicitBoolean": {
		category: ["lint", "jsx", "noImplicitBoolean"],
		cases: normalizeCases(noImplicitBoolean),
	},
	"jsx/noPropSpreading": {
		category: ["lint", "jsx", "noPropSpreading"],
		cases: normalizeCases(noPropSpreading),
	},
	"jsx/useJSXFileExtension": {
		category: ["lint", "jsx", "useJSXFileExtension"],
		cases: normalizeCases(useJSXFileExtension),
	},
	"jsx/usePascalCase": {
		category: ["lint", "jsx", "usePascalCase"],
		cases: normalizeCases(usePascalCase),
	},
	"jsx/useSelfClosingElements": {
		category: ["lint", "jsx", "useSelfClosingElements"],
		cases: normalizeCases(useSelfClosingElements),
	},
	"react/noAccessStateInSetState": {
		category: ["lint", "react", "noAccessStateInSetState"],
		cases: normalizeCases(noAccessStateInSetState),
	},
	"react/noArrayIndexKey": {
		category: ["lint", "react", "noArrayIndexKey"],
		cases: normalizeCases(noArrayIndexKey),
	},
	"react/noChildrenProp": {
		category: ["lint", "react", "noChildrenProp"],
		cases: normalizeCases(noChildrenProp),
	},
	"react/noDanger": {
		category: ["lint", "react", "noDanger"],
		cases: normalizeCases(noDanger),
	},
	"react/noDangerWithChildren": {
		category: ["lint", "react", "noDangerWithChildren"],
		cases: normalizeCases(noDangerWithChildren),
	},
	"react/noDidMountSetState": {
		category: ["lint", "react", "noDidMountSetState"],
		cases: normalizeCases(noDidMountSetState),
	},
	"react/noDidUpdateSetState": {
		category: ["lint", "react", "noDidUpdateSetState"],
		cases: normalizeCases(noDidUpdateSetState),
	},
	"react/noDirectMutationState": {
		category: ["lint", "react", "noDirectMutationState"],
		cases: normalizeCases(noDirectMutationState),
	},
	"react/noFindDOMNode": {
		category: ["lint", "react", "noFindDOMNode"],
		cases: normalizeCases(noFindDOMNode),
	},
	"react/noRedundantShouldComponentUpdate": {
		category: ["lint", "react", "noRedundantShouldComponentUpdate"],
		cases: normalizeCases(noRedundantShouldComponentUpdate),
	},
	"react/noRenderReturnValue": {
		category: ["lint", "react", "noRenderReturnValue"],
		cases: normalizeCases(noRenderReturnValue),
	},
	"react/noStringRefs": {
		category: ["lint", "react", "noStringRefs"],
		cases: normalizeCases(noStringRefs),
	},
	"react/noThisInSFC": {
		category: ["lint", "react", "noThisInSFC"],
		cases: normalizeCases(noThisInSFC),
	},
	"react/noUnsafe": {
		category: ["lint", "react", "noUnsafe"],
		cases: normalizeCases(noUnsafe),
	},
	"react/noUselessFragment": {
		category: ["lint", "react", "noUselessFragment"],
		cases: normalizeCases(noUselessFragment),
	},
	"react/noVoidElementsWithChildren": {
		category: ["lint", "react", "noVoidElementsWithChildren"],
		cases: normalizeCases(noVoidElementsWithChildren),
	},
	"react/noWillUpdateSetState": {
		category: ["lint", "react", "noWillUpdateSetState"],
		cases: normalizeCases(noWillUpdateSetState),
	},
	"react/useButtonType": {
		category: ["lint", "react", "useButtonType"],
		cases: normalizeCases(useButtonType),
	},
	"react/useFragmentSyntax": {
		category: ["lint", "react", "useFragmentSyntax"],
		cases: normalizeCases(useFragmentSyntax),
	},
	"react/useKey": {
		category: ["lint", "react", "useKey"],
		cases: normalizeCases(useKey),
	},
	"react/useRenderReturn": {
		category: ["lint", "react", "useRenderReturn"],
		cases: normalizeCases(useRenderReturn),
	},
	"react/useSortComp": {
		category: ["lint", "react", "useSortComp"],
		cases: normalizeCases(useSortComp),
	},
	"react/useStylePropObject": {
		category: ["lint", "react", "useStylePropObject"],
		cases: normalizeCases(useStylePropObject),
	},
	"regex/noDuplicateGroupNamesInRegularExpressions": {
		category: ["lint", "regex", "noDuplicateGroupNamesInRegularExpressions"],
		cases: normalizeCases(noDuplicateGroupNamesInRegularExpressions),
	},
	"regex/noEmptyCharacterClass": {
		category: ["lint", "regex", "noEmptyCharacterClass"],
		cases: normalizeCases(noEmptyCharacterClass),
	},
	"regex/noEmptyMatches": {
		category: ["lint", "regex", "noEmptyMatches"],
		cases: normalizeCases(noEmptyMatches),
	},
	"regex/noMultipleSpacesInRegularExpressionLiterals": {
		category: ["lint", "regex", "noMultipleSpacesInRegularExpressionLiterals"],
		cases: normalizeCases(noMultipleSpacesInRegularExpressionLiterals),
	},
	"regex/noPosixInRegularExpression": {
		category: ["lint", "regex", "noPosixInRegularExpression"],
		cases: normalizeCases(noPosixInRegularExpression),
	},
	"ts/preferShorthandArrayType": {
		category: ["lint", "ts", "preferShorthandArrayType"],
		cases: normalizeCases(preferShorthandArrayType),
	},
};
/* GENERATED:END(id:main) */
