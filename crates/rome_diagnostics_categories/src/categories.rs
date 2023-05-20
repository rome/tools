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
    // a11y
    "lint/a11y/noAccessKey": "https://docs.rome.tools/lint/rules/noAccessKey",
    "lint/a11y/noAutofocus": "https://docs.rome.tools/lint/rules/noAutofocus",
    "lint/a11y/noBlankTarget": "https://docs.rome.tools/lint/rules/noBlankTarget",
    "lint/a11y/noDistractingElements": "https://docs.rome.tools/lint/rules/noDistractingElements",
    "lint/a11y/noHeaderScope": "https://docs.rome.tools/lint/rules/noHeaderScope",
    "lint/a11y/noNoninteractiveElementToInteractiveRole": "https://docs.rome.tools/lint/rules/noNoninteractiveElementToInteractiveRole",
    "lint/a11y/noPositiveTabindex": "https://docs.rome.tools/lint/rules/noPositiveTabindex",
    "lint/a11y/noRedundantAlt": "https://docs.rome.tools/lint/rules/noRedundantAlt",
    "lint/a11y/noSvgWithoutTitle": "https://docs.rome.tools/lint/rules/noSvgWithoutTitle",
    "lint/a11y/useAltText": "https://docs.rome.tools/lint/rules/useAltText",
    "lint/a11y/useAnchorContent": "https://docs.rome.tools/lint/rules/useAnchorContent",
    "lint/a11y/useAriaPropsForRole": "https://docs.rome.tools/lint/rules/useAriaPropsForRole",
    "lint/a11y/useButtonType": "https://docs.rome.tools/lint/rules/useButtonType",
    "lint/a11y/useHtmlLang": "https://docs.rome.tools/lint/rules/useHtmlLang",
    "lint/a11y/useIframeTitle": "https://docs.rome.tools/lint/rules/useIframeTitle",
    "lint/a11y/useKeyWithClickEvents": "https://docs.rome.tools/lint/rules/useKeyWithClickEvents",
    "lint/a11y/useKeyWithMouseEvents": "https://docs.rome.tools/lint/rules/useKeyWithMouseEvents",
    "lint/a11y/useMediaCaption": "https://docs.rome.tools/lint/rules/useMediaCaption",
    "lint/a11y/useValidAnchor": "https://docs.rome.tools/lint/rules/useValidAnchor",
    "lint/a11y/useValidAriaProps":"https://docs.rome.tools/lint/rules/useValidAriaProps",
    "lint/a11y/useValidLang":"https://docs.rome.tools/lint/rules/useValidLang",

    // complexity
    "lint/complexity/noExtraBooleanCast": "https://docs.rome.tools/lint/rules/noExtraBooleanCast",
    "lint/complexity/noExtraSemicolon": "https://docs.rome.tools/lint/rules/noExtraSemicolon",
    "lint/complexity/noMultipleSpacesInRegularExpressionLiterals": "https://docs.rome.tools/lint/rules/noMultipleSpacesInRegularExpressionLiterals",
    "lint/complexity/noUselessCatch": "https://docs.rome.tools/lint/rules/noUselessCatch",
    "lint/complexity/noUselessConstructor": "https://docs.rome.tools/lint/rules/noUselessConstructor",
    "lint/complexity/noUselessFragments": "https://docs.rome.tools/lint/rules/noUselessFragments",
    "lint/complexity/noUselessLabel":"https://docs.rome.tools/lint/rules/noUselessLabel",
    "lint/complexity/noUselessRename": "https://docs.rome.tools/lint/rules/noUselessRename",
    "lint/complexity/noUselessSwitchCase": "https://docs.rome.tools/lint/rules/noUselessSwitchCase",
    "lint/complexity/noUselessTypeConstraint": "https://docs.rome.tools/lint/rules/noUselessTypeConstraint",
    "lint/complexity/noWith": "https://docs.rome.tools/lint/rules/noWith",
    "lint/complexity/useFlatMap": "https://docs.rome.tools/lint/rules/useFlatMap",
    "lint/complexity/useOptionalChain": "https://docs.rome.tools/lint/rules/useOptionalChain",
    "lint/complexity/useSimplifiedLogicExpression": "https://docs.rome.tools/lint/rules/useSimplifiedLogicExpression",

    // correctness
    "lint/correctness/noChildrenProp": "https://docs.rome.tools/lint/rules/noChildrenProp",
    "lint/correctness/noConstAssign": "https://docs.rome.tools/lint/rules/noConstAssign",
    "lint/correctness/noConstructorReturn": "https://docs.rome.tools/lint/rules/noConstructorReturn",
    "lint/correctness/noEmptyPattern": "https://docs.rome.tools/lint/rules/noEmptyPattern",
    "lint/correctness/noGlobalObjectCalls": "https://docs.rome.tools/lint/rules/noGlobalObjectCalls",
    "lint/correctness/noInnerDeclarations": "https://docs.rome.tools/lint/rules/noInnerDeclarations",
    "lint/correctness/noInvalidConstructorSuper": "https://docs.rome.tools/lint/rules/noInvalidConstructorSuper",
    "lint/correctness/noNewSymbol": "https://docs.rome.tools/lint/rules/noNewSymbol",
    "lint/correctness/noPrecisionLoss": "https://docs.rome.tools/lint/rules/noPrecisionLoss",
    "lint/correctness/noRenderReturnValue": "https://docs.rome.tools/lint/rules/noRenderReturnValue",
    "lint/correctness/noSetterReturn": "https://docs.rome.tools/lint/rules/noSetterReturn",
    "lint/correctness/noStringCaseMismatch": "https://docs.rome.tools/lint/rules/noStringCaseMismatch",
    "lint/correctness/noSwitchDeclarations": "https://docs.rome.tools/lint/rules/noSwitchDeclarations",
    "lint/correctness/noUndeclaredVariables": "https://docs.rome.tools/lint/rules/noUndeclaredVariables",
    "lint/correctness/noUnnecessaryContinue": "https://docs.rome.tools/lint/rules/noUnnecessaryContinue",
    "lint/correctness/noUnreachable": "https://docs.rome.tools/lint/rules/noUnreachable",
    "lint/correctness/noUnreachableSuper": "https://rome.tools/docs/lint/rules/noUnreachableSuper",
    "lint/correctness/noUnsafeFinally": "https://docs.rome.tools/lint/rules/noUnsafeFinally",
    "lint/correctness/noUnsafeOptionalChaining": "https://docs.rome.tools/lint/rules/noUnsafeOptionalChaining",
    "lint/correctness/noUnusedLabels": "https://docs.rome.tools/lint/rules/noUnusedLabels",
    "lint/correctness/noUnusedVariables": "https://docs.rome.tools/lint/rules/noUnusedVariables",
    "lint/correctness/noVoidElementsWithChildren": "https://docs.rome.tools/lint/rules/noVoidElementsWithChildren",
    "lint/correctness/noVoidTypeReturn": "https://docs.rome.tools/lint/rules/noVoidTypeReturn",
    "lint/correctness/useValidForDirection": "https://docs.rome.tools/lint/rules/useValidForDirection",
    "lint/correctness/useYield": "https://docs.rome.tools/lint/rules/useYield",

    // nursery
    "lint/nursery/noAccumulatingSpread": "https://docs.rome.tools/lint/rules/noAccumulatingSpread",
    "lint/nursery/noAriaUnsupportedElements": "https://docs.rome.tools/lint/rules/noAriaUnsupportedElements",
    "lint/nursery/noBannedTypes":"https://docs.rome.tools/lint/rules/noBannedTypes",
    "lint/nursery/noConfusingArrow": "https://docs.rome.tools/lint/rules/noConfusingArrow",
    "lint/nursery/noConsoleLog": "https://docs.rome.tools/lint/rules/noConsoleLog",
    "lint/nursery/noConstantCondition": "https://docs.rome.tools/lint/rules/noConstantCondition",
    "lint/nursery/noDuplicateJsxProps": "https://docs.rome.tools/lint/rules/noDuplicateJsxProps",
    "lint/nursery/noForEach": "https://docs.rome.tools/lint/rules/noForEach",
    "lint/nursery/noNoninteractiveTabindex": "https://docs.rome.tools/lint/rules/noNoninteractiveTabindex",
    "lint/nursery/noRedundantRoles": "https://docs.rome.tools/lint/rules/noRedundantRoles",
    "lint/nursery/noSelfAssign": "https://docs.rome.tools/lint/rules/noSelfAssign",
    "lint/nursery/useAriaPropTypes": "https://docs.rome.tools/lint/rules/useAriaPropTypes",
    "lint/nursery/useCamelCase": "https://docs.rome.tools/lint/rules/useCamelCase",
    "lint/nursery/useExhaustiveDependencies": "https://docs.rome.tools/lint/rules/useExhaustiveDependencies",
    "lint/nursery/useGroupedTypeImport": "https://docs.rome.tools/lint/rules/useGroupedTypeImport",
    "lint/nursery/useHeadingContent": "https://docs.rome.tools/lint/rules/useHeadingContent",
    "lint/nursery/useHookAtTopLevel": "https://docs.rome.tools/lint/rules/useHookAtTopLevel",
    "lint/nursery/useIsNan": "https://docs.rome.tools/lint/rules/useIsNan",
    "lint/nursery/useLiteralEnumMembers": "https://docs.rome.tools/lint/rules/useLiteralEnumMembers",
    "lint/nursery/useLiteralKeys": "https://docs.rome.tools/lint/rules/useLiteralKeys",
"lint/nursery/useSimpleNumberKeys": "https://docs.rome.tools/lint/rules/useSimpleNumberKeys",
"lint/style/noStaticOnlyClass": "https://docs.rome.tools/lint/rules/noStaticOnlyClass",
    // Insert new nursery rule here


    // performance
    "lint/performance/noDelete": "https://docs.rome.tools/lint/rules/noDelete",

    // security
    "lint/security/noDangerouslySetInnerHtml": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtml",
    "lint/security/noDangerouslySetInnerHtmlWithChildren": "https://docs.rome.tools/lint/rules/noDangerouslySetInnerHtmlWithChildren",

    // style
    "lint/style/noArguments": "https://docs.rome.tools/lint/rules/noArguments",
    "lint/style/noCommaOperator": "https://docs.rome.tools/lint/rules/noCommaOperator",
    "lint/style/noImplicitBoolean": "https://docs.rome.tools/lint/rules/noImplicitBoolean",
    "lint/style/noInferrableTypes": "https://docs.rome.tools/lint/rules/noInferrableTypes",
    "lint/style/noNamespace": "https://docs.rome.tools/lint/rules/noNamespace",
    "lint/style/noNegationElse": "https://docs.rome.tools/lint/rules/noNegationElse",
    "lint/style/noNonNullAssertion": "https://docs.rome.tools/lint/rules/noNonNullAssertion",
    "lint/style/noParameterAssign": "https://docs.rome.tools/lint/rules/noParameterAssign",
    "lint/style/noParameterProperties": "https://docs.rome.tools/lint/rules/noParameterProperties",
    "lint/style/noRestrictedGlobals": "https://docs.rome.tools/lint/rules/noRestrictedGlobals",
    "lint/style/noShoutyConstants": "https://docs.rome.tools/lint/rules/noShoutyConstants",
    "lint/style/noUnusedTemplateLiteral": "https://docs.rome.tools/lint/rules/noUnusedTemplateLiteral",
    "lint/style/noVar": "https://docs.rome.tools/lint/rules/noVar",
    "lint/style/useBlockStatements": "https://docs.rome.tools/lint/rules/useBlockStatements",
    "lint/style/useConst":"https://docs.rome.tools/lint/rules/useConst",
    "lint/style/useDefaultParameterLast":"https://docs.rome.tools/lint/rules/useDefaultParameterLast",
    "lint/style/useEnumInitializers":"https://docs.rome.tools/lint/rules/useEnumInitializers",
    "lint/style/useExponentiationOperator": "https://docs.rome.tools/lint/rules/useExponentiationOperator",
    "lint/style/useFragmentSyntax": "https://docs.rome.tools/lint/rules/useFragmentSyntax",
    "lint/style/useNumericLiterals": "https://docs.rome.tools/lint/rules/useNumericLiterals",
    "lint/style/useSelfClosingElements": "https://docs.rome.tools/lint/rules/useSelfClosingElements",
    "lint/style/useShorthandArrayType": "https://docs.rome.tools/lint/rules/useShorthandArrayType",
    "lint/style/useSingleCaseStatement": "https://docs.rome.tools/lint/rules/useSingleCaseStatement",
    "lint/style/useSingleVarDeclarator": "https://docs.rome.tools/lint/rules/useSingleVarDeclarator",
    "lint/style/useTemplate": "https://docs.rome.tools/lint/rules/useTemplate",
    "lint/style/useWhile": "https://docs.rome.tools/lint/rules/useWhile",

    // suspicious
    "lint/suspicious/noArrayIndexKey": "https://docs.rome.tools/lint/rules/noArrayIndexKey",
    "lint/suspicious/noAssignInExpressions": "https://docs.rome.tools/lint/rules/noAssignInExpressions",
    "lint/suspicious/noAsyncPromiseExecutor": "https://docs.rome.tools/lint/rules/noAsyncPromiseExecutor",
    "lint/suspicious/noCatchAssign": "https://docs.rome.tools/lint/rules/noCatchAssign",
    "lint/suspicious/noClassAssign": "https://docs.rome.tools/lint/rules/noClassAssign",
    "lint/suspicious/noCommentText": "https://docs.rome.tools/lint/rules/noCommentText",
    "lint/suspicious/noCompareNegZero": "https://docs.rome.tools/lint/rules/noCompareNegZero",
    "lint/suspicious/noConfusingLabels": "https://docs.rome.tools/lint/rules/noConfusingLabels",
    "lint/suspicious/noConstEnum": "https://docs.rome.tools/lint/rules/noConstEnum",
    "lint/suspicious/noDebugger": "https://docs.rome.tools/lint/rules/noDebugger",
    "lint/suspicious/noDoubleEquals": "https://docs.rome.tools/lint/rules/noDoubleEquals",
    "lint/suspicious/noDuplicateCase": "https://docs.rome.tools/lint/rules/noDuplicateCase",
    "lint/suspicious/noDuplicateClassMembers": "https://docs.rome.tools/lint/rules/noDuplicateClassMembers",
    "lint/suspicious/noDuplicateObjectKeys":"https://docs.rome.tools/lint/rules/noDuplicateObjectKeys",
    "lint/suspicious/noDuplicateParameters": "https://docs.rome.tools/lint/rules/noDuplicateParameters",
    "lint/suspicious/noEmptyInterface": "https://docs.rome.tools/lint/rules/noEmptyInterface",
    "lint/suspicious/noExplicitAny": "https://docs.rome.tools/lint/rules/noExplicitAny",
    "lint/suspicious/noExtraNonNullAssertion":"https://docs.rome.tools/lint/rules/noExtraNonNullAssertion",
    "lint/suspicious/noFunctionAssign": "https://docs.rome.tools/lint/rules/noFunctionAssign",
    "lint/suspicious/noImportAssign": "https://docs.rome.tools/lint/rules/noImportAssign",
    "lint/suspicious/noLabelVar": "https://docs.rome.tools/lint/rules/noLabelVar",
    "lint/suspicious/noPrototypeBuiltins": "https://docs.rome.tools/lint/rules/noPrototypeBuiltins",
    "lint/suspicious/noRedeclare": "https://docs.rome.tools/lint/rules/noRedeclare",
    "lint/suspicious/noRedundantUseStrict": "https://docs.rome.tools/lint/rules/noRedundantUseStrict",
    "lint/suspicious/noSelfCompare": "https://docs.rome.tools/lint/rules/noSelfCompare",
    "lint/suspicious/noShadowRestrictedNames": "https://docs.rome.tools/lint/rules/noShadowRestrictedNames",
    "lint/suspicious/noSparseArray": "https://docs.rome.tools/lint/rules/noSparseArray",
    "lint/suspicious/noUnsafeNegation": "https://docs.rome.tools/lint/rules/noUnsafeNegation",
    "lint/suspicious/useDefaultSwitchClauseLast":"https://docs.rome.tools/lint/rules/useDefaultSwitchClauseLast",
    "lint/suspicious/useNamespaceKeyword": "https://docs.rome.tools/lint/rules/useNamespaceKeyword",
    "lint/suspicious/useValidTypeof": "https://docs.rome.tools/lint/rules/useValidTypeof",

    ;

    // General categories
    "files/missingHandler",
    "format",
    "configuration",
    "organizeImports",
    "migrate",
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
