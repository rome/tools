use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{map_syntax_node, SyntaxNode};

impl ToFormatElement for SyntaxNode {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        map_syntax_node!(
            self.clone(),
            node => node.to_format_element(formatter),
            _ => Ok(formatter.format_verbatim(self))
        )
    }
}

macro_rules! impl_format_verbatim {
    ( $( $node:ident, )* ) => {
        $( impl $crate::ToFormatElement for ::rslint_parser::ast::$node {
            fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                Ok(formatter.format_verbatim(::rslint_parser::AstNode::syntax(self)))
            }
        } )*
    };
}

impl_format_verbatim! {
    // Nodes
    JsExpressionSnipped,
    TsAnyType,
    TsArrayType,
    TsAsExpression,
    TsBigIntLiteralType,
    TsBigintType,
    TsBooleanLiteralType,
    TsBooleanType,
    TsCallSignatureObjectTypeMember,
    TsConditionalType,
    TsConstructSignatureObjectTypeMember,
    TsConstructorParam,
    TsConstructorType,
    TsDefaultTypeClause,
    TsEnum,
    TsEnumMember,
    TsExprWithTypeArgs,
    TsExternalModuleRef,
    TsFunctionType,
    TsGetterSignatureObjectTypeMember,
    TsIdentifierBinding,
    TsImplementsClause,
    TsImportEqualsDecl,
    TsImportType,
    TsImportTypeQualifier,
    TsIndexSignatureObjectTypeMember,
    TsIndexSignatureParameter,
    TsIndexedAccessType,
    TsInferType,
    TsIntersectionType,
    TsMappedType,
    TsMappedTypeAsClause,
    TsMappedTypeOptionalModifierClause,
    TsMappedTypeReadonlyModifierClause,
    TsMethodSignatureObjectTypeMember,
    TsNamedTupleTypeElement,
    TsNeverType,
    TsNonNull,
    TsNonPrimitiveType,
    TsNullLiteralType,
    TsNumberLiteralType,
    TsNumberType,
    TsObjectType,
    TsOptionalTupleTypeElement,
    TsParenthesizedType,
    TsPropertySignatureObjectTypeMember,
    TsQualifiedName,
    TsReferenceType,
    TsRestTupleTypeElement,
    TsReturnTypeAnnotation,
    TsSetterSignatureObjectTypeMember,
    TsStringLiteralType,
    TsStringType,
    TsSymbolType,
    TsTemplateChunkElement,
    TsTemplateElement,
    TsTemplateLiteralType,
    TsThisType,
    TsTupleType,
    TsTypeAlias,
    TsTypeAnnotation,
    TsTypeArguments,
    TsTypeAssertionExpression,
    TsTypeConstraintClause,
    TsTypeOperatorType,
    TsTypeParameter,
    TsTypeParameterName,
    TsTypeParameters,
    TsTypePredicate,
    TsTypeofType,
    TsUndefinedType,
    TsUnionType,
    TsUnknownType,
    TsVoidType,
    // Unknown
    JsUnknown,
    // Separated Lists
    JsArrayAssignmentPatternElementList,
    JsArrayBindingPatternElementList,
    JsArrayElementList,
    JsCallArgumentList,
    JsConstructorParameterList,
    JsExportNamedFromSpecifierList,
    JsExportNamedSpecifierList,
    JsImportAssertionEntryList,
    JsNamedImportSpecifierList,
    JsObjectAssignmentPatternPropertyList,
    JsObjectBindingPatternPropertyList,
    JsObjectMemberList,
    JsParameterList,
    JsVariableDeclarationList,
    TsIntersectionTypeElementList,
    TsAnyObjectTypeMember,
    TsAnyTemplateElement,
    TsTupleTypeElementList,
    TsTypeArgumentList,
    TsTypeList,
    TsTypeParameterList,
    TsUnionTypeVariantList,
}

// Non-separated lists
macro_rules! impl_format_list {
    ( $( $node:ident, )* ) => {
        $( impl $crate::ToFormatElement for ::rslint_parser::ast::$node {
            fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
                Ok(formatter.format_list(self.clone()))
            }
        } )*
    };
}

impl_format_list! {
    JsClassMemberList,
    JsDirectiveList,
    JsModuleItemList,
    JsStatementList,
    JsSwitchCaseList,
    JsTemplateElementList,
    TsEnumMemberList,
    TsObjectTypeMemberList,
    TsTemplateElementList,
}
