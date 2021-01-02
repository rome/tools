/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// Rather than having a generic `string` type, we use string literals so there's one place where
// all category names are defined. This allows the naming scheme to be more easily reviewed and
// made consistent.
export type DiagnosticCategory =
	| DiagnosticLintCategory
	| "analyzeDependencies/cjsExportInES"
	| "args/fileNotFound"
	| "args/invalid"
	| "bundler/moduleCycle"
	| "bundler/topLevelAwait"
	| "childProcess/failure"
	| "commands/init/uncommittedChanges"
	| "commands/init/expectedRepo"
	| "compile/classes"
	| "compile/const-enums"
	| "compile/jsx"
	| "compile/nonnumeric-enum-values"
	| "flags/invalid"
	| "format/disabled"
	| "internalError/fatal"
	| "internalError/fs"
	| "internalError/httpServer"
	| "internalError/request"
	| "lint/disabled"
	| "lint/pendingFixes"
	| "parse"
	| "projectManager/sensitiveDirectory"
	| "projectManager/multipleConfigFiles"
	| "projectManager/typoConfigFilename"
	| "projectManager/misplacedConfig"
	| "projectManager/missing"
	| "projectManager/nameCollision"
	| "projectManager/vscMissing"
	| "recoveryStore/diff"
	| "recoveryStore/notFound"
	| "resolver/fetchFailed"
	| "resolver/importTypeMismatch"
	| "resolver/notFound"
	| "resolver/unknownExport"
	| "resolver/unsupported"
	| "suppressions/duplicate"
	| "suppressions/empty"
	| "suppressions/overlap"
	| "suppressions/incorrectSuppressionStart"
	| "suppressions/incorrectPrefix"
	| "suppressions/invalidCategory"
	| "suppressions/missingSpace"
	| "suppressions/missingTarget"
	| "suppressions/missingExplanation"
	| "suppressions/unused"
	| "tests/cancelled"
	| "tests/disabled"
	| "tests/failure"
	| "tests/fixtureOptions"
	| "tests/logs"
	| "tests/noneDeclared"
	| "tests/snapshots/frozen"
	| "tests/snapshots/inlineMissingReceived"
	| "tests/snapshots/inlineCollision"
	| "tests/snapshots/incorrect"
	| "tests/snapshots/missing"
	| "tests/snapshots/redundant"
	| "tests/timeout"
	| "tests/unhandledRejection"
	| "typeCheck/incompatible"
	| "typeCheck/missingCondition"
	| "typeCheck/notExhaustive"
	| "typeCheck/uncallable"
	| "typeCheck/undeclaredVariable"
	| "typeCheck/unknownImport"
	| "typeCheck/unknownProperty"
	| "vsc/dirty"
	| "v8/syntaxError";

export type DiagnosticCategoryPrefix =
	| "analyzeDependencies"
	| "args"
	| "bundler"
	| "compiler"
	| "flags"
	| "internalError"
	| "lint"
	| "lsp"
	| "projectManager"
	| "resolver"
	| "tests"
	| "typeCheck"
	| "v8";

/* GENERATED:START(hash:1505e392d2d9b5fef8382f095b9af6c7440aa99c,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
export type DiagnosticLintCategory =
	| "lint/html/useClosingNonVoid"
	| "lint/html/useHtmlLang"
	| "lint/html/useValidLang"
	| "lint/js/noArguments"
	| "lint/js/noAsyncPromiseExecutor"
	| "lint/js/noCatchAssign"
	| "lint/js/noCommaOperator"
	| "lint/js/noCompareNegZero"
	| "lint/js/noCondAssign"
	| "lint/js/noDebugger"
	| "lint/js/noDelete"
	| "lint/js/noDeleteVars"
	| "lint/js/noDoubleEquals"
	| "lint/js/noDupeArgs"
	| "lint/js/noDuplicateCase"
	| "lint/js/noDuplicateImportSource"
	| "lint/js/noDuplicateKeys"
	| "lint/js/noEmptyBlocks"
	| "lint/js/noExtraBooleanCast"
	| "lint/js/noFunctionAssign"
	| "lint/js/noGetterReturn"
	| "lint/js/noImportAssign"
	| "lint/js/noLabelVar"
	| "lint/js/noNegationElse"
	| "lint/js/noNestedTernary"
	| "lint/js/noRestrictedGlobals"
	| "lint/js/noSetterReturn"
	| "lint/js/noShadowRestrictedNames"
	| "lint/js/noShoutyConstants"
	| "lint/js/noSingleCharRegexAlternatives"
	| "lint/js/noSparseArray"
	| "lint/js/noTemplateCurlyInString"
	| "lint/js/noUndeclaredVariables"
	| "lint/js/noUnnecessaryContinue"
	| "lint/js/noUnsafeFinally"
	| "lint/js/noUnsafeNegation"
	| "lint/js/noUnusedTemplateLiteral"
	| "lint/js/noUnusedVariables"
	| "lint/js/noVar"
	| "lint/js/preferOptionalChaining"
	| "lint/js/useBlockStatements"
	| "lint/js/useCamelCase"
	| "lint/js/useDefaultExportBasename"
	| "lint/js/useDefaultImportBasename"
	| "lint/js/useFunctionDeclarations"
	| "lint/js/useSingleCaseStatement"
	| "lint/js/useSingleVarDeclarator"
	| "lint/js/useSortedSpecifiers"
	| "lint/js/useTemplate"
	| "lint/js/useWhile"
	| "lint/jsx-a11y/noAccessKey"
	| "lint/jsx-a11y/noAriaUnsupportedElements"
	| "lint/jsx-a11y/noAutofocus"
	| "lint/jsx-a11y/noDistractingElements"
	| "lint/jsx-a11y/noHeaderScope"
	| "lint/jsx-a11y/noNoninteractiveElementToInteractiveRole"
	| "lint/jsx-a11y/noNoninteractiveTabindex"
	| "lint/jsx-a11y/noOnChange"
	| "lint/jsx-a11y/noPositiveTabindex"
	| "lint/jsx-a11y/noRedundantAlt"
	| "lint/jsx-a11y/noRedundantRoles"
	| "lint/jsx-a11y/noTargetBlank"
	| "lint/jsx-a11y/useAltText"
	| "lint/jsx-a11y/useAnchorContent"
	| "lint/jsx-a11y/useAriaProps"
	| "lint/jsx-a11y/useAriaPropsForRole"
	| "lint/jsx-a11y/useAriaProptypes"
	| "lint/jsx-a11y/useHeadingContent"
	| "lint/jsx-a11y/useHtmlLang"
	| "lint/jsx-a11y/useIframeTitle"
	| "lint/jsx-a11y/useKeyWithClickEvents"
	| "lint/jsx-a11y/useKeyWithMouseEvents"
	| "lint/jsx-a11y/useMediaCaption"
	| "lint/jsx-a11y/useValidAnchor"
	| "lint/jsx-a11y/useValidLang"
	| "lint/jsx/noCommentText"
	| "lint/jsx/noDuplicateProps"
	| "lint/jsx/noImplicitBoolean"
	| "lint/jsx/noPropSpreading"
	| "lint/jsx/useJSXFileExtension"
	| "lint/jsx/usePascalCase"
	| "lint/jsx/useSelfClosingElements"
	| "lint/react/noAccessStateInSetState"
	| "lint/react/noArrayIndexKey"
	| "lint/react/noChildrenProp"
	| "lint/react/noDanger"
	| "lint/react/noDangerWithChildren"
	| "lint/react/noDidMountSetState"
	| "lint/react/noDidUpdateSetState"
	| "lint/react/noDirectMutationState"
	| "lint/react/noFindDOMNode"
	| "lint/react/noRedundantShouldComponentUpdate"
	| "lint/react/noRenderReturnValue"
	| "lint/react/noStringRefs"
	| "lint/react/noThisInSFC"
	| "lint/react/noUnsafe"
	| "lint/react/noUselessFragment"
	| "lint/react/noVoidElementsWithChildren"
	| "lint/react/noWillUpdateSetState"
	| "lint/react/useButtonType"
	| "lint/react/useFragmentSyntax"
	| "lint/react/useKey"
	| "lint/react/useRenderReturn"
	| "lint/react/useSortComp"
	| "lint/react/useStylePropObject"
	| "lint/regex/noDuplicateGroupNamesInRegularExpressions"
	| "lint/regex/noEmptyCharacterClass"
	| "lint/regex/noEmptyMatches"
	| "lint/regex/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/regex/noPosixInRegularExpression"
	| "lint/regex/noReferenceToNonExistingGroup"
	| "lint/ts/noExplicitAny"
	| "lint/ts/preferShorthandArrayType"
	| "lint/ts/useInterfaces";

const lintCategoryNameMap: {[name in DiagnosticLintCategory]: true} = {
	"lint/html/useClosingNonVoid": true,
	"lint/html/useHtmlLang": true,
	"lint/html/useValidLang": true,
	"lint/js/noArguments": true,
	"lint/js/noAsyncPromiseExecutor": true,
	"lint/js/noCatchAssign": true,
	"lint/js/noCommaOperator": true,
	"lint/js/noCompareNegZero": true,
	"lint/js/noCondAssign": true,
	"lint/js/noDebugger": true,
	"lint/js/noDelete": true,
	"lint/js/noDeleteVars": true,
	"lint/js/noDoubleEquals": true,
	"lint/js/noDupeArgs": true,
	"lint/js/noDuplicateCase": true,
	"lint/js/noDuplicateImportSource": true,
	"lint/js/noDuplicateKeys": true,
	"lint/js/noEmptyBlocks": true,
	"lint/js/noExtraBooleanCast": true,
	"lint/js/noFunctionAssign": true,
	"lint/js/noGetterReturn": true,
	"lint/js/noImportAssign": true,
	"lint/js/noLabelVar": true,
	"lint/js/noNegationElse": true,
	"lint/js/noNestedTernary": true,
	"lint/js/noRestrictedGlobals": true,
	"lint/js/noSetterReturn": true,
	"lint/js/noShadowRestrictedNames": true,
	"lint/js/noShoutyConstants": true,
	"lint/js/noSingleCharRegexAlternatives": true,
	"lint/js/noSparseArray": true,
	"lint/js/noTemplateCurlyInString": true,
	"lint/js/noUndeclaredVariables": true,
	"lint/js/noUnnecessaryContinue": true,
	"lint/js/noUnsafeFinally": true,
	"lint/js/noUnsafeNegation": true,
	"lint/js/noUnusedTemplateLiteral": true,
	"lint/js/noUnusedVariables": true,
	"lint/js/noVar": true,
	"lint/js/preferOptionalChaining": true,
	"lint/js/useBlockStatements": true,
	"lint/js/useCamelCase": true,
	"lint/js/useDefaultExportBasename": true,
	"lint/js/useDefaultImportBasename": true,
	"lint/js/useFunctionDeclarations": true,
	"lint/js/useSingleCaseStatement": true,
	"lint/js/useSingleVarDeclarator": true,
	"lint/js/useSortedSpecifiers": true,
	"lint/js/useTemplate": true,
	"lint/js/useWhile": true,
	"lint/jsx-a11y/noAccessKey": true,
	"lint/jsx-a11y/noAriaUnsupportedElements": true,
	"lint/jsx-a11y/noAutofocus": true,
	"lint/jsx-a11y/noDistractingElements": true,
	"lint/jsx-a11y/noHeaderScope": true,
	"lint/jsx-a11y/noNoninteractiveElementToInteractiveRole": true,
	"lint/jsx-a11y/noNoninteractiveTabindex": true,
	"lint/jsx-a11y/noOnChange": true,
	"lint/jsx-a11y/noPositiveTabindex": true,
	"lint/jsx-a11y/noRedundantAlt": true,
	"lint/jsx-a11y/noRedundantRoles": true,
	"lint/jsx-a11y/noTargetBlank": true,
	"lint/jsx-a11y/useAltText": true,
	"lint/jsx-a11y/useAnchorContent": true,
	"lint/jsx-a11y/useAriaProps": true,
	"lint/jsx-a11y/useAriaPropsForRole": true,
	"lint/jsx-a11y/useAriaProptypes": true,
	"lint/jsx-a11y/useHeadingContent": true,
	"lint/jsx-a11y/useHtmlLang": true,
	"lint/jsx-a11y/useIframeTitle": true,
	"lint/jsx-a11y/useKeyWithClickEvents": true,
	"lint/jsx-a11y/useKeyWithMouseEvents": true,
	"lint/jsx-a11y/useMediaCaption": true,
	"lint/jsx-a11y/useValidAnchor": true,
	"lint/jsx-a11y/useValidLang": true,
	"lint/jsx/noCommentText": true,
	"lint/jsx/noDuplicateProps": true,
	"lint/jsx/noImplicitBoolean": true,
	"lint/jsx/noPropSpreading": true,
	"lint/jsx/useJSXFileExtension": true,
	"lint/jsx/usePascalCase": true,
	"lint/jsx/useSelfClosingElements": true,
	"lint/react/noAccessStateInSetState": true,
	"lint/react/noArrayIndexKey": true,
	"lint/react/noChildrenProp": true,
	"lint/react/noDanger": true,
	"lint/react/noDangerWithChildren": true,
	"lint/react/noDidMountSetState": true,
	"lint/react/noDidUpdateSetState": true,
	"lint/react/noDirectMutationState": true,
	"lint/react/noFindDOMNode": true,
	"lint/react/noRedundantShouldComponentUpdate": true,
	"lint/react/noRenderReturnValue": true,
	"lint/react/noStringRefs": true,
	"lint/react/noThisInSFC": true,
	"lint/react/noUnsafe": true,
	"lint/react/noUselessFragment": true,
	"lint/react/noVoidElementsWithChildren": true,
	"lint/react/noWillUpdateSetState": true,
	"lint/react/useButtonType": true,
	"lint/react/useFragmentSyntax": true,
	"lint/react/useKey": true,
	"lint/react/useRenderReturn": true,
	"lint/react/useSortComp": true,
	"lint/react/useStylePropObject": true,
	"lint/regex/noDuplicateGroupNamesInRegularExpressions": true,
	"lint/regex/noEmptyCharacterClass": true,
	"lint/regex/noEmptyMatches": true,
	"lint/regex/noMultipleSpacesInRegularExpressionLiterals": true,
	"lint/regex/noPosixInRegularExpression": true,
	"lint/regex/noReferenceToNonExistingGroup": true,
	"lint/ts/noExplicitAny": true,
	"lint/ts/preferShorthandArrayType": true,
	"lint/ts/useInterfaces": true,
};
/* GENERATED:END(id:main) */

// We use these weird objects so ensure that they contain all the strings above. The value is meaningless.

const categoryNameMap: {[name in DiagnosticCategory]: true} = {
	...lintCategoryNameMap,
	"analyzeDependencies/cjsExportInES": true,
	"args/fileNotFound": true,
	"args/invalid": true,
	"bundler/moduleCycle": true,
	"bundler/topLevelAwait": true,
	"childProcess/failure": true,
	"commands/init/uncommittedChanges": true,
	"commands/init/expectedRepo": true,
	"compile/classes": true,
	"compile/const-enums": true,
	"compile/jsx": true,
	"compile/nonnumeric-enum-values": true,
	"flags/invalid": true,
	"format/disabled": true,
	"internalError/fatal": true,
	"internalError/fs": true,
	"internalError/httpServer": true,
	"internalError/request": true,
	"lint/disabled": true,
	"lint/pendingFixes": true,
	parse: true,
	"projectManager/sensitiveDirectory": true,
	"projectManager/multipleConfigFiles": true,
	"projectManager/typoConfigFilename": true,
	"projectManager/misplacedConfig": true,
	"projectManager/missing": true,
	"projectManager/nameCollision": true,
	"projectManager/vscMissing": true,
	"recoveryStore/diff": true,
	"recoveryStore/notFound": true,
	"resolver/fetchFailed": true,
	"resolver/importTypeMismatch": true,
	"resolver/notFound": true,
	"resolver/unknownExport": true,
	"resolver/unsupported": true,
	"suppressions/duplicate": true,
	"suppressions/empty": true,
	"suppressions/overlap": true,
	"suppressions/incorrectSuppressionStart": true,
	"suppressions/incorrectPrefix": true,
	"suppressions/invalidCategory": true,
	"suppressions/missingSpace": true,
	"suppressions/missingTarget": true,
	"suppressions/missingExplanation": true,
	"suppressions/unused": true,
	"tests/cancelled": true,
	"tests/disabled": true,
	"tests/failure": true,
	"tests/fixtureOptions": true,
	"tests/logs": true,
	"tests/noneDeclared": true,
	"tests/snapshots/frozen": true,
	"tests/snapshots/inlineMissingReceived": true,
	"tests/snapshots/inlineCollision": true,
	"tests/snapshots/incorrect": true,
	"tests/snapshots/missing": true,
	"tests/snapshots/redundant": true,
	"tests/timeout": true,
	"tests/unhandledRejection": true,
	"typeCheck/incompatible": true,
	"typeCheck/missingCondition": true,
	"typeCheck/notExhaustive": true,
	"typeCheck/uncallable": true,
	"typeCheck/undeclaredVariable": true,
	"typeCheck/unknownImport": true,
	"typeCheck/unknownProperty": true,
	"vsc/dirty": true,
	"v8/syntaxError": true,
};

const categoryPrefixMap: {[name in DiagnosticCategoryPrefix]: true} = {
	analyzeDependencies: true,
	args: true,
	bundler: true,
	compiler: true,
	flags: true,
	internalError: true,
	lint: true,
	lsp: true,
	projectManager: true,
	resolver: true,
	tests: true,
	typeCheck: true,
	v8: true,
};

export const VALID_DIAGNOSTIC_CATEGORIES: Set<DiagnosticCategory> = new Set(
	Object.keys(categoryNameMap) as DiagnosticCategory[],
);

export const VALID_DIAGNOSTIC_CATEGORY_PREFIXES: Set<DiagnosticCategoryPrefix> = new Set(
	Object.keys(categoryPrefixMap) as DiagnosticCategoryPrefix[],
);

export function isValidDiagnosticCategoryName(
	str: string,
): str is DiagnosticCategory {
	return VALID_DIAGNOSTIC_CATEGORIES.has(str as DiagnosticCategory);
}

// If a diagnostic depends on a file that contains one of these diagnostic categories then we hide it.
// This is so we hide diagnostics like "export not found" if the target file failed to parse, as the former is unactionable until the latter is addressed.
export const DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES: Set<DiagnosticCategory> = new Set([
	"parse",
]);
