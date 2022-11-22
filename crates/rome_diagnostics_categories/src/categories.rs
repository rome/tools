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
    "lint/correctness/noArguments": "https://docs.rome.tools/lint/rules/noArguments",
    "lint/correctness/noAsyncPromiseExecutor": "https://docs.rome.tools/lint/rules/noAsyncPromiseExecutor",
    "lint/correctness/noCatchAssign": "https://docs.rome.tools/lint/rules/noCatchAssign",
    "lint/correctness/noCommentText": "https://docs.rome.tools/lint/rules/noCommentText",
    "lint/correctness/noCompareNegZero": "https://docs.rome.tools/lint/rules/noCompareNegZero",
    "lint/correctness/noDelete": "https://docs.rome.tools/lint/rules/noDelete",
    "lint/correctness/noDoubleEquals": "https://docs.rome.tools/lint/rules/noDoubleEquals",
    "lint/correctness/noDupeArgs": "https://docs.rome.tools/lint/rules/noDupeArgs",
    "lint/correctness/noEmptyPattern": "https://docs.rome.tools/lint/rules/noEmptyPattern",
    "lint/correctness/noFunctionAssign": "https://docs.rome.tools/lint/rules/noFunctionAssign",
    "lint/correctness/noImportAssign": "https://docs.rome.tools/lint/rules/noImportAssign",
    "lint/correctness/noLabelVar": "https://docs.rome.tools/lint/rules/noLabelVar",
    "lint/correctness/noMultipleSpacesInRegularExpressionLiterals": "https://docs.rome.tools/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/correctness/noShadowRestrictedNames": "https://docs.rome.tools/lint/rules/noShadowRestrictedNames",
    "lint/correctness/noSparseArray": "https://docs.rome.tools/lint/rules/noSparseArray",
    "lint/correctness/noUnnecessaryContinue": "https://docs.rome.tools/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnsafeNegation": "https://docs.rome.tools/lint/rules/noUnsafeNegation",
    "lint/correctness/useSingleCaseStatement": "https://docs.rome.tools/lint/rules/useSingleCaseStatement",
    "lint/correctness/useWhile": "https://docs.rome.tools/lint/rules/useWhile",
    "lint/correctness/noNewSymbol": "https://docs.rome.tools/lint/rules/noNewSymbol",
    "lint/correctness/noUselessFragments": "https://docs.rome.tools/lint/rules/noUselessFragments",
    "lint/correctness/noUnusedVariables": "https://docs.rome.tools/lint/rules/noUnusedVariables",
    "lint/correctness/noUnreachable": "https://docs.rome.tools/lint/rules/noUnreachable",
    "lint/correctness/noRestrictedGlobals": "https://docs.rome.tools/lint/rules/noRestrictedGlobals",
    "lint/correctness/noUndeclaredVariables": "https://docs.rome.tools/lint/rules/noUndeclaredVariables",
    "lint/correctness/useValidTypeof": "https://docs.rome.tools/lint/rules/useValidTypeof",
    "lint/correctness/noVoidElementsWithChildren": "https://docs.rome.tools/lint/rules/noVoidElementsWithChildren",
    "lint/correctness/noArrayIndexKey": "https://docs.rome.tools/lint/rules/noArrayIndexKey",
    "lint/correctness/noChildrenProp": "https://docs.rome.tools/lint/rules/noChildrenProp",
    "lint/correctness/noRenderReturnValue": "https://docs.rome.tools/lint/rules/noRenderReturnValue",
    "lint/correctness/noDebugger": "https://docs.rome.tools/lint/rules/noDebugger",

    // style group
    "lint/style/noNegationElse": "https://docs.rome.tools/lint/rules/noNegationElse",
    "lint/style/noShoutyConstants": "https://docs.rome.tools/lint/rules/noShoutyConstants",
    "lint/style/useSelfClosingElements": "https://docs.rome.tools/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://docs.rome.tools/lint/rules/useShorthandArrayType",
    "lint/style/useFragmentSyntax": "https://docs.rome.tools/lint/rules/useFragmentSyntax",
    "lint/style/useTemplate": "https://docs.rome.tools/lint/rules/useTemplate",
    "lint/style/useSingleVarDeclarator": "https://docs.rome.tools/lint/rules/useSingleVarDeclarator",
    "lint/style/useOptionalChain": "https://docs.rome.tools/lint/rules/useOptionalChain",
    "lint/style/useBlockStatements": "https://docs.rome.tools/lint/rules/useBlockStatements",
    "lint/style/noImplicitBoolean": "https://docs.rome.tools/lint/rules/noImplicitBoolean",
    "lint/style/noUnusedTemplateLiteral": "https://docs.rome.tools/lint/rules/noUnusedTemplateLiteral",

    // complexity
    "lint/complexity/useSimplifiedLogicExpression": "https://docs.rome.tools/lint/rules/useSimplifiedLogicExpression",
    "lint/complexity/noExtraBooleanCast": "https://docs.rome.tools/lint/rules/noExtraBooleanCast",

    // a11y group
    "lint/a11y/noAutofocus": "https://docs.rome.tools/lint/rules/noAutofocus",
    "lint/a11y/noPositiveTabindex": "https://docs.rome.tools/lint/rules/noPositiveTabindex",
    "lint/a11y/useKeyWithMouseEvents": "https://docs.rome.tools/lint/rules/useKeyWithMouseEvents",
    "lint/a11y/useAnchorContent": "https://docs.rome.tools/lint/rules/useAnchorContent",
    "lint/a11y/useBlankTarget": "https://docs.rome.tools/lint/rules/useBlankTarget",
    "lint/a11y/useValidAnchor": "https://docs.rome.tools/lint/rules/useValidAnchor",
    "lint/a11y/useKeyWithClickEvents": "https://docs.rome.tools/lint/rules/useKeyWithClickEvents",
    "lint/a11y/useButtonType": "https://docs.rome.tools/lint/rules/useButtonType",
    "lint/a11y/useAltText": "https://docs.rome.tools/lint/rules/useAltText",
    // security
    "lint/security/noDangerouslySetInnerHtml": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtml",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtmlWithChildren",


    // nursery
    "lint/nursery/noAccessKey": "https://docs.rome.tools/lint/rules/noAccessKey",
    "lint/nursery/noBannedTypes":"https://docs.rome.tools/lint/rules/noBannedTypes",
    "lint/nursery/noConditionalAssignment": "https://docs.rome.tools/lint/rules/noConditionalAssignment",
    "lint/nursery/noConstAssign": "https://docs.rome.tools/lint/rules/noConstAssign",
    "lint/nursery/noDupeKeys":"https://docs.rome.tools/lint/rules/noDupeKeys",
    "lint/nursery/noEmptyInterface": "https://docs.rome.tools/lint/rules/noEmptyInterface",
    "lint/nursery/noExplicitAny": "https://docs.rome.tools/lint/rules/noExplicitAny",
    "lint/nursery/noHeaderScope": "https://docs.rome.tools/lint/rules/noHeaderScope",
    "lint/nursery/noInvalidConstructorSuper": "https://docs.rome.tools/lint/rules/noInvalidConstructorSuper",
    "lint/nursery/noPrecisionLoss": "https://docs.rome.tools/lint/rules/noPrecisionLoss",
    "lint/nursery/noStringCaseMismatch": "https://docs.rome.tools/lint/rules/noStringCaseMismatch",
    "lint/nursery/noUnsafeFinally": "https://docs.rome.tools/lint/rules/noUnsafeFinally",
    "lint/nursery/useCamelCase": "https://docs.rome.tools/lint/rules/useCamelCase",
    "lint/nursery/useConst":"https://docs.rome.tools/lint/rules/useConst",
    "lint/nursery/useExhaustiveDependencies": "https://docs.rome.tools/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useFlatMap": "https://docs.rome.tools/lint/rules/useFlatMap",
    "lint/nursery/useNumericLiterals": "https://docs.rome.tools/lint/rules/useNumericLiterals",
    "lint/nursery/useValidForDirection": "https://docs.rome.tools/lint/rules/useValidForDirection",

    ;

    // General categories
    "files/missingHandler",
    "format",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",

    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",

    // Lint groups
    "lint",
    "lint/correctness",
    "lint/style",
    "lint/complexity",
    "lint/a11y",
    "lint/security",
    "lint/nursery",

    // Suppression comments
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSyntax",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
