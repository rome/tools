import {DiagnosticCategory} from "@internal/diagnostics";
import {dedent} from "@internal/string-utils";
import {Dict} from "@internal/typescript-helpers";

type Test = {
	invalid?: string[];
	valid?: string[];
	filename: string;
};

type Tests = Dict<{
	category: DiagnosticCategory;
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
};
/* GENERATED:END(id:main) */
