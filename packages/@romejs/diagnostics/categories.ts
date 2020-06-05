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
	| "compile/classes"
	| "compile/const-enums"
	| "compile/jsx"
	| "flags/invalid"
	| "format/disabled"
	| "internalError/httpServer"
	| "internalError/request"
	| "lint/disabled"
	| "lint/pendingFixes"
	| "lsp/parse"
	| "parse/js"
	| "parse/json"
	| "parse/manifest"
	| "parse/patchMatch"
	| "parse/regex"
	| "parse/semver"
	| "parse/snapshots"
	| "parse/spdxLicense"
	| "parse/stringMarkup"
	| "parse/url"
	| "parse/url/query"
	| "projectManager/incorrectConfigFilename"
	| "projectManager/missing"
	| "projectManager/nameCollision"
	| "projectManager/vscMissing"
	| "resolver/fetchFailed"
	| "resolver/importTypeMismatch"
	| "resolver/notFound"
	| "resolver/unknownExport"
	| "resolver/unsupported"
	| "suppressions/duplicate"
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

// EVERYTHING BELOW IS AUTOGENERATED. SEE SCRIPTS FOLDER FOR UPDATE SCRIPTS

type LintDiagnosticCategory =
	| "lint/js/camelCase"
	| "lint/js/caseSingleStatement"
	| "lint/js/confusingLanguage"
	| "lint/js/defaultExportSameBasename"
	| "lint/js/doubleEquals"
	| "lint/js/duplicateImportSource"
	| "lint/js/emptyBlocks"
	| "lint/js/emptyMatches"
	| "lint/js/getterReturn"
	| "lint/js/importDefaultBasename"
	| "lint/js/negationElse"
	| "lint/js/noArguments"
	| "lint/js/noAsyncPromiseExecutor"
	| "lint/js/noCatchAssign"
	| "lint/js/noCommaOperator"
	| "lint/js/noCompareNegZero"
	| "lint/js/noCondAssign"
	| "lint/js/noDebugger"
	| "lint/js/noDelete"
	| "lint/js/noDeleteVars"
	| "lint/js/noDupeArgs"
	| "lint/js/noDuplicateCase"
	| "lint/js/noDuplicateGroupNamesInRegularExpressions"
	| "lint/js/noDuplicateKeys"
	| "lint/js/noEmptyCharacterClass"
	| "lint/js/noExtraBooleanCast"
	| "lint/js/noFunctionAssign"
	| "lint/js/noImportAssign"
	| "lint/js/noLabelVar"
	| "lint/js/noMultipleSpacesInRegularExpressionLiterals"
	| "lint/js/noPosixInRegularExpression"
	| "lint/js/noReferenceToNonExistingGroup"
	| "lint/js/noSetterReturn"
	| "lint/js/noShadowRestrictedNames"
	| "lint/js/noShorthandArrayType"
	| "lint/js/noTemplateCurlyInString"
	| "lint/js/noUnsafeFinally"
	| "lint/js/noVar"
	| "lint/js/preferBlockStatements"
	| "lint/js/preferFunctionDeclarations"
	| "lint/js/preferTemplate"
	| "lint/js/preferWhile"
	| "lint/js/restrictedGlobals"
	| "lint/js/singleVarDeclarator"
	| "lint/js/sortImportExportSpecifiers"
	| "lint/js/sparseArray"
	| "lint/js/undeclaredVariables"
	| "lint/js/unsafeNegation"
	| "lint/js/unusedVariables"
	| "lint/jsx-a11y/altText"
	| "lint/jsx-a11y/anchorHasContent"
	| "lint/jsx-a11y/anchorIsValid"
	| "lint/jsx-a11y/ariaProps"
	| "lint/jsx-a11y/ariaUnsupportedElements"
	| "lint/jsx-a11y/clickEventsHaveKeyEvents"
	| "lint/jsx-a11y/headingHasContent"
	| "lint/jsx-a11y/htmlHasLang"
	| "lint/jsx-a11y/iframeHasTitle"
	| "lint/jsx-a11y/imgRedundantAlt"
	| "lint/jsx-a11y/lang"
	| "lint/jsx-a11y/mediaHasCaption"
	| "lint/jsx-a11y/mouseEventsHaveKeyEvents"
	| "lint/jsx-a11y/noAccessKey"
	| "lint/jsx-a11y/noAutofocus"
	| "lint/jsx-a11y/noDistractingElements"
	| "lint/jsx-a11y/noNoninteractiveElementToInteractiveRole"
	| "lint/jsx-a11y/noNoninteractiveTabindex"
	| "lint/jsx-a11y/noOnChange"
	| "lint/jsx-a11y/noRedundantRoles"
	| "lint/jsx-a11y/noTargetBlank"
	| "lint/jsx-a11y/roleHasRequiredAriaProps"
	| "lint/jsx-a11y/scope"
	| "lint/jsx-a11y/tabindexNoPositive"
	| "lint/react/buttonHasType"
	| "lint/react/jsxFragments"
	| "lint/react/jsxKey"
	| "lint/react/jsxNoCommentText"
	| "lint/react/jsxNoDuplicateProps"
	| "lint/react/jsxPascalCase"
	| "lint/react/noAccessStateInSetState"
	| "lint/react/noChildrenProp"
	| "lint/react/noDanger"
	| "lint/react/noDangerWithChildren"
	| "lint/react/noDidMountSetState"
	| "lint/react/noDidUpdateSetState"
	| "lint/react/noFindDOMNode"
	| "lint/react/noRedundantShouldComponentUpdate"
	| "lint/react/noStringRefs"
	| "lint/react/noUnsafe"
	| "lint/react/noUselessFragment"
	| "lint/react/noWillUpdateSetState"
	| "lint/react/reactInJsxScope"
	| "lint/react/requireRenderReturn"
	| "lint/react/stylePropObject"
	| "lint/react/voidDomElementsNoChildren"
	| "lint/ts/noExplicitAny";
