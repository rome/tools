//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsType;
impl FormatRule<TsType> for FormatTsType {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsType::TsAnyType(node) => node.format().fmt(f),
            TsType::TsUnknownType(node) => node.format().fmt(f),
            TsType::TsNumberType(node) => node.format().fmt(f),
            TsType::TsBooleanType(node) => node.format().fmt(f),
            TsType::TsBigintType(node) => node.format().fmt(f),
            TsType::TsStringType(node) => node.format().fmt(f),
            TsType::TsSymbolType(node) => node.format().fmt(f),
            TsType::TsVoidType(node) => node.format().fmt(f),
            TsType::TsUndefinedType(node) => node.format().fmt(f),
            TsType::TsNeverType(node) => node.format().fmt(f),
            TsType::TsParenthesizedType(node) => node.format().fmt(f),
            TsType::TsReferenceType(node) => node.format().fmt(f),
            TsType::TsArrayType(node) => node.format().fmt(f),
            TsType::TsTupleType(node) => node.format().fmt(f),
            TsType::TsTypeofType(node) => node.format().fmt(f),
            TsType::TsImportType(node) => node.format().fmt(f),
            TsType::TsTypeOperatorType(node) => node.format().fmt(f),
            TsType::TsIndexedAccessType(node) => node.format().fmt(f),
            TsType::TsMappedType(node) => node.format().fmt(f),
            TsType::TsObjectType(node) => node.format().fmt(f),
            TsType::TsNonPrimitiveType(node) => node.format().fmt(f),
            TsType::TsThisType(node) => node.format().fmt(f),
            TsType::TsNumberLiteralType(node) => node.format().fmt(f),
            TsType::TsBigIntLiteralType(node) => node.format().fmt(f),
            TsType::TsStringLiteralType(node) => node.format().fmt(f),
            TsType::TsNullLiteralType(node) => node.format().fmt(f),
            TsType::TsBooleanLiteralType(node) => node.format().fmt(f),
            TsType::TsTemplateLiteralType(node) => node.format().fmt(f),
            TsType::TsInferType(node) => node.format().fmt(f),
            TsType::TsIntersectionType(node) => node.format().fmt(f),
            TsType::TsUnionType(node) => node.format().fmt(f),
            TsType::TsFunctionType(node) => node.format().fmt(f),
            TsType::TsConstructorType(node) => node.format().fmt(f),
            TsType::TsConditionalType(node) => node.format().fmt(f),
            TsType::TsBogusType(node) => node.format().fmt(f),
        }
    }
}
