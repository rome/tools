/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {equalArray} from "@internal/typescript-helpers";

// Rather than having a generic `string` type, we use string literals so there's one place where
// all category names are defined. This allows the naming scheme to be more easily reviewed and
// made consistent.
export type DiagnosticCategory =
	| DiagnosticLintCategory
	| ["analyzeDependencies", "cjsExportInES"]
	| ["args", "fileNotFound"]
	| ["args", "invalid"]
	| ["bundler", "moduleCycle"]
	| ["bundler", "topLevelAwait"]
	| ["bridge", "closed"]
	| ["bridge", "disconnected"]
	| ["bridge", "timeout"]
	| ["childProcess", "failure"]
	| ["commands", "auto-config", "uncommittedChanges"]
	| ["commands", "auto-config", "expectedRepo"]
	| ["compile", "classes"]
	| ["compile", "const-enums"]
	| ["compile", "jsx"]
	| ["compile", "nonnumeric-enum-values"]
	| ["eslint"]
	| ["files", "missingHandler"]
	| ["files", "tooBig"]
	| ["flags", "invalid"]
	| ["format", "disabled"]
	| ["integration", "load"]
	| ["integration", "missingVersion"]
	| ["integration", "unsupportedVersion"]
	| ["integration", "notFound"]
	| ["internalError", "fatal"]
	| ["internalError", "fs"]
	| ["internalError", "httpServer"]
	| ["internalError", "request"]
	| ["lint", "disabled"]
	| ["lint", "pendingFixes"]
	| ["parse"]
	| ["projectManager", "sensitiveDirectory"]
	| ["projectManager", "multipleConfigFiles"]
	| ["projectManager", "typoConfigFilename"]
	| ["projectManager", "misplacedConfig"]
	| ["projectManager", "missing"]
	| ["projectManager", "nameCollision"]
	| ["projectManager", "vscMissing"]
	| ["recoveryStore", "diff"]
	| ["recoveryStore", "notFound"]
	| ["resolver", "fetchFailed"]
	| ["resolver", "importTypeMismatch"]
	| ["resolver", "notFound"]
	| ["resolver", "unknownExport"]
	| ["resolver", "unsupported"]
	| ["suppressions", "duplicate"]
	| ["suppressions", "empty"]
	| ["suppressions", "overlap"]
	| ["suppressions", "incorrectSuppressionStart"]
	| ["suppressions", "incorrectPrefix"]
	| ["suppressions", "invalidCategory"]
	| ["suppressions", "missingSpace"]
	| ["suppressions", "missingTarget"]
	| ["suppressions", "missingExplanation"]
	| ["suppressions", "unused"]
	| ["tests", "cancelled"]
	| ["tests", "disabled"]
	| ["tests", "failure"]
	| ["tests", "fixtureOptions"]
	| ["tests", "logs"]
	| ["tests", "empty"]
	| ["tests", "snapshots", "frozen"]
	| ["tests", "snapshots", "inlineMissingReceived"]
	| ["tests", "snapshots", "inlineCollision"]
	| ["tests", "snapshots", "incorrect"]
	| ["tests", "snapshots", "missing"]
	| ["tests", "snapshots", "redundant"]
	| ["tests", "timeout"]
	| ["tests", "unhandledRejection"]
	| ["typeCheck", "incompatible"]
	| ["typeCheck", "missingCondition"]
	| ["typeCheck", "notExhaustive"]
	| ["typeCheck", "uncallable"]
	| ["typeCheck", "undeclaredVariable"]
	| ["typeCheck", "unknownImport"]
	| ["typeCheck", "unknownProperty"]
	| ["vsc", "dirty"]
	| ["v8", "syntaxError"];

export type DiagnosticCategoryPrefix = DiagnosticCategory[0];

type StringConverter<T> = T extends [string]
	? T[0]
	: T extends [string, ...infer Rest]
		? `${T[0]}/${StringConverter<Rest>}`
		: never;

type DiagnosticCategoryString = StringConverter<DiagnosticCategory>;

type DiagnosticLintCategoryString = StringConverter<DiagnosticLintCategory>;

/* GENERATED:START(hash:8d85ea45d90ffc05f8ef743a6c38cf41c8152dba,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
export type DiagnosticLintCategory =
	| ["lint", "a11y", "noAccessKey"]
	| ["lint", "a11y", "noAriaUnsupportedElements"]
	| ["lint", "a11y", "noAutofocus"]
	| ["lint", "a11y", "noDistractingElements"]
	| ["lint", "a11y", "noHeaderScope"]
	| ["lint", "a11y", "noNoninteractiveElementToInteractiveRole"]
	| ["lint", "a11y", "noNoninteractiveTabindex"]
	| ["lint", "a11y", "noPositiveTabindex"]
	| ["lint", "a11y", "noRedundantAlt"]
	| ["lint", "a11y", "noSvgWithoutTitle"]
	| ["lint", "a11y", "useAltText"]
	| ["lint", "a11y", "useAriaProps"]
	| ["lint", "a11y", "useAriaProptypes"]
	| ["lint", "a11y", "useHtmlLang"]
	| ["lint", "a11y", "useIframeTitle"]
	| ["lint", "a11y", "useMediaCaption"]
	| ["lint", "a11y", "useValidLang"]
	| ["lint", "css", "noImportantInKeyframes"]
	| ["lint", "css", "noInvalidGridTemplateAreas"]
	| ["lint", "html", "useClosingNonVoid"]
	| ["lint", "js", "noArguments"]
	| ["lint", "js", "noAsyncPromiseExecutor"]
	| ["lint", "js", "noCatchAssign"]
	| ["lint", "js", "noCommaOperator"]
	| ["lint", "js", "noCompareNegZero"]
	| ["lint", "js", "noCondAssign"]
	| ["lint", "js", "noDebugger"]
	| ["lint", "js", "noDelete"]
	| ["lint", "js", "noDeleteVars"]
	| ["lint", "js", "noDoubleEquals"]
	| ["lint", "js", "noDupeArgs"]
	| ["lint", "js", "noDuplicateCase"]
	| ["lint", "js", "noDuplicateImportSource"]
	| ["lint", "js", "noDuplicateKeys"]
	| ["lint", "js", "noEmptyBlocks"]
	| ["lint", "js", "noExtraBooleanCast"]
	| ["lint", "js", "noFunctionAssign"]
	| ["lint", "js", "noGetterReturn"]
	| ["lint", "js", "noImportAssign"]
	| ["lint", "js", "noLabelVar"]
	| ["lint", "js", "noNegationElse"]
	| ["lint", "js", "noNestedTernary"]
	| ["lint", "js", "noRestrictedGlobals"]
	| ["lint", "js", "noSetterReturn"]
	| ["lint", "js", "noShadowRestrictedNames"]
	| ["lint", "js", "noShoutyConstants"]
	| ["lint", "js", "noSingleCharRegexAlternatives"]
	| ["lint", "js", "noSparseArray"]
	| ["lint", "js", "noTemplateCurlyInString"]
	| ["lint", "js", "noUndeclaredVariables"]
	| ["lint", "js", "noUnnecessaryContinue"]
	| ["lint", "js", "noUnsafeFinally"]
	| ["lint", "js", "noUnsafeNegation"]
	| ["lint", "js", "noUnusedTemplateLiteral"]
	| ["lint", "js", "noUnusedVariables"]
	| ["lint", "js", "noVar"]
	| ["lint", "js", "preferOptionalChaining"]
	| ["lint", "js", "useBlockStatements"]
	| ["lint", "js", "useCamelCase"]
	| ["lint", "js", "useDefaultExportBasename"]
	| ["lint", "js", "useDefaultImportBasename"]
	| ["lint", "js", "useFunctionDeclarations"]
	| ["lint", "js", "useSimplifiedLogicalExpression"]
	| ["lint", "js", "useSingleCaseStatement"]
	| ["lint", "js", "useSingleVarDeclarator"]
	| ["lint", "js", "useSortedSpecifiers"]
	| ["lint", "js", "useTemplate"]
	| ["lint", "js", "useWhile"]
	| ["lint", "jsx-a11y", "noAutofocus"]
	| ["lint", "jsx-a11y", "noHeaderScope"]
	| ["lint", "jsx-a11y", "noOnChange"]
	| ["lint", "jsx-a11y", "noRedundantRoles"]
	| ["lint", "jsx-a11y", "noTargetBlank"]
	| ["lint", "jsx-a11y", "useAnchorContent"]
	| ["lint", "jsx-a11y", "useAriaPropsForRole"]
	| ["lint", "jsx-a11y", "useHeadingContent"]
	| ["lint", "jsx-a11y", "useKeyWithClickEvents"]
	| ["lint", "jsx-a11y", "useKeyWithMouseEvents"]
	| ["lint", "jsx-a11y", "useValidAnchor"]
	| ["lint", "jsx", "noCommentText"]
	| ["lint", "jsx", "noDuplicateProps"]
	| ["lint", "jsx", "noImplicitBoolean"]
	| ["lint", "jsx", "noPropSpreading"]
	| ["lint", "jsx", "useJSXFileExtension"]
	| ["lint", "jsx", "usePascalCase"]
	| ["lint", "jsx", "useSelfClosingElements"]
	| ["lint", "react", "noAccessStateInSetState"]
	| ["lint", "react", "noArrayIndexKey"]
	| ["lint", "react", "noChildrenProp"]
	| ["lint", "react", "noDanger"]
	| ["lint", "react", "noDangerWithChildren"]
	| ["lint", "react", "noDidMountSetState"]
	| ["lint", "react", "noDidUpdateSetState"]
	| ["lint", "react", "noDirectMutationState"]
	| ["lint", "react", "noFindDOMNode"]
	| ["lint", "react", "noRedundantShouldComponentUpdate"]
	| ["lint", "react", "noRenderReturnValue"]
	| ["lint", "react", "noStringRefs"]
	| ["lint", "react", "noThisInSFC"]
	| ["lint", "react", "noUnsafe"]
	| ["lint", "react", "noUselessFragment"]
	| ["lint", "react", "noVoidElementsWithChildren"]
	| ["lint", "react", "noWillUpdateSetState"]
	| ["lint", "react", "useButtonType"]
	| ["lint", "react", "useFragmentSyntax"]
	| ["lint", "react", "useKey"]
	| ["lint", "react", "useRenderReturn"]
	| ["lint", "react", "useSortComp"]
	| ["lint", "react", "useStylePropObject"]
	| ["lint", "regex", "noDuplicateGroupNamesInRegularExpressions"]
	| ["lint", "regex", "noEmptyCharacterClass"]
	| ["lint", "regex", "noEmptyMatches"]
	| ["lint", "regex", "noMultipleSpacesInRegularExpressionLiterals"]
	| ["lint", "regex", "noPosixInRegularExpression"]
	| ["lint", "regex", "noReferenceToNonExistingGroup"]
	| ["lint", "ts", "noExplicitAny"]
	| ["lint", "ts", "preferShorthandArrayType"]
	| ["lint", "ts", "useInterfaces"]
	| ["lint", "ts", "useSimplifiedBooleanExpression"];
const lintCategoryNameMap: {
	[name in DiagnosticLintCategoryString]: DiagnosticLintCategory
} = {
	"lint/a11y/noAccessKey": ["lint", "a11y", "noAccessKey"],
	"lint/a11y/noAriaUnsupportedElements": [
		"lint",
		"a11y",
		"noAriaUnsupportedElements",
	],
	"lint/a11y/noAutofocus": ["lint", "a11y", "noAutofocus"],
	"lint/a11y/noDistractingElements": ["lint", "a11y", "noDistractingElements"],
	"lint/a11y/noHeaderScope": ["lint", "a11y", "noHeaderScope"],
	"lint/a11y/noNoninteractiveElementToInteractiveRole": [
		"lint",
		"a11y",
		"noNoninteractiveElementToInteractiveRole",
	],
	"lint/a11y/noNoninteractiveTabindex": [
		"lint",
		"a11y",
		"noNoninteractiveTabindex",
	],
	"lint/a11y/noPositiveTabindex": ["lint", "a11y", "noPositiveTabindex"],
	"lint/a11y/noRedundantAlt": ["lint", "a11y", "noRedundantAlt"],
	"lint/a11y/noSvgWithoutTitle": ["lint", "a11y", "noSvgWithoutTitle"],
	"lint/a11y/useAltText": ["lint", "a11y", "useAltText"],
	"lint/a11y/useAriaProps": ["lint", "a11y", "useAriaProps"],
	"lint/a11y/useAriaProptypes": ["lint", "a11y", "useAriaProptypes"],
	"lint/a11y/useHtmlLang": ["lint", "a11y", "useHtmlLang"],
	"lint/a11y/useIframeTitle": ["lint", "a11y", "useIframeTitle"],
	"lint/a11y/useMediaCaption": ["lint", "a11y", "useMediaCaption"],
	"lint/a11y/useValidLang": ["lint", "a11y", "useValidLang"],
	"lint/css/noImportantInKeyframes": ["lint", "css", "noImportantInKeyframes"],
	"lint/css/noInvalidGridTemplateAreas": [
		"lint",
		"css",
		"noInvalidGridTemplateAreas",
	],
	"lint/html/useClosingNonVoid": ["lint", "html", "useClosingNonVoid"],
	"lint/js/noArguments": ["lint", "js", "noArguments"],
	"lint/js/noAsyncPromiseExecutor": ["lint", "js", "noAsyncPromiseExecutor"],
	"lint/js/noCatchAssign": ["lint", "js", "noCatchAssign"],
	"lint/js/noCommaOperator": ["lint", "js", "noCommaOperator"],
	"lint/js/noCompareNegZero": ["lint", "js", "noCompareNegZero"],
	"lint/js/noCondAssign": ["lint", "js", "noCondAssign"],
	"lint/js/noDebugger": ["lint", "js", "noDebugger"],
	"lint/js/noDelete": ["lint", "js", "noDelete"],
	"lint/js/noDeleteVars": ["lint", "js", "noDeleteVars"],
	"lint/js/noDoubleEquals": ["lint", "js", "noDoubleEquals"],
	"lint/js/noDupeArgs": ["lint", "js", "noDupeArgs"],
	"lint/js/noDuplicateCase": ["lint", "js", "noDuplicateCase"],
	"lint/js/noDuplicateImportSource": ["lint", "js", "noDuplicateImportSource"],
	"lint/js/noDuplicateKeys": ["lint", "js", "noDuplicateKeys"],
	"lint/js/noEmptyBlocks": ["lint", "js", "noEmptyBlocks"],
	"lint/js/noExtraBooleanCast": ["lint", "js", "noExtraBooleanCast"],
	"lint/js/noFunctionAssign": ["lint", "js", "noFunctionAssign"],
	"lint/js/noGetterReturn": ["lint", "js", "noGetterReturn"],
	"lint/js/noImportAssign": ["lint", "js", "noImportAssign"],
	"lint/js/noLabelVar": ["lint", "js", "noLabelVar"],
	"lint/js/noNegationElse": ["lint", "js", "noNegationElse"],
	"lint/js/noNestedTernary": ["lint", "js", "noNestedTernary"],
	"lint/js/noRestrictedGlobals": ["lint", "js", "noRestrictedGlobals"],
	"lint/js/noSetterReturn": ["lint", "js", "noSetterReturn"],
	"lint/js/noShadowRestrictedNames": ["lint", "js", "noShadowRestrictedNames"],
	"lint/js/noShoutyConstants": ["lint", "js", "noShoutyConstants"],
	"lint/js/noSingleCharRegexAlternatives": [
		"lint",
		"js",
		"noSingleCharRegexAlternatives",
	],
	"lint/js/noSparseArray": ["lint", "js", "noSparseArray"],
	"lint/js/noTemplateCurlyInString": ["lint", "js", "noTemplateCurlyInString"],
	"lint/js/noUndeclaredVariables": ["lint", "js", "noUndeclaredVariables"],
	"lint/js/noUnnecessaryContinue": ["lint", "js", "noUnnecessaryContinue"],
	"lint/js/noUnsafeFinally": ["lint", "js", "noUnsafeFinally"],
	"lint/js/noUnsafeNegation": ["lint", "js", "noUnsafeNegation"],
	"lint/js/noUnusedTemplateLiteral": ["lint", "js", "noUnusedTemplateLiteral"],
	"lint/js/noUnusedVariables": ["lint", "js", "noUnusedVariables"],
	"lint/js/noVar": ["lint", "js", "noVar"],
	"lint/js/preferOptionalChaining": ["lint", "js", "preferOptionalChaining"],
	"lint/js/useBlockStatements": ["lint", "js", "useBlockStatements"],
	"lint/js/useCamelCase": ["lint", "js", "useCamelCase"],
	"lint/js/useDefaultExportBasename": ["lint", "js", "useDefaultExportBasename"],
	"lint/js/useDefaultImportBasename": ["lint", "js", "useDefaultImportBasename"],
	"lint/js/useFunctionDeclarations": ["lint", "js", "useFunctionDeclarations"],
	"lint/js/useSimplifiedLogicalExpression": [
		"lint",
		"js",
		"useSimplifiedLogicalExpression",
	],
	"lint/js/useSingleCaseStatement": ["lint", "js", "useSingleCaseStatement"],
	"lint/js/useSingleVarDeclarator": ["lint", "js", "useSingleVarDeclarator"],
	"lint/js/useSortedSpecifiers": ["lint", "js", "useSortedSpecifiers"],
	"lint/js/useTemplate": ["lint", "js", "useTemplate"],
	"lint/js/useWhile": ["lint", "js", "useWhile"],
	"lint/jsx-a11y/noAutofocus": ["lint", "jsx-a11y", "noAutofocus"],
	"lint/jsx-a11y/noHeaderScope": ["lint", "jsx-a11y", "noHeaderScope"],
	"lint/jsx-a11y/noOnChange": ["lint", "jsx-a11y", "noOnChange"],
	"lint/jsx-a11y/noRedundantRoles": ["lint", "jsx-a11y", "noRedundantRoles"],
	"lint/jsx-a11y/noTargetBlank": ["lint", "jsx-a11y", "noTargetBlank"],
	"lint/jsx-a11y/useAnchorContent": ["lint", "jsx-a11y", "useAnchorContent"],
	"lint/jsx-a11y/useAriaPropsForRole": [
		"lint",
		"jsx-a11y",
		"useAriaPropsForRole",
	],
	"lint/jsx-a11y/useHeadingContent": ["lint", "jsx-a11y", "useHeadingContent"],
	"lint/jsx-a11y/useKeyWithClickEvents": [
		"lint",
		"jsx-a11y",
		"useKeyWithClickEvents",
	],
	"lint/jsx-a11y/useKeyWithMouseEvents": [
		"lint",
		"jsx-a11y",
		"useKeyWithMouseEvents",
	],
	"lint/jsx-a11y/useValidAnchor": ["lint", "jsx-a11y", "useValidAnchor"],
	"lint/jsx/noCommentText": ["lint", "jsx", "noCommentText"],
	"lint/jsx/noDuplicateProps": ["lint", "jsx", "noDuplicateProps"],
	"lint/jsx/noImplicitBoolean": ["lint", "jsx", "noImplicitBoolean"],
	"lint/jsx/noPropSpreading": ["lint", "jsx", "noPropSpreading"],
	"lint/jsx/useJSXFileExtension": ["lint", "jsx", "useJSXFileExtension"],
	"lint/jsx/usePascalCase": ["lint", "jsx", "usePascalCase"],
	"lint/jsx/useSelfClosingElements": ["lint", "jsx", "useSelfClosingElements"],
	"lint/react/noAccessStateInSetState": [
		"lint",
		"react",
		"noAccessStateInSetState",
	],
	"lint/react/noArrayIndexKey": ["lint", "react", "noArrayIndexKey"],
	"lint/react/noChildrenProp": ["lint", "react", "noChildrenProp"],
	"lint/react/noDanger": ["lint", "react", "noDanger"],
	"lint/react/noDangerWithChildren": ["lint", "react", "noDangerWithChildren"],
	"lint/react/noDidMountSetState": ["lint", "react", "noDidMountSetState"],
	"lint/react/noDidUpdateSetState": ["lint", "react", "noDidUpdateSetState"],
	"lint/react/noDirectMutationState": ["lint", "react", "noDirectMutationState"],
	"lint/react/noFindDOMNode": ["lint", "react", "noFindDOMNode"],
	"lint/react/noRedundantShouldComponentUpdate": [
		"lint",
		"react",
		"noRedundantShouldComponentUpdate",
	],
	"lint/react/noRenderReturnValue": ["lint", "react", "noRenderReturnValue"],
	"lint/react/noStringRefs": ["lint", "react", "noStringRefs"],
	"lint/react/noThisInSFC": ["lint", "react", "noThisInSFC"],
	"lint/react/noUnsafe": ["lint", "react", "noUnsafe"],
	"lint/react/noUselessFragment": ["lint", "react", "noUselessFragment"],
	"lint/react/noVoidElementsWithChildren": [
		"lint",
		"react",
		"noVoidElementsWithChildren",
	],
	"lint/react/noWillUpdateSetState": ["lint", "react", "noWillUpdateSetState"],
	"lint/react/useButtonType": ["lint", "react", "useButtonType"],
	"lint/react/useFragmentSyntax": ["lint", "react", "useFragmentSyntax"],
	"lint/react/useKey": ["lint", "react", "useKey"],
	"lint/react/useRenderReturn": ["lint", "react", "useRenderReturn"],
	"lint/react/useSortComp": ["lint", "react", "useSortComp"],
	"lint/react/useStylePropObject": ["lint", "react", "useStylePropObject"],
	"lint/regex/noDuplicateGroupNamesInRegularExpressions": [
		"lint",
		"regex",
		"noDuplicateGroupNamesInRegularExpressions",
	],
	"lint/regex/noEmptyCharacterClass": ["lint", "regex", "noEmptyCharacterClass"],
	"lint/regex/noEmptyMatches": ["lint", "regex", "noEmptyMatches"],
	"lint/regex/noMultipleSpacesInRegularExpressionLiterals": [
		"lint",
		"regex",
		"noMultipleSpacesInRegularExpressionLiterals",
	],
	"lint/regex/noPosixInRegularExpression": [
		"lint",
		"regex",
		"noPosixInRegularExpression",
	],
	"lint/regex/noReferenceToNonExistingGroup": [
		"lint",
		"regex",
		"noReferenceToNonExistingGroup",
	],
	"lint/ts/noExplicitAny": ["lint", "ts", "noExplicitAny"],
	"lint/ts/preferShorthandArrayType": ["lint", "ts", "preferShorthandArrayType"],
	"lint/ts/useInterfaces": ["lint", "ts", "useInterfaces"],
	"lint/ts/useSimplifiedBooleanExpression": [
		"lint",
		"ts",
		"useSimplifiedBooleanExpression",
	],
};
/* GENERATED:END(id:main) */

export const DIAGNOSTIC_CATEGORIES: {
	[name in DiagnosticCategoryString]: DiagnosticCategory
} = {
	...lintCategoryNameMap,
	"analyzeDependencies/cjsExportInES": ["analyzeDependencies", "cjsExportInES"],
	"args/fileNotFound": ["args", "fileNotFound"],
	"args/invalid": ["args", "invalid"],
	"bridge/disconnected": ["bridge", "disconnected"],
	"bridge/closed": ["bridge", "closed"],
	"bridge/timeout": ["bridge", "timeout"],
	"bundler/moduleCycle": ["bundler", "moduleCycle"],
	"bundler/topLevelAwait": ["bundler", "topLevelAwait"],
	"childProcess/failure": ["childProcess", "failure"],
	"commands/auto-config/uncommittedChanges": [
		"commands",
		"auto-config",
		"uncommittedChanges",
	],
	"commands/auto-config/expectedRepo": [
		"commands",
		"auto-config",
		"expectedRepo",
	],
	"compile/classes": ["compile", "classes"],
	"compile/const-enums": ["compile", "const-enums"],
	"compile/jsx": ["compile", "jsx"],
	"compile/nonnumeric-enum-values": ["compile", "nonnumeric-enum-values"],
	eslint: ["eslint"],
	"files/missingHandler": ["files", "missingHandler"],
	"files/tooBig": ["files", "tooBig"],
	"flags/invalid": ["flags", "invalid"],
	"format/disabled": ["format", "disabled"],
	"integration/missingVersion": ["integration", "missingVersion"],
	"integration/unsupportedVersion": ["integration", "unsupportedVersion"],
	"integration/notFound": ["integration", "notFound"],
	"integration/load": ["integration", "load"],
	"internalError/fatal": ["internalError", "fatal"],
	"internalError/fs": ["internalError", "fs"],
	"internalError/httpServer": ["internalError", "httpServer"],
	"internalError/request": ["internalError", "request"],
	"lint/disabled": ["lint", "disabled"],
	"lint/pendingFixes": ["lint", "pendingFixes"],
	parse: ["parse"],
	"projectManager/sensitiveDirectory": ["projectManager", "sensitiveDirectory"],
	"projectManager/multipleConfigFiles": [
		"projectManager",
		"multipleConfigFiles",
	],
	"projectManager/typoConfigFilename": ["projectManager", "typoConfigFilename"],
	"projectManager/misplacedConfig": ["projectManager", "misplacedConfig"],
	"projectManager/missing": ["projectManager", "missing"],
	"projectManager/nameCollision": ["projectManager", "nameCollision"],
	"projectManager/vscMissing": ["projectManager", "vscMissing"],
	"recoveryStore/diff": ["recoveryStore", "diff"],
	"recoveryStore/notFound": ["recoveryStore", "notFound"],
	"resolver/fetchFailed": ["resolver", "fetchFailed"],
	"resolver/importTypeMismatch": ["resolver", "importTypeMismatch"],
	"resolver/notFound": ["resolver", "notFound"],
	"resolver/unknownExport": ["resolver", "unknownExport"],
	"resolver/unsupported": ["resolver", "unsupported"],
	"suppressions/duplicate": ["suppressions", "duplicate"],
	"suppressions/empty": ["suppressions", "empty"],
	"suppressions/overlap": ["suppressions", "overlap"],
	"suppressions/incorrectSuppressionStart": [
		"suppressions",
		"incorrectSuppressionStart",
	],
	"suppressions/incorrectPrefix": ["suppressions", "incorrectPrefix"],
	"suppressions/invalidCategory": ["suppressions", "invalidCategory"],
	"suppressions/missingSpace": ["suppressions", "missingSpace"],
	"suppressions/missingTarget": ["suppressions", "missingTarget"],
	"suppressions/missingExplanation": ["suppressions", "missingExplanation"],
	"suppressions/unused": ["suppressions", "unused"],
	"tests/cancelled": ["tests", "cancelled"],
	"tests/disabled": ["tests", "disabled"],
	"tests/failure": ["tests", "failure"],
	"tests/fixtureOptions": ["tests", "fixtureOptions"],
	"tests/logs": ["tests", "logs"],
	"tests/empty": ["tests", "empty"],
	"tests/snapshots/frozen": ["tests", "snapshots", "frozen"],
	"tests/snapshots/inlineMissingReceived": [
		"tests",
		"snapshots",
		"inlineMissingReceived",
	],
	"tests/snapshots/inlineCollision": ["tests", "snapshots", "inlineCollision"],
	"tests/snapshots/incorrect": ["tests", "snapshots", "incorrect"],
	"tests/snapshots/missing": ["tests", "snapshots", "missing"],
	"tests/snapshots/redundant": ["tests", "snapshots", "redundant"],
	"tests/timeout": ["tests", "timeout"],
	"tests/unhandledRejection": ["tests", "unhandledRejection"],
	"typeCheck/incompatible": ["typeCheck", "incompatible"],
	"typeCheck/missingCondition": ["typeCheck", "missingCondition"],
	"typeCheck/notExhaustive": ["typeCheck", "notExhaustive"],
	"typeCheck/uncallable": ["typeCheck", "uncallable"],
	"typeCheck/undeclaredVariable": ["typeCheck", "undeclaredVariable"],
	"typeCheck/unknownImport": ["typeCheck", "unknownImport"],
	"typeCheck/unknownProperty": ["typeCheck", "unknownProperty"],
	"vsc/dirty": ["vsc", "dirty"],
	"v8/syntaxError": ["v8", "syntaxError"],
};

export const categoryPrefixMap: {[name in DiagnosticCategoryPrefix]: true} = {
	analyzeDependencies: true,
	args: true,
	bridge: true,
	bundler: true,
	childProcess: true,
	commands: true,
	compile: true,
	eslint: true,
	files: true,
	integration: true,
	format: true,
	parse: true,
	recoveryStore: true,
	suppressions: true,
	vsc: true,
	flags: true,
	internalError: true,
	lint: true,
	projectManager: true,
	resolver: true,
	tests: true,
	typeCheck: true,
	v8: true,
};

export const VALID_DIAGNOSTIC_CATEGORIES: Set<DiagnosticCategoryString> = new Set(
	Object.keys(DIAGNOSTIC_CATEGORIES) as DiagnosticCategoryString[],
);

export const VALID_DIAGNOSTIC_CATEGORY_PREFIXES: Set<DiagnosticCategoryPrefix> = new Set(
	Object.keys(categoryPrefixMap) as DiagnosticCategoryPrefix[],
);

export function equalCategoryNames(
	a: undefined | DiagnosticCategory,
	b: undefined | DiagnosticCategory,
): boolean {
	if (a === b) {
		return true;
	}

	if (a !== undefined && b !== undefined && equalArray(a, b)) {
		return true;
	}

	return false;
}

export function joinCategoryName(
	category: DiagnosticCategory,
): DiagnosticCategoryString {
	return category.join("/") as DiagnosticCategoryString;
}

export function splitPossibleCategoryName(
	str: string,
): undefined | DiagnosticCategory {
	return DIAGNOSTIC_CATEGORIES[str as DiagnosticCategoryString];
}

export function isValidDiagnosticCategoryName(
	parts: string[],
): parts is DiagnosticCategory {
	return VALID_DIAGNOSTIC_CATEGORIES.has(
		joinCategoryName(parts as DiagnosticCategory),
	);
}

// If a diagnostic depends on a file that contains one of these diagnostic categories then we hide it.
// This is so we hide diagnostics like "export not found" if the target file failed to parse, as the former is unactionable until the latter is addressed.
export const DIAGNOSTIC_CATEGORIES_SUPPRESS_DEPENDENCIES: Set<DiagnosticCategory> = new Set([
	["parse"],
]);
