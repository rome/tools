//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsType;
use crate::prelude::*;
use rome_js_syntax::TsType;
impl FormatRule<TsType> for FormatTsType {
    type Context = JsFormatContext;
    fn format(node: &TsType, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            TsType::TsAnyType(node) => formatted![formatter, [node.format()]],
            TsType::TsUnknownType(node) => formatted![formatter, [node.format()]],
            TsType::TsNumberType(node) => formatted![formatter, [node.format()]],
            TsType::TsBooleanType(node) => formatted![formatter, [node.format()]],
            TsType::TsBigintType(node) => formatted![formatter, [node.format()]],
            TsType::TsStringType(node) => formatted![formatter, [node.format()]],
            TsType::TsSymbolType(node) => formatted![formatter, [node.format()]],
            TsType::TsVoidType(node) => formatted![formatter, [node.format()]],
            TsType::TsUndefinedType(node) => formatted![formatter, [node.format()]],
            TsType::TsNeverType(node) => formatted![formatter, [node.format()]],
            TsType::TsParenthesizedType(node) => formatted![formatter, [node.format()]],
            TsType::TsReferenceType(node) => formatted![formatter, [node.format()]],
            TsType::TsArrayType(node) => formatted![formatter, [node.format()]],
            TsType::TsTupleType(node) => formatted![formatter, [node.format()]],
            TsType::TsTypeofType(node) => formatted![formatter, [node.format()]],
            TsType::TsImportType(node) => formatted![formatter, [node.format()]],
            TsType::TsTypeOperatorType(node) => formatted![formatter, [node.format()]],
            TsType::TsIndexedAccessType(node) => formatted![formatter, [node.format()]],
            TsType::TsMappedType(node) => formatted![formatter, [node.format()]],
            TsType::TsObjectType(node) => formatted![formatter, [node.format()]],
            TsType::TsNonPrimitiveType(node) => formatted![formatter, [node.format()]],
            TsType::TsThisType(node) => formatted![formatter, [node.format()]],
            TsType::TsNumberLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsBigIntLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsStringLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsNullLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsBooleanLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsTemplateLiteralType(node) => formatted![formatter, [node.format()]],
            TsType::TsInferType(node) => formatted![formatter, [node.format()]],
            TsType::TsIntersectionType(node) => formatted![formatter, [node.format()]],
            TsType::TsUnionType(node) => formatted![formatter, [node.format()]],
            TsType::TsFunctionType(node) => formatted![formatter, [node.format()]],
            TsType::TsConstructorType(node) => formatted![formatter, [node.format()]],
            TsType::TsConditionalType(node) => formatted![formatter, [node.format()]],
        }
    }
}
