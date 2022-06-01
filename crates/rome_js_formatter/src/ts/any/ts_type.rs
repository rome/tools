//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsType;
use crate::prelude::*;
use rome_js_syntax::TsType;
impl FormatRule<TsType> for FormatTsType {
    type Context = JsFormatContext;
    fn format(node: &TsType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsType::TsAnyType(node) => node.format().format(f),
            TsType::TsUnknownType(node) => node.format().format(f),
            TsType::TsNumberType(node) => node.format().format(f),
            TsType::TsBooleanType(node) => node.format().format(f),
            TsType::TsBigintType(node) => node.format().format(f),
            TsType::TsStringType(node) => node.format().format(f),
            TsType::TsSymbolType(node) => node.format().format(f),
            TsType::TsVoidType(node) => node.format().format(f),
            TsType::TsUndefinedType(node) => node.format().format(f),
            TsType::TsNeverType(node) => node.format().format(f),
            TsType::TsParenthesizedType(node) => node.format().format(f),
            TsType::TsReferenceType(node) => node.format().format(f),
            TsType::TsArrayType(node) => node.format().format(f),
            TsType::TsTupleType(node) => node.format().format(f),
            TsType::TsTypeofType(node) => node.format().format(f),
            TsType::TsImportType(node) => node.format().format(f),
            TsType::TsTypeOperatorType(node) => node.format().format(f),
            TsType::TsIndexedAccessType(node) => node.format().format(f),
            TsType::TsMappedType(node) => node.format().format(f),
            TsType::TsObjectType(node) => node.format().format(f),
            TsType::TsNonPrimitiveType(node) => node.format().format(f),
            TsType::TsThisType(node) => node.format().format(f),
            TsType::TsNumberLiteralType(node) => node.format().format(f),
            TsType::TsBigIntLiteralType(node) => node.format().format(f),
            TsType::TsStringLiteralType(node) => node.format().format(f),
            TsType::TsNullLiteralType(node) => node.format().format(f),
            TsType::TsBooleanLiteralType(node) => node.format().format(f),
            TsType::TsTemplateLiteralType(node) => node.format().format(f),
            TsType::TsInferType(node) => node.format().format(f),
            TsType::TsIntersectionType(node) => node.format().format(f),
            TsType::TsUnionType(node) => node.format().format(f),
            TsType::TsFunctionType(node) => node.format().format(f),
            TsType::TsConstructorType(node) => node.format().format(f),
            TsType::TsConditionalType(node) => node.format().format(f),
        }
    }
}
