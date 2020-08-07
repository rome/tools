import * as n from "@internal/ast";

export * from "./base";
export * from "./js/constants";

export * from "./html/unions";
export * from "./js/unions";
export * from "./markdown/unions";
export * from "./unions";

export {bindingKeys, nodeNames, visitorKeys} from "./utils";
export type AnyNodes = AnyNode | Array<AnyNode>;

/* GENERATED:START(hash:1aed3775ee865f85eb502c788c23c0d3416f663e,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/ast` to update. */
export * from "./common/comments/CommentBlock";
export * from "./common/comments/CommentLine";
export * from "./common/commit/CommitRoot";
export * from "./css/core/CSSAtRule";
export * from "./css/core/CSSBlock";
export * from "./css/core/CSSDeclaration";
export * from "./css/core/CSSDimension";
export * from "./css/core/CSSFunction";
export * from "./css/core/CSSIdentifier";
export * from "./css/core/CSSNumber";
export * from "./css/core/CSSPercentage";
export * from "./css/core/CSSRaw";
export * from "./css/core/CSSRoot";
export * from "./css/core/CSSRule";
export * from "./html/attributes/HTMLAttribute";
export * from "./html/tags/HTMLDoctypeTag";
export * from "./html/tags/HTMLElement";
export * from "./html/core/HTMLIdentifier";
export * from "./html/core/HTMLRoot";
export * from "./html/core/HTMLString";
export * from "./html/core/HTMLText";
export * from "./js/temp/JSAmbiguousFlowTypeCastExpression";
export * from "./js/expressions/JSArrayExpression";
export * from "./js/auxiliary/JSArrayHole";
export * from "./js/expressions/JSArrowFunctionExpression";
export * from "./js/patterns/JSAssignmentArrayPattern";
export * from "./js/patterns/JSAssignmentAssignmentPattern";
export * from "./js/expressions/JSAssignmentExpression";
export * from "./js/patterns/JSAssignmentIdentifier";
export * from "./js/patterns/JSAssignmentObjectPattern";
export * from "./js/patterns/JSAssignmentObjectPatternProperty";
export * from "./js/expressions/JSAwaitExpression";
export * from "./js/literals/JSBigIntLiteral";
export * from "./js/expressions/JSBinaryExpression";
export * from "./js/patterns/JSBindingArrayPattern";
export * from "./js/patterns/JSBindingAssignmentPattern";
export * from "./js/patterns/JSBindingIdentifier";
export * from "./js/patterns/JSBindingObjectPattern";
export * from "./js/patterns/JSBindingObjectPatternProperty";
export * from "./js/statements/JSBlockStatement";
export * from "./js/literals/JSBooleanLiteral";
export * from "./js/statements/JSBreakStatement";
export * from "./js/expressions/JSCallExpression";
export * from "./js/auxiliary/JSCatchClause";
export * from "./js/classes/JSClassDeclaration";
export * from "./js/classes/JSClassExpression";
export * from "./js/classes/JSClassHead";
export * from "./js/classes/JSClassMethod";
export * from "./js/classes/JSClassPrivateMethod";
export * from "./js/classes/JSClassPrivateProperty";
export * from "./js/classes/JSClassProperty";
export * from "./js/classes/JSClassPropertyMeta";
export * from "./js/auxiliary/JSComputedMemberProperty";
export * from "./js/objects/JSComputedPropertyKey";
export * from "./js/expressions/JSConditionalExpression";
export * from "./js/statements/JSContinueStatement";
export * from "./js/statements/JSDebuggerStatement";
export * from "./js/core/JSDirective";
export * from "./js/expressions/JSDoExpression";
export * from "./js/statements/JSDoWhileStatement";
export * from "./js/statements/JSEmptyStatement";
export * from "./js/modules/JSExportAllDeclaration";
export * from "./js/modules/JSExportDefaultDeclaration";
export * from "./js/modules/JSExportDefaultSpecifier";
export * from "./js/modules/JSExportExternalDeclaration";
export * from "./js/modules/JSExportExternalSpecifier";
export * from "./js/modules/JSExportLocalDeclaration";
export * from "./js/modules/JSExportLocalSpecifier";
export * from "./js/modules/JSExportNamespaceSpecifier";
export * from "./js/statements/JSExpressionStatement";
export * from "./js/statements/JSForInStatement";
export * from "./js/statements/JSForOfStatement";
export * from "./js/statements/JSForStatement";
export * from "./js/statements/JSFunctionDeclaration";
export * from "./js/expressions/JSFunctionExpression";
export * from "./js/auxiliary/JSFunctionHead";
export * from "./js/auxiliary/JSIdentifier";
export * from "./js/statements/JSIfStatement";
export * from "./js/modules/JSImportCall";
export * from "./js/modules/JSImportDeclaration";
export * from "./js/modules/JSImportDefaultSpecifier";
export * from "./js/modules/JSImportNamespaceSpecifier";
export * from "./js/modules/JSImportSpecifier";
export * from "./js/modules/JSImportSpecifierLocal";
export * from "./js/core/JSInterpreterDirective";
export * from "./js/statements/JSLabeledStatement";
export * from "./js/expressions/JSLogicalExpression";
export * from "./js/expressions/JSMemberExpression";
export * from "./js/expressions/JSMetaProperty";
export * from "./js/expressions/JSNewExpression";
export * from "./js/literals/JSNullLiteral";
export * from "./js/literals/JSNumericLiteral";
export * from "./js/objects/JSObjectExpression";
export * from "./js/objects/JSObjectMethod";
export * from "./js/objects/JSObjectProperty";
export * from "./js/expressions/JSOptionalCallExpression";
export * from "./js/patterns/JSPatternMeta";
export * from "./js/classes/JSPrivateName";
export * from "./js/expressions/JSReferenceIdentifier";
export * from "./js/regex/JSRegExpAlternation";
export * from "./js/regex/JSRegExpAnyCharacter";
export * from "./js/regex/JSRegExpCharacter";
export * from "./js/regex/JSRegExpCharSet";
export * from "./js/regex/JSRegExpCharSetRange";
export * from "./js/regex/JSRegExpControlCharacter";
export * from "./js/regex/JSRegExpDigitCharacter";
export * from "./js/regex/JSRegExpEndCharacter";
export * from "./js/regex/JSRegExpGroupCapture";
export * from "./js/regex/JSRegExpGroupNonCapture";
export * from "./js/literals/JSRegExpLiteral";
export * from "./js/regex/JSRegExpNamedBackReference";
export * from "./js/regex/JSRegExpNonDigitCharacter";
export * from "./js/regex/JSRegExpNonWhiteSpaceCharacter";
export * from "./js/regex/JSRegExpNonWordBoundaryCharacter";
export * from "./js/regex/JSRegExpNonWordCharacter";
export * from "./js/regex/JSRegExpNumericBackReference";
export * from "./js/regex/JSRegExpQuantified";
export * from "./js/regex/JSRegExpStartCharacter";
export * from "./js/regex/JSRegExpSubExpression";
export * from "./js/regex/JSRegExpWhiteSpaceCharacter";
export * from "./js/regex/JSRegExpWordBoundaryCharacter";
export * from "./js/regex/JSRegExpWordCharacter";
export * from "./js/statements/JSReturnStatement";
export * from "./js/core/JSRoot";
export * from "./js/expressions/JSSequenceExpression";
export * from "./js/auxiliary/JSSpreadElement";
export * from "./js/objects/JSSpreadProperty";
export * from "./js/auxiliary/JSStaticMemberProperty";
export * from "./js/objects/JSStaticPropertyKey";
export * from "./js/literals/JSStringLiteral";
export * from "./js/expressions/JSSuper";
export * from "./js/auxiliary/JSSwitchCase";
export * from "./js/statements/JSSwitchStatement";
export * from "./js/expressions/JSTaggedTemplateExpression";
export * from "./js/auxiliary/JSTemplateElement";
export * from "./js/literals/JSTemplateLiteral";
export * from "./js/expressions/JSThisExpression";
export * from "./js/statements/JSThrowStatement";
export * from "./js/statements/JSTryStatement";
export * from "./js/expressions/JSUnaryExpression";
export * from "./js/expressions/JSUpdateExpression";
export * from "./js/auxiliary/JSVariableDeclaration";
export * from "./js/statements/JSVariableDeclarationStatement";
export * from "./js/auxiliary/JSVariableDeclarator";
export * from "./js/statements/JSWhileStatement";
export * from "./js/statements/JSWithStatement";
export * from "./js/jsx/JSXAttribute";
export * from "./js/jsx/JSXElement";
export * from "./js/jsx/JSXEmptyExpression";
export * from "./js/jsx/JSXExpressionContainer";
export * from "./js/jsx/JSXFragment";
export * from "./js/jsx/JSXIdentifier";
export * from "./js/jsx/JSXMemberExpression";
export * from "./js/jsx/JSXNamespacedName";
export * from "./js/jsx/JSXReferenceIdentifier";
export * from "./js/jsx/JSXSpreadAttribute";
export * from "./js/jsx/JSXSpreadChild";
export * from "./js/jsx/JSXText";
export * from "./js/expressions/JSYieldExpression";
export * from "./markdown/inline/MarkdownAutomaticLinkInline";
export * from "./markdown/inline/MarkdownBoldInline";
export * from "./markdown/blocks/MarkdownCodeBlock";
export * from "./markdown/inline/MarkdownCodeInline";
export * from "./markdown/inline/MarkdownDefinitionInline";
export * from "./markdown/blocks/MarkdownDividerBlock";
export * from "./markdown/inline/MarkdownEmphasisInline";
export * from "./markdown/blocks/MarkdownHeadingBlock";
export * from "./markdown/inline/MarkdownImageInline";
export * from "./markdown/inline/MarkdownLinkInline";
export * from "./markdown/blocks/MarkdownListBlock";
export * from "./markdown/core/MarkdownListItem";
export * from "./markdown/core/MarkdownParagraph";
export * from "./markdown/blocks/MarkdownQuoteBlock";
export * from "./markdown/core/MarkdownRoot";
export * from "./markdown/core/MarkdownText";
export * from "./common/core/MockParent";
export * from "./js/typescript/TSAnyKeywordTypeAnnotation";
export * from "./js/typescript/TSArrayType";
export * from "./js/typescript/TSAsExpression";
export * from "./js/typescript/TSAssignmentAsExpression";
export * from "./js/typescript/TSAssignmentNonNullExpression";
export * from "./js/typescript/TSAssignmentTypeAssertion";
export * from "./js/typescript/TSBigIntKeywordTypeAnnotation";
export * from "./js/typescript/TSBigIntLiteralTypeAnnotation";
export * from "./js/typescript/TSBooleanKeywordTypeAnnotation";
export * from "./js/typescript/TSBooleanLiteralTypeAnnotation";
export * from "./js/typescript/TSCallSignatureDeclaration";
export * from "./js/typescript/TSConditionalType";
export * from "./js/typescript/TSConstKeyword";
export * from "./js/typescript/TSConstructorType";
export * from "./js/typescript/TSConstructSignatureDeclaration";
export * from "./js/typescript/TSDeclareFunction";
export * from "./js/typescript/TSDeclareMethod";
export * from "./js/typescript/TSEmptyKeywordTypeAnnotation";
export * from "./js/typescript/TSEnumDeclaration";
export * from "./js/typescript/TSEnumMember";
export * from "./js/typescript/TSExportAssignment";
export * from "./js/typescript/TSExpressionWithTypeArguments";
export * from "./js/typescript/TSExternalModuleReference";
export * from "./js/typescript/TSFunctionType";
export * from "./js/typescript/TSImportEqualsDeclaration";
export * from "./js/typescript/TSImportType";
export * from "./js/typescript/TSIndexedAccessType";
export * from "./js/typescript/TSIndexSignature";
export * from "./js/typescript/TSInferType";
export * from "./js/typescript/TSInterfaceBody";
export * from "./js/typescript/TSInterfaceDeclaration";
export * from "./js/typescript/TSIntersectionTypeAnnotation";
export * from "./js/typescript/TSMappedType";
export * from "./js/typescript/TSMethodSignature";
export * from "./js/typescript/TSMixedKeywordTypeAnnotation";
export * from "./js/typescript/TSModuleBlock";
export * from "./js/typescript/TSModuleDeclaration";
export * from "./js/typescript/TSNamespaceExportDeclaration";
export * from "./js/typescript/TSNeverKeywordTypeAnnotation";
export * from "./js/typescript/TSNonNullExpression";
export * from "./js/typescript/TSNullKeywordTypeAnnotation";
export * from "./js/typescript/TSNumberKeywordTypeAnnotation";
export * from "./js/typescript/TSNumericLiteralTypeAnnotation";
export * from "./js/typescript/TSObjectKeywordTypeAnnotation";
export * from "./js/typescript/TSObjectTypeAnnotation";
export * from "./js/typescript/TSParenthesizedType";
export * from "./js/typescript/TSPropertySignature";
export * from "./js/typescript/TSQualifiedName";
export * from "./js/typescript/TSSignatureDeclarationMeta";
export * from "./js/typescript/TSStringKeywordTypeAnnotation";
export * from "./js/typescript/TSStringLiteralTypeAnnotation";
export * from "./js/typescript/TSSymbolKeywordTypeAnnotation";
export * from "./js/typescript/TSTemplateLiteralTypeAnnotation";
export * from "./js/typescript/TSThisType";
export * from "./js/typescript/TSTupleElement";
export * from "./js/typescript/TSTupleType";
export * from "./js/typescript/TSTypeAlias";
export * from "./js/typescript/TSTypeAssertion";
export * from "./js/typescript/TSTypeOperator";
export * from "./js/typescript/TSTypeParameter";
export * from "./js/typescript/TSTypeParameterDeclaration";
export * from "./js/typescript/TSTypeParameterInstantiation";
export * from "./js/typescript/TSTypePredicate";
export * from "./js/typescript/TSTypeQuery";
export * from "./js/typescript/TSTypeReference";
export * from "./js/typescript/TSUndefinedKeywordTypeAnnotation";
export * from "./js/typescript/TSUnionTypeAnnotation";
export * from "./js/typescript/TSUnknownKeywordTypeAnnotation";
export * from "./js/typescript/TSVoidKeywordTypeAnnotation";

export type AnyNode =
	| n.CommentBlock
	| n.CommentLine
	| n.CommitRoot
	| n.CSSAtRule
	| n.CSSBlock
	| n.CSSDeclaration
	| n.CSSDimension
	| n.CSSFunction
	| n.CSSIdentifier
	| n.CSSNumber
	| n.CSSPercentage
	| n.CSSRaw
	| n.CSSRoot
	| n.CSSRule
	| n.HTMLAttribute
	| n.HTMLDoctypeTag
	| n.HTMLElement
	| n.HTMLIdentifier
	| n.HTMLRoot
	| n.HTMLString
	| n.HTMLText
	| n.JSAmbiguousFlowTypeCastExpression
	| n.JSArrayExpression
	| n.JSArrayHole
	| n.JSArrowFunctionExpression
	| n.JSAssignmentArrayPattern
	| n.JSAssignmentAssignmentPattern
	| n.JSAssignmentExpression
	| n.JSAssignmentIdentifier
	| n.JSAssignmentObjectPattern
	| n.JSAssignmentObjectPatternProperty
	| n.JSAwaitExpression
	| n.JSBigIntLiteral
	| n.JSBinaryExpression
	| n.JSBindingArrayPattern
	| n.JSBindingAssignmentPattern
	| n.JSBindingIdentifier
	| n.JSBindingObjectPattern
	| n.JSBindingObjectPatternProperty
	| n.JSBlockStatement
	| n.JSBooleanLiteral
	| n.JSBreakStatement
	| n.JSCallExpression
	| n.JSCatchClause
	| n.JSClassDeclaration
	| n.JSClassExpression
	| n.JSClassHead
	| n.JSClassMethod
	| n.JSClassPrivateMethod
	| n.JSClassPrivateProperty
	| n.JSClassProperty
	| n.JSClassPropertyMeta
	| n.JSComputedMemberProperty
	| n.JSComputedPropertyKey
	| n.JSConditionalExpression
	| n.JSContinueStatement
	| n.JSDebuggerStatement
	| n.JSDirective
	| n.JSDoExpression
	| n.JSDoWhileStatement
	| n.JSEmptyStatement
	| n.JSExportAllDeclaration
	| n.JSExportDefaultDeclaration
	| n.JSExportDefaultSpecifier
	| n.JSExportExternalDeclaration
	| n.JSExportExternalSpecifier
	| n.JSExportLocalDeclaration
	| n.JSExportLocalSpecifier
	| n.JSExportNamespaceSpecifier
	| n.JSExpressionStatement
	| n.JSForInStatement
	| n.JSForOfStatement
	| n.JSForStatement
	| n.JSFunctionDeclaration
	| n.JSFunctionExpression
	| n.JSFunctionHead
	| n.JSIdentifier
	| n.JSIfStatement
	| n.JSImportCall
	| n.JSImportDeclaration
	| n.JSImportDefaultSpecifier
	| n.JSImportNamespaceSpecifier
	| n.JSImportSpecifier
	| n.JSImportSpecifierLocal
	| n.JSInterpreterDirective
	| n.JSLabeledStatement
	| n.JSLogicalExpression
	| n.JSMemberExpression
	| n.JSMetaProperty
	| n.JSNewExpression
	| n.JSNullLiteral
	| n.JSNumericLiteral
	| n.JSObjectExpression
	| n.JSObjectMethod
	| n.JSObjectProperty
	| n.JSOptionalCallExpression
	| n.JSPatternMeta
	| n.JSPrivateName
	| n.JSReferenceIdentifier
	| n.JSRegExpAlternation
	| n.JSRegExpAnyCharacter
	| n.JSRegExpCharacter
	| n.JSRegExpCharSet
	| n.JSRegExpCharSetRange
	| n.JSRegExpControlCharacter
	| n.JSRegExpDigitCharacter
	| n.JSRegExpEndCharacter
	| n.JSRegExpGroupCapture
	| n.JSRegExpGroupNonCapture
	| n.JSRegExpLiteral
	| n.JSRegExpNamedBackReference
	| n.JSRegExpNonDigitCharacter
	| n.JSRegExpNonWhiteSpaceCharacter
	| n.JSRegExpNonWordBoundaryCharacter
	| n.JSRegExpNonWordCharacter
	| n.JSRegExpNumericBackReference
	| n.JSRegExpQuantified
	| n.JSRegExpStartCharacter
	| n.JSRegExpSubExpression
	| n.JSRegExpWhiteSpaceCharacter
	| n.JSRegExpWordBoundaryCharacter
	| n.JSRegExpWordCharacter
	| n.JSReturnStatement
	| n.JSRoot
	| n.JSSequenceExpression
	| n.JSSpreadElement
	| n.JSSpreadProperty
	| n.JSStaticMemberProperty
	| n.JSStaticPropertyKey
	| n.JSStringLiteral
	| n.JSSuper
	| n.JSSwitchCase
	| n.JSSwitchStatement
	| n.JSTaggedTemplateExpression
	| n.JSTemplateElement
	| n.JSTemplateLiteral
	| n.JSThisExpression
	| n.JSThrowStatement
	| n.JSTryStatement
	| n.JSUnaryExpression
	| n.JSUpdateExpression
	| n.JSVariableDeclaration
	| n.JSVariableDeclarationStatement
	| n.JSVariableDeclarator
	| n.JSWhileStatement
	| n.JSWithStatement
	| n.JSXAttribute
	| n.JSXElement
	| n.JSXEmptyExpression
	| n.JSXExpressionContainer
	| n.JSXFragment
	| n.JSXIdentifier
	| n.JSXMemberExpression
	| n.JSXNamespacedName
	| n.JSXReferenceIdentifier
	| n.JSXSpreadAttribute
	| n.JSXSpreadChild
	| n.JSXText
	| n.JSYieldExpression
	| n.MarkdownAutomaticLinkInline
	| n.MarkdownBoldInline
	| n.MarkdownCodeBlock
	| n.MarkdownCodeInline
	| n.MarkdownDefinitionInline
	| n.MarkdownDividerBlock
	| n.MarkdownEmphasisInline
	| n.MarkdownHeadingBlock
	| n.MarkdownImageInline
	| n.MarkdownLinkInline
	| n.MarkdownListBlock
	| n.MarkdownListItem
	| n.MarkdownParagraph
	| n.MarkdownQuoteBlock
	| n.MarkdownRoot
	| n.MarkdownText
	| n.MockParent
	| n.TSAnyKeywordTypeAnnotation
	| n.TSArrayType
	| n.TSAsExpression
	| n.TSAssignmentAsExpression
	| n.TSAssignmentNonNullExpression
	| n.TSAssignmentTypeAssertion
	| n.TSBigIntKeywordTypeAnnotation
	| n.TSBigIntLiteralTypeAnnotation
	| n.TSBooleanKeywordTypeAnnotation
	| n.TSBooleanLiteralTypeAnnotation
	| n.TSCallSignatureDeclaration
	| n.TSConditionalType
	| n.TSConstKeyword
	| n.TSConstructorType
	| n.TSConstructSignatureDeclaration
	| n.TSDeclareFunction
	| n.TSDeclareMethod
	| n.TSEmptyKeywordTypeAnnotation
	| n.TSEnumDeclaration
	| n.TSEnumMember
	| n.TSExportAssignment
	| n.TSExpressionWithTypeArguments
	| n.TSExternalModuleReference
	| n.TSFunctionType
	| n.TSImportEqualsDeclaration
	| n.TSImportType
	| n.TSIndexedAccessType
	| n.TSIndexSignature
	| n.TSInferType
	| n.TSInterfaceBody
	| n.TSInterfaceDeclaration
	| n.TSIntersectionTypeAnnotation
	| n.TSMappedType
	| n.TSMethodSignature
	| n.TSMixedKeywordTypeAnnotation
	| n.TSModuleBlock
	| n.TSModuleDeclaration
	| n.TSNamespaceExportDeclaration
	| n.TSNeverKeywordTypeAnnotation
	| n.TSNonNullExpression
	| n.TSNullKeywordTypeAnnotation
	| n.TSNumberKeywordTypeAnnotation
	| n.TSNumericLiteralTypeAnnotation
	| n.TSObjectKeywordTypeAnnotation
	| n.TSObjectTypeAnnotation
	| n.TSParenthesizedType
	| n.TSPropertySignature
	| n.TSQualifiedName
	| n.TSSignatureDeclarationMeta
	| n.TSStringKeywordTypeAnnotation
	| n.TSStringLiteralTypeAnnotation
	| n.TSSymbolKeywordTypeAnnotation
	| n.TSTemplateLiteralTypeAnnotation
	| n.TSThisType
	| n.TSTupleElement
	| n.TSTupleType
	| n.TSTypeAlias
	| n.TSTypeAssertion
	| n.TSTypeOperator
	| n.TSTypeParameter
	| n.TSTypeParameterDeclaration
	| n.TSTypeParameterInstantiation
	| n.TSTypePredicate
	| n.TSTypeQuery
	| n.TSTypeReference
	| n.TSUndefinedKeywordTypeAnnotation
	| n.TSUnionTypeAnnotation
	| n.TSUnknownKeywordTypeAnnotation
	| n.TSVoidKeywordTypeAnnotation;
/* GENERATED:END(id:main) */
