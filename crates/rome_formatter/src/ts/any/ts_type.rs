//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsType;
impl ToFormatElement for TsType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAnyType(node) => node.format(formatter),
            Self::TsUnknownType(node) => node.format(formatter),
            Self::TsNumberType(node) => node.format(formatter),
            Self::TsBooleanType(node) => node.format(formatter),
            Self::TsBigintType(node) => node.format(formatter),
            Self::TsStringType(node) => node.format(formatter),
            Self::TsSymbolType(node) => node.format(formatter),
            Self::TsVoidType(node) => node.format(formatter),
            Self::TsUndefinedType(node) => node.format(formatter),
            Self::TsNeverType(node) => node.format(formatter),
            Self::TsParenthesizedType(node) => node.format(formatter),
            Self::TsReferenceType(node) => node.format(formatter),
            Self::TsArrayType(node) => node.format(formatter),
            Self::TsTupleType(node) => node.format(formatter),
            Self::TsTypeofType(node) => node.format(formatter),
            Self::TsImportType(node) => node.format(formatter),
            Self::TsTypeOperatorType(node) => node.format(formatter),
            Self::TsIndexedAccessType(node) => node.format(formatter),
            Self::TsMappedType(node) => node.format(formatter),
            Self::TsObjectType(node) => node.format(formatter),
            Self::TsNonPrimitiveType(node) => node.format(formatter),
            Self::TsThisType(node) => node.format(formatter),
            Self::TsNumberLiteralType(node) => node.format(formatter),
            Self::TsBigIntLiteralType(node) => node.format(formatter),
            Self::TsStringLiteralType(node) => node.format(formatter),
            Self::TsNullLiteralType(node) => node.format(formatter),
            Self::TsBooleanLiteralType(node) => node.format(formatter),
            Self::TsTemplateLiteralType(node) => node.format(formatter),
            Self::TsInferType(node) => node.format(formatter),
            Self::TsIntersectionType(node) => node.format(formatter),
            Self::TsUnionType(node) => node.format(formatter),
            Self::TsFunctionType(node) => node.format(formatter),
            Self::TsConstructorType(node) => node.format(formatter),
            Self::TsConditionalType(node) => node.format(formatter),
        }
    }
}
