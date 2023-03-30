// This file contains the list of all diagnostic categories for the Rome
// toolchain
//
// The `define_categories` macro is preprocessed in the build script for the
// crate in order to generate the static registry. The body of the macro
// consists of a list of key-value pairs defining the categories that have an
// associated hyperlink, then a list of string literals defining the remaining
// categories without a link.

define_categories! {
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
    "lint/a11y/useHtmlLang": "https://docs.rome.tools/lint/rules/useHtmlLang",
    "lint/a11y/noDistractingElements": "https://docs.rome.tools/lint/rules/noDistractingElements",
    "lint/a11y/noHeaderScope": "https://docs.rome.tools/lint/rules/noHeaderScope",
    "lint/a11y/noAccessKey": "https://docs.rome.tools/lint/rules/noAccessKey",

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
    "lint/correctness/noUnsafeFinally": "https://docs.rome.tools/lint/rules/noUnsafeFinally",
    "lint/correctness/noConstructorReturn": "https://docs.rome.tools/lint/rules/noConstructorReturn",
    "lint/correctness/noPrecisionLoss": "https://docs.rome.tools/lint/rules/noPrecisionLoss",
    "lint/correctness/noVoidTypeReturn": "https://docs.rome.tools/lint/rules/noVoidTypeReturn",
    "lint/correctness/noStringCaseMismatch": "https://docs.rome.tools/lint/rules/noStringCaseMismatch",
    "lint/correctness/noSetterReturn": "https://docs.rome.tools/lint/rules/noSetterReturn",


    // nursery
    "lint/nursery/noAssignInExpressions": "https://docs.rome.tools/lint/rules/noAssignInExpressions",
    "lint/nursery/noWith": "https://docs.rome.tools/lint/rules/noWith",
    "lint/nursery/noExtraSemicolons": "https://docs.rome.tools/lint/rules/noExtraSemicolons",
    "lint/nursery/noBannedTypes":"https://docs.rome.tools/lint/rules/noBannedTypes",
    "lint/nursery/noClassAssign": "https://docs.rome.tools/lint/rules/noClassAssign",
    "lint/nursery/noCommaOperator": "https://docs.rome.tools/lint/rules/noCommaOperator",
    "lint/nursery/noDuplicateCase": "https://docs.rome.tools/lint/rules/noDuplicateCase",
    "lint/nursery/noExtraLabels":"https://docs.rome.tools/lint/rules/noExtraLabels",
    "lint/nursery/noInferrableTypes": "https://docs.rome.tools/lint/rules/noInferrableTypes",
    "lint/nursery/noInnerDeclarations": "https://docs.rome.tools/lint/rules/noInnerDeclarations",
    "lint/nursery/noInvalidConstructorSuper": "https://docs.rome.tools/lint/rules/noInvalidConstructorSuper",
    "lint/nursery/noConfusingLabels": "https://docs.rome.tools/lint/rules/noConfusingLabels",
    "lint/nursery/noParameterProperties": "https://docs.rome.tools/lint/rules/noParameterProperties",
    "lint/nursery/noRedundantAlt": "https://docs.rome.tools/lint/rules/noRedundantAlt",
    "lint/nursery/noRestrictedGlobals": "https://docs.rome.tools/lint/rules/noRestrictedGlobals",
    "lint/nursery/noSelfCompare": "https://docs.rome.tools/lint/rules/noSelfCompare",
    "lint/nursery/noSelfAssign": "https://docs.rome.tools/lint/rules/noSelfAssign",
    "lint/nursery/noSetterReturn": "https://docs.rome.tools/lint/rules/noSetterReturn",
    "lint/nursery/noStringCaseMismatch": "https://docs.rome.tools/lint/rules/noStringCaseMismatch",
    "lint/nursery/noSwitchDeclarations": "https://docs.rome.tools/lint/rules/noSwitchDeclarations",
    "lint/nursery/noUnreachableSuper": "https://rome.tools/docs/lint/rules/noUnreachableSuper",
    "lint/nursery/noUnusedLabels": "https://docs.rome.tools/lint/rules/noUnusedLabels",
    "lint/nursery/noUselessSwitchCase": "https://docs.rome.tools/lint/rules/noUselessSwitchCase",
    "lint/nursery/useAriaPropsForRole": "https://docs.rome.tools/lint/rules/useAriaPropsForRole",
    "lint/nursery/useAriaPropTypes": "https://docs.rome.tools/lint/rules/useAriaPropTypes",
    "lint/nursery/useCamelCase": "https://docs.rome.tools/lint/rules/useCamelCase",
    "lint/nursery/useValidLang":"https://docs.rome.tools/lint/rules/useValidLang",
    "lint/nursery/useValidAriaProps":"https://docs.rome.tools/lint/rules/useValidAriaProps",
    "lint/nursery/useExhaustiveDependencies": "https://docs.rome.tools/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useIsNan": "https://docs.rome.tools/lint/rules/useIsNan",
    "lint/nursery/useMediaCaption": "https://docs.rome.tools/lint/rules/useMediaCaption",
    "lint/nursery/useIframeTitle": "https://docs.rome.tools/lint/rules/useIframeTitle",
    "lint/nursery/noNoninteractiveElementToInteractiveRole": "https://docs.rome.tools/lint/rules/noNoninteractiveElementToInteractiveRole",
    "lint/nursery/noUselessRename": "https://docs.rome.tools/lint/rules/noUselessRename",
    "lint/nursery/useValidForDirection": "https://docs.rome.tools/lint/rules/useValidForDirection",
    "lint/nursery/useHookAtTopLevel": "https://docs.rome.tools/lint/rules/useHookAtTopLevel",
    "lint/nursery/noUnsafeOptionalChaining": "https://docs.rome.tools/lint/rules/noUnsafeOptionalChaining",
    "lint/nursery/noDuplicateJsxProps": "https://docs.rome.tools/lint/rules/noDuplicateJsxProps",
    "lint/nursery/noDuplicateClassMembers": "https://docs.rome.tools/lint/rules/noDuplicateClassMembers",
    "lint/nursery/useYield": "https://docs.rome.tools/lint/rules/useYield",
    "lint/nursery/noGlobalObjectCalls": "https://docs.rome.tools/lint/rules/noGlobalObjectCalls",
    "lint/nursery/noPrototypeBuiltins": "https://docs.rome.tools/lint/rules/noPrototypeBuiltins",
    "lint/nursery/noSvgWithoutTitle": "https://docs.rome.tools/lint/rules/noSvgWithoutTitle",
    "lint/nursery/noUselessCatch": "https://docs.rome.tools/lint/rules/noUselessCatch",
"lint/nursery/noParameterAssign": "https://docs.rome.tools/lint/rules/noParameterAssign",
"lint/nursery/noNamespace": "https://docs.rome.tools/lint/rules/noNamespace",
"lint/nursery/noConfusingArrow": "https://docs.rome.tools/lint/rules/noConfusingArrow",
    // Insert new nursery rule here
    "lint/nursery/noRedeclare": "https://docs.rome.tools/lint/rules/noRedeclare",
    "lint/nursery/useNamespaceKeyword": "https://docs.rome.tools/lint/rules/useNamespaceKeyword",

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
    "lint/style/useExponentiationOperator": "https://docs.rome.tools/lint/rules/useExponentiationOperator",
    "lint/style/useNumericLiterals": "https://docs.rome.tools/lint/rules/useNumericLiterals",
    "lint/style/useDefaultParameterLast":"https://docs.rome.tools/lint/rules/useDefaultParameterLast",
    "lint/style/useConst":"https://docs.rome.tools/lint/rules/useConst",
    "lint/style/noVar": "https://docs.rome.tools/lint/rules/noVar",
    "lint/style/noNonNullAssertion": "https://docs.rome.tools/lint/rules/noNonNullAssertion",
    "lint/style/useEnumInitializers":"https://docs.rome.tools/lint/rules/useEnumInitializers",


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
    "lint/suspicious/noEmptyInterface": "https://docs.rome.tools/lint/rules/noEmptyInterface",
    "lint/suspicious/noExtraNonNullAssertion":"https://docs.rome.tools/lint/rules/noExtraNonNullAssertion",
    "lint/suspicious/noRedundantUseStrict": "https://docs.rome.tools/lint/rules/noRedundantUseStrict",
    "lint/suspicious/noConstEnum": "https://docs.rome.tools/lint/rules/noConstEnum",
    "lint/suspicious/useDefaultSwitchClauseLast":"https://docs.rome.tools/lint/rules/useDefaultSwitchClauseLast",
    "lint/suspicious/noDuplicateObjectKeys":"https://docs.rome.tools/lint/rules/noDuplicateObjectKeys",



    ;


    // General categories
    "files/missingHandler",
    "format",
    "configuration",
    "organizeImports",
    "deserialize",
    "internalError/io",
    "internalError/fs",
    "internalError/panic",
    // parse categories
    "parse",
    "parse/noSuperWithoutExtends",
    "parse/noDuplicatePrivateClassMembers",

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

    // Suppression comments
    "suppressions/parse",
    "suppressions/unknownGroup",
    "suppressions/unknownRule",
    "suppressions/unused",
    "suppressions/deprecatedSyntax",



    // Used in tests and examples
    "args/fileNotFound",
    "flags/invalid",
    "semanticTests",
}
