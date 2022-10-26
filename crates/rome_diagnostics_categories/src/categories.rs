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
    // correctness
    "lint/correctness/noArguments": "https://rome.tools/docs/lint/rules/noArguments",
    "lint/correctness/noAsyncPromiseExecutor": "https://rome.tools/docs/lint/rules/noAsyncPromiseExecutor",
    "lint/correctness/noCatchAssign": "https://rome.tools/docs/lint/rules/noCatchAssign",
    "lint/correctness/noCommentText": "https://rome.tools/docs/lint/rules/noCommentText",
    "lint/correctness/noCompareNegZero": "https://rome.tools/docs/lint/rules/noCompareNegZero",
    "lint/correctness/noDelete": "https://rome.tools/docs/lint/rules/noDelete",
    "lint/correctness/noDoubleEquals": "https://rome.tools/docs/lint/rules/noDoubleEquals",
    "lint/correctness/noDupeArgs": "https://rome.tools/docs/lint/rules/noDupeArgs",
    "lint/correctness/noEmptyPattern": "https://rome.tools/docs/lint/rules/noEmptyPattern",
    "lint/correctness/noFunctionAssign": "https://rome.tools/docs/lint/rules/noFunctionAssign",
    "lint/correctness/noImportAssign": "https://rome.tools/docs/lint/rules/noImportAssign",
    "lint/correctness/noLabelVar": "https://rome.tools/docs/lint/rules/noLabelVar",
    "lint/correctness/noMultipleSpacesInRegularExpressionLiterals": "https://rome.tools/docs/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/correctness/noShadowRestrictedNames": "https://rome.tools/docs/lint/rules/noShadowRestrictedNames",
    "lint/correctness/noSparseArray": "https://rome.tools/docs/lint/rules/noSparseArray",
    "lint/correctness/noUnnecessaryContinue": "https://rome.tools/docs/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnsafeNegation": "https://rome.tools/docs/lint/rules/noUnsafeNegation",
    "lint/correctness/useSingleCaseStatement": "https://rome.tools/docs/lint/rules/useSingleCaseStatement",
    "lint/correctness/useWhile": "https://rome.tools/docs/lint/rules/useWhile",
    "lint/correctness/noNewSymbol": "https://rome.tools/docs/lint/rules/noNewSymbol",
    "lint/correctness/noUselessFragments": "https://rome.tools/docs/lint/rules/noUselessFragments",
    "lint/correctness/noUnusedVariables": "https://rome.tools/docs/lint/rules/noUnusedVariables",
    "lint/correctness/noUnreachable": "https://rome.tools/docs/lint/rules/noUnreachable",
    "lint/correctness/noRestrictedGlobals": "https://rome.tools/docs/lint/rules/noRestrictedGlobals",
    "lint/correctness/noUndeclaredVariables": "https://rome.tools/docs/lint/rules/noUndeclaredVariables",
    "lint/correctness/useValidTypeof": "https://rome.tools/docs/lint/rules/useValidTypeof",
    "lint/correctness/noVoidElementsWithChildren": "https://rome.tools/docs/lint/rules/noVoidElementsWithChildren",
    "lint/correctness/noArrayIndexKey": "https://rome.tools/docs/lint/rules/noArrayIndexKey",
    "lint/correctness/noChildrenProp": "https://rome.tools/docs/lint/rules/noChildrenProp",
    "lint/correctness/noRenderReturnValue": "https://rome.tools/docs/lint/rules/noRenderReturnValue",

    // style group
    "lint/style/noNegationElse": "https://rome.tools/docs/lint/rules/noNegationElse",
    "lint/style/noShoutyConstants": "https://rome.tools/docs/lint/rules/noShoutyConstants",
    "lint/style/useSelfClosingElements": "https://rome.tools/docs/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://rome.tools/docs/lint/rules/useShorthandArrayType",
    "lint/style/useFragmentSyntax": "https://rome.tools/docs/lint/rules/useFragmentSyntax",
    "lint/style/useTemplate": "https://rome.tools/docs/lint/rules/useTemplate",
    "lint/style/useSingleVarDeclarator": "https://rome.tools/docs/lint/rules/useSingleVarDeclarator",
    "lint/style/useOptionalChain": "https://rome.tools/docs/lint/rules/useOptionalChain",
    "lint/style/useBlockStatements": "https://rome.tools/docs/lint/rules/useBlockStatements",
    "lint/style/noImplicitBoolean": "https://rome.tools/docs/lint/rules/noImplicitBoolean",
    "lint/style/noUnusedTemplateLiteral": "https://rome.tools/docs/lint/rules/noUnusedTemplateLiteral",

    // complexity
    "lint/complexity/useSimplifiedLogicExpression": "https://rome.tools/docs/lint/rules/useSimplifiedLogicExpression",
    "lint/complexity/noExtraBooleanCast": "https://rome.tools/docs/lint/rules/noExtraBooleanCast",

    // a11y group
    "lint/a11y/noAutofocus": "https://rome.tools/docs/lint/rules/noAutofocus",
    "lint/a11y/noPositiveTabindex": "https://rome.tools/docs/lint/rules/noPositiveTabindex",
    "lint/a11y/useKeyWithMouseEvents": "https://rome.tools/docs/lint/rules/useKeyWithMouseEvents",
    "lint/a11y/useAnchorContent": "https://rome.tools/docs/lint/rules/useAnchorContent",
    "lint/a11y/useBlankTarget": "https://rome.tools/docs/lint/rules/useBlankTarget",
    "lint/a11y/useValidAnchor": "https://rome.tools/docs/lint/rules/useValidAnchor",
    "lint/a11y/useKeyWithClickEvents": "https://rome.tools/docs/lint/rules/useKeyWithClickEvents",
    "lint/a11y/useButtonType": "https://rome.tools/docs/lint/rules/useButtonType",
    "lint/a11y/useAltText": "https://rome.tools/docs/lint/rules/useAltText",
    // security
    "lint/security/noDangerouslySetInnerHtml": "https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtml",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://rome.tools/docs/lint/rules/noDangerouslySetInnerHtmlWithChildren",
    "lint/security/noDebugger": "https://rome.tools/docs/lint/rules/noDebugger",


    // nursery
    "lint/nursery/useFlatMap": "https://rome.tools/docs/lint/rules/useFlatMap",
    "lint/nursery/noConstAssign": "https://rome.tools/docs/lint/rules/noConstAssign",
    "lint/nursery/noExplicitAny": "https://rome.tools/docs/lint/rules/noExplicitAny",
    "lint/nursery/useValidForDirection": "https://rome.tools/docs/lint/rules/useValidForDirection",
    "lint/nursery/noInvalidConstructorSuper": "https://rome.tools/docs/lint/rules/noInvalidConstructorSuper",
    "lint/nursery/useExhaustiveDependencies": "https://rome.tools/docs/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useCamelCase": "https://rome.tools/docs/lint/rules/useCamelCase",

    ;

    // General categories
    "files/missingHandler",
    "format",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    "lint",
    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",

    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
