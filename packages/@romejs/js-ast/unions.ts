/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import * as n from './index';

export type AnyTSEntityName = n.ReferenceIdentifier | n.TSQualifiedName;

export type AnyTSTypeElement =
  | n.TSCallSignatureDeclaration
  | n.TSConstructSignatureDeclaration
  | n.TSIndexSignature
  | n.TSPropertySignature
  | n.TSMethodSignature;

export type AnyTSModuleReference = AnyTSEntityName | n.TSExternalModuleReference;

export type AnyTSFunctionOrConstructorType =
  | n.TSFunctionType
  | n.TSConstructorType;

export type AnyTypeParameter =
  | n.TSTypeParameterDeclaration
  | n.FlowTypeParameterDeclaration;

export type AnyTypeArguments =
  | n.FlowTypeParameterInstantiation
  | n.TSTypeParameterInstantiation;

export type ObjectProperties = Array<
  | n.ObjectProperty
  | n.ObjectMethod
  | n.SpreadProperty>;

export type AnyFunction =
  | n.ArrowFunctionExpression
  | n.FunctionDeclaration
  | n.FunctionExpression
  | n.ObjectMethod
  | n.ClassMethod;

export type AnyVariableIdentifier =
  | n.BindingIdentifier
  | n.AssignmentIdentifier
  | n.ReferenceIdentifier
  | n.JSXReferenceIdentifier;

export type AnyObjectOrClassMember = AnyClassMember | AnyObjectMember;

export type AnyClassMember =
  | n.ClassMethod
  | n.ClassPrivateMethod
  | n.ClassProperty
  | n.ClassPrivateProperty
  | n.TSDeclareMethod
  | n.TSIndexSignature;

export type AnyObjectMember = n.ObjectProperty | n.ObjectMethod;

export type AnyObjectPropertyKey = n.StaticPropertyKey | n.ComputedPropertyKey;

export type AnyForStatement = AnyForInOfStatement | n.ForStatement;

export type AnyForInOfStatement = n.ForInStatement | n.ForOfStatement;

export type AnyLiteral =
  | n.StringLiteral
  | n.BooleanLiteral
  | n.NumericLiteral
  | n.RegExpLiteral
  | n.NullLiteral;

export type AnyClass = n.ClassDeclaration | n.ClassExpression;

export type AnyFlowPredicate = n.FlowInferredPredicate | n.FlowDeclaredPredicate;

export type AnyFlowDeclare =
  | n.FlowDeclareClass
  | n.FlowDeclareFunction
  | n.FlowDeclareVariable
  | n.FlowDeclareModuleExports
  | n.FlowDeclareModule
  | n.FlowDeclareOpaqueType
  | n.FlowDeclareInterface
  | n.FlowDeclareExportDefault
  | n.FlowDeclareExportNamed
  | n.FlowDeclareExportAll
  | n.TypeAliasTypeAnnotation;

export type AnyAuxiliary =
  | AnyClassMember
  | AnyObjectMember
  | n.ObjectProperty
  | n.ObjectMethod
  | n.SpreadElement
  | n.SpreadProperty
  | n.JSXSpreadAttribute
  | n.JSXText
  | n.JSXNamespacedName
  | n.JSXSpreadChild
  | n.JSXExpressionContainer
  | n.JSXAttribute
  | n.ImportSpecifier
  | n.ImportDefaultSpecifier
  | n.ImportNamespaceSpecifier
  | n.ExportLocalSpecifier
  | n.ExportNamespaceSpecifier
  | n.Directive
  | n.InterpreterDirective
  | n.JSXReferenceIdentifier
  | AnyBindingPattern
  | AnyAssignmentPattern
  | AnyComment
  | n.Identifier
  | n.VariableDeclaration
  | n.CatchClause
  | n.ClassHead
  | n.ClassPropertyMeta
  | n.ComputedMemberProperty
  | n.ComputedPropertyKey
  | n.AnyExportExternalSpecifier
  | n.ExportLocalSpecifier
  | n.FunctionHead
  | n.PatternMeta
  | n.PrivateName
  | n.StaticMemberProperty
  | n.StaticPropertyKey
  | n.SwitchCase
  | n.TemplateElement
  | n.VariableDeclarator
  | n.AmbiguousFlowTypeCastExpression
  | n.MockParent;

export type AnyComment = n.CommentBlock | n.CommentLine;

export type AnyIdentifier =
  | n.Identifier
  | n.JSXIdentifier
  | n.BindingIdentifier
  | n.AssignmentIdentifier
  | n.ReferenceIdentifier
  | n.JSXReferenceIdentifier;

export type AnyReference = n.ReferenceIdentifier | n.MemberExpression;

export type AnyExpression =
  | n.ReferenceIdentifier
  | n.JSXElement
  | n.JSXFragment
  | n.JSXMemberExpression
  | n.JSXEmptyExpression
  | n.JSXIdentifier
  | n.ClassExpression
  | n.FunctionExpression
  | n.UnaryExpression
  | n.UpdateExpression
  | n.BinaryExpression
  | n.AssignmentExpression
  | n.LogicalExpression
  | n.MemberExpression
  | n.ConditionalExpression
  | n.CallExpression
  | n.NewExpression
  | n.DoExpression
  | n.SequenceExpression
  | n.RegExpLiteral
  | n.NullLiteral
  | n.StringLiteral
  | n.BooleanLiteral
  | n.NumericLiteral
  | n.BigIntLiteral
  | n.Super
  | n.ImportCall
  | n.ThisExpression
  | n.ArrowFunctionExpression
  | n.YieldExpression
  | n.AwaitExpression
  | n.ArrayExpression
  | n.ObjectExpression
  | n.OptionalCallExpression
  | n.TemplateLiteral
  | n.TaggedTemplateExpression
  | n.MetaProperty
  | n.FlowTypeCastExpression
  | n.TSNonNullExpression
  | n.TSAsExpression
  | n.TSTypeAssertion;

export type AnyWhileStatement = n.WhileStatement | n.DoWhileStatement;

type AnyStatementWithBodyReducer<T> = T extends {body: AnyStatement} ? T : never;

export type AnyStatementWithBody = AnyStatementWithBodyReducer<AnyStatement>;

export type AnyStatement =
  | n.AnyDeclaration
  | n.ExpressionStatement
  | n.ForStatement
  | n.ForOfStatement
  | n.ForInStatement
  | n.WhileStatement
  | n.DoWhileStatement
  | n.BlockStatement
  | n.EmptyStatement
  | n.DebuggerStatement
  | n.ReturnStatement
  | n.WithStatement
  | n.LabeledStatement
  | n.BreakStatement
  | n.ContinueStatement
  | n.IfStatement
  | n.SwitchStatement
  | n.ThrowStatement
  | n.TryStatement;

export type AnyBindingPattern =
  | n.BindingAssignmentPattern
  | n.BindingIdentifier
  | n.BindingObjectPattern
  | n.BindingArrayPattern
  | n.BindingObjectPatternProperty;

export type AnyParamBindingPattern =
  | n.AnyTargetBindingPattern
  | n.BindingAssignmentPattern;

export type AnyTargetBindingPattern =
  | n.BindingIdentifier
  | n.BindingArrayPattern
  | n.BindingObjectPattern;

export type AnyAssignmentPattern =
  | n.AssignmentAssignmentPattern
  | n.AssignmentObjectPatternProperty
  | n.AnyTargetAssignmentPattern;

export type AnyTargetAssignmentPattern =
  | n.MemberExpression
  | n.AssignmentIdentifier
  | n.AssignmentArrayPattern
  | n.AssignmentObjectPattern
  | n.TSAssignmentAsExpression
  | n.TSAssignmentNonNullExpression
  | n.TSAssignmentTypeAssertion;

export type AnyDeclaration =
  | n.VariableDeclarationStatement
  | n.ClassDeclaration
  | n.FunctionDeclaration
  | n.ImportDeclaration
  | n.ExportLocalDeclaration
  | n.ExportDefaultDeclaration
  | n.ExportExternalDeclaration
  | n.ExportAllDeclaration
  | n.TypeAliasTypeAnnotation
  | n.FlowDeclareVariable
  | n.FlowDeclareOpaqueType
  | n.FlowOpaqueType
  | n.FlowInterfaceDeclaration
  | n.FlowInterface
  | n.FlowInterfaceDeclaration
  | n.TSEnumDeclaration
  | n.TypeAliasTypeAnnotation
  | n.TSInterfaceDeclaration
  | n.TSNamespaceExportDeclaration
  | n.TSExportAssignment
  | n.TSImportEqualsDeclaration
  | n.TSDeclareFunction
  | n.TSModuleDeclaration
  | AnyFlowDeclare;

export type AnyFlowPrimary =
  | n.FlowFunctionTypeAnnotation
  | n.FlowTupleTypeAnnotation
  | n.FlowObjectTypeAnnotation
  | n.FlowTypeofTypeAnnotation
  | n.IntersectionTypeAnnotation
  | n.FlowInterfaceTypeAnnotation
  | n.UnionTypeAnnotation
  | n.FlowNullableTypeAnnotation
  | n.IntersectionTypeAnnotation
  | n.UnionTypeAnnotation
  | n.FlowArrayTypeAnnotation
  | n.IntersectionTypeAnnotation
  | n.FlowNullableTypeAnnotation
  | n.UnionTypeAnnotation
  | n.NullKeywordTypeAnnotation
  | n.FlowThisTypeAnnotation
  | n.FlowExistsTypeAnnotation
  | n.FlowGenericTypeAnnotation
  | AnyLiteralTypeAnnotation
  | AnyFlowKeywordTypeAnnotation;

export type AnyLiteralTypeAnnotation =
  | n.StringLiteralTypeAnnotation
  | n.BooleanLiteralTypeAnnotation
  | n.NumericLiteralTypeAnnotation;

// Keyword type annotations that both Flow and TS share
export type AnySharedKeywordTypeAnnotation =
  | n.AnyKeywordTypeAnnotation
  | n.BooleanKeywordTypeAnnotation
  | n.NumberKeywordTypeAnnotation
  | n.StringKeywordTypeAnnotation
  | n.BigIntKeywordTypeAnnotation;

export type AnyFlowKeywordTypeAnnotation =
  | AnySharedKeywordTypeAnnotation
  | n.VoidKeywordTypeAnnotation
  | n.MixedKeywordTypeAnnotation
  | n.EmptyKeywordTypeAnnotation;

export type AnyTSKeywordTypeAnnotation =
  | AnySharedKeywordTypeAnnotation
  | n.NeverKeywordTypeAnnotation
  | n.NumberKeywordTypeAnnotation
  | n.ObjectKeywordTypeAnnotation
  | n.SymbolKeywordTypeAnnotation
  | n.UndefinedKeywordTypeAnnotation
  | n.UnknownKeywordTypeAnnotation;

export type AnyTSPrimary =
  | n.TSOptionalType
  | n.TSTypeLiteral
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
  | n.UnionTypeAnnotation
  | n.TSConditionalType
  | n.TSFunctionType
  | n.TSImportType
  | n.TSConstructorType
  | n.IntersectionTypeAnnotation
  | n.TemplateLiteralTypeAnnotation
  | AnyLiteralTypeAnnotation
  | AnyTSKeywordTypeAnnotation;

export type AnyPrimaryType = AnyFlowPrimary | AnyTSPrimary;

export type AnyRegExpEscapedCharacter =
  | n.RegExpCharacter
  | n.RegExpDigitCharacter
  | n.RegExpNonDigitCharacter
  | n.RegExpWordBoundaryCharacter
  | n.RegExpNonWordBoundaryCharacter
  | n.RegExpNumericBackReference
  | n.RegExpWhiteSpaceCharacter
  | n.RegExpNonWhiteSpaceCharacter
  | n.RegExpWordCharacter
  | n.RegExpNonWordCharacter
  | n.RegExpControlCharacter;

export type AnyRegExpBodyItem =
  | AnyRegExpEscapedCharacter
  | n.RegExpStartCharacter
  | n.RegExpEndCharacter
  | n.RegExpAnyCharacter
  | n.RegExpDigitCharacter
  | n.RegExpNonDigitCharacter
  | n.RegExpWhiteSpaceCharacter
  | n.RegExpNonWhiteSpaceCharacter
  | n.RegExpWordCharacter
  | n.RegExpNonWordCharacter
  | n.RegExpWordBoundaryCharacter
  | n.RegExpNonWordBoundaryCharacter
  | n.RegExpControlCharacter
  | n.RegExpCharacter
  | n.RegExpQuantified
  | n.RegExpGroupCapture
  | n.RegExpCharSet
  | n.RegExpGroupNonCapture
  | n.RegExpSubExpression
  | n.RegExpAlternation;

export type AnyRegExpExpression = n.RegExpSubExpression | n.RegExpAlternation;

// EVERYTHING BELOW IS AUTOGENERATED. SEE SCRIPTS FOLDER FOR UPDATE SCRIPTS
export type AnyNode =
  | n.AmbiguousFlowTypeCastExpression
  | n.AnyKeywordTypeAnnotation
  | n.ArrayExpression
  | n.ArrowFunctionExpression
  | n.AssignmentArrayPattern
  | n.AssignmentAssignmentPattern
  | n.AssignmentExpression
  | n.AssignmentIdentifier
  | n.AssignmentObjectPattern
  | n.AssignmentObjectPatternProperty
  | n.AwaitExpression
  | n.BigIntKeywordTypeAnnotation
  | n.BigIntLiteral
  | n.BinaryExpression
  | n.BindingArrayPattern
  | n.BindingAssignmentPattern
  | n.BindingIdentifier
  | n.BindingObjectPattern
  | n.BindingObjectPatternProperty
  | n.BlockStatement
  | n.BooleanKeywordTypeAnnotation
  | n.BooleanLiteral
  | n.BooleanLiteralTypeAnnotation
  | n.BreakStatement
  | n.CallExpression
  | n.CatchClause
  | n.ClassDeclaration
  | n.ClassExpression
  | n.ClassHead
  | n.ClassMethod
  | n.ClassPrivateMethod
  | n.ClassPrivateProperty
  | n.ClassProperty
  | n.ClassPropertyMeta
  | n.CommentBlock
  | n.CommentLine
  | n.ComputedMemberProperty
  | n.ComputedPropertyKey
  | n.ConditionalExpression
  | n.ContinueStatement
  | n.DebuggerStatement
  | n.Directive
  | n.DoExpression
  | n.DoWhileStatement
  | n.EmptyKeywordTypeAnnotation
  | n.EmptyStatement
  | n.ExportAllDeclaration
  | n.ExportDefaultDeclaration
  | n.ExportDefaultSpecifier
  | n.ExportExternalDeclaration
  | n.ExportExternalSpecifier
  | n.ExportLocalDeclaration
  | n.ExportLocalSpecifier
  | n.ExportNamespaceSpecifier
  | n.ExpressionStatement
  | n.FlowArrayTypeAnnotation
  | n.FlowClassImplements
  | n.FlowDeclareClass
  | n.FlowDeclaredPredicate
  | n.FlowDeclareExportAll
  | n.FlowDeclareExportDefault
  | n.FlowDeclareExportNamed
  | n.FlowDeclareFunction
  | n.FlowDeclareInterface
  | n.FlowDeclareModule
  | n.FlowDeclareModuleExports
  | n.FlowDeclareOpaqueType
  | n.FlowDeclareVariable
  | n.FlowExistsTypeAnnotation
  | n.FlowFunctionTypeAnnotation
  | n.FlowFunctionTypeParam
  | n.FlowGenericTypeAnnotation
  | n.FlowInferredPredicate
  | n.FlowInterface
  | n.FlowInterfaceDeclaration
  | n.FlowInterfaceExtends
  | n.FlowInterfaceTypeAnnotation
  | n.FlowNullableTypeAnnotation
  | n.FlowNumericLiteral
  | n.FlowObjectTypeAnnotation
  | n.FlowObjectTypeCallProperty
  | n.FlowObjectTypeIndexer
  | n.FlowObjectTypeInternalSlot
  | n.FlowObjectTypeProperty
  | n.FlowObjectTypeSpreadProperty
  | n.FlowOpaqueType
  | n.FlowQualifiedTypeIdentifier
  | n.FlowThisTypeAnnotation
  | n.FlowTupleTypeAnnotation
  | n.FlowTypeCastExpression
  | n.FlowTypeofTypeAnnotation
  | n.FlowTypeParameter
  | n.FlowTypeParameterDeclaration
  | n.FlowTypeParameterInstantiation
  | n.FlowVariance
  | n.ForInStatement
  | n.ForOfStatement
  | n.ForStatement
  | n.FunctionDeclaration
  | n.FunctionExpression
  | n.FunctionHead
  | n.Identifier
  | n.IfStatement
  | n.ImportCall
  | n.ImportDeclaration
  | n.ImportDefaultSpecifier
  | n.ImportNamespaceSpecifier
  | n.ImportSpecifier
  | n.ImportSpecifierLocal
  | n.InterpreterDirective
  | n.IntersectionTypeAnnotation
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
  | n.LabeledStatement
  | n.LogicalExpression
  | n.MemberExpression
  | n.MetaProperty
  | n.MixedKeywordTypeAnnotation
  | n.MockParent
  | n.NeverKeywordTypeAnnotation
  | n.NewExpression
  | n.NullKeywordTypeAnnotation
  | n.NullLiteral
  | n.NumberKeywordTypeAnnotation
  | n.NumericLiteral
  | n.NumericLiteralTypeAnnotation
  | n.ObjectExpression
  | n.ObjectKeywordTypeAnnotation
  | n.ObjectMethod
  | n.ObjectProperty
  | n.OptionalCallExpression
  | n.PatternMeta
  | n.PrivateName
  | n.Program
  | n.ReferenceIdentifier
  | n.RegExpAlternation
  | n.RegExpAnyCharacter
  | n.RegExpCharacter
  | n.RegExpCharSet
  | n.RegExpCharSetRange
  | n.RegExpControlCharacter
  | n.RegExpDigitCharacter
  | n.RegExpEndCharacter
  | n.RegExpGroupCapture
  | n.RegExpGroupNonCapture
  | n.RegExpLiteral
  | n.RegExpNonDigitCharacter
  | n.RegExpNonWhiteSpaceCharacter
  | n.RegExpNonWordBoundaryCharacter
  | n.RegExpNonWordCharacter
  | n.RegExpNumericBackReference
  | n.RegExpQuantified
  | n.RegExpStartCharacter
  | n.RegExpSubExpression
  | n.RegExpWhiteSpaceCharacter
  | n.RegExpWordBoundaryCharacter
  | n.RegExpWordCharacter
  | n.ReturnStatement
  | n.SequenceExpression
  | n.SpreadElement
  | n.SpreadProperty
  | n.StaticMemberProperty
  | n.StaticPropertyKey
  | n.StringKeywordTypeAnnotation
  | n.StringLiteral
  | n.StringLiteralTypeAnnotation
  | n.Super
  | n.SwitchCase
  | n.SwitchStatement
  | n.SymbolKeywordTypeAnnotation
  | n.TaggedTemplateExpression
  | n.TemplateElement
  | n.TemplateLiteral
  | n.TemplateLiteralTypeAnnotation
  | n.ThisExpression
  | n.ThrowStatement
  | n.TryStatement
  | n.TSArrayType
  | n.TSAsExpression
  | n.TSAssignmentAsExpression
  | n.TSAssignmentNonNullExpression
  | n.TSAssignmentTypeAssertion
  | n.TSCallSignatureDeclaration
  | n.TSConditionalType
  | n.TSConstructorType
  | n.TSConstructSignatureDeclaration
  | n.TSDeclareFunction
  | n.TSDeclareMethod
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
  | n.TSMappedType
  | n.TSMethodSignature
  | n.TSModuleBlock
  | n.TSModuleDeclaration
  | n.TSNamespaceExportDeclaration
  | n.TSNonNullExpression
  | n.TSOptionalType
  | n.TSParenthesizedType
  | n.TSPropertySignature
  | n.TSQualifiedName
  | n.TSSignatureDeclarationMeta
  | n.TSThisType
  | n.TSTupleType
  | n.TSTypeAssertion
  | n.TSTypeLiteral
  | n.TSTypeOperator
  | n.TSTypeParameter
  | n.TSTypeParameterDeclaration
  | n.TSTypeParameterInstantiation
  | n.TSTypePredicate
  | n.TSTypeQuery
  | n.TSTypeReference
  | n.TypeAliasTypeAnnotation
  | n.UnaryExpression
  | n.UndefinedKeywordTypeAnnotation
  | n.UnionTypeAnnotation
  | n.UnknownKeywordTypeAnnotation
  | n.UpdateExpression
  | n.VariableDeclaration
  | n.VariableDeclarationStatement
  | n.VariableDeclarator
  | n.VoidKeywordTypeAnnotation
  | n.WhileStatement
  | n.WithStatement
  | n.YieldExpression;
