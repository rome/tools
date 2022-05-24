//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyReturnType;
use crate::prelude::*;
use rome_js_syntax::TsAnyReturnType;
impl FormatRule<TsAnyReturnType> for FormatTsAnyReturnType {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyReturnType,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyReturnType::TsType(node) => formatted![formatter, [node.format()]],
            TsAnyReturnType::TsPredicateReturnType(node) => formatted![formatter, [node.format()]],
            TsAnyReturnType::TsAssertsReturnType(node) => formatted![formatter, [node.format()]],
        }
    }
}
