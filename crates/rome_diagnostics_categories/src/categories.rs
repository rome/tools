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
    // a11y group
    "lint/a11y/noAutofocus": "https://docs.rome.tools/lint/rules/noAutofocus",
    "lint/a11y/noBlankTarget": "https://docs.rome.tools/lint/rules/noBlankTarget",
    "lint/a11y/noPositiveTabindex": "https://docs.rome.tools/lint/rules/noPositiveTabindex",
    "lint/a11y/useAltText": "https://docs.rome.tools/lint/rules/useAltText",
    "lint/a11y/useAnchorContent": "https://docs.rome.tools/lint/rules/useAnchorContent",
    "lint/a11y/useButtonType": "https://docs.rome.tools/lint/rules/useButtonType",
    "lint/a11y/useKeyWithClickEvents": "https://docs.rome.tools/lint/rules/useKeyWithClickEvents",
    "lint/a11y/useKeyWithMouseEvents": "https://docs.rome.tools/lint/rules/useKeyWithMouseEvents",
    "lint/a11y/useValidAnchor": "https://docs.rome.tools/lint/rules/useValidAnchor",

    // complexity
    "lint/complexity/noExtraBooleanCast": "https://docs.rome.tools/lint/rules/noExtraBooleanCast",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://docs.rome.tools/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/complexity/noUselessFragments": "https://docs.rome.tools/lint/rules/noUselessFragments",
    "lint/complexity/useFlatMap": "https://docs.rome.tools/lint/rules/useFlatMap",
    "lint/complexity/useOptionalChain": "https://docs.rome.tools/lint/rules/useOptionalChain",
    "lint/complexity/useSimplifiedLogicExpression": "https://docs.rome.tools/lint/rules/useSimplifiedLogicExpression",

    // correctness
    "lint/correctness/noChildrenProp": "https://docs.rome.tools/lint/rules/noChildrenProp",
    "lint/correctness/noConstAssign": "https://docs.rome.tools/lint/rules/noConstAssign",
    "lint/correctness/noEmptyPattern": "https://docs.rome.tools/lint/rules/noEmptyPattern",
    "lint/correctness/noNewSymbol": "https://docs.rome.tools/lint/rules/noNewSymbol",
    "lint/correctness/noRenderReturnValue": "https://docs.rome.tools/lint/rules/noRenderReturnValue",
    "lint/correctness/noUndeclaredVariables": "https://docs.rome.tools/lint/rules/noUndeclaredVariables",
    "lint/correctness/noUnnecessaryContinue": "https://docs.rome.tools/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnreachable": "https://docs.rome.tools/lint/rules/noUnreachable",
    "lint/correctness/noUnusedVariables": "https://docs.rome.tools/lint/rules/noUnusedVariables",
    "lint/correctness/noVoidElementsWithChildren": "https://docs.rome.tools/lint/rules/noVoidElementsWithChildren",
    "lint/correctness/useValidForDirection": "https://docs.rome.tools/lint/rules/useValidForDirection",

    // nursery
    "lint/nursery/noAccessKey": "https://docs.rome.tools/lint/rules/noAccessKey",
    "lint/nursery/noBannedTypes":"https://docs.rome.tools/lint/rules/noBannedTypes",
    "lint/nursery/noConditionalAssignment": "https://docs.rome.tools/lint/rules/noConditionalAssignment",
    "lint/nursery/noConstEnum": "https://docs.rome.tools/lint/rules/noConstEnum",
    "lint/nursery/noConstructorReturn": "https://docs.rome.tools/lint/rules/noConstructorReturn",
    "lint/nursery/noDistractingElements": "https://docs.rome.tools/lint/rules/noDistractingElements",
    "lint/nursery/noDuplicateObjectKeys":"https://docs.rome.tools/lint/rules/noDuplicateObjectKeys",
    "lint/nursery/noEmptyInterface": "https://docs.rome.tools/lint/rules/noEmptyInterface",
    "lint/nursery/noExtraNonNullAssertion":"https://docs.rome.tools/lint/rules/noExtraNonNullAssertion",
    "lint/nursery/noHeaderScope": "https://docs.rome.tools/lint/rules/noHeaderScope",
    "lint/nursery/noInvalidConstructorSuper": "https://docs.rome.tools/lint/rules/noInvalidConstructorSuper",
    "lint/nursery/noNonNullAssertion": "https://docs.rome.tools/lint/rules/noNonNullAssertion",
    "lint/nursery/noPrecisionLoss": "https://docs.rome.tools/lint/rules/noPrecisionLoss",
    "lint/nursery/noRedundantUseStrict": "https://docs.rome.tools/lint/rules/noRedundantUseStrict",
    "lint/nursery/noRestrictedGlobals": "https://docs.rome.tools/lint/rules/noRestrictedGlobals",
    "lint/nursery/noSetterReturn": "https://docs.rome.tools/lint/rules/noSetterReturn",
    "lint/nursery/noStringCaseMismatch": "https://docs.rome.tools/lint/rules/noStringCaseMismatch",
    "lint/nursery/noUnsafeFinally": "https://docs.rome.tools/lint/rules/noUnsafeFinally",
    "lint/nursery/noVar": "https://docs.rome.tools/lint/rules/noVar",
    "lint/nursery/noVoidTypeReturn": "https://docs.rome.tools/lint/rules/noVoidTypeReturn",
    "lint/nursery/useAriaPropsForRole": "https://docs.rome.tools/lint/rules/useAriaPropsForRole",
    "lint/nursery/useAriaPropTypes": "https://docs.rome.tools/lint/rules/useAriaPropTypes",
    "lint/nursery/useCamelCase": "https://docs.rome.tools/lint/rules/useCamelCase",
    "lint/nursery/useConst":"https://docs.rome.tools/lint/rules/useConst",
    "lint/nursery/useDefaultParameterLast":"https://docs.rome.tools/lint/rules/useDefaultParameterLast",
    "lint/nursery/useDefaultSwitchClauseLast":"https://docs.rome.tools/lint/rules/useDefaultSwitchClauseLast",
    "lint/nursery/useEnumInitializers":"https://docs.rome.tools/lint/rules/useEnumInitializers",
    "lint/nursery/useExhaustiveDependencies": "https://docs.rome.tools/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useExponentiationOperator": "https://docs.rome.tools/lint/rules/useExponentiationOperator",
    "lint/nursery/useNumericLiterals": "https://docs.rome.tools/lint/rules/useNumericLiterals",
    "lint/nursery/useValidForDirection": "https://docs.rome.tools/lint/rules/useValidForDirection",

    // performance
    "lint/performance/noDelete": "https://docs.rome.tools/lint/rules/noDelete",

    // security
    "lint/security/noDangerouslySetInnerHtml": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtml",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtmlWithChildren",

    // style group
    "lint/style/noArguments": "https://docs.rome.tools/lint/rules/noArguments",
    "lint/style/noImplicitBoolean": "https://docs.rome.tools/lint/rules/noImplicitBoolean",
    "lint/style/noNegationElse": "https://docs.rome.tools/lint/rules/noNegationElse",
    "lint/style/noShoutyConstants": "https://docs.rome.tools/lint/rules/noShoutyConstants",
    "lint/style/noUnusedTemplateLiteral": "https://docs.rome.tools/lint/rules/noUnusedTemplateLiteral",
    "lint/style/useBlockStatements": "https://docs.rome.tools/lint/rules/useBlockStatements",
    "lint/style/useFragmentSyntax": "https://docs.rome.tools/lint/rules/useFragmentSyntax",
    "lint/style/useSelfClosingElements": "https://docs.rome.tools/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://docs.rome.tools/lint/rules/useShorthandArrayType",
    "lint/style/useSingleCaseStatement": "https://docs.rome.tools/lint/rules/useSingleCaseStatement",
    "lint/style/useSingleVarDeclarator": "https://docs.rome.tools/lint/rules/useSingleVarDeclarator",
    "lint/style/useTemplate": "https://docs.rome.tools/lint/rules/useTemplate",
    "lint/style/useWhile": "https://docs.rome.tools/lint/rules/useWhile",

    // suspicious
    "lint/suspicious/noArrayIndexKey": "https://docs.rome.tools/lint/rules/noArrayIndexKey",
    "lint/suspicious/noAsyncPromiseExecutor": "https://docs.rome.tools/lint/rules/noAsyncPromiseExecutor",
    "lint/suspicious/noCatchAssign": "https://docs.rome.tools/lint/rules/noCatchAssign",
    "lint/suspicious/noCommentText": "https://docs.rome.tools/lint/rules/noCommentText",
    "lint/suspicious/noCompareNegZero": "https://docs.rome.tools/lint/rules/noCompareNegZero",
    "lint/suspicious/noDebugger": "https://docs.rome.tools/lint/rules/noDebugger",
    "lint/suspicious/noDoubleEquals": "https://docs.rome.tools/lint/rules/noDoubleEquals",
    "lint/suspicious/noDuplicateParameters": "https://docs.rome.tools/lint/rules/noDuplicateParameters",
    "lint/suspicious/noExplicitAny": "https://docs.rome.tools/lint/rules/noExplicitAny",
    "lint/suspicious/noFunctionAssign": "https://docs.rome.tools/lint/rules/noFunctionAssign",
    "lint/suspicious/noImportAssign": "https://docs.rome.tools/lint/rules/noImportAssign",
    "lint/suspicious/noLabelVar": "https://docs.rome.tools/lint/rules/noLabelVar",
    "lint/suspicious/noShadowRestrictedNames": "https://docs.rome.tools/lint/rules/noShadowRestrictedNames",
    "lint/suspicious/noSparseArray": "https://docs.rome.tools/lint/rules/noSparseArray",
    "lint/suspicious/noUnsafeNegation": "https://docs.rome.tools/lint/rules/noUnsafeNegation",
    "lint/suspicious/useValidTypeof": "https://docs.rome.tools/lint/rules/useValidTypeof",
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
    "lint/a11y",
    "lint/complexity",
    "lint/correctness",
    "lint/nursery",
    "lint/performance",
    "lint/security",
    "lint/style",
    "lint/suspicious",
    "lint/configuration",

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSyntax",

    "configuration",

    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
