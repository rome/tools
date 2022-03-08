//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsType;
impl ToFormatElement for TsType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAnyType(node) => node.to_format_element(formatter),
            Self::TsUnknownType(node) => node.to_format_element(formatter),
            Self::TsNumberType(node) => node.to_format_element(formatter),
            Self::TsBooleanType(node) => node.to_format_element(formatter),
            Self::TsBigintType(node) => node.to_format_element(formatter),
            Self::TsStringType(node) => node.to_format_element(formatter),
            Self::TsSymbolType(node) => node.to_format_element(formatter),
            Self::TsVoidType(node) => node.to_format_element(formatter),
            Self::TsUndefinedType(node) => node.to_format_element(formatter),
            Self::TsNeverType(node) => node.to_format_element(formatter),
            Self::TsParenthesizedType(node) => node.to_format_element(formatter),
            Self::TsReferenceType(node) => node.to_format_element(formatter),
            Self::TsArrayType(node) => node.to_format_element(formatter),
            Self::TsTupleType(node) => node.to_format_element(formatter),
            Self::TsTypeofType(node) => node.to_format_element(formatter),
            Self::TsImportType(node) => node.to_format_element(formatter),
            Self::TsTypeOperatorType(node) => node.to_format_element(formatter),
            Self::TsIndexedAccessType(node) => node.to_format_element(formatter),
            Self::TsMappedType(node) => node.to_format_element(formatter),
            Self::TsObjectType(node) => node.to_format_element(formatter),
            Self::TsNonPrimitiveType(node) => node.to_format_element(formatter),
            Self::TsThisType(node) => node.to_format_element(formatter),
            Self::TsNumberLiteralType(node) => node.to_format_element(formatter),
            Self::TsBigIntLiteralType(node) => node.to_format_element(formatter),
            Self::TsStringLiteralType(node) => node.to_format_element(formatter),
            Self::TsNullLiteralType(node) => node.to_format_element(formatter),
            Self::TsBooleanLiteralType(node) => node.to_format_element(formatter),
            Self::TsTemplateLiteralType(node) => node.to_format_element(formatter),
            Self::TsInferType(node) => node.to_format_element(formatter),
            Self::TsIntersectionType(node) => node.to_format_element(formatter),
            Self::TsUnionType(node) => node.to_format_element(formatter),
            Self::TsFunctionType(node) => node.to_format_element(formatter),
            Self::TsConstructorType(node) => node.to_format_element(formatter),
            Self::TsConditionalType(node) => node.to_format_element(formatter),
        }
    }
}
