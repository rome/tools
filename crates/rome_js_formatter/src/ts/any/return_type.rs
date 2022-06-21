//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyReturnType;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyReturnType;
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
