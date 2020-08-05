/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import * as n from "../index";
import {OptionalProps} from "@internal/typescript-helpers";

export type AnyTSEntityName = n.JSReferenceIdentifier | n.TSQualifiedName;

export type AnyTSTypeElement =
	| n.TSCallSignatureDeclaration
	| n.TSConstructSignatureDeclaration
	| n.TSIndexSignature
	| n.TSPropertySignature
	| n.TSMethodSignature;

export type AnyTSModuleReference = AnyTSEntityName | n.TSExternalModuleReference;

export type JSObjectProperties = Array<
	n.JSObjectProperty | n.JSObjectMethod | n.JSSpreadProperty
>;

export type AnyJSFunction =
	| n.JSArrowFunctionExpression
	| n.JSFunctionDeclaration
	| n.JSFunctionExpression
	| n.JSObjectMethod
	| n.JSClassMethod;

export type AnyJSVariableIdentifier =
	| n.JSBindingIdentifier
	| n.JSAssignmentIdentifier
	| n.JSReferenceIdentifier
	| n.JSXReferenceIdentifier;

export type AnyJSObjectOrClassMember = AnyJSClassMember | AnyJSObjectMember;

export type AnyJSClassMember =
	| n.JSClassMethod
	| n.JSClassPrivateMethod
	| n.JSClassProperty
	| n.JSClassPrivateProperty
	| n.TSDeclareMethod
	| n.TSIndexSignature;

export type AnyJSObjectMember = n.JSObjectProperty | n.JSObjectMethod;

export type AnyJSObjectPropertyKey =
	| n.JSStaticPropertyKey
	| n.JSComputedPropertyKey;

export type AnyJSForStatement = AnyJSForInOfStatement | n.JSForStatement;

export type AnyJSForInOfStatement = n.JSForInStatement | n.JSForOfStatement;

export type AnyJSClass = n.JSClassDeclaration | n.JSClassExpression;

export type AnyJSAuxiliary =
	| AnyJSClassMember
	| AnyJSObjectMember
	| n.JSObjectProperty
	| n.JSObjectMethod
	| n.JSArrayHole
	| n.JSSpreadElement
	| n.JSSpreadProperty
	| n.JSXSpreadAttribute
	| n.JSXText
	| n.JSXNamespacedName
	| n.JSXSpreadChild
	| n.JSXExpressionContainer
	| n.JSXAttribute
	| n.JSImportSpecifier
	| n.JSImportDefaultSpecifier
	| n.JSImportNamespaceSpecifier
	| n.JSExportNamespaceSpecifier
	| n.JSDirective
	| n.JSInterpreterDirective
	| n.JSXReferenceIdentifier
	| AnyJSBindingPattern
	| AnyJSAssignmentPattern
	| n.JSIdentifier
	| n.JSVariableDeclaration
	| n.JSCatchClause
	| n.JSClassHead
	| n.JSClassPropertyMeta
	| n.JSComputedMemberProperty
	| n.JSComputedPropertyKey
	| n.AnyExportExternalSpecifier
	| n.JSExportLocalSpecifier
	| n.JSFunctionHead
	| n.JSPatternMeta
	| n.JSPrivateName
	| n.JSStaticMemberProperty
	| n.JSStaticPropertyKey
	| n.JSSwitchCase
	| n.JSTemplateElement
	| n.JSVariableDeclarator
	| n.JSAmbiguousFlowTypeCastExpression
	| n.MockParent;

export type AnyCommentOptionalId =
	| OptionalProps<n.CommentBlock, "id">
	| OptionalProps<n.CommentLine, "id">;

export type AnyJSIdentifier =
	| n.JSIdentifier
	| n.JSXIdentifier
	| n.JSBindingIdentifier
	| n.JSAssignmentIdentifier
	| n.JSReferenceIdentifier
	| n.JSXReferenceIdentifier;

export type AnyJSReference = n.JSReferenceIdentifier | n.JSMemberExpression;

export type AnyJSExpression =
	| n.JSReferenceIdentifier
	| n.JSXElement
	| n.JSXFragment
	| n.JSXMemberExpression
	| n.JSXEmptyExpression
	| n.JSXIdentifier
	| n.JSClassExpression
	| n.JSFunctionExpression
	| n.JSUnaryExpression
	| n.JSUpdateExpression
	| n.JSBinaryExpression
	| n.JSAssignmentExpression
	| n.JSLogicalExpression
	| n.JSMemberExpression
	| n.JSConditionalExpression
	| n.JSCallExpression
	| n.JSNewExpression
	| n.JSDoExpression
	| n.JSSequenceExpression
	| n.JSRegExpLiteral
	| n.JSNullLiteral
	| n.JSStringLiteral
	| n.JSBooleanLiteral
	| n.JSNumericLiteral
	| n.JSBigIntLiteral
	| n.JSSuper
	| n.JSImportCall
	| n.JSThisExpression
	| n.JSArrowFunctionExpression
	| n.JSYieldExpression
	| n.JSAwaitExpression
	| n.JSArrayExpression
	| n.JSObjectExpression
	| n.JSOptionalCallExpression
	| n.JSTemplateLiteral
	| n.JSTaggedTemplateExpression
	| n.JSMetaProperty
	| n.TSNonNullExpression
	| n.TSAsExpression
	| n.TSTypeAssertion;

export type AnyJSWhileStatement = n.JSWhileStatement | n.JSDoWhileStatement;

type AnyStatementWithBodyReducer<T> = T extends {
	readonly body: AnyJSStatement;
}
	? T
	: never;

export type AnyJSStatementWithBody = AnyStatementWithBodyReducer<AnyJSStatement>;

export type AnyJSStatement =
	| n.AnyJSDeclaration
	| n.JSExpressionStatement
	| n.JSForStatement
	| n.JSForOfStatement
	| n.JSForInStatement
	| n.JSWhileStatement
	| n.JSDoWhileStatement
	| n.JSBlockStatement
	| n.JSEmptyStatement
	| n.JSDebuggerStatement
	| n.JSReturnStatement
	| n.JSWithStatement
	| n.JSLabeledStatement
	| n.JSBreakStatement
	| n.JSContinueStatement
	| n.JSIfStatement
	| n.JSSwitchStatement
	| n.JSThrowStatement
	| n.JSTryStatement;

export type AnyJSBindingPattern =
	| n.JSBindingAssignmentPattern
	| n.JSBindingIdentifier
	| n.JSBindingObjectPattern
	| n.JSBindingArrayPattern
	| n.JSBindingObjectPatternProperty;

export type AnyJSParamBindingPattern =
	| n.AnyJSTargetBindingPattern
	| n.JSBindingAssignmentPattern;

export type AnyJSTargetBindingPattern =
	| n.JSBindingIdentifier
	| n.JSBindingArrayPattern
	| n.JSBindingObjectPattern;

export type AnyJSAssignmentPattern =
	| n.JSAssignmentAssignmentPattern
	| n.JSAssignmentObjectPatternProperty
	| n.AnyJSTargetAssignmentPattern;

export type AnyJSTargetAssignmentPattern =
	| n.JSMemberExpression
	| n.JSAssignmentIdentifier
	| n.JSAssignmentArrayPattern
	| n.JSAssignmentObjectPattern
	| n.TSAssignmentAsExpression
	| n.TSAssignmentNonNullExpression
	| n.TSAssignmentTypeAssertion;

export type AnyJSArrayPattern =
	| n.JSAssignmentArrayPattern
	| n.JSBindingArrayPattern;

export type AnyJSDeclaration =
	| n.JSVariableDeclarationStatement
	| n.JSClassDeclaration
	| n.JSFunctionDeclaration
	| n.JSImportDeclaration
	| n.JSExportLocalDeclaration
	| n.JSExportDefaultDeclaration
	| n.JSExportExternalDeclaration
	| n.JSExportAllDeclaration
	| n.TSTypeAlias
	| n.TSEnumDeclaration
	| n.TSInterfaceDeclaration
	| n.TSNamespaceExportDeclaration
	| n.TSExportAssignment
	| n.TSImportEqualsDeclaration
	| n.TSDeclareFunction
	| n.TSModuleDeclaration;

export type AnyTSLiteralTypeAnnotation =
	| n.TSStringLiteralTypeAnnotation
	| n.TSBooleanLiteralTypeAnnotation
	| n.TSNumericLiteralTypeAnnotation;

export type AnyTSKeywordTypeAnnotation =
	| n.TSAnyKeywordTypeAnnotation
	| n.TSBooleanKeywordTypeAnnotation
	| n.TSStringKeywordTypeAnnotation
	| n.TSBigIntKeywordTypeAnnotation
	| n.TSNeverKeywordTypeAnnotation
	| n.TSNumberKeywordTypeAnnotation
	| n.TSObjectKeywordTypeAnnotation
	| n.TSSymbolKeywordTypeAnnotation
	| n.TSUndefinedKeywordTypeAnnotation
	| n.TSUnknownKeywordTypeAnnotation;

export type AnyTSPrimary =
	| n.TSObjectTypeAnnotation
	| n.TSTypeReference
	| n.TSThisType
	| n.TSParenthesizedType
	| n.TSArrayType
	| n.TSIndexedAccessType
	| n.TSTupleType
	| n.TSTypeQuery
	| n.TSMappedType
	| n.TSTypePredicate
	| n.TSTypeOperator
	| n.TSInferType
	| n.TSUnionTypeAnnotation
	| n.TSConditionalType
	| n.TSFunctionType
	| n.TSImportType
	| n.TSConstructorType
	| n.TSIntersectionTypeAnnotation
	| n.TSTemplateLiteralTypeAnnotation
	| n.TSConstKeyword
	| AnyTSLiteralTypeAnnotation
	| AnyTSKeywordTypeAnnotation;

export type AnyJSRegExpEscapedCharacter =
	| n.JSRegExpCharacter
	| n.JSRegExpDigitCharacter
	| n.JSRegExpNonDigitCharacter
	| n.JSRegExpWordBoundaryCharacter
	| n.JSRegExpNamedBackReference
	| n.JSRegExpNonWordBoundaryCharacter
	| n.JSRegExpNumericBackReference
	| n.JSRegExpWhiteSpaceCharacter
	| n.JSRegExpNonWhiteSpaceCharacter
	| n.JSRegExpWordCharacter
	| n.JSRegExpNonWordCharacter
	| n.JSRegExpControlCharacter;

export type AnyJSRegExpBodyItem =
	| AnyJSRegExpEscapedCharacter
	| n.JSRegExpStartCharacter
	| n.JSRegExpEndCharacter
	| n.JSRegExpAnyCharacter
	| n.JSRegExpDigitCharacter
	| n.JSRegExpNonDigitCharacter
	| n.JSRegExpWhiteSpaceCharacter
	| n.JSRegExpNonWhiteSpaceCharacter
	| n.JSRegExpWordCharacter
	| n.JSRegExpNonWordCharacter
	| n.JSRegExpWordBoundaryCharacter
	| n.JSRegExpNonWordBoundaryCharacter
	| n.JSRegExpControlCharacter
	| n.JSRegExpCharacter
	| n.JSRegExpQuantified
	| n.JSRegExpGroupCapture
	| n.JSRegExpCharSet
	| n.JSRegExpGroupNonCapture
	| n.JSRegExpSubExpression
	| n.JSRegExpAlternation;

export type AnyJSRegExpExpression =
	| n.JSRegExpSubExpression
	| n.JSRegExpAlternation;
