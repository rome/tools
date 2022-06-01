//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyReturnType;
use crate::prelude::*;
use rome_js_syntax::TsAnyReturnType;
impl FormatRule<TsAnyReturnType> for FormatTsAnyReturnType {
    type Context = JsFormatContext;
    fn format(node: &TsAnyReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyReturnType::TsType(node) => node.format().format(f),
            TsAnyReturnType::TsPredicateReturnType(node) => node.format().format(f),
            TsAnyReturnType::TsAssertsReturnType(node) => node.format().format(f),
        }
    }
}
