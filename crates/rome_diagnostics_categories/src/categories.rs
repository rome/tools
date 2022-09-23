// This file contains the list of all diagnostic categories for the Rome
// toolchain
//
// The `define_dategories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

define_dategories! {
    // Lint categories
    "lint/correctness/noArguments": "https://rome.tools/docs/lint/rules/noArguments",
    "lint/correctness/noAsyncPromiseExecutor": "https://rome.tools/docs/lint/rules/noAsyncPromiseExecutor",
    "lint/correctness/noCatchAssign": "https://rome.tools/docs/lint/rules/noCatchAssign",
    "lint/correctness/noCommentText": "https://rome.tools/docs/lint/rules/noCommentText",
    "lint/correctness/noCompareNegZero": "https://rome.tools/docs/lint/rules/noCompareNegZero",
    "lint/correctness/noDebugger": "https://rome.tools/docs/lint/rules/noDebugger",
    "lint/correctness/noDelete": "https://rome.tools/docs/lint/rules/noDelete",
    "lint/correctness/noDoubleEquals": "https://rome.tools/docs/lint/rules/noDoubleEquals",
    "lint/correctness/noDupeArgs": "https://rome.tools/docs/lint/rules/noDupeArgs",
    "lint/correctness/noEmptyPattern": "https://rome.tools/docs/lint/rules/noEmptyPattern",
    "lint/correctness/noExtraBooleanCast": "https://rome.tools/docs/lint/rules/noExtraBooleanCast",
    "lint/correctness/noFunctionAssign": "https://rome.tools/docs/lint/rules/noFunctionAssign",
    "lint/correctness/noImplicitBoolean": "https://rome.tools/docs/lint/rules/noImplicitBoolean",
    "lint/correctness/noImportAssign": "https://rome.tools/docs/lint/rules/noImportAssign",
    "lint/correctness/noLabelVar": "https://rome.tools/docs/lint/rules/noLabelVar",
    "lint/correctness/noMultipleSpacesInRegularExpressionLiterals": "https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/correctness/noShadowRestrictedNames": "https://rome.tools/docs/lint/rules/noShadowRestrictedNames",
    "lint/correctness/noSparseArray": "https://rome.tools/docs/lint/rules/noSparseArray",
    "lint/correctness/noUnnecessaryContinue": "https://rome.tools/docs/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnsafeNegation": "https://rome.tools/docs/lint/rules/noUnsafeNegation",
    "lint/correctness/noUnusedTemplateLiteral": "https://rome.tools/docs/lint/rules/noUnusedTemplateLiteral",
    "lint/correctness/useBlockStatements": "https://rome.tools/docs/lint/rules/useBlockStatements",
    "lint/correctness/useSimplifiedLogicExpression": "https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression",
    "lint/correctness/useSingleCaseStatement": "https://rome.tools/docs/lint/rules/useSingleCaseStatement",
    "lint/correctness/useSingleVarDeclarator": "https://rome.tools/docs/lint/rules/useSingleVarDeclarator",
    "lint/correctness/useTemplate": "https://rome.tools/docs/lint/rules/useTemplate",
    "lint/correctness/useValidTypeof": "https://rome.tools/docs/lint/rules/useValidTypeof",
    "lint/correctness/useWhile": "https://rome.tools/docs/lint/rules/useWhile",
    "lint/nursery/noDangerouslySetInnerHtml": "https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml",
    "lint/nursery/noNewSymbol": "https://rome.tools/docs/lint/rules/noNewSymbol",
    "lint/nursery/noRenderReturnValue": "https://rome.tools/docs/lint/rules/noRenderReturnValue",
    "lint/nursery/noUnreachable": "https://rome.tools/docs/lint/rules/noUnreachable",
    "lint/nursery/noUnusedVariables": "https://rome.tools/docs/lint/rules/noUnusedVariables",
    "lint/nursery/noUselessFragments": "https://rome.tools/docs/lint/rules/noUselessFragments",
    "lint/nursery/useButtonType": "https://rome.tools/docs/lint/rules/useButtonType",
    "lint/nursery/useCamelCase": "https://rome.tools/docs/lint/rules/useCamelCase",
    "lint/nursery/useOptionalChain": "https://rome.tools/docs/lint/rules/useOptionalChain",
    "lint/style/noNegationElse": "https://rome.tools/docs/lint/rules/noNegationElse",
    "lint/style/noShoutyConstants": "https://rome.tools/docs/lint/rules/noShoutyConstants",
    "lint/style/useSelfClosingElements": "https://rome.tools/docs/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://rome.tools/docs/lint/rules/useShorthandArrayType",
    ;
    // General categories
    "args/fileNotFound",
    "ci/formatMismatch",
    "files/missingHandler",
    "flags/invalid",
    "format/diff",
    "internalError/io",
    "internalError/panic",
    "io/skippedFile",
    "io/unhandledFile",
    "suppressions/invalidGroup",
    "suppressions/invalidRule",
    // Legacy codes
    "SyntaxError",
    "IO",
    "Lint",
    "Format",
}
