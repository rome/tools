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
	| LintDiagnosticCategory
	| "analyzeDependencies/cjsExportInES"
	| "args/fileNotFound"
	| "args/invalid"
	| "bundler/moduleCycle"
	| "bundler/topLevelAwait"
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
	| "lsp/parse"
	| "parse/html"
	| "parse/commit"
	| "parse/css"
	| "parse/js"
	| "parse/json"
	| "parse/manifest"
	| "parse/markdown"
	| "parse/patchMatch"
	| "parse/regex"
	| "parse/semver"
	| "parse/snapshots"
	| "parse/spdxLicense"
	| "parse/stringMarkup"
	| "parse/url"
	| "parse/url/query"
	| "parse/vscodeTheme"
	| "projectManager/sensitiveDirectory"
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
	| "suppressions/overlap"
	| "suppressions/incorrectSuppressionStart"
	| "suppressions/incorrectPrefix"
	| "suppressions/missingSpace"
	| "suppressions/missingTarget"
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
	| "parse"
	| "projectManager"
	| "resolver"
	| "tests"
	| "typeCheck"
	| "v8";

/* GENERATED:START(hash:a9dbbe8880fdb546a0b5377abfec1e1512553002,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
type LintDiagnosticCategory =
	| "lint/html/useClosingNonVoid"
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
	| "lint/js/noShorthandArrayType"
	| "lint/js/noShoutyConstants"
	| "lint/js/noSparseArray"
	| "lint/js/noTemplateCurlyInString"
	| "lint/js/noUndeclaredVariables"
	| "lint/js/noUnsafeFinally"
	| "lint/js/noUnsafeNegation"
	| "lint/js/noUnusedTemplateLiteral"
	| "lint/js/noUnusedVariables"
	| "lint/js/noVar"
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
	| "lint/ts/useInterfaces";
/* GENERATED:END(id:main) */
