//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyReturnType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyReturnType;
impl FormatRule<TsAnyReturnType> for FormatTsAnyReturnType {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyReturnType::TsType(node) => node.format().fmt(f),
            TsAnyReturnType::TsPredicateReturnType(node) => node.format().fmt(f),
            TsAnyReturnType::TsAssertsReturnType(node) => node.format().fmt(f),
        }
    }
}
