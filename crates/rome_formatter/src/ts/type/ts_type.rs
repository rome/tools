use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsType;

impl ToFormatElement for TsType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            TsType::TsAnyType(node) => node.to_format_element(formatter),
            TsType::TsArrayType(_) => todo!(),
            TsType::TsBigIntLiteralType(node) => node.to_format_element(formatter),
            TsType::TsBigintType(node) => node.to_format_element(formatter),
            TsType::TsBooleanLiteralType(node) => node.to_format_element(formatter),
            TsType::TsBooleanType(node) => node.to_format_element(formatter),
            TsType::TsConditionalType(_) => todo!(),
            TsType::TsConstructorType(_) => todo!(),
            TsType::TsFunctionType(_) => todo!(),
            TsType::TsImportType(_) => todo!(),
            TsType::TsIndexedAccessType(_) => todo!(),
            TsType::TsInferType(node) => node.to_format_element(formatter),
            TsType::TsIntersectionType(_) => todo!(),
            TsType::TsMappedType(_) => todo!(),
            TsType::TsNeverType(node) => node.to_format_element(formatter),
            TsType::TsNonPrimitiveType(node) => node.to_format_element(formatter),
            TsType::TsNullLiteralType(node) => node.to_format_element(formatter),
            TsType::TsNumberLiteralType(node) => node.to_format_element(formatter),
            TsType::TsNumberType(node) => node.to_format_element(formatter),
            TsType::TsObjectType(node) => node.to_format_element(formatter),
            TsType::TsParenthesizedType(_) => todo!(),
            TsType::TsReferenceType(node) => node.to_format_element(formatter),
            TsType::TsStringLiteralType(node) => node.to_format_element(formatter),
            TsType::TsStringType(node) => node.to_format_element(formatter),
            TsType::TsSymbolType(node) => node.to_format_element(formatter),
            TsType::TsTemplateLiteralType(_) => todo!(),
            TsType::TsThisType(node) => node.to_format_element(formatter),
            TsType::TsTupleType(_) => todo!(),
            TsType::TsTypeOperatorType(_) => todo!(),
            TsType::TsTypeofType(node) => node.to_format_element(formatter),
            TsType::TsUndefinedType(node) => node.to_format_element(formatter),
            TsType::TsUnionType(_) => todo!(),
            TsType::TsUnknownType(node) => node.to_format_element(formatter),
            TsType::TsVoidType(node) => node.to_format_element(formatter),
        }
    }
}
